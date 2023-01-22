use crate::library::*;
use xmltree::Element;
use core::fmt;
use std::io::Read;
use std::str::FromStr;

fn read_return(e: &Element) -> Option<Parameter> {
    let ret = e.get_child("return-value")?;
    read_param(ret)
}

impl FromStr for ParameterScope {
    type Err = String;

    fn from_str(name: &str) -> Result<ParameterScope, String> {
        match name {
            "call" => Ok(ParameterScope::Call),
            "async" => Ok(ParameterScope::Async),
            "notified" => Ok(ParameterScope::Notified),
            _ => Err(format!("Unknown parameter scope type: {}", name)),
        }
    }
}

impl FromStr for Version {
    type Err = String;

    fn from_str(s: &str) -> Result<Version, String> {
        if s.contains('.') {
            let mut parts = s
                .splitn(4, '.')
                .map(str::parse)
                .take_while(Result::is_ok)
                .map(Result::unwrap);
            Ok(Version(
                parts.next().unwrap_or(0),
                parts.next().unwrap_or(0),
                parts.next().unwrap_or(0),
            ))
        } else {
            let val = s.parse::<u16>();
            Ok(Version(val.unwrap_or(0), 0, 0))
        }
    }
}

impl FromStr for ParameterDirection {
    type Err = String;
    fn from_str(name: &str) -> Result<ParameterDirection, String> {
        use self::ParameterDirection::*;
        match name {
            "in" => Ok(In),
            "out" => Ok(Out),
            "inout" => Ok(InOut),
            _ => Err(format!("Unknown parameter direction '{}'", name)),
        }
    }
}

impl FromStr for Transfer {
    type Err = String;
    fn from_str(name: &str) -> Result<Transfer, String> {
        use self::Transfer::*;
        match name {
            "container" => Ok(Container),
            "full" => Ok(Full),
            _ => Err(format!("Unknown parameter direction '{}'", name)),
        }
    }
}

fn get_attribute(e: &Element, attr: &str) -> Option<String> {
    e.attributes.get(attr).map(|x| x.to_string())
}

fn r_anytype(e: &Element) -> Option<AnyType> {
    match e.name.as_str() {
        "type" => { 
            let name = attribute(e, "name");
            let ctype = attribute(e, "type");
            let introspectable = attr_bool(e, "introspectable");
            let doc = read_infoelements(e)?;
            let mut children = vec![];
            for node in e.children.iter() {
                if let Some(e) = node.as_element() {
                    if let Some(typ) = read_anytype(e) {
                        children.push(typ)
                    }
                }
            }
            if Some("GLib.HashTable") == name.as_deref() {
                for node in e.children.iter() {
                    if let Some(e) = node.as_element() {
                        println!("{:?}", e);
                        if let Some(typ) = read_anytype(e) {
                            children.push(typ)
                        }
                    }
                }
            }
            return Some(AnyType::Type(Type{
                name,
                ctype,
                introspectable,
                doc,
                children,
            }))
        }
        "array" => {
            let name = attribute(e, "name");
            let zero_terminated = attr_bool(e, "zero-terminated");
            let fixed_size = attr_bool(e, "fixed-size");
            let introspectable = attr_bool(e, "introspectable");
            let length = attr_value(e, "length");
            let ctype = attribute(e, "type");
            for node in e.children.iter() {
                if let Some(e) = node.as_element() {
                    let typ = attribute(e, "name")?;
                    return Some(AnyType::Array(Array{
                        name,
                        zero_terminated,
                        fixed_size,
                        introspectable,
                        length,
                        ctype,
                        typ,
                    }))
                }
            }
            None
        }
        "varargs" => {
            return Some(AnyType::VarArg)
        }
        _ => None
    }
}

