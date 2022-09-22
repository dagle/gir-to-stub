use xmltree::Element;
use xmltree::XMLNode;
use std::collections::HashMap as AttributeMap;
// use xmltree::AttributeMap;
use std::fs::File;
use std::env;

    
fn print_header(attribs: &AttributeMap<String, String>) {
    println!("{:#?}", attribs);
}

fn print_entry_type(attribs: &AttributeMap<String, String>) {
    println!("{:#?}", attribs["name"]);
}

fn print_enum(e: &Element, parentns: &str) {
    let ns = &e.attributes["name"];
    for child in e.children.iter() {
        match child {
            XMLNode::Element(e) =>  {
                if e.name == "member" {
                   println!("{}.{}.{}", parentns, ns, e.attributes["name"].to_uppercase()) 
                }
            }
            _ => {}
        }
    }
}

fn translate(str: &str, ns: &str) -> String {
    match str {
        "utf8"|"const char*"|"char*" => "string".to_string(),
        "gboolean" => "boolean".to_string(),
        "glong"|"gssize"|"gint64"|"gint"|"gsize"|"guint32" => "num".to_string(),
        "none" => "nil".to_string(),
        rest => {
            if !rest.contains(".") {
                return format!("{}.{}", ns, str)
            }
            rest.to_string()
        }
    }
}

// both of these are kinda wrong, because error values etc
fn get_return(e: &Element, ns: &str) -> Option<Vec<String>> {
    let e2 = e.get_child("return-value")?;
    if let Some(t) = e2.get_child("type") {
        let name = &t.attributes.get("name")?;
        return Some(vec![translate(name, ns)])
    }
    if let Some(t) = e2.get_child("array") {
        let name = &t.attributes.get("name")?;
        return Some(vec![translate(name, ns)])
    }
    None
}

fn filter(typ: &String) -> bool {
    match typ.as_ref() {
        "gpointer" => true,
        _ => false,
    }
}


// TODO a way to push arguments to return values, when a argument is returned
// and not passed as an argument. Example a function apa(int a, err **Error) 
// becomes apa(int) -> Error

fn get_params(e: &Element, ns: &str) -> Option<Vec<String>> {
    let mut ret: Vec<String> = vec![];
    let e2 = e.get_child("parameters")?;
    for child in e2.children.iter() {
        match child {
            XMLNode::Element(e) => {
                if e.name == "parameter" {
                    let argname = &e.attributes["name"];
                    if let Some(e2) = e.get_child("type") {
                        let argtype = &e2.attributes.get("name")?;
                        if !filter(&argtype) {
                            let string = format!("{}: {}", argname, translate(argtype, ns));
                            ret.push(string);
                        }
                    }
                }
            }
            _ => {}
        }
    }
    return Some(ret)
}

fn print_args(args: Option<Vec<String>>) {
    if let Some(vec) = args {
        let str = vec.join(", ");
        print!("({})", str)
    } else {
        print!("()")
    }
}

fn print_ret(ret: Option<Vec<String>>) {
    if let Some(vec) = ret {
        let str = vec.join(", ");
        println!(" -> {}", str)
    }
}

fn callable(e: &Element, ns: &String, parentns: &str) {
    let name = &e.attributes["name"];
    let intro = e.attributes.get("introspectable");
    if let Some(enabl) = intro {
        if enabl == "0" {
            return;
        }
    }
    print!("{}.{}", ns, name);
    let args = get_params(&e, parentns);
    print_args(args);
    let ret = get_return(&e, parentns);
    print_ret(ret);
}

fn print_class(parentns: &str, e: &Element) {
    // println!("{:#?}", e);
    let ns = &e.attributes["name"];
    for node in e.children.iter() {
        match node {
            XMLNode::Element(ref e) => {
                match e.name.as_str() {
                    "doc" => {}
                    "source-position" => {}
                    "constructor" => {
                        let ns = format!("{}.{}", parentns, ns);
                        callable(e, &ns, parentns);
                    }
                    "function" => {
                        let ns = format!("{}.{}", parentns, ns);
                        callable(e, &ns, parentns);
                    }
                    "method" => {
                        let ns = ns.to_lowercase();
                        callable(e, &ns, parentns);
                    }
                    "virtual-method" => { 
                        // do we want these?
                        // callable(e, ns);
                    }
                    "field" => {
                        // parent stuff?
                        // println!("{:#?}", e)
                    }
                    "property" => {

                    }
                    "signal" => {
                    }
                    "implements" => {

                    }
                    name => {
                        panic!("Name: {} not matched against\n", name)
                    }
                }
            }
            _ => {}
        }
    }
}

fn print_macro(e: &Element) {
    println!("{:#?}", e);
}

// add namespace
fn print_entry(parentns: &str, node: &XMLNode) {
    match node {
        XMLNode::Element(ref e) => {
            match e.name.as_str() {
                "enumeration" => {
                    print_enum(&e, parentns)
                }
                "class" => {
                    print_class(parentns, &e)
                }
                "function-macro" => {
                    print_macro(&e)
                }
                "function" => {
                    callable(e, &parentns.to_string(), parentns);
                }
                "record" => {
                    // println!("{:#?}", e)
                }
                "constant" => {
                    // println!("{:#?}", e)
                }
                "callback" => {
                    // println!("{:#?}", e)
                }
                "bitfield" => {
                    // println!("{:#?}", e)
                }
                "docsection" => {

                }
                "name" => {
                }
                "alias" => {
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
        _ => {}
    }
}

fn parse_toplevel(node: &XMLNode) {
    match node {
        XMLNode::Element(ref e) => {
            if e.name == "namespace" {
                // print_header(&e.attributes);
                for node in e.children.iter() {
                    print_entry(&e.attributes["name"], node)
                }
            }
        }
        _ => {}
    }
}

fn main() {
    for arg in env::args().skip(1) {
        let f = File::open(arg).expect("Can't read file");

        let names_element = Element::parse(f).unwrap();

        // for name in names_element {
        //
        // }
        for child in names_element.children.into_iter() {
            parse_toplevel(&child);
        }
        {
            // get first `name` element
            // name.attributes.insert("suffix".to_owned(), "mr".to_owned());
        }
    }
    // names_element.write(File::create("result.xml").unwrap());

}
