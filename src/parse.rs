// use crate::vimdoc::*;
use crate::library::*;
use xmltree::Element;
use core::fmt;
// use std::any::Any;
// use std::default::default;
use std::io::Read;
use std::str::FromStr;

fn read_return(e: &Element) -> Option<Parameter> {
    let ret = e.get_child("return-value")?;
    read_param(ret)
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

impl FromStr for ParameterDirection {
    type Err = String;
    fn from_str(name: &str) -> Result<ParameterDirection, String> {
        use self::ParameterDirection::*;
        match name {
            "in" => Ok(In),
            "out" => Ok(Out),
            "inout" => Ok(InOut),
            _ => Err(format!("Unknown parameter direction '{}'", name)),
        }
    }
}

impl FromStr for Transfer {
    type Err = String;
    fn from_str(name: &str) -> Result<Transfer, String> {
        use self::Transfer::*;
        match name {
            "container" => Ok(Container),
            "full" => Ok(Full),
            _ => Err(format!("Unknown parameter direction '{}'", name)),
        }
    }
}
fn read_type(e: &Element, is_array: bool) -> Option<(String, Option<String>, Option<u32>)> {
    if is_array {
        let typ = e.get_child("type")?.attributes.get("name")?.to_string();
       let ctype = e.attributes.get("type").map(|x| x.to_string());
        let length = e.attributes.get("length").map(|x| x.parse().unwrap_or(1));
        return Some((typ, ctype, length))
    }
    let typ = e.attributes.get("name")?.to_string();
    let ctype = e.attributes.get("type").map(|x| x.to_string());
    Some((typ, ctype, None))
}

fn get_attribute(e: &Element, attr: &str) -> Option<String> {
    e.attributes.get(attr).map(|x| x.to_string())
}

fn read_anytype(e: &Element) -> Option<AnyType> {
    for node in e.children.iter() {
        if let Some(e) = node.as_element() {
            match e.name.as_str() {
                "type" => { 
                    let name = attribute(e, "name");
                    let ctype = attribute(e, "type");
                    let introspectable = attr_bool(e, "introspectable");
                    let doc = read_infoelements(e)?;
                    let mut children = vec![];
                    for node in e.children.iter() {
                        if let Some(e) = node.as_element() {
                            if let Some(typ) = read_anytype(e) {
                                children.push(typ)
                            }
                        }
                    }
                    return Some(AnyType::Type(Type{
                        name,
                        ctype,
                        introspectable,
                        doc,
                        children,
                    }))
                }
                "array" => {
                    let name = attribute(e, "name");
                    let zero_terminated = attr_bool(e, "zero-terminated");
                    let fixed_size = attr_bool(e, "fixed-size");
                    let introspectable = attr_bool(e, "introspectable");
                    let length = attr_value(e, "length");
                    let ctype = attribute(e, "type");
                    for node in e.children.iter() {
                        if let Some(e) = node.as_element() {
                            let typ = read_anytype(e)?;
                            let btyp = Box::new(typ);
                            return Some(AnyType::Array(Array{
                                name,
                                zero_terminated,
                                fixed_size,
                                introspectable,
                                length,
                                ctype,
                                typ: btyp,
                            }))
                        }
                    }
                }
                "varargs" => {
                    return Some(AnyType::VarArg)
                }
                _ => {}
            }
        }
    }
    None
}

fn read_param(e: &Element) -> Option<Parameter> {
    let name = attribute(e, "name")?;
    let nullable = attribute(e, "nullable");
    let allow_none = attribute(e, "allow-none");
    let introspectable = attribute(e, "introspectable");
    let closure = attribute(e, "closure");
    let destroy = attribute(e, "destroy");
    let scope = attribute(e, "scope");
    let direction = attribute(e, "direction");
    let caller_allocates = attribute(e, "caller-allocates");
    let optional = attribute(e, "optional");
    let skip = attribute(e, "skip");
    let transfer = attr_value(e, "transfer");
    let doc = read_infoelements(e)?;
    let typ = read_anytype(e)?;

    Some(Parameter{
        name,
        nullable,
        allow_none,
        introspectable,
        closure,
        destroy,
        scope,
        direction,
        caller_allocates,
        optional,
        skip,
        transfer,
        doc,
        typ,
    })
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

// fn read_inner_doc(e: &Element) -> Option<String> {
//     e.get_child("doc").map(|doc| get_doc(doc)).flatten()
// }
// fn read_inner_doc_depricated(e: &Element) -> Option<String> {
//     e.get_child("doc-deprecated").map(|doc| get_doc(doc)).flatten()
// }
//
// fn get_inne_name<'a>(e: &'a Element, name: &'a str) -> Option<&'a String> {
//     e.get_child(name).map(|doc| doc.attributes.get("name")).flatten()
// }
//
// fn get_inner_type(e: &Element) -> Option<&String> {
//     e.get_child("type").map(|doc| doc.attributes.get("name")).flatten()
// }
//
// fn get_introspectable(e: &Element, def: bool) -> bool {
//     e.attributes.get("introspectable").map(|x| x == "1").unwrap_or(def)
// }
//
// fn attribute_bool(e: &Element, attr: &str, def: bool) -> bool {
//     e.attributes.get(attr).map(|x| x == "1").unwrap_or(def)
// }
//
// fn read_signal(e: &Element) -> Option<Function> {
//     let name = e.attributes.get("name")?;
//
//     let ret = read_return(&e)?;
//     let parameters = read_params(&e).unwrap_or(vec![]);
//     let introspectable = get_introspectable(e, true);
//
//     let is_action = attribute_bool(e, "action", false);
//     let is_detailed = attribute_bool(e, "detailed", false);
//
//
//     let version = e.attributes.get("version");
//     let deprecated = e.attributes.get("deprecated");
//     let deprecated_version = e.attributes.get("deprecated-version");
//
//     let introspectable = get_introspectable(e, true);
//
//     let doc = read_inner_doc(&e);
//     let doc_deprecated = read_inner_doc_depricated(&e);
//
//     let ret = read_return(&e);
//     let parameters = read_params(&e).unwrap_or(vec![]);
//
//     Some(Signal { 
//         name,
//         introspectable,
//         parameters,
//         ret,
//         is_action,
//         is_detailed,
//         version,
//         deprecated_version,
//         doc,
//         doc_deprecated
//     })
// }

fn read_function(e: &Element) -> Option<Function> {
    let name = attribute(e, "name")?;
    let info = read_infoattrs(e)?;
    let doc = read_infoelements(e)?;

    let c_identifier = attribute(e, "identifier");
    let shadowed_by = attribute(e, "shadowed-by");
    let shadows = attribute(e, "shadows");
    let throws = attr_bool(e, "throws");
    let moved_to = attribute(e, "moved-to");

    let ret = read_return(e);
    let parameters = read_params(e).unwrap_or(vec![]);

    Some(Function {
        info,
        doc,
        name,
        c_identifier,
        shadowed_by,
        shadows,
        throws,
        moved_to,
        parameters,
        ret,
    })
}

fn get_doc(e: &Element) -> Option<Doc> {
    let preserve_space = attribute(e, "space");
    let preserve_white  = attribute(e, "whitespace");
    let filename = attribute(e, "filename")?;
    let line = attribute(e, "line")?;
    let column = attribute(e, "column");
    let content = e.get_text().map(|x| x.into())?;
    Some(Doc{
        preserve_space,
        preserve_white,
        filename,
        line,
        column,
        content,
    })
}
fn get_doc_versioned(e: &Element) -> Option<DocVersioned> {
    let preserve_space = attribute(e, "space");
    let preserve_white  = attribute(e, "whitespace");
    let content = e.get_text().map(|x| x.into())?;
    Some(DocVersioned{
        preserve_space,
        preserve_white,
        content,
    })
}

fn get_source_position(e: &Element) -> Option<DocPosition> {
    let filename = attribute(e, "filename")?;
    let line = attribute(e, "line")?;
    let column = attribute(e, "column");
    Some(DocPosition{
        filename,
        line,
        column,
    })
}

//
// fn read_field(e: &Element) -> Option<Field> {
//     let name = e.attributes.get("name")?;
//     let private = e.attributes.get("private").unwrap_or(false);
//     let introspectable = get_introspectable(e, false);
//     let bits = e.attributes.get("bits").and_then(|s| s.parse().ok());
//     let doc = read_inner_doc(e)?;
//     let Some(typ, ctype, _) = read_type(e, false);
//     Some((name.to_string(), doc))
// }

fn read_property(e: &Element) -> Option<Property> {
    None
}

pub fn attr_bool(e: &Element, name: &str) -> Option<bool>
{
    if let Some(value_str) = e.attributes.get(name) {
        match value_str.as_str() {
            "0" => Some(false),
            "1" => Some(true),
            _ => None
        }
    } else {
        None
    }
}

fn read_infoattrs(e: &Element) -> Option<InfoAttrs> {
    let introspectable = attr_bool(e, "introspectable");
    let deprecated = attribute(e, "deprecated");
    let deprecated_version = attribute(e, "deprecated-version");
    let version = attribute(e, "version");
    let stability = attribute(e, "stability");
    Some(InfoAttrs {
        introspectable,
        deprecated,
        deprecated_version,
        version,
        stability,
    })
}


fn read_infoelements(e: &Element) -> Option<InfoElements> {
    let mut doc = None;
    let mut doc_stability = None;
    let mut doc_version = None;
    let mut doc_deprecated = None;
    let mut doc_pos = None;
    for node in e.children.iter() {
        if let Some(e) = node.as_element() {
            match e.name.as_str() {
                "doc" => {
                    let docs = get_doc(&e);
                    doc = docs;
                }
                "doc-stability" => {
                    let docs = get_doc_versioned(&e);
                    doc_stability = docs;
                }
                "doc-deprecated" => {
                    let docs = get_doc_versioned(&e);
                    doc_deprecated = docs;
                }
                "doc-version" => {
                    let docs = get_doc_versioned(&e);
                    doc_version = docs;
                }
                "source-position" => {
                    let docs = get_source_position(&e);
                    doc_pos = docs;
                }
                _ => {}
            }
        }
    }
    Some(InfoElements{
        doc,
        doc_stability,
        doc_version,
        doc_deprecated,
        doc_pos,
    })
}

fn read_class(e: &Element) -> Option<Class> {
    let name = attribute(e, "name")?;
    let glib_type_name = attribute(e, "type-name")?;
    let glib_get_type = attribute(e, "get-type")?;
    let parent = attribute(e, "parent");
    let glib_type_struct = attribute(e, "type-stuct");
    let ref_func = attribute(e, "ref-func");
    let unref_func = attribute(e, "unref-func");
    let set_value_func = attribute(e, "set-value-func");
    let get_value_func = attribute(e, "get-value-func");
    let ctype = attribute(e, "type");
    let symbol_prefix = attribute(e, "symbol-prefix");
    let abstracts = attribute(e, "abstract");
    let glib_fundamental = attribute(e, "fundamental");
    let finals = attribute(e, "final");

    let info = read_infoattrs(e)?;

    let mut constructor = vec![];
    let mut functions = vec![];
    let mut method = vec![];
    let mut virtual_method = vec![];
    let mut callbacks = vec![];

    let doc = read_infoelements(e)?;

    for node in e.children.iter() {
        if let Some(e) = node.as_element() {
            match e.name.as_str() {
                "constructor" => {
                    if let Some(fun) = read_function(e) {
                        constructor.push(fun)
                    }
                }
                "function" => {
                    if let Some(fun) = read_function(e) {
                        functions.push(fun)
                    }
                }
                "method" => {
                    if let Some(fun) = read_function(e) {
                        method.push(fun)
                    }
                }
                "virtual-method" => { 
                    if let Some(fun) = read_function(e) {
                        virtual_method.push(fun)
                    }
                }
                "callback" => { 
                    if let Some(fun) = read_function(e) {
                        callbacks.push(fun)
                    }
                }
                // "property" => {
                //     if let Some(prop) = read_property(e) {
                //         class.properties.push(prop)
                //     }
                // }
                // "signal" => {
                //     if let Some(fun) = read_signal(e) {
                //         class.signals.push(fun)
                //     }
                // }
                // "implements" => {
                //     if let Some((typ, ctype, _)) = read_type(e, false) {
                //         class.implements.push(typ)
                //     }
                // }
                name => {
                    // panic!("Name: {} not matched against\n", name)
                }
            }
        }
    }
    Some(Class {
        info,
        name,
        glib_type_name,
        glib_get_type,
        parent,
        glib_type_struct,
        ref_func,
        unref_func,
        set_value_func,
        get_value_func,
        ctype,
        symbol_prefix,
        abstracts,
        glib_fundamental,
        finals,
        constructor,
        functions,
        method,
        virtual_method,
        callbacks,
        doc,
    })
}

// fn read_constant(e: &Element) -> Option<Constant> {
//     let name = e.attributes.get("name")?.to_string();
//     let c_identifier = e.attributes.get("type")?.to_string();
//     let value = e.attributes.get("value")?.to_string();
//     let introspectable = get_introspectable(e, true);
//
//     let version = e.attributes.get("version").map(|x| x.to_string());
//     let deprecated_version = e.attributes.get("deprecated-version").map(|x| x.to_string());
//
//     let mut inner = None;
//     let mut doc = None;
//     let mut doc_deprecated = None;
//
//     let mut typ = None;
//     for node in e.children {
//         let e = node.as_element()?;
//         match e.name.as_ref() {
//             "type" | "array" => {
//                 if typ.is_some() {
//                     return None;
//                 }
//                 typ = read_type(e, e.name == "array");
//             }
//             "doc" => doc = get_doc(e),
//             "doc-deprecated" => doc_deprecated = get_doc(e),
//             "attribute" => {}
//             "source-position" => {}
//             "attribute" => {}
//             _ => panic!("Error parsing param")
//         }
//     }
//     if let Some((typ, c_type, _)) = typ {
//         return Some(Constant {
//             name,
//             c_identifier,
//             introspectable,
//             typ,
//             c_type,
//             value,
//             version,
//             deprecated_version,
//             doc,
//             doc_deprecated
//         })
//     }
//     None
// }

// fn read_bitfield(e: &Element) -> Option<Comp> {
//     let name = e.attributes.get("name")?;
//     let mut fields = vec![];
//     let doc = read_inner_doc(e).unwrap_or("".to_string());
//     for node in e.children.iter() {
//         if let Some(e) = node.as_element() {
//             if e.name == "member" {
//                 if let Some(id) = e.attributes.get("name") {
//                     let inner_doc = read_inner_doc(e).unwrap_or("".to_string());
//                     fields.push((id.to_owned(), inner_doc))
//                 }
//             }
//         }
//     }
//     Some(Comp {
//         name: name.to_string(),
//         doc,
//         members: fields,
//     })
// }

fn read_namespace(e: &Element) -> Option<Namespace> {
    let name = attribute(e, "name");
    let version = attribute(e, "version");

    let shared_library = attribute(e, "shared-library");
    let identifier_prefixes = attribute(e, "identifier-prefixes");
    let symbol_prefixes= attribute(e, "symbol-prefixes");
    let prefix = attribute(e, "prefix");


    let mut classes = vec![];
    let mut functions = vec![];
    let mut macros = vec![];
    let mut callback = vec![];
    // if let Some(s) = attribute(e, "shared-library") {
    //     ns.shared_library = s.split(',').collect();
    // }
    for node in e.children.iter() {
        if let Some(e) = node.as_element() {
            match e.name.as_str() {
                "class" => {
                    if let Some(class) = read_class(&e) {
                        classes.push(class);
                    }
                }
                "function" => {
                    if let Some(fun) = read_function(e) {
                        functions.push(fun);
                    }
                }
                "function-macro" => {
                    if let Some(fun) = read_function(e) {
                        macros.push(fun);
                    }
                }
                "callback" => {
                    if let Some(cb) = read_function(e) {
                        callback.push(cb);
                    }
                }
                // "enumeration" => {
                //     if let Some(enu) = read_enum(e) {
                //         ns.enums.push(enu);
                //     }
                // }
                // "record" => {
                //     if let Some(record) = read_record(e) {
                //         ns.record.push(record);
                //     }
                // }
                // "constant" => {
                //     if let Some(consts) = read_constant(e) {
                //         ns.constant.push(consts);
                //     }
                // }
                // "bitfield" => {
                //     if let Some(bf) = read_bitfield(e) {
                //         ns.bitfield.push(bf)
                //     }
                // }
                // "docsection" => {
                //     // println!("{:#?}", e)
                //     ns.doc = get_doc(e)
                // }
                // "name" => {
                //     println!("{:#?}", e)
                // }
                // "alias" => {
                //     // println!("{:#?}", e)
                // }
                // "interface" => {
                // }
                // "boxed" => {
                // }
                name => {
                    // panic!("Name: {} not matched against\n", name)
                }
            }
        }
    }
    Some(Namespace {
        name,
        version,
        shared_library,
        identifier_prefixes,
        symbol_prefixes,
        prefix,
        classes,
        functions,
        macros,
        callback,
    })
}

pub fn attribute(e: &Element, attr: &str) -> Option<String> {
    e.attributes.get(attr).map(|x| x.to_owned())
}

pub fn attr_value<T>(e: &Element, name: &str) -> Option<T>
where
    T: FromStr,
    T::Err: fmt::Display,
{
    if let Some(value_str) = e.attributes.get(name) {
        match T::from_str(value_str) {
            Ok(value) => Some(value),
            Err(error) => None 
        }
    } else {
        None
    }
}

fn read_include(e: &Element) -> Option<Include> {
    let name = attribute(e, "name")?;
    let version = attribute(e, "version");
    Some(Include { 
        name, 
        version
    })
}
fn read_cinclude(e: &Element) -> Option<CInclude> {
    let name = attribute(e, "name")?;
    Some(CInclude { 
        name, 
    })
}
fn read_package(e: &Element) -> Option<Package> {
    let name = attribute(e, "name")?;
    Some(Package { 
        name, 
    })
}

// should return repo?
fn read_repository(e: &Element) -> Option<Repository> {
    let version = attribute(e, "version");
    let xmlns = attribute(e, "xmlns");
    let identifier_prefixes = attribute(e, "identifier-prefixes");
    let symbol_prefixes = attribute(e, "symbol-prefixes");

    let mut include = vec![];
    let mut cinclude = vec![];
    let mut package = vec![];
    let mut namespace = vec![];
    
    for node in e.children.iter() {
        if let Some(e) = node.as_element() {
            match e.name.as_str() {
                "include" => {
                    if let Some(ns) = read_include(e) {
                        include.push(ns)
                    }
                }
                "cinclude" => {
                    if let Some(ns) = read_cinclude(e) {
                        cinclude.push(ns)
                    }
                }
                "package" => {
                    if let Some(ns) = read_package(e) {
                        package.push(ns)
                    }
                }
                "namespace" => {
                    if let Some(ns) = read_namespace(e) {
                        namespace.push(ns)
                    }
                }
                "attribute" => { }
                name => {
                    panic!("Name: {} not matched against\n", name)
                }
            }
        }
    }
    Some(Repository {
        version,
        xmlns,
        identifier_prefixes,
        symbol_prefixes,
        include,
        cinclude,
        package,
        namespace,
    })
}

pub fn parse_gir<R: Read>(read: R) -> Result<Repository, xmltree::ParseError> {
    let names_element = Element::parse(read)?;
    let repo = read_repository(&names_element);
    if let Some(repo) = repo {
        return Ok(repo)
    }
    Err(xmltree::ParseError::CannotParse)
}