fn read_anytype(e: &Element) -> Option<AnyType> {
    for node in e.children.iter() {
        if let Some(e) = node.as_element() {
            match e.name.as_str() {
                "type" => { 
                    let name = attribute(e, "name");
                    let ctype = attribute(e, "type");
                    let introspectable = attr_bool(e, "introspectable");
                    let doc = read_infoelements(e)?;
                    let mut children = vec![];
                    for node in e.children.iter() {
                        if let Some(e) = node.as_element() {
                            if let Some(typ) = r_anytype(e) {
                                children.push(typ)
                            }
                        }
                    }
                    return Some(AnyType::Type(Type{
                        name,
                        ctype,
                        introspectable,
                        doc,
                        children,
                    }))
                }
                "array" => {
                    let name = attribute(e, "name");
                    let zero_terminated = attr_bool(e, "zero-terminated");
                    let fixed_size = attr_bool(e, "fixed-size");
                    let introspectable = attr_bool(e, "introspectable");
                    let length = attr_value(e, "length");
                    let ctype = attribute(e, "type");
                    for node in e.children.iter() {
                        if let Some(e) = node.as_element() {
                            let typ = attribute(e, "name")?;
                            // let typ = read_anytype(e)?;
                            // let btyp = Box::new(typ);
                            return Some(AnyType::Array(Array{
                                name,
                                zero_terminated,
                                fixed_size,
                                introspectable,
                                length,
                                ctype,
                                typ,
                            }))
                        }
                    }
                }
                "varargs" => {
                    return Some(AnyType::VarArg)
                }
                _ => {}
            }
        }
    }
    None
}

fn read_param(e: &Element) -> Option<Parameter> {
    let name = attribute(e, "name").unwrap_or("".to_string());
    let nullable = attr_bool(e, "nullable").unwrap_or(false);
    let allow_none = attr_bool(e, "allow-none").unwrap_or(false);
    let introspectable = attr_bool(e, "introspectable");
    let closure = attribute(e, "closure");
    let destroy = attribute(e, "destroy");
    let scope = attribute(e, "scope");
    let direction = attr_value(e, "direction");
    let caller_allocates = attr_bool(e, "caller-allocates").unwrap_or(false);
    let optional = attr_bool(e, "optional").unwrap_or(false);
    let skip = attr_bool(e, "skip").unwrap_or(false);
    let transfer = attr_value(e, "transfer");
    let doc = read_infoelements(e)?;
    let typ = read_anytype(e)?;

    Some(Parameter{
        name,
        nullable,
        allow_none,
        introspectable,
        closure,
        destroy,
        scope,
        direction,
        caller_allocates,
        optional,
        skip,
        transfer,
        doc,
        typ,
    })
}

fn read_params(e: &Element) -> Option<Vec<Parameter>> {
    let mut ret: Vec<Parameter> = vec![];
    let parameters = e.get_child("parameters")?;

    for parameter in parameters.children.iter() {
        if let Some(e) = parameter.as_element() {
            match e.name.as_ref() {
                "parameter" | "instance-parameter" => {
                    let para = read_param(e);
                    if let Some(para) = para {
                        ret.push(para);
                    }
                },
                _ => return None,
            }
        }
    }
    return Some(ret)
}

fn read_macro_param(e: &Element) -> Option<MacroParam> {
    let name = attribute(e, "name")?;
    let doc = read_infoelements(e)?;
    Some(MacroParam{
        name,
        doc,
    })
}

fn read_macro(e: &Element) -> Option<Macro> {
    let name = attribute(e, "name")?;
    let info = read_infoattrs(e)?;
    let doc = read_infoelements(e)?;

    let c_identifier = attribute(e, "identifier");

    let mut param: Vec<MacroParam> = vec![];

    let parameters = e.get_child("parameters")?;

    for parameter in parameters.children.iter() {
        if let Some(e) = parameter.as_element() {
            if e.name == "parameter" {
                let para = read_macro_param(e);
                if let Some(para) = para {
                    param.push(para);
                }
            }
        }
    }
    Some(Macro {
        info,
        doc,
        name,
        c_identifier,
        parameters: param,
    })
}

