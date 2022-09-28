// add tabstop at bottom
// reference |item|
// `for parameters or filenames`
// heading: xyz~
// column: === 
// > quote
// tag: *tag*
// vim:tw=78:ts=8:noet:ft=help:norl:

struct Document {
    ns: String,
    header: String,
    intro: string
}

struct Section {
    
}

impl Documnt {
    fn new(ns: &str) -> Document {
        Document {ns: ns.to_string()}
    }
    fn add_section(&mut self, section: Section) {
    }
    // implement writer
    fn write(&self, path: &str) {
        // open file for writing
        // write first line
        // ex gtk.txt {gir generated documentation for gtk}

        for section in sections.iter() { 
            section.write(&ns, &file)
        }
        // write sections

        // write vim:tw=78:ts=8:noet:ft=help:norl:
        // close file
    }
}

enum SectionType {
    Class,
    Function,
    Macro,
    Rest,
}

impl Section {
    fn new() -> Section {
        section {ns: ns.to_string()}
    }
    // add kind or something?
    fn add_entry(&mut self, entry: &Entry) {
    }

    fn make_header(&self, writer: &Writer) {
    }

    fn write(&self, writer: &Writer) {
        make_header(self, writer);
        // write intro
        // add example? TODO
    }
}

type Type = String;

struct Function {
    name: String,
    ns: String,
    args: Vec<(String, Type)>,
    ret: Vec<Type>,
}

struct Callback {
}

struct Class {
    name: String,
    doc: Option<String>,
    fields: Vec<String>,
    constructor: Vec<Function>,
    method: Vec<Function>,
    virtal: Vec<Function>,
    // property: 
    // signal: 
    // implements: 
}


struct Global {
    classes: Vec<Class>,
    functions: Vec<Function>,
    macros: Vec<Macro>,
    enums: Vec<String>,
    record: Vec<String>,
    constant: Vec<String>,
    callback: Vec<Callback>,
    bitfield: Vec<String>,
    // docsection: 
    // name: 
    // alias: 
    // interface: 
    // boxed: 
}

impl Entry {
    fn new() -> Eentry {
    }
    fn write(&self, writer: &Writer) {
    }
}
