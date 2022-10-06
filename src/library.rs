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
    pub macros: Vec<Function>,
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
    //
    // pub fields: Vec<Field>,
    // pub signals: Vec<Signal>,
    // pub properties: Vec<Property>,
    // pub implements: Vec<String>,

    pub doc: InfoElements,

}

#[derive(Default, Debug)]
pub struct Record {
    pub name: String,

    pub c_type: String,
    pub symbol_prefix: Option<String>,
    pub glib_get_type: Option<String>,
    pub introspectable: bool,
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
    pub name: String,
    pub c_identifier: String,
    pub introspectable: bool,

    pub typ: _Type,
    pub c_type: Option<String>,
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
    pub introspectable: bool,
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
    pub introspectable: bool,

    pub members: Vec<Member>,
    pub functions: Vec<Function>,

    pub version: Option<Version>,
    pub deprecated_version: Option<Version>,

    pub doc: Option<String>,
    pub doc_deprecated: Option<String>,

    // pub error_domain: Option<ErrorDomain>,
    // pub glib_get_type: Option<String>,
}



    // pub name: String,
    // attribute name { xsd:string },
    // # C identifier in the source code of the Callable
    // attribute c:identifier { xsd:string }?,
    // ## Callable it is shadowed by. For example, in C++, only one version of an overloaded callable will appear
    // attribute shadowed-by { xsd:string }?,
    // ## Callable it shadows. For example, in C++, only one version of an overloaded callable will appear
    // attribute shadows { xsd:string }?,
    // ## Binary attribute, true if the callable can throw an error
    // attribute throws { "0" | "1" }?,
    // ## if for backward compatibility reason the callable has a name in the source code but should be known by another one, this attribute contains the new name    
    // attribute moved-to { xsd:string }?

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
    pub name: String,

    pub c_type: Option<String>,
    pub symbol_prefix: Option<String>,
    pub introspectable: bool,

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
    pub ret: Option<Parameter>,
    pub introspectable: bool,

    pub is_action: bool,
    pub is_detailed: bool,

    // when
    // recurse

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
    pub name: String,
    pub c_identifier: String,
    pub introspectable: bool,
    pub value: String,
    pub doc: Option<String>,
    pub doc_deprecated: Option<String>,
    /// XXX add this back?
    // pub status: GStatus,
    pub version: Option<Version>,
    pub deprecated_version: Option<Version>,
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

#[derive(Default, Debug)]
pub struct Field {
    pub name: String,
    pub typ: _Type,
    pub c_type: Option<String>,
    pub introspectable: bool,
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
    pub introspectable: bool,
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
