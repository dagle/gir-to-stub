use std::{fs::File, io::Write};
use std::fs;
use crate::{library::*, parse};
use std::path::Path;
use super::*;

// The amount of code/doc we should generate
// Generating full docs for a mock file could be
// a bit to much for a lsp.

// struct LuaDoc {}
pub struct LuaCodegen {
    level: Level
}

type Result<T> = std::io::Result<T>;

impl LuaCodegen {
    pub fn new(level: Level) -> LuaCodegen {
        LuaCodegen{
            level,
        }
    }
}
impl Generator for LuaCodegen {
    fn gen(&self, filename: &str) -> Result<()> {
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

// #[macro_export]
macro_rules! section {
    ( $w:ident, $self:ident, $name:ident, $section:ident ) => {
        {
            if !$self.$section.is_empty() {
                create_section(&$name, stringify!($section), $w)?;
                for section in $self.$section.iter() {
                    section.gen(&$name, $w)?;
                }
            }
        };
    }
}

impl Namespace {
    pub fn gen<W: Write>(&self, w: &mut W) -> Result<()> {
        // let name = self.name.unwrap_or_else("".to_owned());
        let name = self.name.as_ref().unwrap();
        writeln!(w, "local {} = {{}}\n", &name)?;

        section!(w, self, name, classes);

        // functions are different
        if !self.functions.is_empty() {
            create_section(&name, "Function", w)?;
            for function in self.functions.iter() {
                function.gen(&name, &name, w)?;
            }
        }

        section!(w, self, name, enums);
        section!(w, self, name, record);
        section!(w, self, name, constant);
        section!(w, self, name, bitfield);
        section!(w, self, name, alias);
        section!(w, self, name, unions);
        writeln!(w, "return {}", &name)?;
        w.flush()?;
        Ok(())
    }
}

impl Alias {
    pub fn gen<W: Write>(&self, ns: &str, w: &mut W) -> Result<()> {
        Ok(())
    }
}

impl Class {
    pub fn gen<W: Write>(&self, ns: &str, w: &mut W) -> Result<()> {
        // writeln!(w, "-- @class {}", self.name)?;

        let class_ns = format!("{}.{}", ns, self.name);

        section!(w, self, class_ns, implements);

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
        if !self.callbacks.is_empty() {
            for virt in self.virtual_method.iter() {
                virt.gen(&class_ns, ns, w)?;
            }
        }

        section!(w, self, class_ns, record);
        section!(w, self, class_ns, fields);
        section!(w, self, class_ns, signals);
        section!(w, self, class_ns, unions);
        section!(w, self, class_ns, constant);
        section!(w, self, class_ns, properties);
        Ok(())
    }
}

impl Implement {
    pub fn gen<W: Write>(&self, _ns: &str, w: &mut W) -> Result<()> {
        writeln!(w, "-- implements: {}", self.name)?;
        Ok(())
    }
}

impl Union {
    pub fn gen<W: Write>(&self, ns: &str, w: &mut W) -> Result<()> {
        writeln!(w, "{}.{} = union", &ns, self.name.as_ref().unwrap())?;
        Ok(())
    }
}
impl Property {
    pub fn gen<W: Write>(&self, ns: &str, w: &mut W) -> Result<()> {
        writeln!(w, "-- {}.{} = prop", &ns, self.name)?;
        Ok(())
    }
}
impl Signal {
    pub fn gen<W: Write>(&self, ns: &str, w: &mut W) -> Result<()> {
        writeln!(w, "-- {}.{} = signal", &ns, self.name)?;
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

fn show_anytyp(typ: &AnyType, ns: &str) -> String {
    match typ {
        AnyType::Array(_) => "array".to_owned(), // fix this
        AnyType::Type(typ) => translate(&typ.name, ns),
        AnyType::VarArg => "...".to_owned(),
    }
}

// todo we should just remove const etc from the type
fn translate(name: &Option<String>, ns: &str) -> String {
    if let Some(typ_str) = name {
        match typ_str.as_ref() {
            "gboolean" => "boolean".to_string(),
            "gpointer" => "any".to_string(),
            "gint" | "guint" 
                | "gint8" | "guint8"
                | "gint16" | "guint16"
                | "gint32" | "guint32"
                | "gint64" | "guint64"
                | "gszise" | "gssize" => "num".to_string(),
            "glong"| "gulong"
                | "glong64"| "gulong64"
                | "gshort"| "gushort"
                | "gshort64"| "gushort64"
                | "gfloat"| "gdouble" => "num".to_string(),
            "const char*"|"char*"
                | "gchar" | "guchar"
                | "string" | "GString"
                | "utf8" => "string".to_string(),
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
        // TODO can we progate the types?
        // writeln!(w, "-- {}.{} = {{}}", &ns, self.name, typ)?;
        writeln!(w, "-- {}.{} = {{}}", &ns, self.name)?;
        Ok(())
    }
}

fn gen_infoattr<W: Write>(info: &InfoAttrs, w: &mut W) -> Result<()> {
    if let Some(_) = info.deprecated {
        writeln!(w, "@deprecated")?;
    }
    // if let Some(ref dep_version) = info.deprecated {
    // }
    if let Some(ref version) = info.deprecated {
        writeln!(w, "@version {}", version)?;
    }
    // if let Some(ref stability) = info.deprecated {
    // }
    Ok(())
}

// TODO change %NULL to nil?
// TODO gen_info_attr and infoelements should be done together?
fn gen_doc<W: Write>(doc: &InfoElements, w: &mut W) -> Result<()> {
    if let Some(ref docs) = doc.doc {
        let lines = docs.content.split("\n");
        for line in lines {
            writeln!(w, "-- {}", line)?;
        }
    }
    // if let Some(ref stability) = doc.doc_stability {
    // }
    // if let Some(ref version) = doc.doc_version {
    // }
    // if let Some(ref deprecated) = doc.doc_deprecated {
    // }
    // if let Some(ref pos) = doc.doc_pos {
    // }
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

fn gen_doc_return<W: Write>(fun: &Function, ns: &str, w: &mut W) -> Result<()> {
    let mut params = vec![];

    fun.ret.as_ref().map(|p| params.push(p));
    let _ = fun.parameters.iter().filter(|p| out_param(&p.direction)).map(|p| params.push(p));
    if !params.is_empty() {
        write!(w, "--- @return")?;
        for param in params.iter() {
            let type_str = show_anytyp(&param.typ, ns);
            write!(w, " {}", type_str)?;
        }
        writeln!(w)?;
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
    pub fn gen<W: Write>(&self, ns: &str, root_ns: &str, w: &mut W) -> Result<()> {
        let introspectable = self.info.introspectable.unwrap_or(true);
        if introspectable {
            gen_doc(&self.doc, w)?;
            gen_doc_params(&self.parameters, root_ns, w)?;
            gen_doc_return(&self, root_ns, w)?;
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
            gen_doc_return(&self, root_ns, w)?;
            let param_names = gen_param_names(&self.parameters, true);
            writeln!(w, "function {}.{}({}) end", &ns, self.name, param_names)?;
        }
        Ok(())
    }
    pub fn gen_member<W: Write>(&self, root_ns: &str, w: &mut W) -> Result<()> {
        let introspectable = self.info.introspectable.unwrap_or(true);
        if introspectable {
            gen_doc(&self.doc, w)?;
            gen_doc_params(&self.parameters, root_ns, w)?;
            gen_doc_return(&self, root_ns, w)?;
            let param_names = gen_param_names(&self.parameters, true);
            writeln!(w, "\t[\"{}\"] = function({}) end,", self.name.to_uppercase(), param_names)?;
        }
        Ok(())
    }
}

impl Enumeration {
    pub fn gen<W: Write>(&self, ns: &str, w: &mut W) -> Result<()> {
        writeln!(w, "{}.{} = {{", &ns, self.name)?;
        for mem in self.members.iter() {
            mem.gen(w)?;
        }
        for func in self.functions.iter() {
            func.gen_member(ns, w)?;
        }
        writeln!(w, "}}", )?;
        Ok(())
    }
}

impl Member {
    pub fn gen<W: Write>(&self, w: &mut W) -> Result<()> {
        writeln!(w, "\t[\"{}\"] = {},", self.name.to_uppercase(), self.value)?;
        Ok(())
    }
}

impl Record {
    pub fn gen<W: Write>(&self, ns: &str, w: &mut W) -> Result<()> {
        let record_ns = format!("{}.{}", ns, self.name);

        if !self.constructor.is_empty() {
            for constructor in self.constructor.iter() {
                constructor.gen(&record_ns, ns, w)?;
            }
        }
        if !self.method.is_empty() {
            for method in self.method.iter() {
                method.gen_method(&record_ns, ns, w)?;
            }
        }
        if !self.functions.is_empty() {
            for func in self.functions.iter() {
                func.gen(&record_ns, ns, w)?;
            }
        }
        if !self.fields.is_empty() {
            for field in self.fields.iter() {
                field.gen(&record_ns, w)?;
            }
        }
        if !self.unions.is_empty() {
            for unio in self.unions.iter() {
                unio.gen(&record_ns, w)?;
            }
        }
        Ok(())
    }
}

impl Constant {
    pub fn gen<W: Write>(&self, ns: &str, w: &mut W) -> Result<()> {
        if self.value.parse::<u32>().is_ok() {
            writeln!(w, "{}.{} = {}", ns, self.name, self.value)
        } else {
            writeln!(w, "{}.{} = \"{}\"", ns, self.name, self.value)
        }
    }
}

impl Bitfield {
    pub fn gen<W: Write>(&self, ns: &str, w: &mut W) -> Result<()> {
        writeln!(w, "{}.{} = {{", &ns, self.name)?;
        for mem in self.members.iter() {
            mem.gen(w)?;
        }
        for func in self.functions.iter() {
            func.gen_member(ns, w)?;
        }
        writeln!(w, "}}", )?;
        Ok(())
    }
}


fn filter(typ: &str) -> bool {
    match typ.as_ref() {
        "gpointer" => true,
        _ => false,
    }
}

