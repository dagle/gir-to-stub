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

// TODO have a target
impl Generator for LuaCodegen {
    fn gen(&self, filename: &str) -> Result<()> {
        let types = "types";
        if !is_dir(types) {
            fs::create_dir(types)?;
        }
        generate_gobject(types)?;
        let mut path = Path::new(types).join(filename);
        path.set_extension("lua");
        let mut out_file = fs::File::create(path)?;
        let in_file = open_gir(filename)?;
        let repo = parse::parse_gir(in_file).expect("Couldn't parse gir file");
        repo.namespace[0].gen(&mut out_file)?;

        Ok(())
    }
}

fn generate_gobject<P: AsRef<Path>>(path: P) -> Result<()> {
    let path = Path::new(path.as_ref()).join("GObject.lua");
    let mut w = &fs::File::create(path)?;

    writeln!(w, "--- @class GObject.Object")?;
    writeln!(w, "local Object = {{}}")?;

    Ok(())
}

macro_rules! section {
    ( $w:ident, $self:ident, $name:ident, $section:ident ) => {
        {
            if !$self.$section.is_empty() {
                // create_section(&$name, stringify!($section), $w)?;
                for section in $self.$section.iter() {
                    section.gen(&$name, $w)?;
                }
            }
        };
    }
}

impl Namespace {
    pub fn gen<W: Write>(&self, w: &mut W) -> Result<()> {
        let name = self.name.as_ref().unwrap();
        writeln!(w, "---@diagnostic disable: unused-local")?;
        writeln!(w, "---@meta")?;
        writeln!(w, "local {} = {{}}\n", &name)?;

        for types in self.classes.iter() {
            types.gen_type(name, w)?;
        }
        for types in self.record.iter() {
            types.gen_type(name, w)?;
        }
        for types in self.callback.iter() {
            types.gen_callback(name, w)?;

        }
        section!(w, self, name, enums);
        section!(w, self, name, bitfield);

        // functions are different
        if !self.functions.is_empty() {
            // create_section(&name, "Function", w)?;
            for function in self.functions.iter() {
                function.gen(&name, &name, w)?;
            }
        }


        section!(w, self, name, classes);
        section!(w, self, name, record);
        section!(w, self, name, constant);
        section!(w, self, name, alias);
        section!(w, self, name, unions);
        writeln!(w, "return {}", &name)?;
        w.flush()?;
        Ok(())
    }
}

impl Alias {
    pub fn gen<W: Write>(&self, ns: &str, w: &mut W) -> Result<()> {
        writeln!(w, "--- @alias {}.{} {}", ns, &self.name, show_anytyp(&self.typ, ns))?;
        Ok(())
    }
}

impl Class {
    pub fn gen_type<W: Write>(&self, ns: &str, w: &mut W) -> Result<()> {
        writeln!(w, "-- A class")?;
        if let Some(ref parent) = self.parent {
            writeln!(w, "--- @class {}.{} : {}", ns, self.name, translate_ns(parent, &ns))?;
        } else {
            writeln!(w, "--- @class {}.{}", ns, self.name)?;
        }
        section!(w, self, ns, signals);
        section!(w, self, ns, fields);
        section!(w, self, ns, properties);
        writeln!(w, "local {} = {{}}", self.name)?;
        Ok(())
    }
    pub fn gen<W: Write>(&self, ns: &str, w: &mut W) -> Result<()> {

        let class_ns = format!("{}.{}", ns, self.name);

        // section!(w, self, class_ns, implements);

        if !self.constructor.is_empty() {
            for constructor in self.constructor.iter() {
                constructor.gen_constructor(&class_ns, ns, w)?;
            }
        }
        if !self.method.is_empty() {
            for method in self.method.iter() {
                method.gen_method(&self.name, ns, w)?;
            }
        }
        if !self.functions.is_empty() {
            for func in self.functions.iter() {
                func.gen(&class_ns, ns, w)?;
            }
        }
        // TODO: Should we re-add this in some way?

        // if !self.virtual_method.is_empty() {
        //     for virt in self.virtual_method.iter() {
        //         virt.gen_method(&class_ns, ns, w)?;
        //     }
        // }
        if !self.callbacks.is_empty() {
            for callback in self.callbacks.iter() {
                callback.gen(&class_ns, ns, w)?;
            }
        }

        section!(w, self, class_ns, record);
        section!(w, self, class_ns, unions);
        section!(w, self, class_ns, constant);

        writeln!(w, "--- @param obj GObject.Object")?;
        writeln!(w, "--- @return boolean")?;
        writeln!(w, "function {}:is_type_of(obj) end", class_ns)?;
        Ok(())
    }
}

