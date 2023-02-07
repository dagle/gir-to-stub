use std::{ffi::OsStr, io::BufWriter};
use std::io::{Write, Read};
use std::fs;
use crate::{library::*, parse};
use std::path::Path;
use super::*;
use anyhow::{Result, Context};

// The amount of code/doc we should generate
// Generating full docs for a mock file could be
// a bit to much for a lsp.

// struct LuaDoc {}
pub struct LuaCodegen {
    // level: Level
}

impl LuaCodegen {
    pub fn new() -> LuaCodegen {
        LuaCodegen{}
    }
}

impl LuaCodegen {
    fn gen<R: Read>(&self, r: R,  p: &Path) -> Result<()> {
        let repo = parse::parse_gir(r).expect("Couldn't parse gir file");

        repo.namespace[0].gen(p)?;
        Ok(())
    }
}

fn fix_filename(str: &str) -> String {
    str.chars()
    .map(|x| match x { 
        '-' | '.' => '_', 
        _ => x
    }).collect()
}

impl Generator for LuaCodegen {
    fn genfile(&self, filename: &str, output_dir: Option<&str>) -> Result<()> {
        let path = Path::new(filename);
        if path.extension() != Some(OsStr::new("gir")) {
            return Err(anyhow::anyhow!(format!("{} Filetype isn't gir", path.to_string_lossy())))
        }
        let file = path.file_stem().ok_or_else(|| 
            anyhow::anyhow!(format!("Cannot get filename for outputwriter")))?;
        let file = fix_filename(file.to_str().ok_or_else(||
            anyhow::anyhow!(format!("Cannot convert filename")))?);

        let output_dir = Path::new(output_dir.unwrap_or("types"));
        let output_dir = output_dir.join(file);
        if !output_dir.is_dir() {
            fs::create_dir(&output_dir)?;
        }

        generate_gobject(&output_dir)?;
        // let path = output_dir.join("init.lua");
        // path.set_extension("lua");
        // let mut out_file = BufWriter::new(fs::File::create(path)?);
        let in_file = open_gir(filename)?;
        self.gen(in_file, &output_dir)?;
        Ok(())
    }
}

fn generate_gobject<P: AsRef<Path>>(path: &P) -> Result<()> {
    let path = Path::new(path.as_ref()).join("GObject.lua");
    let mut w = &fs::File::create(path)?;

    writeln!(w, "--- @class GObject.Object")?;
    writeln!(w, "local Object = {{}}")?;

    Ok(())
}

