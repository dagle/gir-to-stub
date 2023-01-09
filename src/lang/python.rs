use std::{fs::File, io::Write};
use std::fs;
use crate::{library::*, parse};
use std::path::Path;
use super::*;

pub struct PythonCodeGen {
    level: Level
}

type Result<T> = std::io::Result<T>;

fn open_gir(filename: &str) -> Result<File> {
    match fs::File::open(filename) {
        Ok(f) => Ok(f),
        Err(_) => {
            // If we don't find the gir locally, we use the global file
            let path = Path::new("/usr/share/gir-1.0/").join(filename);
            fs::File::open(path)
        }
    }
}

impl PythonCodeGen {
    pub fn new(level: Level) -> PythonCodeGen {
        PythonCodeGen{
            level,
        }
    }
}

impl Generator for PythonCodeGen {
    fn gen(&self, filename: &str) -> Result<()> {
        let types = "gi-stubs";
        if !super::is_dir(types) {
            fs::create_dir(types)?;
        }
        let mut path = Path::new(types).join(filename);
        path.set_extension("py");
        let mut out_file = fs::File::create(path)?;
        let in_file = open_gir(filename)?;
        let repo = parse::parse_gir(in_file).expect("Couldn't parse gir file");
        repo.namespace[0].gen_python(&mut out_file)?;

        Ok(())
    }
}

// TODO Typehinting
// TODO docstrings inside a class

fn create_section<W: Write>(ns: &str, str: &str, w: &mut W) -> Result<()> {
    writeln!(w, "# {} {}\n", ns, str)
}

macro_rules! section {
    ( $w:ident, $self:ident, $name:ident, $section:ident ) => {
        {
            if !$self.$section.is_empty() {
                create_section(&$name, stringify!($section), $w)?;
                for section in $self.$section.iter() {
                    section.gen_python(&$name, $w)?;
                }
            }
        };
    }
}

impl Namespace {
    pub fn gen_python<W: Write>(&self, w: &mut W) -> Result<()> {
        // let name = self.name.unwrap_or_else("".to_owned());
        let name = self.name.as_ref().unwrap();
        // writeln!(w, "local {} = {{}}\n", &name)?;


        // functions are different
        if !self.functions.is_empty() {
            create_section(&name, "Function", w)?;
            for function in self.functions.iter() {
                function.gen_python(0, w)?;
            }
        }

        section!(w, self, name, classes);
        // section!(w, self, name, enums);
        // section!(w, self, name, record);
        // section!(w, self, name, constant);
        // section!(w, self, name, bitfield);
        // section!(w, self, name, alias);
        // section!(w, self, name, unions);
        // writeln!(w, "return {}", &name)?;
        w.flush()?;
        Ok(())
    }
}

static KEYWORDS: &'static [&str] = &[
    "and",       "break",     "do",        "else",      "elseif",
    "end",       "false",     "for",       "function",  "if",
    "in",        "local",     "nil",       "not",       "or",
    "repeat",    "return",    "then",      "true",      "until",     "while",
];

pub fn unkeyword(param_name: &str) -> String {
    for key in KEYWORDS.iter() {
        if *key == param_name {
            return format!("{}_", key);
        }
    }
    param_name.to_owned()
}

fn in_param(direction: &Option<ParameterDirection>) -> bool {
    if let Some(direct) = direction {
        return matches!(direct, ParameterDirection::In | ParameterDirection::InOut);
    }
    true
}

fn gen_param_names(params: &Vec<Parameter>, method: bool) -> String {
    let mut param_names: Vec<String> = params
        .iter()
        .filter(|p| in_param(&p.direction))
        .map(|p| unkeyword(&p.name))
        .collect();
    if method {
        param_names.insert(0, "self".to_owned());
    }
    param_names.join(", ")
}

impl Function {
    pub fn gen_python<W: Write>(&self, repeats: usize, w: &mut W) -> Result<()> {
        let introspectable = self.info.introspectable.unwrap_or(true);
        if introspectable {
            // gen_doc(&self.doc, w)?;
            // gen_doc_params(&self.parameters, root_ns, w)?;
            // gen_doc_return(&self, root_ns, w)?;
            let param_names = gen_param_names(&self.parameters, false);
            let indent = "    ".repeat(repeats);
            writeln!(w, "def {}({}):", self.name, param_names)?;
            writeln!(w, "{}pass\n", &indent)?;
        }
        Ok(())
    }
    // pub fn gen_method_python<W: Write>(&self, ns: &str, root_ns: &str, w: &mut W) -> Result<()> {
    //     let introspectable = self.info.introspectable.unwrap_or(true);
    //     if introspectable {
    //         gen_doc(&self.doc, w)?;
    //         writeln!(w, "--- @param self {}", ns)?;
    //         gen_doc_params(&self.parameters, root_ns, w)?;
    //         gen_doc_return(&self, root_ns, w)?;
    //         let param_names = gen_param_names(&self.parameters, true);
    //         writeln!(w, "function {}.{}({}) end\n", &ns, self.name, param_names)?;
    //     }
    //     Ok(())
    // }
    // pub fn gen_member_python<W: Write>(&self, root_ns: &str, w: &mut W) -> Result<()> {
    //     let introspectable = self.info.introspectable.unwrap_or(true);
    //     if introspectable {
    //         gen_doc(&self.doc, w)?;
    //         gen_doc_params(&self.parameters, root_ns, w)?;
    //         gen_doc_return(&self, root_ns, w)?;
    //         let param_names = gen_param_names(&self.parameters, true);
    //         writeln!(w, "\t[\"{}\"] = function({}) end,\n", self.name.to_uppercase(), param_names)?;
    //     }
    //     Ok(())
    // }
}
//
// impl Alias {
//     pub fn gen<W: Write>(&self, ns: &str, w: &mut W) -> Result<()> {
//         Ok(())
//     }
// }
//
impl Class {
    pub fn gen_python<W: Write>(&self, ns: &str, w: &mut W) -> Result<()> {
        // writeln!(w, "-- @class {}", self.name)?;

        let class_ns = format!("{}.{}", ns, self.name);

        writeln!(w, "class {}:", self.name)?;

        // section!(w, self, class_ns, implements);

        // if !self.constructor.is_empty() {
        //     for constructor in self.constructor.iter() {
        //         constructor.gen(&class_ns, ns, w)?;
        //     }
        // }
        if !self.method.is_empty() {
            for method in self.method.iter() {
                // TODO change 1 to level + 1
                method.gen_python(1, w)?;
            }
        }
        // if !self.functions.is_empty() {
        //     for func in self.functions.iter() {
        //         func.gen(&class_ns, ns, w)?;
        //     }
        // }
        // if !self.virtual_method.is_empty() {
        //     for virt in self.virtual_method.iter() {
        //         virt.gen_method(&class_ns, ns, w)?;
        //     }
        // }
        // if !self.callbacks.is_empty() {
        //     for virt in self.virtual_method.iter() {
        //         virt.gen(&class_ns, ns, w)?;
        //     }
        // }
        //
        // section!(w, self, class_ns, record);
        // section!(w, self, class_ns, fields);
        // section!(w, self, class_ns, signals);
        // section!(w, self, class_ns, unions);
        // section!(w, self, class_ns, constant);
        // section!(w, self, class_ns, properties);
        Ok(())
    }
}
