// TODO Fix version to Version in structs

#[derive(Debug)]
pub struct Version(pub u16, pub u16, pub u16);

#[derive(Debug)]
pub struct Repository {
    pub version: Option<Version>,
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

#[derive(Debug)]
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

    pub interfaces: Vec<Interface>,
    pub enums: Vec<Enumeration>,
    pub record: Vec<Record>,
    pub constant: Vec<Constant>,
    pub bitfield: Vec<Bitfield>,
    pub alias: Vec<Alias>,
    pub unions: Vec<Union>,
    pub boxed: Vec<Boxed>,
}

#[derive(Debug)]
pub struct InfoAttrs {
    pub introspectable: Option<bool>,
    // should be a bool
    pub deprecated: Option<bool>,
    pub deprecated_version: Option<String>,
    pub version: Option<String>,
    pub stability: Option<String>,
}

#[derive(Debug)]
pub struct InfoElements {
    pub doc: Option<Doc>,
    pub doc_stability: Option<DocVersioned>,
    pub doc_version: Option<DocVersioned>,
    pub doc_deprecated: Option<DocVersioned>,
    pub doc_pos: Option<DocPosition>,
}

#[derive(Debug)]
pub struct Doc {
    pub preserve_space: Option<String>, // bools? default false?
    pub preserve_white: Option<String>, // bools? default false?
    pub filename: String,
    pub line: String,
    pub column: Option<String>,
    pub content: String,
}

#[derive(Debug)]
pub struct DocVersioned {
    pub preserve_space: Option<String>, // bools? default false?
    pub preserve_white: Option<String>, // bools? default false?
    pub content: String,
}

#[derive(Debug)]
pub struct DocPosition {
    pub filename: String,
    pub line: String,
    pub column: Option<String>,
}

#[derive(Debug)]
pub struct Class {
    pub info: InfoAttrs,
    pub doc: InfoElements,

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
    pub implements: Vec<Implement>,
}

#[derive(Debug)]
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

#[derive(Debug)]
pub struct Interface {
    pub name: String,
    pub info: InfoAttrs,
    pub doc: InfoElements,

    pub glib_type_name: String,
    pub glib_get_type: String,
    pub symbol_prefix: Option<String>,
    pub c_type: Option<String>,
    pub glib_type_struct: Option<String>,

    pub constructor: Option<Function>,
    pub prerequisites: Vec<String>,
    pub implements: Vec<String>,
    pub functions: Vec<Function>,
    pub method: Vec<Function>,
    pub virtual_method: Vec<Function>,
    pub callbacks: Vec<Function>,

    pub fields: Vec<Field>,
    pub properties: Vec<Property>,
    pub signals: Vec<Signal>,
    pub constant: Vec<Constant>,
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

#[derive(Debug)]
pub struct Macro {
    pub info: InfoAttrs,
    pub doc: InfoElements,
    
    pub name: String,
    pub c_identifier: Option<String>,

    pub parameters: Vec<MacroParam>,
}

#[derive(Debug, PartialEq)]
pub enum FunctionType {
    Function,
    Callback,
    Constructor,
    Method,
    Virtual,
    Member,
}

#[derive(Debug)]
pub struct Function {
    pub info: InfoAttrs,
    pub doc: InfoElements,
    pub typ: FunctionType,
    // TODO add kind
    
    pub name: String,
    pub introspectable: Option<bool>,
    pub c_identifier: Option<String>,
    pub shadowed_by: Option<String>,
    pub shadows: Option<String>,
    pub throws: Option<bool>,
    pub moved_to: Option<String>,

    pub parameters: Vec<Parameter>,
    pub ret: Option<Parameter>,
}

#[derive(Debug)]
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
    pub glib_name: String,
    pub info: InfoAttrs,
    pub doc: InfoElements,

    pub symbol_prefix: Option<String>,
    pub glib_type_name: Option<String>,
    pub glib_get_type: Option<String>,

    pub functions: Vec<Function>,
}

#[derive(Debug)]
pub struct Implement {
    pub name: String
}

#[derive(Debug)]
pub struct Alias {
    pub name: String,
    pub info: InfoAttrs,
    pub doc: InfoElements,
    pub c_type: String,
    pub typ: AnyType,
}


#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ParameterDirection {
    None,
    In, // default
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
    pub nullable: bool, // default false
    pub allow_none: bool, // default false
    pub introspectable: Option<bool>,
    pub closure: Option<String>, // u32
    pub destroy: Option<String>, // u32
    pub scope: Option<String>, // ParameterScope
    pub direction: Option<ParameterDirection>,
    pub caller_allocates: bool, // default false
    pub optional: bool, // default false
    pub skip: bool, // default false
    pub transfer: Option<Transfer>,
    pub doc: InfoElements,
    pub typ: AnyType,
}

// In most cases we don't care about what kind the type is, hence we don't care
// if it's a record, a class enum, etc. Because we are not generating real code but 
// only doing anotations and anytype is used for type referencing so specifics are not important. 
// Calling a class method or a function might have different syntax but we don't do the semantics, 
// that is why we can get away with this simplified version of anytype. 
#[derive(Debug)]
pub enum AnyType {
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
    pub typ: String,
    // pub typ: Box<AnyType>,
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
    pub writeable: bool, // default is false
    pub readable: bool, // default is true
    pub private: bool, // default is false
    pub bits: Option<u32>,
}

#[derive(Debug)]
pub struct Property {
    pub name: String,
    pub info: InfoAttrs,
    pub doc: InfoElements,

    pub writable: bool, // default is false
    pub readable: bool, // default is true
    pub construct: bool, // default is false
    pub construct_only: bool, // default is false
    pub setter: Option<String>,
    pub getter: Option<String>,
    pub transfer: Option<Transfer>,
    pub typ: AnyType,
}
