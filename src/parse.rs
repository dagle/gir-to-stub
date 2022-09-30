use crate::vimdoc::*;
use xmltree::Element;
use xmltree::XMLNode;
use std::io::Read;

// both of these are kinda wrong, because error values etc
fn get_return(e: &Element) -> Option<Vec<String>> {
    let ret = e.get_child("return-value")?;
    if let Some(t) = ret.get_child("type") {
        let name = &t.attributes.get("name")?;
        return Some(vec![name.to_string()])
    }
    if let Some(t) = ret.get_child("array") {
        let name = &t.attributes.get("name")?;
        return Some(vec![name.to_string()])
    }
    None
}

fn get_params(e: &Element) -> Option<Vec<(String, String)>> {
    let mut ret: Vec<(String, String)> = vec![];
    let parameters = e.get_child("parameters")?;

    for parameter in parameters.children.iter() {
        match parameter {
            XMLNode::Element(element) => {
                if element.name == "parameter" {
                    let argname = element.attributes.get("name")?.clone();
                    let e2 = element.get_child("type")?;
                    let argtype = &e2.attributes.get("name")?;
                    ret.push((argname, argtype.to_string()));
                }
            }
            _ => {}
        }
    }
    return Some(ret)
}

fn get_inner_doc(e: &Element) -> Option<String> {
    e.get_child("doc").map(|doc| get_doc(doc)).flatten()
}

fn get_inne_name<'a>(e: &'a Element, name: &'a str) -> Option<&'a String> {
    e.get_child(name).map(|doc| doc.attributes.get("name")).flatten()
}

fn get_inner_type(e: &Element) -> Option<&String> {
    e.get_child("type").map(|doc| doc.attributes.get("name")).flatten()
}

fn callable(e: &Element) -> Option<Function> {
    let name = e.attributes.get("name")?;
    let intros = e.attributes.get("introspectable");

    let introspectable = intros.map(|x| x == "0").unwrap_or(true);

    let doc = get_inner_doc(&e);
    let ret = get_return(&e).unwrap_or(vec![]);
    let args = get_params(&e).unwrap_or(vec![]);
    Some(Function::new(name.to_string(), introspectable, doc, args, ret))
}

fn get_doc(e: &Element) -> Option<String> {
    e.get_text().map(|x| x.into())
}

fn get_field(e: &Element) -> Option<(String, String)> {
    let name = e.attributes.get("name")?;
    let doc = get_inner_doc(e)?;
    Some((name.to_string(), doc))
}

fn get_class(parentns: &str, e: &Element) -> Class {
    let ns = &e.attributes["name"];
    let mut class = Class::new(ns);
    for node in e.children.iter() {
        if let Some(e) = node.as_element() {
            match e.name.as_str() {
                "doc" => {
                    let docs = get_doc(&e);
                    class.set_docs(docs);
                }
                "field" => {
                    if let Some(field) = get_field(e) {
                        class.add_field(field)
                    }
                }
                "source-position" => {
                }
                "constructor" => {
                    if let Some(fun) = callable(e) {
                        class.add_constructor(fun)
                    }
                }
                "function" => {
                    if let Some(fun) = callable(e) {
                        class.add_function(fun)
                    }
                }
                "method" => {
                    if let Some(fun) = callable(e) {
                        class.add_method(fun)
                    }
                }
                "virtual-method" => { 
                    if let Some(fun) = callable(e) {
                        class.add_virtual(fun)
                    }
                }
                "property" => {
                    // println!("{:#?}", e)
                }
                "signal" => {
                    // println!("{:#?}", e)
                    // TODO
                }
                "implements" => {
                    // TODO
                }
                name => {
                    panic!("Name: {} not matched against\n", name)
                }
            }
        }
    }
    class
}

fn get_constants(e: &Element) -> Option<Variable> {
    let name = e.attributes.get("name")?.to_string();
    let doc = get_inner_doc(e).unwrap_or("".to_string());
    let rtype = get_inner_type(e).map(|x| x.to_string()).unwrap_or("".to_string());
    Some(Variable {
        name,
        doc,
        rtype
    })
}

fn get_composite(e: &Element) -> Option<Comp> {
    let name = e.attributes.get("name")?;
    let mut fields = vec![];
    let doc = get_inner_doc(e).unwrap_or("".to_string());
    for node in e.children.iter() {
        if let Some(e) = node.as_element() {
            if e.name == "member" {
                if let Some(id) = e.attributes.get("name") {
                    let inner_doc = get_inner_doc(e).unwrap_or("".to_string());
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

fn get_global(doc: &mut Document, node: &XMLNode) {
    if let Some(e) = node.as_element() {
        match e.name.as_str() {
            "class" => {
                let class = get_class(&doc.ns, e);
                doc.add_class(class);
            }
            "function" => {
                if let Some(fun) = callable(e) {
                    doc.add_function(fun)
                }
            }
            "function-macro" => {
                if let Some(fun) = callable(e) {
                    doc.add_macro(fun)
                }
            }
            "enumeration" => {
                if let Some(enu) = get_composite(e) {
                    doc.add_enum(enu)
                }
            }
            "record" => {
                // println!("{:#?}", e)
                // is-gtype-struct-for ? Is that the dynamic type check function?
            }
            "constant" => {
                if let Some(consts) = get_constants(e) {
                    doc.add_constants(consts);
                }
            }
            "callback" => {
                if let Some(cb) = callable(e) {
                    doc.add_callback(cb)
                }
            }
            "bitfield" => {
                if let Some(bf) = get_composite(e) {
                    doc.add_bitfield(bf)
                }
            }
            "docsection" => {
                // println!("{:#?}", e)
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

fn parse_toplevel(node: &XMLNode) -> Option<Document> {
    if let Some(e) = node.as_element() {
        if e.name == "namespace" {
            let ns = &e.attributes["name"];
            let mut doc = Document::new(ns,
                "",
                "");
            for node in e.children.iter() {
                get_global(&mut doc, node)
            }
            return Some(doc);
        }
    }
    None
}

// TODO, atm we only handle one namespace
pub fn parse_gir<R: Read>(read: R) -> Result<Document, xmltree::ParseError> {
    let names_element = Element::parse(read)?;

    for child in names_element.children.into_iter() {
        let doc = parse_toplevel(&child);
        if let Some(doc) = doc {
            return Ok(doc)
        }
    }
    Err(xmltree::ParseError::CannotParse)
}
