use std::collections::HashSet;

#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Version(pub u16, pub u16, pub u16);

// For now
type _Type = String;
type Nullable = bool;

#[derive(Default, Debug)]
pub struct Repository {
    pub imports: HashSet<String>,
    pub namespaces: Vec<Namespace>,
}

#[derive(Default, Debug)]
pub struct Namespace {
    pub name: String,
    pub package_name: Option<String>,

    pub version: Version,
    pub identifier_prefixes: Vec<String>,
    pub symbol_prefixes: Vec<String>,
    pub shared_library: Vec<String>,

    pub classes: Vec<Class>,
    pub functions: Vec<Function>,
    pub macros: Vec<Function>,
    pub callback: Vec<Function>,

    pub interafes: Vec<Interface>,
    pub enums: Vec<Enumeration>,
    pub record: Vec<Record>,
    pub constant: Vec<Constant>,
    pub bitfield: Vec<Bitfield>,
    pub alias: Vec<Alias>,
    pub unions: Vec<Union>,
    pub boxed: Vec<Boxed>,

    pub doc: Option<String>,
    pub doc_deprecated: Option<String>,
}

#[derive(Default, Debug)]
pub struct Class {
    pub name: String,

    pub c_type: String,
    pub symbol_prefix: String,
    pub type_struct: Option<String>,
    // pub c_class_type: Option<String>,
    // pub glib_get_type: String,

    pub constructor: Vec<Function>,
    pub functions: Vec<Function>,
    pub method: Vec<Function>,
    pub virt: Vec<Function>,
    pub callbacks: Vec<Function>,

    pub fields: Vec<Field>,
    pub signals: Vec<Signal>,
    pub properties: Vec<Property>,

    // is this enough? We don't really need to walk a tree of types
    pub parent: Option<_Type>,
    pub implements: Vec<_Type>,
    // pub final_type: bool,

    pub version: Option<Version>,
    pub deprecated_version: Option<Version>,

    pub doc: Option<String>,
    pub doc_deprecated: Option<String>,
    pub is_abstract: bool,
    pub is_fundamental: bool,
    /// Specific to fundamental types
    pub ref_fn: Option<String>,
    pub unref_fn: Option<String>,
}
#[derive(Default, Debug)]
pub struct Record {
    pub name: String,

    pub c_type: String,
    pub symbol_prefix: Option<String>,
    pub glib_get_type: Option<String>,
    // pub gtype_struct_for: Option<String>,

    pub fields: Vec<Field>,
    pub union: Vec<Union>,

    pub constructor: Vec<Function>,
    pub functions: Vec<Function>,
    pub method: Vec<Function>,

    pub version: Option<Version>,
    pub deprecated_version: Option<Version>,

    pub doc: Option<String>,
    pub doc_deprecated: Option<String>,
    /// A 'disguised' record is one where the c:type is a typedef that
    /// doesn't look like a pointer, but is internally: typedef struct _X *X;
    pub disguised: bool,
}

#[derive(Default, Debug)]
pub struct Interface {
    pub name: String,

    pub c_type: String,
    pub symbol_prefix: String,
    pub type_struct: Option<String>,
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
    pub name: String,
    pub c_identifier: String,
    pub typ: _Type,
    pub c_type: String,
    pub value: String,
    pub version: Option<Version>,
    pub deprecated_version: Option<Version>,
    pub doc: Option<String>,
    pub doc_deprecated: Option<String>,
}


#[derive(Debug)]
pub struct Bitfield {
    pub name: String,
    pub c_type: String,
    pub symbol_prefix: Option<String>,
    pub members: Vec<Member>,
    pub functions: Vec<Function>,
    pub version: Option<Version>,
    pub deprecated_version: Option<Version>,
    pub doc: Option<String>,
    pub doc_deprecated: Option<String>,
    pub glib_get_type: Option<String>,
}

#[derive(Debug)]
pub struct Enumeration {
    pub name: String,

