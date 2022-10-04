// use crate::vimdoc::*;
use crate::library::*;
use xmltree::Element;
use xmltree::XMLNode;
use core::fmt;
use std::default::default;
use std::io::Read;
use std::str::FromStr;

fn read_return(e: &Element) -> Option<Parameter> {
    let ret = e.get_child("return-value")?;
    read_param(ret)
}

pub fn attr_from_str<T>(e: &Element, name: &str) -> Result<Option<T>, String>
where
    T: FromStr,
    T::Err: fmt::Display,
{
    if let Some(value_str) = e.attributes.get(name) {
        match T::from_str(value_str) {
            Ok(value) => Ok(Some(value)),
            Err(error) => {
                let message = format!(
                    "Attribute `{}` on element <{}> has invalid value: {}",
                    name,
                    e.name,
                    error
                );
                Err(message)
            }
        }
    } else {
        Ok(None)
    }
}

impl FromStr for ParameterScope {
    type Err = String;

    fn from_str(name: &str) -> Result<ParameterScope, String> {
        match name {
            "call" => Ok(ParameterScope::Call),
            "async" => Ok(ParameterScope::Async),
            "notified" => Ok(ParameterScope::Notified),
            _ => Err(format!("Unknown parameter scope type: {}", name)),
        }
    }
}

fn read_type(e: &Element, is_array: bool) -> Option<(String, Option<String>, Option<u32>)> {
    if is_array {
        let typ = e.get_child("type")?.attributes.get("name")?.to_string();
        let ctype = e.attributes.get("c:type").map(|x| x.to_string());
        let length = e.attributes.get("length").map(|x| x.parse().unwrap_or(1));
        return Some((typ, ctype, length))
    }
    let typ = e.attributes.get("name")?.to_string();
    let ctype = e.attributes.get("c:type").map(|x| x.to_string());
    Some((typ, ctype, None))
}

fn get_attribute(e: &Element, attr: &str) -> Option<String> {
    e.attributes.get(attr).map(|x| x.to_string())
}

fn read_param(e: &Element) -> Option<Parameter> {
    let argname = get_attribute(e, "name").unwrap_or(String::new());

    let instance_parameter = e.name == "instance-parameter";
    let transfer = attr_from_str(e, "transfer-ownership").ok().flatten().unwrap_or(Transfer::None);

    let nullable = attribute_bool(e, "nullable", false);
    let allow_none = attribute_bool(e, "allow-none", false);

    let scope = attr_from_str(e, "scope").ok().flatten().unwrap_or(ParameterScope::None);

    let closure = attribute_bool(e, "closure", false);
    let destroy = attribute_bool(e, "destroy", false);
    let caller_allocates = attribute_bool(e, "caller-allocates", false);

    let direction = if e.name == "return-value" {
        Ok(ParameterDirection::Return)
    } else {
        ParameterDirection::from_str(e.attr("direction").unwrap_or("in")).ok()
    }?;

    let mut typ = None;
    let mut varargs = false;
    let mut doc = None;


    for node in e.children {
        let e = node.as_element()?;
        match e.name.as_ref() {
            "type" | "array" => {
                if typ.is_some() {
                    return None;
                }
                typ = read_type(e, e.name == "array");
            }
            "varargs" => {
                varargs = true;
            }
            "doc" => doc = get_doc(e),
            "attribute" => {}
            _ => panic!("Error parsing param")
        }
    }

    if let Some((typ, c_type, array_length)) = typ {
        return Some(Parameter{
            name: argname.to_string(),
            typ,
            c_type,
            instance_parameter,
            direction,
            transfer,
            vararg: varargs,
            caller_allocates,
            nullable,
            allow_none,
            array_length,
            doc,
            scope,
            closure,
            destroy,
        })
    }
    None
}

fn read_params(e: &Element) -> Option<Vec<Parameter>> {
    let mut ret: Vec<Parameter> = vec![];
    let parameters = e.get_child("parameters")?;

    for parameter in parameters.children.iter() {
        if let Some(e) = parameter.as_element() {
            if e.name == "parameter" {
                let para = read_param(e);
                if let Some(para) = para {
                    ret.push(para);
                }
            }
        }
    }
    return Some(ret)
}

fn read_inner_doc(e: &Element) -> Option<String> {
    e.get_child("doc").map(|doc| get_doc(doc)).flatten()
}
fn read_inner_doc_depricated(e: &Element) -> Option<String> {
    e.get_child("doc-deprecated").map(|doc| get_doc(doc)).flatten()
}

fn get_inne_name<'a>(e: &'a Element, name: &'a str) -> Option<&'a String> {
    e.get_child(name).map(|doc| doc.attributes.get("name")).flatten()
}

fn get_inner_type(e: &Element) -> Option<&String> {
    e.get_child("type").map(|doc| doc.attributes.get("name")).flatten()
}

