/// Bindings for the lgi, the standard lua loader

use std::io::Write;
use crate::library::*;

struct LuaDoc {}
struct LuaCodegen {}
type Result<T> = std::io::Result<T>;

// we need 3 versions of show_function. 1 to only show the name (Function.name?),
// show function with without definition, types or return (but arguments)
// then show with everything: function my_function(args) (used for lsp)
impl LuaDoc {
    fn show_function(&self, fun: &Function) -> String {
    }

    fn show_callback(&self, fun: &Function) -> String {
    }

    fn show_class(&self, class: &Class) -> String {
    }

    fn show_enum(&self, enu: &Enumeration) -> String {
    }

    fn show_record(&self, rec: &Record) -> String {
    }

    fn show_constant(&self, constant: &Constant) -> String {
    }

    fn show_type(&self, typ: AnyType) -> String {
    }
}


impl Namespace {
    pub fn gen<W: Write>(&self, ns: Option<String>, w: &mut W) -> Result<()> {
        let name = self.name.unwrap_or_else(|| ns.unwrap_or("".to_owned()));
        writeln!(w, "local {} = {{}}", name);
        // set up meta table

        
        if !self.classes.is_empty() {
            // create_section(&self.ns, "Functions", w)?;
            for class in self.classes {
                class.gen(name, w);
                writeln!(w, "{}.{} = {}", name, class.name, class.name);
            }
        }
        if !self.functions.is_empty() {
            for function in self.functions {
                function.gen(name, w);
                writeln!(w, "{}.{} = {}", name, function.name, function.name);
            }
        }
        let ns = Some(name);
        if !self.enums.is_empty() {
            for enu in self.enums {
                enu.gen(ns, w);
                writeln!(w, "{}.{} = {}", name, enu.name, enu.name);
            }
        }
        if !self.record.is_empty() {
            for record in self.record {
                record.gen(ns, w);
                writeln!(w, "{}.{} = {}", name, record.name, record.name);
            }
        }
        if !self.constant.is_empty() {
            for cons in self.constant {
                cons.gen(ns, w);
                writeln!(w, "{}.{} = {}", name, cons.name, cons.name);
            }
        }
        if !self.bitfield.is_empty() {
            for bitfield in self.bitfield {
                bitfield.gen(ns, w);
                writeln!(w, "{}.{} = {}", name, bitfield.name, bitfield.name);
            }
        }
        w.flush()?;
        Ok(())
    }
}

impl Class {
    pub fn gen<W: Write>(&self, ns: String, w: &mut W) -> Result<()> {
        let name = self.name;
        writeln!(w, "local {} = {{}}", name);
        // 

        if !self.fields.is_empty() {
            for field in self.fields {
                // let str = format!("{}.{}", static_ns, field.0);
                // w.write_all(&str.as_bytes())?;
                writeln!(w, "")?;
                writeln!(w, "{}.{} = {}", name, field.name, field.name);
            }
        }

        // constructors are speciall because they return an object?
        if !self.constructor.is_empty() {
            for constructor in self.constructor {
                constructor.gen(name, w);
                writeln!(w, "{}.{} = {}", name, constructor.name, constructor.name);
            }
        }
        if !self.method.is_empty() {
            for method in self.method {
                writeln!(w, "")?;
                writeln!(w, "{}.{} = {}", name, method.name, method.name);
            }
        }
        if !self.functions.is_empty() {
            for func in self.functions {
                writeln!(w, "")?;
                writeln!(w, "{}.{} = {}", name, func.name, func.name);
            }
        }
        if !self.virtual_method.is_empty() {
            for virt in self.virtual_method {
                writeln!(w, "")?;
                writeln!(w, "{}.{} = {}", name, virt.name, virt.name);
            }
        }

        Ok(())
    }
}

impl Function {
    pub fn gen<W: Write>(&self, ns: String, w: &mut W) -> Result<()> {
        Ok(())
    }
}

impl Enumeration {
    pub fn gen<W: Write>(&self, ns: Option<String>, w: &mut W) -> Result<()> {
        Ok(())
    }
}

impl Record {
    pub fn gen<W: Write>(&self, ns: Option<String>, w: &mut W) -> Result<()> {
        Ok(())
    }
}

impl Constant {
    pub fn gen<W: Write>(&self, ns: Option<String>, w: &mut W) -> Result<()> {
        Ok(())
    }
}

impl Bitfield {
    pub fn gen<W: Write>(&self, ns: Option<String>, w: &mut W) -> Result<()> {
        Ok(())
    }
}


pub trait Langbinding {

    // When parsing an arg in a callable, how should an arg be translated?
    // Look at Translate for more info.
    fn translate_arg(&self, arg: &str) -> Translate;

    // something namespace

    // remove?
    fn filter(&self, typ: &str) -> bool;

    // We use only_introspectable to filter out what definitions are acceable
    // for the bindings. For example C can use all functions where lua can only 
    // use introspectable functions.
    
    fn write_static(&self, top_ns: &str, local_ns: Option<&str>, id: &str) -> String;
    fn write_local(&self, top_ns: &str, local_ns: &str, id: &str) -> String;
    fn only_introspectable(&self) -> bool {
        true
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