    pub c_type: String,
    pub symbol_prefix: Option<String>,

    pub members: Vec<Member>,
    pub functions: Vec<Function>,

    pub version: Option<Version>,
    pub deprecated_version: Option<Version>,

    pub doc: Option<String>,
    pub doc_deprecated: Option<String>,

    // pub error_domain: Option<ErrorDomain>,
    // pub glib_get_type: Option<String>,
}

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub c_identifier: Option<String>,

    pub parameters: Vec<Parameter>,
    pub ret: Parameter,
    pub throws: bool,

    pub version: Option<Version>,
    pub deprecated_version: Option<Version>,

    pub doc: Option<String>,
    pub doc_deprecated: Option<String>,
}

#[derive(Default, Debug)]
pub struct Union {
    pub name: String,

    pub c_type: Option<String>,
    pub symbol_prefix: Option<String>,

    // pub glib_get_type: Option<String>,
    pub fields: Vec<Field>,

    pub constructor: Vec<Function>,
    pub functions: Vec<Function>,
    pub method: Vec<Function>,

    pub doc: Option<String>,
}

#[derive(Debug)]
pub struct Signal {
    pub name: String,

    pub parameters: Vec<Parameter>,
    pub ret: Parameter,

    pub is_action: bool,
    pub is_detailed: bool,

    pub version: Option<Version>,
    pub deprecated_version: Option<Version>,

    pub doc: Option<String>,
    pub doc_deprecated: Option<String>,
}

#[derive(Debug)]
pub struct Boxed {
    pub name: String,

    pub typename: Option<String>,
    pub get_type: Option<_Type>,
    pub parameters: Vec<Parameter>,
    pub funs: Vec<Function>,

    pub version: Option<Version>,
    pub deprecated_version: Option<Version>,

    pub doc: Option<String>,
    pub doc_deprecated: Option<String>,
}

#[derive(Debug)]
pub struct Alias {
    pub name: String,
    pub c_identifier: String,
    pub typ: _Type,
    pub target_c_type: String,
    pub doc: Option<String>,
    pub doc_deprecated: Option<String>,
}

//////////////////////

#[derive(Debug, Clone)]
pub enum Type {
    Primitive(String),
    LocalClass(String),
    ExternalClass { module: String, name: String },
    Any,
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
    pub name: String,
    pub c_identifier: String,
    pub value: String,
    pub doc: Option<String>,
    pub doc_deprecated: Option<String>,
    /// XXX add this back?
    // pub status: GStatus,
    pub version: Option<Version>,
    pub deprecated_version: Option<Version>,
}

#[derive(Clone, Debug)]
pub struct Parameter {
    pub name: String,
    pub typ: _Type,

    pub c_type: String,
    pub instance_parameter: bool,

    pub direction: ParameterDirection,
    pub transfer: Transfer,

    pub caller_allocates: bool,
    pub nullable: Nullable,
    pub allow_none: bool,
    pub array_length: Option<u32>,
    pub is_error: bool,
    pub doc: Option<String>,
    pub scope: ParameterScope,
    /// Index of the user data parameter associated with the callback.
    pub closure: Option<usize>,
    /// Index of the destroy notification parameter associated with the callback.
    pub destroy: Option<usize>,
}

#[derive(Default, Debug)]
pub struct Field {
    pub name: String,
    pub typ: _Type,
    pub c_type: Option<String>,
    pub private: bool,
    pub bits: Option<u8>,
    pub array_length: Option<u32>,
    pub doc: Option<String>,
}

#[derive(Debug)]
pub struct Property {
    pub name: String,
    pub readable: bool,
    pub writable: bool,
    pub construct: bool,
    pub construct_only: bool,
    pub typ: _Type,
    pub c_type: Option<String>,
    pub transfer: Transfer,
    pub version: Option<Version>,
    pub deprecated_version: Option<Version>,
    pub doc: Option<String>,
    pub doc_deprecated: Option<String>,
}
