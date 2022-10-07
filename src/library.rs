type _Type = String;
type Nullable = bool;
type Version = String;

#[derive(Debug)]
pub struct Repository {
    pub version: Option<String>,
    pub xmlns: Option<String>,
    pub identifier_prefixes: Option<String>,
    pub symbol_prefixes : Option<String>,
    pub include: Vec<Include>,
    pub cinclude: Vec<CInclude>,
    pub package: Vec<Package>,
    pub namespace: Vec<Namespace>,
}

#[derive(Debug)]
pub struct Include {
    pub name: String,
    pub version: Option<String>,
}

#[derive(Debug)]
pub struct CInclude {
    pub name: String,
}

#[derive(Debug)]
pub struct Package {
    pub name: String,
}

#[derive(Default, Debug)]
pub struct Namespace {
    pub name: Option<String>,

    pub version: Option<String>,

    pub shared_library: Option<String>,

    pub identifier_prefixes: Option<String>,
    pub symbol_prefixes: Option<String>,
    pub prefix: Option<String>,

    pub classes: Vec<Class>,
    pub functions: Vec<Function>,
    pub macros: Vec<Macro>,
    pub callback: Vec<Function>,

    // pub interfaces: Vec<Interface>,
    // pub enums: Vec<Enumeration>,
    // pub record: Vec<Record>,
    // pub constant: Vec<Constant>,
    // pub bitfield: Vec<Bitfield>,
    // pub alias: Vec<Alias>,
    // pub unions: Vec<Union>,
    // pub boxed: Vec<Boxed>,

    // pub doc: Option<String>,
    // pub doc_deprecated: Option<String>,
}

#[derive(Default, Debug)]
pub struct InfoAttrs {
    pub introspectable: Option<bool>,
    pub deprecated: Option<String>,
    pub deprecated_version: Option<String>,
    pub version: Option<String>,
    pub stability: Option<String>,
}

#[derive(Default, Debug)]
pub struct InfoElements {
    pub doc: Option<Doc>,
    pub doc_stability: Option<DocVersioned>,
    pub doc_version: Option<DocVersioned>,
    pub doc_deprecated: Option<DocVersioned>,
    pub doc_pos: Option<DocPosition>,
}

#[derive(Default, Debug)]
pub struct Doc {
    pub preserve_space: Option<String>,
    pub preserve_white: Option<String>,
    pub filename: String,
    pub line: String,
    pub column: Option<String>,
    pub content: String,
}

// doc-versioned
#[derive(Default, Debug)]
pub struct DocVersioned {
    pub preserve_space: Option<String>,
    pub preserve_white: Option<String>,
    pub content: String,
}

#[derive(Default, Debug)]
pub struct DocPosition {
    pub filename: String,
    pub line: String,
    pub column: Option<String>,
}

#[derive(Default, Debug)]
pub struct Class {
    pub info: InfoAttrs,

    pub name: String,

    pub glib_type_name: String,
    pub glib_get_type: String,

    pub parent: Option<String>,
    pub glib_type_struct: Option<String>,

    pub ref_func: Option<String>,
    pub unref_func: Option<String>,

    pub set_value_func: Option<String>,
    pub get_value_func: Option<String>,

    pub ctype: Option<String>,

    pub symbol_prefix: Option<String>,
    pub abstracts: Option<String>,

    pub glib_fundamental: Option<String>,
    pub finals: Option<String>,

    pub constructor: Vec<Function>,
    pub functions: Vec<Function>,
    pub method: Vec<Function>,
    pub virtual_method: Vec<Function>,
    pub callbacks: Vec<Function>,

    pub record: Vec<Record>,
    pub fields: Vec<Field>,
    pub signals: Vec<Signal>,
    pub unions: Vec<Union>,
    pub constant: Vec<Constant>,
    pub properties: Vec<Property>,
    pub implements: Vec<String>,

    pub doc: InfoElements,

}

#[derive(Default, Debug)]
pub struct Record {
    pub name: String,
    pub info: InfoAttrs,
    pub doc: InfoElements,

    pub c_type: Option<String>,
    pub disguised: Option<bool>,
    pub symbol_prefix: Option<String>,
    pub glib_get_type: Option<String>,
    pub glib_type_name: Option<String>,
    pub glib_is_gtype_struct_for: Option<String>,
    pub foreign: Option<bool>,

    pub fields: Vec<Field>,
    pub unions: Vec<Union>,

    pub constructor: Vec<Function>,
    pub functions: Vec<Function>,
    pub method: Vec<Function>,
}

#[derive(Default, Debug)]
pub struct Interface {
    pub name: String,

    pub c_type: String,
    pub symbol_prefix: String,
    pub type_struct: Option<String>,
    pub introspectable: bool,
    // pub c_class_type: Option<String>,
    // pub glib_get_type: String,

    // pub functions: Vec<Function>,
    // this should work?

    pub constructor: Vec<Function>,
    pub functions: Vec<Function>,
    pub method: Vec<Function>,
    pub virt: Vec<Function>,

    pub signals: Vec<Signal>,
    pub properties: Vec<Property>,
    pub prerequisites: Vec<_Type>,

    pub version: Option<Version>,
    pub deprecated_version: Option<Version>,

    pub doc: Option<String>,
    pub doc_deprecated: Option<String>,
}

#[derive(Debug)]
pub struct Constant {
    pub info: InfoAttrs,
    pub doc: InfoElements,

    pub name: String,
    pub value: String,
    pub c_identifier: Option<String>,
    pub c_type: Option<String>,

    pub typ: Option<AnyType>,
}


