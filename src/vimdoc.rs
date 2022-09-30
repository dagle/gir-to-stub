use chrono::{DateTime, Utc};
use std::io::prelude::*;

type Result<T> = std::io::Result<T>;

// TODO add c-name to all fields?

// add tabstop at bottom
// reference |item|
// `for parameters or filenames`
// heading: xyz~
// column: === 
// > quote
// tag: *tag*
// vim:tw=78:ts=8:noet:ft=help:norl:

pub struct Document {
    pub ns: String,
    header: String,
    intro: String,
    global: Global,
}

struct Global {
    // name: String,
    classes: Vec<Class>,
    functions: Vec<Function>,
    macros: Vec<Function>,
    enums: Vec<Enum>,
    record: Vec<Variable>,
    constant: Vec<Variable>,
    callback: Vec<Function>,
    bitfield: Vec<String>,
    // unions?
    // docsection: Option<String>,
    // alias: 
    // interface: 
    // boxed: 
}

type Entry = String;
type Doc = String;

#[derive(Debug)]
pub struct Enum {
    pub name: String,
    pub doc: Doc,
    pub members: Vec<(Entry, Doc)>
}

struct Union {
    name: String,
    variants: Vec<(String, Type)>
}

// change name, variable isn't a good name
// we just want a common name for anything that
// name and a type.
#[derive(Debug)]
pub struct Variable {
    pub name: String,
    pub doc: String,
    pub rtype: String,
}

struct Alias {
    oldname: String,
    newname: String,
}

pub fn write_enum<W: Write>(e: &Enum, w: &mut W) -> Result<()> {
    writeln!(w, "{}", e.name)
}

fn create_section<W: Write>(ns: &str, name: &str, w: &mut W) -> Result<()> {
    writeln!(w, "{:=>76}", "=")?;
    let str = format!("*{}.{}*", ns, name); 
    writeln!(w, "{}.{} {:>pad$}", ns, name, str, pad=79-str.len())?;
    Ok(())
}

impl Document {
    pub fn new(ns: &str, header: &str, intro: &str) -> Document {
        Document {
            ns: ns.to_string(),
            header: header.to_string(),
            intro: intro.to_string(),
            global: Global::new(),
        }
    }
    pub fn add_class(&mut self, class: Class) {
        self.global.classes.push(class);
    }
    pub fn add_function(&mut self, func: Function) {
        self.global.functions.push(func);
    }
    pub fn add_macro(&mut self, macr: Function) {
        self.global.macros.push(macr);
    }
    pub fn add_enum(&mut self, enu: Enum) {
        self.global.enums.push(enu);
    }
    pub fn add_constants(&mut self, var: Variable) {
        self.global.constant.push(var);
    }
    pub fn write<W: Write>(&self, w: &mut W) -> Result<()> {
        let now: DateTime<Utc> = Utc::now();
        let datestr = format!("Last Generated: {}", now.format("%d/%m/%Y %H:%M"));
        writeln!(w, "*{}.txt* {:>pad$}", self.ns, datestr, pad=76-5-self.ns.len())?;

        for classes in self.global.classes.iter() {
            classes.write(&self.ns, w)?;
        }
        
        if ! self.global.functions.is_empty() {
            writeln!(w, "")?;

            create_section(&self.ns, "Functions", w)?;
            writeln!(w, "")?;
            for function in self.global.functions.iter() {
                function.write(&self.ns, w)?;
            }
        }

        if ! self.global.macros.is_empty() {
            writeln!(w, "")?;

            create_section(&self.ns, "Macros", w)?;
            writeln!(w, "")?;
            for macr in self.global.macros.iter() {
                macr.write(&self.ns, w)?;
            }
        }

        if ! self.global.functions.is_empty() {
            writeln!(w, "")?;

            create_section(&self.ns, "Enums", w)?;
            writeln!(w, "")?;
            for enu in self.global.enums.iter() {
                write_enum(enu, w)?;
            }
        }
        writeln!(w, "")?;
        write!(w, "vim:tw=78:ts=8:noet:ft=help:norl:")?;
        w.flush()?;
        Ok(())
    }
}

impl Global {
    pub fn new() -> Global {
        Global {
            classes: vec![],
            functions: vec![],
            macros: vec![],
            enums: vec![],
            record: vec![],
            constant: vec![],
            callback: vec![],
            bitfield: vec![],
        }
    }
}