fn read_signal(e: &Element) -> Option<Signal> {
    let name = attribute(e, "name")?;
    let info = read_infoattrs(e)?;
    let doc = read_infoelements(e)?;

    let detailed = attr_bool(e, "detailed");
    let when = attribute(e, "action");
    let action = attr_bool(e, "action");
    let no_hooks = attr_bool(e, "detailed");
    let no_recurse = attr_bool(e, "action");
    let emitter = attribute(e, "detailed");

    let ret = read_return(e);
    let parameters = read_params(e).unwrap_or(vec![]);

    Some(Signal { 
        name,
        info,
        doc,
        detailed,
        when,
        action,
        no_hooks,
        no_recurse,
        emitter,
        parameters,
        ret,
    })
}

fn read_function(e: &Element, typ: FunctionType) -> Option<Function> {
    let name = attribute(e, "name")?;
    let info = read_infoattrs(e)?;
    let doc = read_infoelements(e)?;

    let c_identifier = attribute(e, "identifier");
    let shadowed_by = attribute(e, "shadowed-by");
    let shadows = attribute(e, "shadows");
    let throws = attr_bool(e, "throws");
    let moved_to = attribute(e, "moved-to");
    let introspectable = attr_bool(e, "introspectable");

    let ret = read_return(e);
    let parameters = read_params(e).unwrap_or(vec![]);

    Some(Function {
        info,
        doc,
        typ,
        name,
        introspectable,
        c_identifier,
        shadowed_by,
        shadows,
        throws,
        moved_to,
        parameters,
        ret,
    })
}

fn get_doc(e: &Element) -> Option<Doc> {
    let preserve_space = attribute(e, "space");
    let preserve_white = attribute(e, "whitespace");
    let filename = attribute(e, "filename")?;
    let line = attribute(e, "line")?;
    let column = attribute(e, "column");
    let content = e.get_text().map(|x| x.into())?;
    Some(Doc{
        preserve_space,
        preserve_white,
        filename,
        line,
        column,
        content,
    })
}
fn get_doc_versioned(e: &Element) -> Option<DocVersioned> {
    let preserve_space = attribute(e, "space");
    let preserve_white = attribute(e, "whitespace");
    let content = e.get_text().map(|x| x.into())?;
    Some(DocVersioned{
        preserve_space,
        preserve_white,
        content,
    })
}

fn get_source_position(e: &Element) -> Option<DocPosition> {
    let filename = attribute(e, "filename")?;
    let line = attribute(e, "line")?;
    let column = attribute(e, "column");
    Some(DocPosition{
        filename,
        line,
        column,
    })
}


fn read_property(e: &Element) -> Option<Property> {
    let name = attribute(e, "name")?;
    let info = read_infoattrs(e)?;
    let doc = read_infoelements(e)?;

    let readable = attr_bool(e, "readable").unwrap_or(true);
    let writable = attr_bool(e, "writable").unwrap_or(false);
    let construct = attr_bool(e, "construct").unwrap_or(false);
    let construct_only = attr_bool(e, "construct-only").unwrap_or(false);
    let setter = attribute(e, "setter");
    let getter = attribute(e, "getter");
    let transfer = attr_value(e, "transfer");
    let typ = read_anytype(e)?;

    Some(Property{
        name,
        info,
        doc,
        readable,
        writable,
        construct,
        construct_only,
        setter,
        getter,
        transfer,
        typ,
    })
}

pub fn attr_bool(e: &Element, name: &str) -> Option<bool>
{
    if let Some(value_str) = e.attributes.get(name) {
        match value_str.as_str() {
            "0" => Some(false),
            "1" => Some(true),
            _ => None
        }
    } else {
        None
    }
}

fn read_infoattrs(e: &Element) -> Option<InfoAttrs> {
    let introspectable = attr_bool(e, "introspectable");
    let deprecated = attr_bool(e, "deprecated");
    let deprecated_version = attribute(e, "deprecated-version");
    let version = attribute(e, "version");
    let stability = attribute(e, "stability");
    Some(InfoAttrs {
        introspectable,
        deprecated,
        deprecated_version,
        version,
        stability,
    })
}


