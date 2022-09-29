use std::fs::File;
use std::io::prelude::*;

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
    // ns?
    classes: Vec<Class>,
    functions: Vec<Function>,
    macros: Vec<Function>,
    enums: Vec<String>,
    record: Vec<String>,
    constant: Vec<String>,
    callback: Vec<Function>,
    bitfield: Vec<String>,
    // docsection: 
    // name: 
    // alias: 
    // interface: 
    // boxed: 
}



type Type = String;

pub struct Function {
    name: String,
    doc: Option<String>,
    args: Vec<(String, Type)>,
    ret: Vec<Type>,
}

enum Macro {
    Func(Function),
    // can we have a better name?
    Var(String),
}

pub struct Class {
    name: String,
    doc: Option<String>,
    fields: Vec<String>,
    constructor: Vec<Function>,
    method: Vec<Function>,
    func: Vec<Function>,
    virt: Vec<Function>,
    // property: 
    // signal: 
    // implements: 
}
struct Section {
    
}

    // pub ns: String,
    // header: String,
    // intro: String,
    // global: Global,
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
    pub fn add_enum(&mut self, enu: String) {
        self.global.enums.push(enu);
    }
    pub fn write(&self, path: &str) -> std::io::Result<()> {
        let mut file = File::create(path)?;
        let str = format!("{}.txt, {{gir generated documentatiotion for {}}}", path, self.ns);
        file.write_all(&str.as_bytes())?;

        for classes in self.global.classes.iter() {
            classes.write()
        }
        for function in self.global.functions.iter() {
            function.write()
        }
        for macr in self.global.macros.iter() {
            macr.write()
        }
        for enu in self.global.enums.iter() {
            enu.write()
        }
        file.write_all(b"vim:tw=78:ts=8:noet:ft=help:norl:")?;
        Ok(())
        // for section in sections.iter() { 
        //     section.write(&ns, &file)
        // }
        // write sections

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
    pub fn set_docs(&mut self, doc: Option<String>) {
        self.doc = doc;
    }
}