macro_rules! section {
    ( $w:expr, $self:ident, $name:ident, $section:ident ) => {
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

fn gen_file(ns: &str, p: &Path) -> Result<BufWriter<File>> {
    let mut path = p.join(ns);
    path.set_extension("lua");
    let mut w = BufWriter::new(fs::File::create(path)?);
    writeln!(w, "---@diagnostic disable: unused-local, duplicate-doc-field")?;
    writeln!(w, "---@meta")?;
    writeln!(w, "-- THIS FILE WAS GENERATED BY gir-to-stub! DO NOT MODIFY!\n")?;
    Ok(w)
}

impl Namespace {
    pub fn gen(&self, p: &Path) -> Result<()> {
        let name = self.name.as_ref().context("Failed to read name")?;
        let mut w = gen_file("init", p)?;
        writeln!(w, "local {} = {{}}\n", name)?;

        for types in self.record.iter() {
            types.gen_type(name, &mut w)?;
        }
        for types in self.callback.iter() {
            types.gen_callback_type(name, &mut w)?;
        }
        for types in self.unions.iter() {
            types.gen_type(name, &mut w)?;
        }
        writeln!(w)?;
        for class in self.classes.iter() {
            writeln!(w, "local _{} = require('{}')", class.name, class.name)?;
            // writeln!(w, "---@module {}", class.name)?;
            // writeln!(w, "local {}", class.name)?;
            writeln!(w, "{}.{} = _{}\n", name, class.name, class.name)?;
            class.gen(name, p)?;
        }
        for record in self.record.iter() {
            if record.name.ends_with("Class") {
                continue;
            }
            writeln!(w, "local _{} = require('{}')", record.name, record.name)?;
            writeln!(w, "{}.{} = _{}\n", name, record.name, record.name)?;
            record.gen(name, p)?;
        }
        // section!(&mut w, self, name, record);

        section!(&mut w, self, name, enums);
        section!(&mut w, self, name, bitfield);

        for function in self.functions.iter() {
            function.gen(&name, &name, &mut w)?;
        }

        section!(&mut w, self, name, constant);
        section!(&mut w, self, name, alias);
        section!(&mut w, self, name, unions);
        writeln!(&mut w, "return {}", name)?;
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

fn gen_default_construtor<W:Write>(ns: &str, constrs: &[Function], w: &mut W) -> Result<()> {
    for constr in constrs {
        if constr.name == "new" {
            if let Some(ret) = gen_return_names_typed(&constr, ns) {
                writeln!(w, "--- @overload fun(params: {{}}):{}", ret)?;
            }
            return Ok(())
        }
    }
    Ok(())
}

macro_rules! introspectable {
    ($id:ident) => {
        if let Some(false) = $id.info.introspectable {
            return Ok(())
        }
    };
}

impl Class {
    pub fn gen_type<W: Write>(&self, ns: &str, w: &mut W) -> Result<()> {
        introspectable!(self);
        self.info.gen(w)?;
        if let Some(ref parent) = self.parent {
            writeln!(w, "--- @class {}.{} : {}", ns, self.name, translate_ns(parent, &ns))?;
        } else {
            writeln!(w, "--- @class {}.{}", ns, self.name)?;
        }
        section!(w, self, ns, signals);
        section!(w, self, ns, fields);
        section!(w, self, ns, properties);
        gen_default_construtor(ns, &self.constructor, w)?;
        writeln!(w, "local {} = {{}}", self.name)?;
        Ok(())
    }
    pub fn gen(&self, ns: &str, p: &Path) -> Result<()> {
        introspectable!(self);
        let mut w = gen_file(&self.name, p)?;

        self.gen_type(ns, &mut w)?;

        // let class_ns = format!("{}.{}", ns, self.name);
        let class_ns = format!("{}", self.name);

        // section!(&mut w, self, class_ns, implements);

        for constructor in self.constructor.iter() {
            constructor.gen(&class_ns, ns, &mut w)?;
        }
        for method in self.method.iter() {
            method.gen(&self.name, ns, &mut w)?;
        }
        for func in self.functions.iter() {
            func.gen(&class_ns, ns, &mut w)?;
        }
        // TODO: Should we re-add this in some way?

        // if !self.virtual_method.is_empty() {
        //     for virt in self.virtual_method.iter() {
        //         virt.gen_method(&class_ns, ns, &mut w)?;
        //     }
        // }
        for callback in self.callbacks.iter() {
            callback.gen(&class_ns, ns, &mut w)?;
        }

        // section!(&mut w, self, class_ns, record);
        // section!(&mut w, self, class_ns, unions);
        // section!(&mut w, self, class_ns, constant);

        writeln!(w, "--- @param obj GObject.Object")?;
        writeln!(w, "--- @return boolean")?;
        writeln!(w, "function {}:is_type_of(obj) end", class_ns)?;

        writeln!(w, "return {}", &self.name)?;
        w.flush()?;

        Ok(())
    }
}

//
// impl Implement {
//     pub fn gen<W: Write>(&self, _ns: &str, w: &mut W) -> Result<()> {
//         Ok(writeln!(w, "-- implements: {}", self.name)?)
//     }
// }

impl Union {
    pub fn gen_type<W: Write>(&self, ns: &str, w: &mut W) -> Result<()> {
        introspectable!(self);

        if let Some(ref name) = self.name {
            writeln!(w, "--- @class {}.{}", ns, name)?;
            section!(w, self, ns, fields);
            writeln!(w, "local {} = {{}}", name)?;
        }
        Ok(())
    }
    /// this is like a record (but not), we should generate it the same way
    pub fn gen<W: Write>(&self, ns: &str, w: &mut W) -> Result<()> {
        introspectable!(self);
        if let Some(false) = self.info.introspectable {
            return Ok(())
        }
        if let Some(ref name) = self.name {
            let union_ns = format!("{}.{}", ns, name);

            for constructor in self.constructor.iter() {
                constructor.gen(&union_ns, ns, w)?;
            }
            for method in self.method.iter() {
                method.gen(&union_ns, ns, w)?;
            }
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
        Ok(writeln!(w, "--- @field {} {}", self.name.replace("-", "_"), typ)?)
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

fn signal_name(str: &str) -> String {
    format!("on_{}", str.replace("-", "_"))
}

impl Signal {
    pub fn gen<W: Write>(&self, ns: &str, w: &mut W) -> Result<()> {
        introspectable!(self);
        let mut param_names = gen_param_names_typed(&self.parameters, ns);
        param_names.insert(0, "self".to_string());
        let param_names = param_names.join(", ");
        let name = signal_name(&self.name);
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

impl InfoAttrs {
    fn gen<W: Write>(&self, w: &mut W) -> Result<()> {
        if let Some(true) = self.deprecated {
            writeln!(w, "--- @deprecated")?;
        }
        Ok(())
    }
}

impl InfoElements {
    fn gen<W: Write>(&self, w: &mut W) -> Result<()> {
        if let Some(ref docs) = self.doc {
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
}

fn optional(param: &Parameter) -> &str {
    if param.optional || param.allow_none {
        return "?"
    }
    ""
}

impl Parameter {
    fn gen<W: Write>(&self, ns: &str, w: &mut W) -> Result<()> {
        let type_str = show_anytyp(&self.typ, ns);
        let opt = optional(self);
        if let Some(ref doc) = self.doc.doc {
            let luafied = doc.content.replace("%NULL", "nil");
            let docstr = luafied.replace("\n", "");
            writeln!(w, "--- @param {} {}{} {}", unkeyword(&self.name), type_str, opt, docstr)?;
        } else {
            writeln!(w, "--- @param {} {}{}", unkeyword(&self.name), opt, type_str)?;
        }
        Ok(())
    }
}

fn gen_doc_params<W: Write>(params: &Vec<Parameter>, ns: &str, skip: bool, w: &mut W) -> Result<()> {
    let mut num = 0;
    if skip {
        num = 1;
    }
    for param in params.iter().skip(num).filter(|p| in_param(&p.direction)) {
        param.gen(ns,w)?;
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

fn gen_param_names(params: &Vec<Parameter>, skip: bool) -> String {
    let mut num = 0;
    if skip {
        num = 1;
    }
    let param_names: Vec<String> = params
        .iter()
        .skip(num)
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

    if params.len() <= 0 {
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
        introspectable!(self);
        self.info.gen(w)?;
        self.doc.gen(w)?;
        let skip = self.typ == FunctionType::Method;
        gen_doc_params(&self.parameters, root_ns, skip, w)?;
        gen_doc_return(&self, root_ns,  w)?;
        let param_names = gen_param_names(&self.parameters, skip);
        match self.typ {
            FunctionType::Callback => panic!("Use gen_callback for callbacks!"),
            FunctionType::Method =>
                writeln!(w, "function {}:{}({}) end\n", &ns, self.name, param_names)?,
            FunctionType::Virtual => todo!(),
            FunctionType::Member =>
                writeln!(w, "\t[\"{}\"] = function({}) end,\n", self.name.to_uppercase(), param_names)?,
            FunctionType::Function =>
                writeln!(w, "function {}.{}({}) end\n", &ns, self.name, param_names)?,
            FunctionType::Constructor => {
                writeln!(w, "function {}.{}({}) end\n", &ns, self.name, param_names)?;
            },
        }
        Ok(())
    }
    pub fn gen_callback_type<W: Write>(&self, ns: &str, w: &mut W) -> Result<()> {
        introspectable!(self);
        let param_names = gen_param_names_typed(&self.parameters, ns).join(", ");
        if let Some(ret) = gen_return_names_typed(&self, ns) {
            writeln!(w, "--- @alias {}.{} fun({}):{}", ns, self.name, param_names, ret)?;
        } else {
            writeln!(w, "--- @alias {}.{} fun({})", ns, self.name, param_names)?;
        }
        Ok(())
    }
}

impl Member {
    pub fn gen<W: Write>(&self, w: &mut W) -> Result<()> {
        introspectable!(self);
        Ok(writeln!(w, "\t[\"{}\"] = {},", self.name.to_uppercase(), self.value)?)
    }
}

impl Record {
    pub fn gen_type<W: Write>(&self, ns: &str, w: &mut W) -> Result<()> {
        introspectable!(self);
        // Skip these
        if self.name.ends_with("Class") {
            return Ok(())
        }

        if let Some(false) = self.info.introspectable {
            return Ok(())
        }

        writeln!(w, "--- @class {}.{}", ns, self.name)?;
        section!(w, self, ns, fields);
        Ok(writeln!(w, "local {} = {{}}", self.name)?)
    }
    pub fn gen(&self, ns: &str, p: &Path) -> Result<()> {
    // pub fn gen<W: Write>(&self, ns: &str, w: &mut W) -> Result<()> {
        introspectable!(self);
        if let Some(false) = self.info.introspectable {
            return Ok(())
        }

        let mut w = gen_file(&self.name, p)?;

        let record_ns = format!("{}.{}", ns, self.name);

        for constructor in self.constructor.iter() {
            constructor.gen(&record_ns, ns, &mut w)?;
        }
        for method in self.method.iter() {
            method.gen(&record_ns, ns, &mut w)?;
        }
        for func in self.functions.iter() {
            func.gen(&record_ns, ns, &mut w)?;
        }

        for unio in self.unions.iter() {
            unio.gen(&record_ns, &mut w)?;
        }
        Ok(())
    }
}

impl Constant {
    pub fn gen<W: Write>(&self, ns: &str, w: &mut W) -> Result<()> {
        introspectable!(self);
        if self.value.parse::<u32>().is_ok() {
            Ok(writeln!(w, "{}.{} = {}", ns, self.name, self.value)?)
        } else {
            Ok(writeln!(w, "{}.{} = \"{}\"", ns, self.name, self.value)?)
        }
    }
}

impl Enumeration {
    pub fn gen<W: Write>(&self, ns: &str, w: &mut W) -> Result<()> {
        introspectable!(self);
        writeln!(w, "--- @enum {}.{}", &ns, self.name)?;
        writeln!(w, "{}.{} = {{", &ns, self.name)?;
        for mem in self.members.iter() {
            mem.gen(w)?;
        }
        for func in self.functions.iter() {
            func.gen("", ns, w)?;
        }
        Ok(writeln!(w, "}}")?)
    }
}

impl Bitfield {
    pub fn gen<W: Write>(&self, ns: &str, w: &mut W) -> Result<()> {
        introspectable!(self);
        writeln!(w, "--- @enum {}.{}", &ns, self.name)?;
        writeln!(w, "--- @overload fun({{any}}): {}.{}", &ns, self.name)?;
        writeln!(w, "{}.{} = {{", &ns, self.name)?;
        for mem in self.members.iter() {
            mem.gen(w)?;
        }
        for func in self.functions.iter() {
            func.gen("", ns, w)?;
        }
        writeln!(w, "}}", )?;
        Ok(())
    }
}

// #[cfg(test)]
// mod test {
//
//     use super::*;
//     use simple_xml_builder::XMLElement;
//
//     macro_rules! snapshot {
//         ($name:ident, $path:literal) => {
//             #[test]
//             fn $name() {
//                 let gir = include_str!(concat!("../../testdata/girs/", $path));
//                 let mut settings = insta::Settings::clone_current();
//                 settings.set_snapshot_path("../../testdata/lua/output/");
//                 let mut buf = Vec::new();
//                 settings.bind(|| {
//                     LuaCodegen::new().gen(gir.as_bytes(), &mut buf).unwrap();
//                     let string = String::from_utf8(buf).unwrap();
//                     insta::assert_snapshot!(
//                         string);
//                 });
//             }
//         };
//     }
//
//     fn gengir(child :XMLElement) -> String {
//         let mut repo = XMLElement::new("repository");
//         repo.add_attribute("version", "1.2");
//         repo.add_attribute("xmlns", "http://www.gtk.org/introspection/core/1.0");
//         repo.add_attribute("xmlns:c", "http://www.gtk.org/introspection/c/1.0");
//         repo.add_attribute("xmlns:glib", "http://www.gtk.org/introspection/c/1.0");
//         let mut ns = XMLElement::new("namespace");
//         ns.add_attribute("name", "Test");
//         ns.add_child(child);
//         repo.add_child(ns);
//
//         let mut buf = Vec::new();
//         repo.write(&mut buf).unwrap();
//         String::from_utf8(buf).unwrap()
//     }
//
//     fn parse_test(child :XMLElement) -> String {
//         let gir = gengir(child);
//
//         let mut buf = Vec::new();
//         LuaCodegen::new().gen(gir.as_bytes(), &mut buf).unwrap();
//         String::from_utf8(buf).unwrap()
//     }
//
//     snapshot!(test_gmime, "GMime-3.0.gir");
//
//     #[test]
//     pub fn test_class() {
//         let mut class = XMLElement::new("class");
//         class.add_attribute("name", "TestClass");
//         class.add_attribute("type-name", "TestTestClass");
//         class.add_attribute("get-type", "test_test_get_type");
//
//         let code = parse_test(class);
//
//         insta::assert_snapshot!(code, @r###"
//         ---@diagnostic disable: unused-local, duplicate-doc-field
//         ---@meta
//         -- THIS FILE WAS GENERATED BY gir-to-stub! DO NOT MODIFY!
//         local Test = {}
//
//         --- @class Test.TestClass
//         local TestClass = {}
//         --- @param obj GObject.Object
//         --- @return boolean
//         function Test.TestClass:is_type_of(obj) end
//         return Test
//         "###)
//     }
//
//     fn gen_function(fun :&mut XMLElement) {
//         fun.add_attribute("name", "testFunc");
//         fun.add_attribute("c:identifier", "g_test");
//
//         let mut ret = XMLElement::new("return-value");
//         ret.add_attribute("transfer-ownership", "none");
//
//         let mut params = XMLElement::new("parameters");
//         let mut param = XMLElement::new("parameter");
//         param.add_attribute("name", "num");
//
//         let mut typ = XMLElement::new("type");
//         typ.add_attribute("name", "guint8");
//         typ.add_attribute("c:type", "unsigned char*");
//         param.add_child(typ);
//         params.add_child(param);
//         fun.add_child(params);
//         fun.add_child(ret);
//     }
//
//     
//     #[test]
//     pub fn test_functions() {
//         let mut fun = XMLElement::new("function");
//
//         gen_function(&mut fun);
//
//         let code = parse_test(fun);
//
//         insta::assert_snapshot!(code, @r###"
//         ---@diagnostic disable: unused-local, duplicate-doc-field
//         ---@meta
//         -- THIS FILE WAS GENERATED BY gir-to-stub! DO NOT MODIFY!
//         local Test = {}
//
//         --- @param num number
//         function Test.testFunc(num) end
//
//         return Test
//         "###);
//     }
//
//     #[test]
//     pub fn test_callback() {
//         let mut cb = XMLElement::new("callback");
//         gen_function(&mut cb);
//
//         let code = parse_test(cb);
//
//         insta::assert_snapshot!(code, @r###"
//         ---@diagnostic disable: unused-local, duplicate-doc-field
//         ---@meta
//         -- THIS FILE WAS GENERATED BY gir-to-stub! DO NOT MODIFY!
//         local Test = {}
//
//         --- @alias Test.testFunc fun(num: number)
//         return Test
//         "###);
//     }
//
//     #[test]
//     pub fn test_enum() {
//         let mut enu = XMLElement::new("enumeration");
//         enu.add_attribute("name", "TestEnum");
//         enu.add_attribute("c:type", "TestTestEnum");
//         let mut member = XMLElement::new("member");
//         member.add_attribute("name", "key");
//         member.add_attribute("value", "3");
//         enu.add_child(member);
//
//         let code = parse_test(enu);
//         insta::assert_snapshot!(code, @r###"
//         ---@diagnostic disable: unused-local, duplicate-doc-field
//         ---@meta
//         -- THIS FILE WAS GENERATED BY gir-to-stub! DO NOT MODIFY!
//         local Test = {}
//
//         --- @enum Test.TestEnum
//         Test.TestEnum = {
//         	["KEY"] = 3,
//         }
//         return Test
//         "###);
//     }
//
//     #[test]
//     pub fn test_record() {
//     }
//
//     #[test]
//     pub fn test_constant() {
//         let mut constant = XMLElement::new("constant");
//         constant.add_attribute("name", "MYCONSTANT");
//         constant.add_attribute("c:type", "TEST_MYCONSTANT");
//         constant.add_attribute("value", "555");
//         let mut typ = XMLElement::new("type");
//         typ.add_attribute("name", "gint");
//         typ.add_attribute("c:tpe", "gint");
//         constant.add_child(typ);
//
//         let code = parse_test(constant);
//         insta::assert_snapshot!(code, @r###"
//         ---@diagnostic disable: unused-local, duplicate-doc-field
//         ---@meta
//         -- THIS FILE WAS GENERATED BY gir-to-stub! DO NOT MODIFY!
//         local Test = {}
//
//         Test.MYCONSTANT = 555
//         return Test
//         "###);
//     }
//
//     #[test]
//     pub fn test_bitfield() {
//         let mut field = XMLElement::new("bitfield");
//         field.add_attribute("name", "TestField");
//         field.add_attribute("c:type", "TestTestField");
//         let mut member = XMLElement::new("member");
//         member.add_attribute("name", "key");
//         member.add_attribute("value", "3");
//         field.add_child(member);
//
//         let code = parse_test(field);
//         insta::assert_snapshot!(code, @r###"
//         ---@diagnostic disable: unused-local, duplicate-doc-field
//         ---@meta
//         -- THIS FILE WAS GENERATED BY gir-to-stub! DO NOT MODIFY!
//         local Test = {}
//
//         --- @enum Test.TestField
//         --- @overload fun({any}): Test.TestField
//         Test.TestField = {
//         	["KEY"] = 3,
//         }
//         return Test
//         "###);
//     }
//
//     #[test]
//     pub fn test_alias() {
//     }
//
//     #[test]
//     pub fn test_unions() {
//     }
//
//     #[test]
//     pub fn test_boxed() {
//     }
//
//     #[test]
//     pub fn test_interfaces() {
//     }
// }