fn read_infoelements(e: &Element) -> Option<InfoElements> {
    let mut doc = None;
    let mut doc_stability = None;
    let mut doc_version = None;
    let mut doc_deprecated = None;
    let mut doc_pos = None;
    for node in e.children.iter() {
        if let Some(e) = node.as_element() {
            match e.name.as_str() {
                "doc" => {
                    let docs = get_doc(&e);
                    doc = docs;
                }
                "doc-stability" => {
                    let docs = get_doc_versioned(&e);
                    doc_stability = docs;
                }
                "doc-deprecated" => {
                    let docs = get_doc_versioned(&e);
                    doc_deprecated = docs;
                }
                "doc-version" => {
                    let docs = get_doc_versioned(&e);
                    doc_version = docs;
                }
                "source-position" => {
                    let docs = get_source_position(&e);
                    doc_pos = docs;
                }
                _ => {}
            }
        }
    }
    Some(InfoElements{
        doc,
        doc_stability,
        doc_version,
        doc_deprecated,
        doc_pos,
    })
}

fn read_class(e: &Element) -> Option<Class> {
    let name = attribute(e, "name")?;
    let glib_type_name = attribute(e, "type-name")?;
    let glib_get_type = attribute(e, "get-type")?;
    let parent = attribute(e, "parent");
    let glib_type_struct = attribute(e, "type-stuct");
    let ref_func = attribute(e, "ref-func");
    let unref_func = attribute(e, "unref-func");
    let set_value_func = attribute(e, "set-value-func");
    let get_value_func = attribute(e, "get-value-func");
    let ctype = attribute(e, "type");
    let symbol_prefix = attribute(e, "symbol-prefix");
    let abstracts = attribute(e, "abstract");
    let glib_fundamental = attribute(e, "fundamental");
    let finals = attribute(e, "final");

    let info = read_infoattrs(e)?;

    let mut constructor = vec![];
    let mut functions = vec![];
    let mut method = vec![];
    let mut virtual_method = vec![];
    let mut callbacks = vec![];

    let mut record = vec![];
    let mut fields = vec![];
    let mut signals = vec![];
    let mut unions = vec![];
    let mut constant = vec![];
    let mut properties = vec![];
    let mut implements = vec![];

    let doc = read_infoelements(e)?;

    for node in e.children.iter() {
        if let Some(e) = node.as_element() {
            match e.name.as_str() {
                "constructor" => {
                    if let Some(fun) = read_function(e, FunctionType::Constructor) {
                        constructor.push(fun)
                    }
                }
                "function" => {
                    if let Some(fun) = read_function(e, FunctionType::Function) {
                        functions.push(fun)
                    }
                }
                "method" => {
                    if let Some(fun) = read_function(e, FunctionType::Method) {
                        method.push(fun)
                    }
                }
                "virtual-method" => { 
                    if let Some(fun) = read_function(e, FunctionType::Virtual) {
                        virtual_method.push(fun)
                    }
                }
                "callback" => { 
                    if let Some(fun) = read_function(e, FunctionType::Callback) {
                        callbacks.push(fun)
                    }
                }
                "union" => { 
                    if let Some(fun) = read_union(e) {
                        unions.push(fun)
                    }
                }
                "constant" => { 
                    if let Some(fun) = read_constant(e) {
                        constant.push(fun)
                    }
                }
                "record" => { 
                    if let Some(fun) = read_record(e) {
                        record.push(fun)
                    }
                }
                "field" => { 
                    if let Some(fun) = read_field(e) {
                        fields.push(fun)
                    }
                }
                "property" => {
                    if let Some(prop) = read_property(e) {
                        properties.push(prop)
                    }
                }
                "signal" => {
                    if let Some(fun) = read_signal(e) {
                        signals.push(fun)
                    }
                }
                "implements" => {
                    if let Some(str) = attribute(e, "name"){
                        implements.push(Implement { name: str })
                    }
                }
                _ => {
                }
            }
        }
    }
    Some(Class {
        info,
        name,
        glib_type_name,
        glib_get_type,
        parent,
        glib_type_struct,
        ref_func,
        unref_func,
        set_value_func,
        get_value_func,
        ctype,
        symbol_prefix,
        abstracts,
        glib_fundamental,
        finals,
        constructor,
        functions,
        method,
        virtual_method,
        callbacks,
        doc,
        record,
        fields,
        signals,
        unions,
        constant,
        properties,
        implements,
    })
}

