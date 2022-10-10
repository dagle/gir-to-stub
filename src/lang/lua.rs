/// Bindings for the lgi, the standard lua loader

use std::{fs::File, io::Write};
use std::fs;
use crate::{library::*, parse};
use std::path::Path;

// The amount of code/doc we should generate
// Generating full docs for a mock file could be
// a bit to much for a lsp.
pub enum Level {
    Code,
    CodeDoc,
    Full,
}

impl std::fmt::Display for Level {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Level::Code => { f.write_str("Code")}
            Level::CodeDoc => {f.write_str("CodeDoc")}
            Level::Full => {f.write_str("Full")}
        }
    }
}

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
        // /usr/share/gir-1.0/
        let types = "types";
        if is_dir(types) {
            fs::create_dir(types)?;
        }
        let mut path = Path::new(types).join(filename);
        path.set_extension(".lua");
        let mut out_file = fs::File::create(path)?;
        let in_file = open_gir(filename)?;
        let repo = parse::parse_gir(in_file).expect("Couldn't parse gir file");
        repo.namespace[0].gen(&mut out_file)?;

        Ok(())
    }
}

impl Namespace {
    pub fn gen<W: Write>(&self, w: &mut W) -> Result<()> {
        // let name = self.name.unwrap_or_else("".to_owned());
        let name = self.name.as_ref().unwrap();
        // writeln!(w, "local {} = {{}}", &name);
        // set up meta table

        
        if !self.classes.is_empty() {
            // create_section(&self.ns, "Functions", w)?;
            for class in self.classes.iter() {
                // class.gen(&name, w);
                // writeln!(w, "{}.{} = {}", &name, class.name, class.name);
            }
        }
        if !self.functions.is_empty() {
            for function in self.functions.iter() {
                function.gen(&name, w);
                // writeln!(w, "{}.{} = {}", &name, function.name, function.name);
            }
        }
        if !self.enums.is_empty() {
            for enu in self.enums.iter() {
                enu.gen(&name, w);
                // writeln!(w, "{}.{} = {}", &name, enu.name, enu.name);
            }
        }
        if !self.record.is_empty() {
            for record in self.record.iter() {
                record.gen(&name, w);
                // writeln!(w, "{}.{} = {}", &name, record.name, record.name);
            }
        }
        if !self.constant.is_empty() {
            for cons in self.constant.iter() {
                cons.gen(&name, w);
                // writeln!(w, "{}.{} = {}", &name, cons.name, cons.name);
            }
        }
        if !self.bitfield.is_empty() {
            for bitfield in self.bitfield.iter() {
                bitfield.gen(&name, w);
                // writeln!(w, "{}.{} = {}", &name, bitfield.name, bitfield.name);
            }
        }
        w.flush()?;
        Ok(())
    }
}

impl Class {
    // pub fn gen<W: Write>(&self, ns: &str, w: &mut W) -> Result<()> {
    //     // let name = self.name;
    //     writeln!(w, "local {} = {{}}", name);
    //     // 
    //
    //     if !self.fields.is_empty() {
    //         for field in self.fields {
    //             // let str = format!("{}.{}", static_ns, field.0);
    //             // w.write_all(&str.as_bytes())?;
    //             writeln!(w, "")?;
    //             writeln!(w, "{}.{} = {}", name, field.name, field.name);
    //         }
    //     }
    //
    //     // constructors are speciall because they return an object?
    //     if !self.constructor.is_empty() {
    //         for constructor in self.constructor {
    //             constructor.gen(&name, w);
    //             writeln!(w, "{}.{} = {}", name, constructor.name, constructor.name);
    //         }
    //     }
    //     if !self.method.is_empty() {
    //         for method in self.method {
    //             writeln!(w, "")?;
    //             writeln!(w, "{}.{} = {}", name, method.name, method.name);
    //         }
    //     }
    //     if !self.functions.is_empty() {
    //         for func in self.functions {
    //             writeln!(w, "")?;
    //             writeln!(w, "{}.{} = {}", name, func.name, func.name);
    //         }
    //     }
    //     if !self.virtual_method.is_empty() {
    //         for virt in self.virtual_method {
    //             writeln!(w, "")?;
    //             writeln!(w, "{}.{} = {}", name, virt.name, virt.name);
    //         }
    //     }
    //
    //     Ok(())
    // }
}

impl Function {
    pub fn gen<W: Write>(&self, ns: &str, w: &mut W) -> Result<()> {
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