pub struct Class {
    name: String,
    doc: Option<String>,
    fields: Vec<(String, String)>,
    constructor: Vec<Function>,
    method: Vec<Function>,
    func: Vec<Function>,
    virt: Vec<Function>,
    // c_name,

    // property: 
    // signal: 
    // implements: 
}

impl Class {
    pub fn new(name: &str) -> Class {
        Class {
            name: name.to_string(),
            doc: None,
            fields: vec![],
            func: vec![],
            constructor: vec![],
            method: vec![],
            virt: vec![],
        }
    }
    pub fn write<W: Write>(&self, ns: &str, w: &mut W) -> Result<()>{
        let static_ns = format!("{}.{}", ns, self.name);

        create_section(ns, &self.name, w)?;
        let local_ns = self.name.to_lowercase();

        if let Some(ref doc) = self.doc {
            writeln!(w, "{}\n", doc)?;
        }

        if !self.fields.is_empty() {
            writeln!(w, "Fields~")?;
            for field in self.fields.iter() {
                let str = format!("{}.{}", static_ns, field.0);
                w.write_all(&str.as_bytes())?;
                writeln!(w, "")?;
            }
        }
        if !self.constructor.is_empty() {
            writeln!(w, "Constructors~")?;
            for constr in self.constructor.iter() {
                constr.write(&static_ns, w)?;
                writeln!(w, "")?;
            }
        }
        if !self.method.is_empty() {
            writeln!(w, "Methods~")?;
            for method in self.method.iter() {
                method.write(&local_ns, w)?;
                writeln!(w, "")?;
            }
        }
        if !self.func.is_empty() {
            writeln!(w, "Functions~")?;
            for func in self.func.iter() {
                func.write(&static_ns, w)?;
                writeln!(w, "")?;
            }
        }
        if !self.virt.is_empty() {
            writeln!(w, "Virtual~")?;
            for virt in self.virt.iter() {
                virt.write(&local_ns, w)?;
                writeln!(w, "")?;
            }
        }
        Ok(())
    }

    pub fn add_constructor(&mut self, fun: Function) {
        self.constructor.push(fun);
    }
    pub fn add_method(&mut self, fun: Function) {
        self.method.push(fun);
    }
    pub fn add_virtual(&mut self, fun: Function) {
        self.virt.push(fun);
    }
    pub fn add_function(&mut self, fun: Function) {
        self.func.push(fun);
    }
    pub fn add_field(&mut self, field: (String, String)) {
        self.fields.push(field)
    }
    pub fn set_docs(&mut self, doc: Option<String>) {
        self.doc = doc;
    }
}

type Type = String;

pub struct Function {
    name: String,
    introspectable: bool,
    doc: Option<String>, // todo, doc should be a Vec<String>
    args: Vec<(String, Type)>,
    ret: Vec<Type>,
    // c_name,
}

fn get_typeless(args: &Vec<(String, Type)>) -> Vec<String> {
    args.into_iter().map(|x| format!("{{{}}}", x.0)).collect()
}


//
fn write_doc<W: Write>(doc: &Option<String>, w: &mut W) -> Result<()> {
    if let Some(ref doc) = doc {
        let lines = doc.lines();
        for line in lines.into_iter() {
            writeln!(w, "\t{}", line)?;
        }
        writeln!(w, "")?;
    }
    Ok(())
}
impl Function {
    pub fn new(name: String, intro: bool, doc: Option<String>, 
        args: Vec<(String, Type)>, ret: Vec<Type>) -> Function {
        Function {
            name,
            introspectable: intro,
            doc,
            args,
            ret,
        }
    }
    
    pub fn write<W: Write>(&self, ns: &str, w: &mut W) -> Result<()> {
        let typeless = get_typeless(&self.args);
        let fmt = format!("*{}.{}()*", ns, self.name);
        writeln!(w, "{:>78}", fmt)?;
        writeln!(w, "{}.{}({})\n", ns, self.name, typeless.join(", "))?;

        write_doc(&self.doc, w)?;
        writeln!(w, "\tArguments:~")?;
        for arg in self.args.iter() {
            // writeln!(w, "`{}`: `{}` {}", arg.0, arg.1, arg.2)?;
            writeln!(w, "\t\t{{{}}} `{}`", arg.0, arg.1)?;
        }
        writeln!(w, "\tReturns:~\n\t\t`{}`", self.ret.join(", "))?;
        Ok(())
    }
} 