fn read_field(e: &Element) -> Option<Field> {
    let name = attribute(e, "name")?;
    let info = read_infoattrs(e)?;
    let doc = read_infoelements(e)?;
    
    let typ = read_anytype(e)?;
    let writeable = attr_bool(e, "writeable").unwrap_or(false);
    let readable = attr_bool(e, "readable").unwrap_or(true);
    let private = attr_bool(e, "private").unwrap_or(false);
    let bits = attr_value(e, "bits");

    Some(Field {
        name,
        info,
        doc,
        typ,
        writeable,
        readable,
        private,
        bits,
    })
}

fn read_record(e: &Element) -> Option<Record> {
    let name = attribute(e, "name")?;
    let info = read_infoattrs(e)?;
    let doc = read_infoelements(e)?;

    let c_type = attribute(e, "value");
    let disguised = attr_bool(e, "disguised");
    let symbol_prefix = attribute(e, "symbol-prefix");
    let glib_get_type = attribute(e, "get-type");
    let glib_type_name = attribute(e, "type-name");
    let glib_is_gtype_struct_for = attribute(e, "is-gtype-struct-for");
    let foreign = attr_bool(e, "foreign");

    let mut constructor = vec![];
    let mut functions = vec![];
    let mut method = vec![];

    let mut fields = vec![];
    let mut unions = vec![];

    for node in e.children.iter() {
        if let Some(e) = node.as_element() {
            match e.name.as_str() {
                "constructor" => {
                    if let Some(fun) = read_function(e, FunctionType::Constructor) {
                        constructor.push(fun)
                    }
                }
                "function" => {
                    if let Some(fun) = read_function(e, FunctionType::Function) {
                        functions.push(fun)
                    }
                }
                "method" => {
                    if let Some(fun) = read_function(e, FunctionType::Method) {
                        method.push(fun)
                    }
                }
                "union" => { 
                    if let Some(fun) = read_union(e) {
                        unions.push(fun)
                    }
                }
                "field" => { 
                    if let Some(fun) = read_field(e) {
                        fields.push(fun)
                    }
                }
                _ => {
                }
            }
        }
    }
    Some(Record{
        name,
        info,
        doc,
        c_type,
        disguised,
        symbol_prefix,
        glib_get_type,
        glib_type_name,
        glib_is_gtype_struct_for,
        foreign,
        fields,
        unions,
        constructor,
        functions,
        method,
    })
}

fn read_constant(e: &Element) -> Option<Constant> {
    let info = read_infoattrs(e)?;
    let doc = read_infoelements(e)?;

    let name = attribute(e, "name")?;
    let value = attribute(e, "value")?;
    let c_identifier = attribute(e, "identifier");
    let c_type = attribute(e, "type");
    let typ = read_anytype(e);

    return Some(Constant {
        name,
        c_identifier,
        info,
        doc,
        value,
        c_type,
        typ,
    })
}
fn read_union(e: &Element) -> Option<Union> {
    let name = attribute(e, "name");
    let c_type = attribute(e, "type");
    let glib_type_name = attribute(e, "type-name");
    let glib_get_type = attribute(e, "get-type");
    let symbol_prefix = attribute(e, "symbol-prefix");

    let info = read_infoattrs(e)?;
    let doc = read_infoelements(e)?;

    let mut constructor = vec![];
    let mut functions = vec![];
    let mut method = vec![];

    let mut record = vec![];
    let mut fields = vec![];
    for node in e.children.iter() {
        if let Some(e) = node.as_element() {
            match e.name.as_str() {
                "constructor" => {
                    if let Some(fun) = read_function(e, FunctionType::Constructor) {
                        constructor.push(fun)
                    }
                }
                "function" => {
                    if let Some(fun) = read_function(e, FunctionType::Function) {
                        functions.push(fun)
                    }
                }
                "method" => {
                    if let Some(fun) = read_function(e, FunctionType::Method) {
                        method.push(fun)
                    }
                }
                "record" => { 
                    if let Some(fun) = read_record(e) {
                        record.push(fun)
                    }
                }
                "field" => { 
                    if let Some(fun) = read_field(e) {
                        fields.push(fun)
                    }
                }
                name => {
                }
            }
        }
    }

    Some(Union {
        name,
        info,
        doc,
        c_type,
        symbol_prefix,
        glib_type_name,
        glib_get_type,
        fields,
        constructor,
        method,
        functions,
        record,
    })
}