#[derive(Debug)]
pub struct Bitfield {
    pub info: InfoAttrs,
    pub doc: InfoElements,
    pub name: String,

    pub c_type: String,
    pub glib_type_name: Option<String>,
    pub glib_get_type: Option<String>,

    pub members: Vec<Member>,
    pub functions: Vec<Function>,
}

#[derive(Debug)]
pub struct Enumeration {
    pub info: InfoAttrs,
    pub doc: InfoElements,

    pub name: String,

    pub c_type: String,
    pub glib_type_name: Option<String>,
    pub glib_get_type: Option<String>,
    pub glib_error_domain: Option<String>,

    pub members: Vec<Member>,
    pub functions: Vec<Function>,
}

#[derive(Default, Debug)]
pub struct Macro {
    pub info: InfoAttrs,
    pub doc: InfoElements,
    
    pub name: String,
    pub c_identifier: Option<String>,

    pub parameters: Vec<MacroParam>,
}

#[derive(Default, Debug)]
pub struct Function {
    pub info: InfoAttrs,
    pub doc: InfoElements,
    
    pub name: String,
    pub c_identifier: Option<String>,
    pub shadowed_by: Option<String>,
    pub shadows: Option<String>,
    pub throws: Option<bool>,
    pub moved_to: Option<String>,

    pub parameters: Vec<Parameter>,
    pub ret: Option<Parameter>,
}

#[derive(Default, Debug)]
pub struct Union {
    pub name: Option<String>,
    pub info: InfoAttrs,
    pub doc: InfoElements,

    pub c_type: Option<String>,
    pub symbol_prefix: Option<String>,
    pub glib_type_name: Option<String>,
    pub glib_get_type: Option<String>,

    pub fields: Vec<Field>,
    pub constructor: Vec<Function>,
    pub method: Vec<Function>,
    pub functions: Vec<Function>,
    pub record: Vec<Record>,
}

#[derive(Debug)]
pub struct Signal {
    pub name: String,
    pub info: InfoAttrs,
    pub doc: InfoElements,

    pub detailed: Option<bool>,
    pub when: Option<String>,
    pub action: Option<bool>,
    pub no_hooks: Option<bool>,
    pub no_recurse: Option<bool>,
    pub emitter: Option<String>,

    pub parameters: Vec<Parameter>,
    pub ret: Option<Parameter>,
}

#[derive(Debug)]
pub struct Boxed {
    pub name: String,

    pub typename: Option<String>,
    pub get_type: Option<_Type>,
    pub parameters: Vec<Parameter>,
    pub introspectable: bool,
    pub funs: Vec<Function>,

    pub version: Option<Version>,
    pub deprecated_version: Option<Version>,

    pub doc: Option<String>,
    pub doc_deprecated: Option<String>,
}

#[derive(Debug)]
pub struct Alias {
    pub name: String,
    pub introspectable: bool,
    pub c_identifier: String,
    pub typ: _Type,
    pub target_c_type: String,
    pub doc: Option<String>,
    pub doc_deprecated: Option<String>,
}


#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ParameterDirection {
    None,
    In,
    Out,
    InOut,
    Return,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Transfer {
    None,
    Container,
    Full,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ParameterScope {
    None,
    Call,
    Async,
    Notified,
}


#[derive(Debug)]
pub struct Member {
    pub info: InfoAttrs,
    pub doc: InfoElements,
    pub name: String,
    pub value: String,
    pub c_identifier: Option<String>,
    pub glib_nick: Option<String>,
    // pub glib_name: Option<String>,
}

#[derive(Debug)]
pub struct MacroParam {
    pub name: String,
    pub doc: InfoElements,
}

#[derive(Debug)]
pub struct Parameter {
    pub name: String,
    pub nullable: Option<String>,
    pub allow_none: Option<String>,
    pub introspectable: Option<String>,
    pub closure: Option<String>,
    pub destroy: Option<String>,
    pub scope: Option<String>,
    pub direction: Option<String>,
    pub caller_allocates: Option<String>,
    pub optional: Option<String>,
    pub skip: Option<String>,
    pub transfer: Option<Transfer>,
    pub doc: InfoElements,
    pub typ: AnyType,
}

#[derive(Debug)]
pub enum AnyType {
    // Callback(Function),
    Array(Array),
    Type(Type),
    VarArg,
}

#[derive(Debug)]
pub struct Array {
    pub name: Option<String>,
    pub zero_terminated: Option<bool>,
    pub fixed_size: Option<bool>,
    pub introspectable: Option<bool>,
    pub length: Option<usize>,
    pub ctype: Option<String>,
    pub typ: Box<AnyType>,
}

#[derive(Debug)]
pub struct Type {
    pub name: Option<String>,
    pub ctype: Option<String>,
    pub introspectable: Option<bool>,

    pub doc: InfoElements,
    pub children: Vec<AnyType>,
}

#[derive(Debug)]
pub struct Field {
    pub name: String,
    pub info: InfoAttrs,
    pub doc: InfoElements,

    pub typ: AnyType,
    pub writeable: Option<bool>,
    pub readable: Option<bool>,
    pub private: Option<bool>,
    pub bits: Option<u32>,
}

#[derive(Debug)]

pub struct Property {
    pub name: String,
    pub info: InfoAttrs,
    pub doc: InfoElements,

    pub readable: Option<bool>,
    pub writable: Option<bool>,
    pub construct: Option<bool>,
    pub construct_only: Option<bool>,
    pub setter: Option<String>,
    pub getter: Option<String>,
    pub transfer: Option<Transfer>,
    pub typ: AnyType,
}
