use std::str::FromStr;
/// Bindings for the lgi, the standard lua loader

use std::{fs::File, io::Write};
use std::fs;
use crate::{library::*, parse};
use std::path::Path;
use super::Level;

// The amount of code/doc we should generate
// Generating full docs for a mock file could be
// a bit to much for a lsp.

// struct LuaDoc {}
pub struct LuaCodegen {
    level: Level
}

type Result<T> = std::io::Result<T>;

fn is_dir(dir: &str) -> bool {
    Path::new(dir).is_dir()
}

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

impl LuaCodegen {
    pub fn new(level: Level) -> LuaCodegen {
        LuaCodegen{
            level,
        }
    }

    pub fn gen(&self, filename: &str) -> Result<()> {
        let types = "types";
        if !is_dir(types) {
            fs::create_dir(types)?;
        }
        let mut path = Path::new(types).join(filename);
        path.set_extension("lua");
        let mut out_file = fs::File::create(path)?;
        let in_file = open_gir(filename)?;
        let repo = parse::parse_gir(in_file).expect("Couldn't parse gir file");
        repo.namespace[0].gen(&mut out_file)?;

        Ok(())
    }
}

fn create_section<W: Write>(ns: &str, str: &str, w: &mut W) -> Result<()> {
    writeln!(w, "-- {} {}", ns, str)
}

impl Namespace {
    pub fn gen<W: Write>(&self, w: &mut W) -> Result<()> {
        // let name = self.name.unwrap_or_else("".to_owned());
        let name = self.name.as_ref().unwrap();
        writeln!(w, "local {} = {{}}\n", &name)?;

        if !self.classes.is_empty() {
            create_section(&name, "Class", w)?;
            for class in self.classes.iter() {
                writeln!(w, "{}.{} = {{}}", &name, class.name)?;
                class.gen(&name, w)?;
            }
        }
        if !self.functions.is_empty() {
            create_section(&name, "Function", w)?;
            for function in self.functions.iter() {
                function.gen(&name, &name, w)?;
            }
        }
        if !self.enums.is_empty() {
            create_section(&name, "Enums", w)?;
            for enu in self.enums.iter() {
                enu.gen(&name, w)?;
            }
        }
        if !self.record.is_empty() {
            create_section(&name, "Record", w)?;
            for record in self.record.iter() {
                record.gen(&name, w)?;
            }
        }
        if !self.constant.is_empty() {
            create_section(&name, "constant", w)?;
            for cons in self.constant.iter() {
                cons.gen(&name, w)?;
            }
        }
        if !self.bitfield.is_empty() {
            create_section(&name, "bitfield", w)?;
            for bitfield in self.bitfield.iter() {
                bitfield.gen(&name, w)?;
            }
        }
        w.flush()?;
        writeln!(w, "return {}", &name)?;
        Ok(())
    }
}

impl Class {
    pub fn gen<W: Write>(&self, ns: &str, w: &mut W) -> Result<()> {
        // let name = self.name;

        let class_ns = format!("{}.{}", ns, self.name);

        if !self.constructor.is_empty() {
            for constructor in self.constructor.iter() {
                constructor.gen(&class_ns, ns, w)?;
            }
        }
        if !self.method.is_empty() {
            for method in self.method.iter() {
                method.gen_method(&class_ns, ns, w)?;
            }
        }
        if !self.functions.is_empty() {
            for func in self.functions.iter() {
                func.gen(&class_ns, ns, w)?;
            }
        }
        if !self.virtual_method.is_empty() {
            for virt in self.virtual_method.iter() {
                virt.gen_method(&class_ns, ns, w)?;
            }
        }
        if !self.fields.is_empty() {
            for field in self.fields.iter() {
                field.gen(&class_ns, w)?;
            }
        }
        if !self.signals.is_empty() {
            for signal in self.signals.iter() {
                signal.gen(&class_ns, w)?;
            }
        }
        if !self.unions.is_empty() {
            for unio in self.unions.iter() {
                unio.gen(&class_ns, w)?;
            }
        }
        if !self.constant.is_empty() {
            for cons in self.constant.iter() {
                cons.gen(&class_ns, w)?;
            }
        }
        if !self.properties.is_empty() {
            for prop in self.properties.iter() {
                prop.gen(&class_ns, w)?;
            }
        }
        // if !self.implements.is_empty() {
        //     for prop in self.implements.iter() {
        //         prop.gen(&class_ns, w)?;
        //     }
        // }

        Ok(())
    }
}
impl Union {
    pub fn gen<W: Write>(&self, ns: &str, w: &mut W) -> Result<()> {
        writeln!(w, "{}.{} = 5", &ns, self.name.as_ref().unwrap())?;
        Ok(())
    }
}
impl Property {
    pub fn gen<W: Write>(&self, ns: &str, w: &mut W) -> Result<()> {
        writeln!(w, "-- {}.{} = 6", &ns, self.name)?;
        Ok(())
    }
}
impl Signal {
    pub fn gen<W: Write>(&self, ns: &str, w: &mut W) -> Result<()> {
        writeln!(w, "-- {}.{} = 7", &ns, self.name)?;
        Ok(())
    }
}


pub fn unkeyword(param_name: &str) -> &str {
    match param_name {
        "end" => "_end",
        "for" => "_for",
        name => name
    }
}