fn read_namespace(e: &Element) -> Option<Namespace> {
    let name = attribute(e, "name");
    let version = attribute(e, "version");

    let shared_library = attribute(e, "shared-library");
    let identifier_prefixes = attribute(e, "identifier-prefixes");
    let symbol_prefixes= attribute(e, "symbol-prefixes");
    let prefix = attribute(e, "prefix");
    // let shared_library = s.split(',').collect();


    let mut classes = vec![];
    let mut functions = vec![];
    let mut macros = vec![];
    let mut callback = vec![];

    let mut enums = vec![];
    let mut record = vec![];
    let mut constant = vec![];
    let mut bitfield = vec![];
    let mut interfaces = vec![];
    let mut alias = vec![];
    let mut unions = vec![];
    let mut boxed = vec![];

    for node in e.children.iter() {
        if let Some(e) = node.as_element() {
            match e.name.as_str() {
                "class" => {
                    if let Some(class) = read_class(&e) {
                        classes.push(class);
                    }
                }
                "function" => {
                    if let Some(fun) = read_function(e, FunctionType::Function) {
                        functions.push(fun);
                    }
                }
                "function-macro" => {
                    if let Some(fun) = read_macro(e) {
                        macros.push(fun);
                    }
                }
                "callback" => {
                    if let Some(cb) = read_function(e, FunctionType::Callback) {
                        callback.push(cb);
                    }
                }
                "enumeration" => {
                    if let Some(enu) = read_enum(e) {
                        enums.push(enu);
                    }
                }
                "record" => {
                    if let Some(rec) = read_record(e) {
                        record.push(rec);
                    }
                }
                "constant" => {
                    if let Some(consts) = read_constant(e) {
                        constant.push(consts);
                    }
                }
                "bitfield" => {
                    if let Some(bf) = read_bitfield(e) {
                        bitfield.push(bf)
                    }
                }
                "union" => {
                    if let Some(union) = read_union(e) {
                        unions.push(union)
                    }
                }
                "docsection" => {
                }
                "name" => {
                }
                "alias" => {
                    if let Some(bf) = read_alias(e) {
                        alias.push(bf)
                    }
                }
                "interface" => {
                    if let Some(bf) = read_interface(e) {
                        interfaces.push(bf)
                    }
                }
                "boxed" => {
                    if let Some(bf) = read_boxed(e) {
                        boxed.push(bf)
                    }
                }
                _ => {
                }
            }
        }
    }
    Some(Namespace {
        name,
        version,
        shared_library,
        identifier_prefixes,
        symbol_prefixes,
        prefix,
        classes,
        functions,
        macros,
        callback,
        interfaces,
        enums,
        record,
        constant,
        bitfield,
        alias,
        unions,
        boxed,
    })
}

fn read_alias(e: &Element) -> Option<Alias> {
    let info = read_infoattrs(e)?;
    let doc = read_infoelements(e)?;
    let name = attribute(e, "name")?;
    let c_type = attribute(e, "type")?;
    let typ = read_anytype(e)?;

    let mut functions = vec![];

    // XXX: Should we even do this?
    for node in e.children.iter() {
        if let Some(e) = node.as_element() {
            match e.name.as_str() {
                "constructor" => {
                    if let Some(fun) = read_function(e, FunctionType::Constructor) {
                        functions.push(fun) 
                    }
                }
                _ => {}
            }
        }
    }
    Some(Alias {
        name,
        info,
        doc,
        c_type,
        typ,
    })
}