fn get_introspectable(e: &Element, def: bool) -> bool {
    e.attributes.get("introspectable").map(|x| x == "1").unwrap_or(def)
}

fn attribute_bool(e: &Element, attr: &str, def: bool) -> bool {
    e.attributes.get(attr).map(|x| x == "1").unwrap_or(def)
}

fn read_signal(e: &Element) -> Option<Function> {
    let name = e.attributes.get("name")?;

    let ret = read_return(&e)?;
    let parameters = read_params(&e).unwrap_or(vec![]);
    let introspectable = get_introspectable(e, true);

    let is_action = attribute_bool(e, "action", false);
    let is_detailed = attribute_bool(e, "detailed", false);


    let version = e.attributes.get("version");
    let deprecated = e.attributes.get("deprecated");
    let deprecated_version = e.attributes.get("deprecated-version");

    let introspectable = get_introspectable(e, true);

    let doc = read_inner_doc(&e);
    let doc_deprecated = read_inner_doc_depricated(&e);

    let ret = read_return(&e);
    let parameters = read_params(&e).unwrap_or(vec![]);

    Some(Signal { 
        name,
        introspectable,
        parameters,
        ret,
        is_action,
        is_detailed,
        version,
        deprecated_version,
        doc,
        doc_deprecated
    })
}

fn read_callable(e: &Element) -> Option<Function> {
    let name = e.attributes.get("name")?;
    let c_identifier = e.attributes.get("c:identifier")?;
    let version = e.attributes.get("version");
    let deprecated = e.attributes.get("deprecated");
    let deprecated_version = e.attributes.get("deprecated-version");
    let throws = e.attribbutes.get("throws").map(|x| x == "1").unwrap_or(false);

    let introspectable = get_introspectable(e, true);

    let doc = read_inner_doc(e);
    let doc_deprecated = read_inner_doc_depricated(e);
    let ret = read_return(e);
    let parameters = read_params(e).unwrap_or(vec![]);

    Some(Function { 
        name,
        c_identifier,
        introspectable,
        parameters,
        ret,
        throws,
        version,
        deprecated_version,
        doc,
        doc_deprecated
    })
}

fn get_doc(e: &Element) -> Option<String> {
    e.get_text().map(|x| x.into())
}


fn read_field(e: &Element) -> Option<Field> {
    let name = e.attributes.get("name")?;
    let private = e.attributes.get("private").unwrap_or(false);
    let introspectable = get_introspectable(e, false);
    let bits = e.attributes.get("bits").and_then(|s| s.parse().ok());
    let doc = read_inner_doc(e)?;
    let Some(typ, ctype, _) = read_type(e, false);
    Some((name.to_string(), doc))
}

fn read_property(e: &Element) -> Option<Property> {
    None
}

fn get_class(e: &Element) -> Option<Class> {
    let name = e.attributes.get("name")?;

    let version = e.attributes.get("version");
    let doc_deprecated = e.attributes.get("depricated-version");
    let c_type = e.attributes.get("c:type")?;
    let symbol_prefix = e.attributes.get("c:symbol-prefix")?;
    let introspectable = get_introspectable(e, true);

    let mut class = Class::new(name);
    for node in e.children.iter() {
        if let Some(e) = node.as_element() {
            match e.name.as_str() {
                "doc" => {
                    let docs = get_doc(&e);
                    class.doc = docs;
                }
                "doc" => {
                    let docs = get_doc(&e);
                    class.doc = docs;
                }
                "doc-deprecated" => {
                    let docs = get_doc(&e);
                    class.doc_deprecated = docs;
                }
                "field" => {
                    if let Some(field) = read_field(e) {
                        class.fields.push(field)
                    }
                }
                "source-position" => {
                }
                "constructor" => {
                    if let Some(fun) = read_callable(e) {
                        class.constructor.push(fun)
                    }
                }
                "function" => {
                    if let Some(fun) = read_callable(e) {
                        class.functions.push(fun)
                    }
                }
                "method" => {
                    if let Some(fun) = read_callable(e) {
                        class.method.push(fun)
                    }
                }
                "virtual-method" => { 
                    if let Some(fun) = read_callable(e) {
                        class.virt.push(fun)
                    }
                }
                "property" => {
                    if let Some(prop) = read_property(e) {
                        class.properties.push(prop)
                    }
                }
                "signal" => {
                    if let Some(fun) = read_signal(e) {
                        class.signals.push(fun)
                    }
                }
                "implements" => {
                    if let Some((typ, ctype, _)) = read_type(e, false) {
                        class.implements.push(typ)
                    }
                }
                name => {
                    panic!("Name: {} not matched against\n", name)
                }
            }
        }
    }
    class
}