impl Implement {
    pub fn gen<W: Write>(&self, _ns: &str, w: &mut W) -> Result<()> {
        writeln!(w, "-- implements: {}", self.name)
    }
}

impl Union {
    /// this is like a record (but not), we should generate it the same way
    pub fn gen<W: Write>(&self, ns: &str, w: &mut W) -> Result<()> {
        if let Some(ref name) = self.name {
        // writeln!(w, "{}.{} = union", &ns, self.name.as_ref().unwrap())?;
            // println!("{}.{} = {:#?}", &ns, name, self);
        }
        Ok(())
    }
}

impl Field {
    pub fn gen<W: Write>(&self, ns: &str, w: &mut W) -> Result<()> {
        if !self.private {
            let typ = show_anytyp(&self.typ, ns);
            writeln!(w, "--- @field {} {}", self.name, typ)?;
        }
        Ok(())
    }
}

impl Property {
    pub fn gen<W: Write>(&self, ns: &str, w: &mut W) -> Result<()> {
        let typ = show_anytyp(&self.typ, ns);
        writeln!(w, "--- @field {} {}", translate_name(&self.name), typ)
    }
}

fn gen_return_signal(fun: &Signal, ns: &str) -> Option<String> {
    let mut params = vec![];

    fun.ret.as_ref().map(|p| params.push(p));
    for p in fun.parameters.iter().filter(|p| out_param(&p.direction)) {
        params.push(p)
    }

    if params.len() > 0 {
        return None
    }

    let param_names: Vec<String> = params
        .iter()
        .map(|p| show_anytyp(&p.typ, &ns))
        .collect();
    Some(param_names.join(", "))
}

fn translate_name(str: &str) -> String {
    format!("on_{}", str.replace("-", "_"))
}

impl Signal {
    pub fn gen<W: Write>(&self, ns: &str, w: &mut W) -> Result<()> {
        let mut param_names = gen_param_names_typed(&self.parameters, ns);
        param_names.insert(0, "self".to_string());
        let param_names = param_names.join(", ");
        let name = translate_name(&self.name);
        if let Some(ret) = gen_return_signal(self, ns) {
            writeln!(w, "--- @field {} fun({}):{}", name, param_names, ret)?;
        } else {
            writeln!(w, "--- @field {} fun({})", name, param_names)?;
        }
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
        AnyType::Array(array) => {
            let typ = array.typ.clone();
            let mut array = translate(&typ, ns);
            array.push_str("[]");
            array
        },
        AnyType::Type(typ) => {
            if let Some(name) = &typ.name {
                match name.as_ref() {
                    "GLib.SList" | "GLib.List" => 
                        format!("{}[]", show_anytyp(&typ.children[0], ns)),
                    "GLib.HashTable" => {
                        let key = show_anytyp(&typ.children[0], ns);
                        let value = show_anytyp(&typ.children[1], ns);
                        format!("table<{}, {}>", key, value)
                    },
                    _ => translate(&name, ns),
                }
            } else {
                "any".to_string()
            }
            // translate(&typ.name, ns)
        }
        AnyType::VarArg => "...".to_owned(),
    }
}

// todo we should just remove const etc from the type
fn translate(name: &str, ns: &str) -> String {
    match name {
        "gboolean" => "boolean".to_string(),
        "gpointer" => "any".to_string(),
        "GType" => "Glib.GType".to_string(),
        "gint" | "guint" 
            | "gint8" | "guint8"
            | "gint16" | "guint16"
            | "gint32" | "guint32"
            | "gint64" | "guint64"
            | "gsize" | "gssize" => "number".to_string(),
            "glong"| "gulong"
                | "glong64"| "gulong64"
                | "gshort"| "gushort"
                | "gshort64"| "gushort64"
                | "gfloat"| "gdouble" => "number".to_string(),
                "const char*"|"char*"
                    | "gchar" | "guchar"
                    | "string" | "GString"
                    | "utf8" => "string".to_string(),
                "none" => "nil".to_string(),
                rest => {
                    translate_ns(rest, ns)
                }
    }
}