fn read_interface(e: &Element) -> Option<Interface> {
    let info = read_infoattrs(e)?;
    let doc = read_infoelements(e)?;
    let name = attribute(e, "name")?;

    let glib_type_name = attribute(e, "type-name")?;
    let glib_get_type = attribute(e, "get-type")?;
    let symbol_prefix = attribute(e, "symbol-prefix");
    let c_type = attribute(e, "type");
    let glib_type_struct = attribute(e, "type-struct");

    let mut constructor = None;
    let mut functions = vec![];
    let mut method = vec![];
    let mut virtual_method = vec![];
    let mut callbacks = vec![];

    let mut prerequisites = vec![];
    let mut implements = vec![];

    let mut fields = vec![];
    let mut signals = vec![];
    let mut constant = vec![];
    let mut properties = vec![];

    let doc = read_infoelements(e)?;

    for node in e.children.iter() {
        if let Some(e) = node.as_element() {
            match e.name.as_str() {
                "constructor" => {
                    if let Some(fun) = read_function(e, FunctionType::Constructor) {
                        constructor = Some(fun)
                    }
                }
                "function" => {
                    if let Some(fun) = read_function(e, FunctionType::Function) {
                        functions.push(fun)
                    }
                }
                "method" => {
                    if let Some(fun) = read_function(e, FunctionType::Method) {
                        method.push(fun)
                    }
                }
                "virtual-method" => { 
                    if let Some(fun) = read_function(e, FunctionType::Virtual) {
                        virtual_method.push(fun)
                    }
                }
                "callback" => { 
                    if let Some(fun) = read_function(e, FunctionType::Callback) {
                        callbacks.push(fun)
                    }
                }
                "constant" => { 
                    if let Some(fun) = read_constant(e) {
                        constant.push(fun)
                    }
                }
                "field" => { 
                    if let Some(fun) = read_field(e) {
                        fields.push(fun)
                    }
                }
                "property" => {
                    if let Some(prop) = read_property(e) {
                        properties.push(prop)
                    }
                }
                "signal" => {
                    if let Some(fun) = read_signal(e) {
                        signals.push(fun)
                    }
                }
                "prerequisites" => {
                    if let Some(str) = attribute(e, "name"){
                        prerequisites.push(str)
                    }
                }
                "implements" => {
                    if let Some(str) = attribute(e, "name"){
                        implements.push(str)
                    }
                }
                _ => {
                }
            }
        }
    }
    Some(Interface {
        name,
        info,
        doc,
        glib_type_name,
        glib_get_type,
        symbol_prefix,
        c_type,
        glib_type_struct,
        constructor,
        prerequisites,
        implements,
        functions,
        method,
        virtual_method,
        callbacks,
        fields,
        properties,
        signals,
        constant,
    })
}

fn read_boxed(e: &Element) -> Option<Boxed> {
    let info = read_infoattrs(e)?;
    let doc = read_infoelements(e)?;
    let glib_name = attribute(e, "name")?;

    let symbol_prefix = attribute(e, "symbol-prefix");
    let glib_type_name = attribute(e, "type-name");
    let glib_get_type = attribute(e, "get-type");
    let mut functions = vec![];
    for node in e.children.iter() {
        if let Some(e) = node.as_element() {
            match e.name.as_str() {
                "function" => {
                    if let Some(fun) = read_function(e, FunctionType::Function) {
                        functions.push(fun);
                    }
                },
                _ => panic!("Only functions allowed as boxed children")
            }
        }
    }

    Some(Boxed {
        glib_name,
        info,
        doc,
        symbol_prefix,
        glib_type_name,
        glib_get_type,
        functions,
    })
}

fn read_bitfield(e: &Element) -> Option<Bitfield> {
    let info = read_infoattrs(e)?;
    let doc = read_infoelements(e)?;
    let name = attribute(e, "name")?;

    let c_type = attribute(e, "type")?;
    let glib_type_name = attribute(e, "type-name");
    let glib_get_type = attribute(e, "get-type");

    let mut functions = vec![];
    let mut members = vec![];

    for node in e.children.iter() {
        if let Some(e) = node.as_element() {
            match e.name.as_str() {
                "function" => {
                    if let Some(fun) = read_function(e, FunctionType::Member) {
                        functions.push(fun);
                    }
                }
                "member" => {
                    if let Some(mem) = read_member(e) {
                        members.push(mem);
                    }
                }
                _ => {
                }
            }
        }
    }
    Some(Bitfield {
        info,
        doc,
        name,
        c_type,
        glib_type_name,
        glib_get_type,
        members,
        functions,
    })
}