fn show_anytyp(typ: &AnyType, ns: &str) -> String {
    match typ {
        AnyType::Array(_) => "array".to_owned(), // fix this
        AnyType::Type(typ) => translate(&typ.name, ns),
        AnyType::VarArg => "...".to_owned(),
    }
}

fn translate(name: &Option<String>, ns: &str) -> String {
    if let Some(typ_str) = name {
        match typ_str.as_ref() {
            "utf8"|"const char*"|"char*" => "string".to_string(),
            "gboolean" => "boolean".to_string(),
            "glong"|"gssize"|"gint64"|"gint"|"gsize"|"guint32" => "num".to_string(),
            "none" => "nil".to_string(),
            rest => {
                if !rest.contains(".") {
                    return format!("{}.{}", ns, typ_str)
                }
                rest.to_string()
            }
        }
    } else {
        String::new()
    }
}
impl Field {
    pub fn gen<W: Write>(&self, ns: &str, w: &mut W) -> Result<()> {
        let typ = show_anytyp(&self.typ, ns);
        writeln!(w, "-- {}.{} = {}", &ns, self.name, typ)?;
        Ok(())
    }
}

// fn gen_infoattr(info: &InfoAttrs) -> Result<()> {
//     if let Some(ref deprecated) = info.deprecated {
//     }
//     if let Some(ref dep_version) = info.deprecated {
//     }
//     if let Some(ref version) = info.deprecated {
//     }
//     if let Some(ref stability) = info.deprecated {
//     }
//     Ok(())
// }

/// TODO change %NULL to nil?
fn gen_doc<W: Write>(doc: &InfoElements, w: &mut W) -> Result<()> {
    if let Some(ref docs) = doc.doc {
        let lines = docs.content.split("\n");
        for line in lines {
            writeln!(w, "-- {}", line)?;
        }
    }
    if let Some(ref stability) = doc.doc_stability {
    }
    if let Some(ref version) = doc.doc_version {
    }
    if let Some(ref deprecated) = doc.doc_deprecated {
    }
    if let Some(ref pos) = doc.doc_pos {
    }
    Ok(())
}

fn gen_doc_param<W: Write>(param: &Parameter, ns: &str, w: &mut W) -> Result<()> {
    let type_str = show_anytyp(&param.typ, ns);
    if let Some(ref doc) = param.doc.doc {
        let docstr = doc.content.replace("\n", "");
        // docstr.retain(|c| c != '\n');
        writeln!(w, "--- @param {} {} {}", param.name, type_str, docstr)?;
    } else {
        writeln!(w, "--- @param {} {}", param.name, type_str)?;
    }
    Ok(())
}
fn gen_doc_params<W: Write>(params: &Vec<Parameter>, ns: &str, w: &mut W) -> Result<()> {
    for param in params.iter() {
        gen_doc_param(&param, ns, w)?;
    }
    Ok(())
}

fn in_param(direction: &Option<ParameterDirection>) -> bool {
    if let Some(direct) = direction {
        return matches!(direct, ParameterDirection::In | ParameterDirection::InOut);
    }
    true
}

fn out_param(direction: &Option<ParameterDirection>) -> bool {
    if let Some(direct) = direction {
        return matches!(direct, ParameterDirection::Out | ParameterDirection::InOut);
    }
    true
}

fn gen_param_names(params: &Vec<Parameter>, method: bool) -> String {
    let mut param_names: Vec<&str> = params
        .iter()
        .filter(|p| in_param(&p.direction))
        .map(|p| unkeyword(&p.name))
        .collect();
    if method {
        param_names.insert(0, "self");
    }
    param_names.join(", ")
}

impl Function {
    pub fn gen<W: Write>(&self, ns: &str, root_ns: &str, w: &mut W) -> Result<()> {
        // if self.name == "utils_text_is_8bit" {
        //     println!("{:#?}", self)
        // }
        let introspectable = self.info.introspectable.unwrap_or(true);
        if introspectable {
            gen_doc(&self.doc, w)?;
            gen_doc_params(&self.parameters, root_ns, w)?;
            let param_names = gen_param_names(&self.parameters, false);
            writeln!(w, "function {}.{}({}) end", &ns, self.name, param_names)?;
        }
        Ok(())
    }
    pub fn gen_method<W: Write>(&self, ns: &str, root_ns: &str, w: &mut W) -> Result<()> {
        let introspectable = self.info.introspectable.unwrap_or(true);
        if introspectable {
            gen_doc(&self.doc, w)?;
            writeln!(w, "--- @param self {}", ns)?;
            gen_doc_params(&self.parameters, root_ns, w)?;
            let param_names = gen_param_names(&self.parameters, true);
            writeln!(w, "function {}.{}({}) end", &ns, self.name, param_names)?;
        }
        Ok(())
    }
}

impl Enumeration {
    pub fn gen<W: Write>(&self, ns: &str, w: &mut W) -> Result<()> {
        Ok(())
    }
}

impl Record {
    pub fn gen<W: Write>(&self, ns: &str, w: &mut W) -> Result<()> {
        Ok(())
    }
}

impl Constant {
    pub fn gen<W: Write>(&self, ns: &str, w: &mut W) -> Result<()> {
        Ok(())
    }
}

impl Bitfield {
    pub fn gen<W: Write>(&self, ns: &str, w: &mut W) -> Result<()> {
        Ok(())
    }
}


fn filter(typ: &str) -> bool {
    match typ.as_ref() {
        "gpointer" => true,
        _ => false,
    }
}

