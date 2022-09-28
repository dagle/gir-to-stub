use xmltree::Element;
use xmltree::XMLNode;
use std::collections::HashMap as AttributeMap;
// use xmltree::AttributeMap;
use std::fs::File;
use std::env;

    
fn print_header(attribs: &AttributeMap<String, String>) {
    // println!("{:#?}", attribs);
}

fn print_entry_type(attribs: &AttributeMap<String, String>) {
    // println!("{:#?}", attribs["name"]);
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

fn get_params(e: &Element, ns: &str) -> Option<Vec<(String, String)>> {
    let mut ret: Vec<(String, String)> = vec![];
    let e2 = e.get_child("parameters")?;
    for child in e2.children.iter() {
        match child {
            XMLNode::Element(e) => {
                if e.name == "parameter" {
                    let argname = e.attributes["name"].clone();
                    if let Some(e2) = e.get_child("type") {
                        let argtype = &e2.attributes.get("name")?;
                        if !filter(&argtype) {
                            ret.push((argname, translate(argtype, ns)));
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
        print!(" -> {}", str)
    }
}

fn get_doc(e: &Element) -> Option<String> {
    if let Some(doc) = e.get_child("doc") {
        let txt = doc.get_text()?;
        return Some(txt.into_owned())
    }
    None
}


fn callable(e: &Element, ns: &String, parentns: &str) {
    let name = &e.attributes["name"];
    let intro = e.attributes.get("introspectable");
    if let Some(enabl) = intro {
        if enabl == "0" {
            return;
        }
    }
    let args = get_params(&e, parentns);

    // ugly as hell
    let call = format!("{}.{}({})", ns, name, 
        args.map_or("".to_string(), |vec| vec.into_iter().map(|x| x.0).collect::<Vec<String>>().join(", ")));
    // print!("{}({}){:<pad$}*{}*", call, ,
    let len = call.len();
    print!("{}   *{}.{}()*", call, ns, name);
    // print_args(args);
    let ret = get_return(&e, parentns);
    // print_ret(ret);

    println!("");
    if let Some(doc) = get_doc(&e) {
        println!("{}", doc);
    }
    println!("");
}

fn print_doc(e: &Element) {
    println!("{}\n", e.get_text().unwrap_or("".into()));
}

fn print_class(parentns: &str, e: &Element) {
    // println!("{:#?}", e);
    let ns = &e.attributes["name"];
    println!("{:=>76}", "=");
    let class = format!("{}.{}", parentns, ns);
    let len = class.len();

    println!("{:<pad$} *{}*\n", class, class, pad=76-len-3);
    for node in e.children.iter() {
        match node {
            XMLNode::Element(ref e) => {
                match e.name.as_str() {
                    "doc" => {
                        // print_doc(e)
                    }
                    "field" => {
                        // println!("{:#?}", e)
                    }
                    "source-position" => {
                    }
                    "constructor" => {
                        let ns = format!("{}.{}", parentns, ns);
                        // callable(e, &ns, parentns);
                    }
                    "function" => {
                        let ns = format!("{}.{}", parentns, ns);
                        // callable(e, &ns, parentns);
                    }
                    "method" => {
                        let ns = ns.to_lowercase();
                        // callable(e, &ns, parentns);
                    }
                    "virtual-method" => { 
                        let ns = ns.to_lowercase();
                        // callable(e, &ns, parentns);
                    }
                    "property" => {
                        // TODO
                        // println!("{:#?}", e)
                    }
                    "signal" => {
                        // TODO
                        // println!("{:#?}", e)
                    }
                    "implements" => {
                        // TODO
                        println!("{:#?}", e)
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
    // println!("{:#?}", e);
}

// add namespace
fn print_entry(parentns: &str, node: &XMLNode) {
    let class = "";
    match node {
        XMLNode::Element(ref e) => {
            // print classes first,
            // then print functions,
            // then macros
            // then print the rest
            match e.name.as_str() {
                "class" => {
                    print_class(parentns, &e)
                }
                "function" => {
                    // println!("{:=>76}", "=");
                    // let class = format!("{}.{}", parentns, ns);
                    // let len = class.len();

                    // println!("{:<pad$} *{}*\n", class, class, pad=76-len-3);
                    // callable(e, &parentns.to_string(), parentns);
                }
                "function-macro" => {
                    // print_macro(&e)
                    // only print if introspectable
                    // println!("{:#?}", e)
                }
                "enumeration" => {
                    // print_enum(&e, parentns)
                }
                "record" => {
                    // println!("{:#?}", e)
                    // is-gtype-struct-for ? Is that the dynamic type check function?
                }
                "constant" => {
                    // println!("{:#?}", e)
                }
                "callback" => {
                    // TODO
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