fn read_constant(e: &Element) -> Option<Constant> {
    let name = e.attributes.get("name")?.to_string();
    let c_identifier = e.attributes.get("type")?.to_string();
    let value = e.attributes.get("value")?.to_string();
    let introspectable = get_introspectable(e, true);

    let version = e.attributes.get("version").map(|x| x.to_string());
    let deprecated_version = e.attributes.get("deprecated-version").map(|x| x.to_string());

    let mut inner = None;
    let mut doc = None;
    let mut doc_deprecated = None;

    let mut typ = None;
    for node in e.children {
        let e = node.as_element()?;
        match e.name.as_ref() {
            "type" | "array" => {
                if typ.is_some() {
                    return None;
                }
                typ = read_type(e, e.name == "array");
            }
            "doc" => doc = get_doc(e),
            "doc-deprecated" => doc_deprecated = get_doc(e),
            "attribute" => {}
            "source-position" => {}
            "attribute" => {}
            _ => panic!("Error parsing param")
        }
    }
    if let Some((typ, c_type, _)) = typ {
        return Some(Constant {
            name,
            c_identifier,
            introspectable,
            typ,
            c_type,
            value,
            version,
            deprecated_version,
            doc,
            doc_deprecated
        })
    }
    None
}

fn read_bitfield(e: &Element) -> Option<Comp> {
    let name = e.attributes.get("name")?;
    let mut fields = vec![];
    let doc = read_inner_doc(e).unwrap_or("".to_string());
    for node in e.children.iter() {
        if let Some(e) = node.as_element() {
            if e.name == "member" {
                if let Some(id) = e.attributes.get("name") {
                    let inner_doc = read_inner_doc(e).unwrap_or("".to_string());
                    fields.push((id.to_owned(), inner_doc))
                }
            }
        }
    }
    Some(Comp {
        name: name.to_string(),
        doc,
        members: fields,
    })
}

fn read_namespace(e: &Element) -> Option<Namespace> {
    let name = e.attributes.get("name")?;
    // let package_name = e.attributes.get("package-name");
    let version = e.attributes.get("version")?;


    let mut ns = Namespace::new(
        name,
        version,
        );

    if let Some(s) = e.attributes.get("shared-library") {
        ns.shared_library = s.split(',').map(String::from).collect();
    }
    if let Some(s) = e.attributes.get("identifier-prefixes") {
        ns.identifier_prefixes = s.split(',').map(String::from).collect();
    }
    if let Some(s) = e.attributes.get("symbol-prefixes") {
        ns.symbol_prefixes = s.split(',').map(String::from).collect();
    }
    for node in e.children.iter() {
        if let Some(e) = node.as_element() {
            match e.name.as_str() {
                "class" => {
                    let class = get_class(&e);
                    ns.classes.push(class);
                }
                "function" => {
                    if let Some(fun) = read_callable(e) {
                        ns.functions.push(fun);
                    }
                }
                "function-macro" => {
                    if let Some(fun) = read_callable(e) {
                        ns.macros.push(fun);
                    }
                }
                "enumeration" => {
                    if let Some(enu) = read_enum(e) {
                        ns.enums.push(enu);
                    }
                }
                "record" => {
                    if let Some(record) = read_record(e) {
                        ns.record.push(record);
                    }
                }
                "constant" => {
                    if let Some(consts) = read_constant(e) {
                        ns.constant.push(consts);
                    }
                }
                "callback" => {
                    if let Some(cb) = read_callable(e) {
                        ns.callback.push(cb);
                    }
                }
                "bitfield" => {
                    if let Some(bf) = read_bitfield(e) {
                        ns.bitfield.push(bf)
                    }
                }
                "docsection" => {
                    // println!("{:#?}", e)
                    ns.doc = get_doc(e)
                }
                "name" => {
                    println!("{:#?}", e)
                }
                "alias" => {
                    // println!("{:#?}", e)
                }
                "interface" => {
                }
                "boxed" => {
                }
                name => {
                    panic!("Name: {} not matched against\n", name)
                }
            }
        }
    }
    Some(ns)
}

// should return repo?
fn read_repository(node: &XMLNode) -> Option<Namespace> {
    if let Some(e) = node.as_element() {
        match e.name.as_ref() {
            "include" => {}
            "package" => {}
            "namespace" => {
                return read_namespace(e)
            }
            "attribute" => { }
            name => {
                panic!("Name: {} not matched against\n", name)
            }
        }
    }
    None
}

// TODO, atm we only handle one namespace
pub fn parse_gir<R: Read>(read: R) -> Result<Namespace, xmltree::ParseError> {
    let names_element = Element::parse(read)?;

    for child in names_element.children.into_iter() {
        let doc = read_repository(&child);
        if let Some(doc) = doc {
            return Ok(doc)
        }
    }
    Err(xmltree::ParseError::CannotParse)
}