fn translate_ns(name: &str, ns: &str) -> String {
    if !name.contains(".") {
        return format!("{}.{}", ns, name)
    }
    name.to_string()
}

fn gen_infoattr<W: Write>(info: &InfoAttrs, w: &mut W) -> Result<()> {
    if let Some(true) = info.deprecated {
        writeln!(w, "--- @deprecated")?;
    }
    // if let Some(ref dep_version) = info.deprecated {
    // }
    // if let Some(ref version) = info.deprecated {
    //     writeln!(w, "---@version {}", version)?;
    // }
    // if let Some(ref stability) = info.deprecated {
    // }
    Ok(())
}

// TODO change %NULL to nil?
// TODO gen_info_attr and infoelements should be done together?
fn gen_doc<W: Write>(doc: &InfoElements, w: &mut W) -> Result<()> {
    if let Some(ref docs) = doc.doc {
        let luafied = docs.content.replace("%NULL", "nil");
        let lines = luafied.split("\n");
        for line in lines {
            writeln!(w, "--- {}", line)?;
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

fn optional(param: &Parameter) -> &str {
    if param.optional || param.allow_none {
        return "?"
    }
    ""
}

fn gen_doc_param<W: Write>(param: &Parameter, ns: &str, w: &mut W) -> Result<()> {
    let type_str = show_anytyp(&param.typ, ns);
    let opt = optional(param);
    if let Some(ref doc) = param.doc.doc {
        let luafied = doc.content.replace("%NULL", "nil");
        let docstr = luafied.replace("\n", "");
        writeln!(w, "--- @param {} {}{} {}", unkeyword(&param.name), type_str, opt, docstr)?;
    } else {
        writeln!(w, "--- @param {} {}{}", unkeyword(&param.name), opt, type_str)?;
    }
    Ok(())
}

fn gen_doc_params<W: Write>(params: &Vec<Parameter>, ns: &str, w: &mut W) -> Result<()> {
    for param in params.iter().filter(|p| in_param(&p.direction)) {
        gen_doc_param(&param, ns, w)?;
    }
    Ok(())
}

fn gen_doc_return<W: Write>(fun: &Function, ns: &str, w: &mut W) -> Result<()> {
    let mut params = vec![];

    fun.ret.as_ref().map(|p| params.push(p));
    for p in fun.parameters.iter().filter(|p| out_param(&p.direction)) {
        params.push(p)
    }
    if !params.is_empty() {
        let mut rets = vec![];
        for param in params.iter() {
            let mut type_str = show_anytyp(&param.typ, ns);
            if type_str != "nil" {
                if param.nullable {
                    type_str = format!("{}|nil", type_str);
                }
                rets.push(type_str);
            }
        }
        if rets.len() > 0 {
            let retlist = rets.join(", ");
            writeln!(w, "--- @return {}", retlist)?;
        }
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
    false
}

fn gen_param_names(params: &Vec<Parameter>) -> String {
    let param_names: Vec<String> = params
        .iter()
        .filter(|p| in_param(&p.direction))
        .map(|p| unkeyword(&p.name))
        .collect();
    param_names.join(", ")
}

fn gen_param_names_typed(params: &Vec<Parameter>, ns: &str) -> Vec<String> {
    params
        .iter()
        .filter(|p| in_param(&p.direction))
        .map(|p| format!("{}: {}", unkeyword(&p.name), show_anytyp(&p.typ, &ns)))
        .collect()
}

fn gen_return_names_typed(fun: &Function, ns: &str) -> Option<String> {
    let mut params = vec![];

    fun.ret.as_ref().map(|p| params.push(p));
    for p in fun.parameters.iter().filter(|p| out_param(&p.direction)) {
        params.push(p)
    }

    if params.len() > 0 {
        return None
    }

    let param_names: Vec<String> = params
        .iter()
        .map(|p| show_anytyp(&p.typ, &ns))
        .collect();
    Some(param_names.join(", "))
}

impl Function {
    pub fn gen<W: Write>(&self, ns: &str, root_ns: &str, w: &mut W) -> Result<()> {
        gen_infoattr(&self.info, w)?;
        let introspectable = self.info.introspectable.unwrap_or(true);
        if introspectable {
            gen_doc(&self.doc, w)?;
            gen_doc_params(&self.parameters, root_ns, w)?;
            gen_doc_return(&self, root_ns, w)?;
            let param_names = gen_param_names(&self.parameters);
            writeln!(w, "function {}.{}({}) end\n", &ns, self.name, param_names)?;
        }
        Ok(())
    }
    pub fn gen_constructor<W: Write>(&self, ns: &str, root_ns: &str, w: &mut W) -> Result<()> {
        let introspectable = self.info.introspectable.unwrap_or(true);
        if introspectable {
            gen_doc(&self.doc, w)?;
            gen_doc_params(&self.parameters, root_ns, w)?;
            gen_doc_return(&self, root_ns, w)?;
            let param_names = gen_param_names(&self.parameters);
            writeln!(w, "function {}.{}({}) end\n", &ns, self.name, param_names)?;
        }
        Ok(())
    }
    pub fn gen_callback<W: Write>(&self, ns: &str, w: &mut W) -> Result<()> {
        let param_names = gen_param_names_typed(&self.parameters, ns).join(", ");
        if let Some(ret) = gen_return_names_typed(&self, ns) {
            writeln!(w, "--- @alias {}.{} fun({}):{}", ns, self.name, param_names, ret)?;
        } else {
            writeln!(w, "--- @alias {}.{} fun({})", ns, self.name, param_names)?;
        }
        Ok(())
    }
    pub fn gen_method<W: Write>(&self, ns: &str, root_ns: &str, w: &mut W) -> Result<()> {
        gen_infoattr(&self.info, w)?;
        let introspectable = self.info.introspectable.unwrap_or(true);
        if introspectable {
            gen_doc(&self.doc, w)?;
            gen_doc_params(&self.parameters, root_ns, w)?;
            gen_doc_return(&self, root_ns, w)?;
            let param_names = gen_param_names(&self.parameters);
            writeln!(w, "function {}:{}({}) end\n", &ns, self.name, param_names)?;
        }
        Ok(())
    }
    pub fn gen_member<W: Write>(&self, root_ns: &str, w: &mut W) -> Result<()> {
        let introspectable = self.info.introspectable.unwrap_or(true);
        if introspectable {
            gen_doc(&self.doc, w)?;
            gen_doc_params(&self.parameters, root_ns, w)?;
            gen_doc_return(&self, root_ns, w)?;
            let param_names = gen_param_names(&self.parameters);
            writeln!(w, "\t[\"{}\"] = function({}) end,\n", self.name.to_uppercase(), param_names)?;
        }
        Ok(())
    }
}

impl Enumeration {
    pub fn gen<W: Write>(&self, ns: &str, w: &mut W) -> Result<()> {
        writeln!(w, "--- @enum {}.{}", &ns, self.name)?;
        writeln!(w, "{}.{} = {{", &ns, self.name)?;
        for mem in self.members.iter() {
            mem.gen(w)?;
        }
        for func in self.functions.iter() {
            func.gen_member(ns, w)?;
        }
        writeln!(w, "}}")
    }
}

impl Member {
    pub fn gen<W: Write>(&self, w: &mut W) -> Result<()> {
        writeln!(w, "\t[\"{}\"] = {},", self.name.to_uppercase(), self.value)
    }
}

impl Record {
    pub fn gen_type<W: Write>(&self, ns: &str, w: &mut W) -> Result<()> {
        writeln!(w, "-- A record")?;
        writeln!(w, "--- @class {}.{}", ns, self.name)?;
        section!(w, self, ns, fields);
        writeln!(w, "local {} = {{}}", self.name)
    }
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
        // if !self.fields.is_empty() {
        //     for field in self.fields.iter() {
        //         field.gen(&record_ns, w)?;
        //     }
        // }
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
        writeln!(w, "--- @enum {}.{}", &ns, self.name)?;
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