fn read_member(e: &Element) -> Option<Member> {
    let info = read_infoattrs(e)?;
    let doc = read_infoelements(e)?;
    let name = attribute(e, "name")?;
    let value = attribute(e, "value")?;
    let c_identifier = attribute(e, "identifier");
    let glib_nick = attribute(e, "nick");

    Some(Member{
        info,
        doc,
        name,
        value,
        c_identifier,
        glib_nick,
    })
}

fn read_enum(e: &Element) -> Option<Enumeration> {
    let name = attribute(e, "name")?;
    let info = read_infoattrs(e)?;
    let doc = read_infoelements(e)?;

    let c_type = attribute(e, "type")?;
    let glib_type_name = attribute(e, "type-name");
    let glib_get_type = attribute(e, "get-type");
    let glib_error_domain = attribute(e, "error-domain");

    let mut members = vec![];
    let mut functions = vec![];
    for node in e.children.iter() {
        if let Some(e) = node.as_element() {
            match e.name.as_str() {
                "member" => {
                    if let Some(class) = read_member(e) {
                        members.push(class);
                    }
                }
                "function" => {
                    if let Some(fun) = read_function(e, FunctionType::Member) {
                        functions.push(fun);
                    }
                }
                _ => {
                }
            }
        }
    }
    Some(Enumeration{
        info,
        doc,
        name,
        c_type,
        glib_type_name,
        glib_get_type,
        glib_error_domain,
        members,
        functions,
    })

}

pub fn attribute(e: &Element, attr: &str) -> Option<String> {
    e.attributes.get(attr).map(|x| x.to_owned())
}

pub fn attr_value<T>(e: &Element, name: &str) -> Option<T>
where
    T: FromStr,
    T::Err: fmt::Display,
{
    if let Some(value_str) = e.attributes.get(name) {
        match T::from_str(value_str) {
            Ok(value) => Some(value),
            Err(error) => None 
        }
    } else {
        None
    }
}

fn read_include(e: &Element) -> Option<Include> {
    let name = attribute(e, "name")?;
    let version = attribute(e, "version");
    Some(Include { 
        name, 
        version
    })
}
fn read_cinclude(e: &Element) -> Option<CInclude> {
    let name = attribute(e, "name")?;
    Some(CInclude { 
        name, 
    })
}
fn read_package(e: &Element) -> Option<Package> {
    let name = attribute(e, "name")?;
    Some(Package { 
        name, 
    })
}

// should return repo?
fn read_repository(e: &Element) -> Option<Repository> {
    let version = attr_value(e, "version");
    let xmlns = attribute(e, "xmlns");
    let identifier_prefixes = attribute(e, "identifier-prefixes");
    let symbol_prefixes = attribute(e, "symbol-prefixes");

    let mut include = vec![];
    let mut cinclude = vec![];
    let mut package = vec![];
    let mut namespace = vec![];
    
    for node in e.children.iter() {
        if let Some(e) = node.as_element() {
            match e.name.as_str() {
                "include" => {
                    if let Some(ns) = read_include(e) {
                        include.push(ns)
                    }
                }
                "cinclude" => {
                    if let Some(ns) = read_cinclude(e) {
                        cinclude.push(ns)
                    }
                }
                "package" => {
                    if let Some(ns) = read_package(e) {
                        package.push(ns)
                    }
                }
                "namespace" => {
                    if let Some(ns) = read_namespace(e) {
                        namespace.push(ns)
                    }
                }
                _ => {
                }
            }
        }
    }
    Some(Repository {
        version,
        xmlns,
        identifier_prefixes,
        symbol_prefixes,
        include,
        cinclude,
        package,
        namespace,
    })
}

pub fn parse_gir<R: Read>(read: R) -> Result<Repository, xmltree::ParseError> {
    let names_element = Element::parse(read)?;
    let repo = read_repository(&names_element);
    if let Some(repo) = repo {
        return Ok(repo)
    }
    Err(xmltree::ParseError::CannotParse)
}
