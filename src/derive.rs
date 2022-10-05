use std::borrow::Cow;
use std::fs::File;
use strong_xml::{XmlRead, XmlWrite};

use yaserde_derive::YaDeserialize;


// #[derive(XmlWrite, XmlRead, PartialEq, Debug)]
// #[xml(tag = "repository")]
// #[yaserde(root = "repository")]
#[derive(Default, Debug, Clone, PartialEq, YaDeserialize)]
#[yaserde(
    root = "repository"
        // prefix = "microsoft_spreadsheet",
    namespace = "http://www.gtk.org/introspection/core/1.0"
)]
struct Repository {
    // #[xml(attr = "version")]
    #[yaserde(attribute)]
    version: Option<String>,

    #[yaserde(attribute)]
    xmlns: Option<String>,
    // #[xml(attr = "c:identifier-prefixes")]
    #[yaserde(attribute)]
    identifier_prefix: Option<String>,
    // #[xml(attr = "c:symbol-prefixes ")]
    #[yaserde(attribute)]
    symbol_prefixes : Option<String>,

    // #[xml(child = "include")]
    #[yaserde(child)]
    include: Vec<Include>,
    // #[xml(child = "c:include")]
    #[yaserde(child)]
    cinclude: Vec<CInclude>,
    // #[xml(child = "package")]
    #[yaserde(child)]
    package: Vec<Package>,
    // #[yaserde(child)]
    namespace: Vec<Namespace>,
}

// #[derive(XmlWrite, XmlRead, PartialEq, Debug)]
// #[xml(tag = "include")]
#[derive(Default, Debug, Clone, PartialEq, YaDeserialize)]
#[yaserde( rename = "include")]
struct Include {
    #[yaserde(attribute)]
    name: String,
    #[yaserde(attribute)]
    version: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, YaDeserialize)]
#[yaserde( rename = "c:include")]
struct CInclude {
    #[yaserde(attribute)]
    name: String,
}

#[derive(Default, Debug, Clone, PartialEq, YaDeserialize)]
#[yaserde( rename = "package")]
struct Package {
    #[yaserde(attribute)]
    name: String,
}

#[derive(Default, Debug, Clone, PartialEq, YaDeserialize)]
#[yaserde( rename = "namespace")]
struct Namespace {
    #[yaserde(attribute)]
    name: Option<String>,

    #[yaserde(attribute)]
    version: Option<String>,

    #[yaserde(attribute, rename = "shared-library")]
    shared_library: Option<String>,

    #[yaserde(attribute, rename = "c:identifier-prefixes")]
    identifier_prefix: Option<String>,
    #[yaserde(attribute, rename = "c:symbol-prefixes")]
    symbol_prefixes: Option<String>,
    #[yaserde(attribute, rename = "c:prefixes")]
    prefix: Option<String>,

    #[yaserde(child)]
    alias: Vec<Alias>,
    //
    #[yaserde(child)]
    class: Vec<Class>,
    // 
    #[yaserde(child)]
    interface: Vec<Interface>,
    //
    #[yaserde(child)]
    record: Vec<Record>,
    //
    #[yaserde(child, rename = "enumeration")]
    enums: Vec<Enum>,
    //
    #[yaserde(child)]
    function: Vec<Function>,

    #[yaserde(child)]
    union: Vec<Union>,

    #[yaserde(child)]
    bitfield: Vec<Bitfield>,

    #[yaserde(child)]
    callback: Vec<Callback>,

    #[yaserde(child)]
    constant: Vec<Constant>,

    #[yaserde(child)]
    annotation: Vec<Annotation>,

    #[yaserde(child, rename = "glib:boxed")]
    boxed: Vec<Boxed>,
}

#[derive(Default, Debug, Clone, PartialEq, YaDeserialize)]
#[yaserde( rename = "doc")]
struct InfoElements {
    doc: Option<Doc>,
    #[yaserde( rename = "doc-stability")]
    stability: Option<DocVersioned>,
    #[yaserde( rename = "doc-deprecated")]
    version: Option<DocVersioned>,
    #[yaserde( rename = "doc-version")]
    deprecated: Option<DocVersioned>,
    doc_pos: Option<DocPosition>,
}


#[derive(Default, Debug, Clone, PartialEq, YaDeserialize)]
struct Alias {
    // Info.attrs,
    info: InfoAttrs,

    #[yaserde(attribute)]
    name: String,
    #[yaserde(attribute, rename = "c:type")]
    ctype: String,
    
    // #[xml(child = "doc")]
    // doc: Option<Doc<'a>>,
    // #[xml(child = "doc-version")]
    // doc_version: Option<Doc<'a>>,
    // #[xml(child = "doc-stability")]
    // doc_stability: Option<Doc<'a>>,
    // #[xml(child = "doc-deprecated")]
    // doc_deprecated: Option<Doc<'a>>,
    // #[xml(child = "source-position")]
    // source_position: Option<Doc<'a>>,

    // #[xml(child = "type")]
    // typ: Type,
       //    (Info.elements
       // & Type)
  // Info.elements = (
  //   DocElements
  //   & Annotation*
  // )
}

#[derive(Default, Debug, Clone, PartialEq, YaDeserialize)]
#[yaserde( rename = "doc")]
struct Doc {
    #[yaserde(attribute, rename = "xml:space")]
    preserve_space: Option<String>,

    #[yaserde(attribute, rename = "xml:whitespace")]
    preserve_white: Option<String>,

    #[yaserde(attribute)]
    filename: String,

    #[yaserde(attribute)]
    line: String,

    #[yaserde(attribute)]
    column: String,

    #[yaserde(text)]
    content: String,
}

// doc-versioned
#[derive(Default, Debug, Clone, PartialEq, YaDeserialize)]
struct DocVersioned {
    #[yaserde(attribute, rename = "xml:space")]
    preserve_space: Option<String>,

    #[yaserde(attribute, rename = "xml:whitespace")]
    preserve_white: Option<String>,

    #[yaserde(text)]
    content: String,
}

#[derive(Default, Debug, Clone, PartialEq, YaDeserialize)]
#[yaserde( rename = "source-position")]
struct DocPosition {

    #[yaserde(text)]
    filename: String,

    #[yaserde(text)]
    line: String,

    #[yaserde(text)]
    column: String,
}

#[derive(XmlWrite, XmlRead, PartialEq, Debug)]
#[xml(tag = "array")]
struct ArrayType<'a> {

    #[xml(attr = "name")]
    name: Option<Cow<'a, str>>,

    #[xml(attr = "zero-terminated")]
    zero_terminated: Option<Cow<'a, str>>,

    #[xml(attr = "fixed-size")]
    fixed_size: Option<Cow<'a, str>>,

    #[xml(attr = "introspectable")]
    introspectable: Option<Cow<'a, str>>,

    #[xml(attr = "length")]
    length: Option<Cow<'a, str>>,

    #[xml(attr = "c:type")]
    ctype: Option<Cow<'a, str>>,

    //     # Type of the values contained in the array
    //     AnyType
}


#[derive(Default, Debug, Clone, PartialEq, YaDeserialize)]
struct Type {
    
    #[yaserde(attribute)]
    name: Option<String>,

    #[yaserde(attribute, rename = "type")]
    ctype: Option<String>,

    #[yaserde(attribute)]
    introspectable: Option<String>,

    // #[yaserde(attribute, rename = "type")]
    // children_type: Vec<Type<'a>>,

    // #[xml(child = "array")]
    // children_array: Vec<Array<'a>>,

    // #[xml(attr = "column")]
    // column: String,

    // Type =
    //   # A simple type of data (as opposed to an array)
    //   element type {
    //     ## name of the type
    //     attribute name { xsd:string }?,
    //     ## the C representation of the type
    //     attribute c:type { xsd:string }?,
    //     ## Binary attribute which is "0" (false) if the element is not introspectable. It doesn't exist in the bindings, due in general to missing information in the annotations in the original C code
    //     attribute introspectable { "0" | "1" }?,
    //
    //     (DocElements & AnyType*)
    //   }
}

#[derive(Default, Debug, Clone, PartialEq, YaDeserialize)]
struct InfoAttrs {
    #[yaserde(attribute)]
    introspectable: Option<String>,
    #[yaserde(attribute)]
    deprecated: Option<String>,
    #[yaserde(attribute, rename="deprecated-version")]
    deprecated_version: Option<String>,
    #[yaserde(attribute)]
    version: Option<String>,
    #[yaserde(attribute)]
    stability: Option<String>,

}


#[derive(Default, Debug, Clone, PartialEq, YaDeserialize)]
#[yaserde(rename = "class")]
struct Class {
    info: InfoAttrs,

    #[yaserde(attribute)]
    name: String,
    #[yaserde(attribute, rename="type-name")]
    glib_type_name: String,
    #[yaserde(attribute, rename="get-type")]
    glib_get_type: String,

    #[yaserde(attribute)]
    parent: Option<String>,

    #[yaserde(attribute, rename="glib:type-struct")]
    glib_type_struct: Option<String>,

    #[yaserde(attribute, rename="glib:ref-func")]
    ref_func: Option<String>,

    #[yaserde(attribute, rename="glib:unref-func")]
    unref_func: Option<String>,

    #[yaserde(attribute, rename="glib:set-value-func")]
    set_value_func: Option<String>,

    #[yaserde(attribute, rename="glib:get-value-func")]
    get_value_func: Option<String>,

    #[yaserde(attribute, rename="c:type")]
    ctype: Option<String>,

    #[yaserde(attribute, rename="c:symbol-prefix")]
    symbol_prefix: Option<String>,

    #[yaserde(attribute)]
    abstracts: Option<String>,

    #[yaserde(attribute, rename="fundamental")]
    glib_fundamental: Option<String>,

    #[yaserde(attribute, rename="final")]
    finals: Option<String>,

    // info element, copy paste, idk
    // XXX fix this
    doc: Option<Doc>,
    #[yaserde( rename = "doc-stability")]
    doc_stability: Option<DocVersioned>,
    #[yaserde( rename = "doc-deprecated")]
    doc_version: Option<DocVersioned>,
    #[yaserde( rename = "doc-version")]
    doc_deprecated: Option<DocVersioned>,
    doc_pos: Option<DocPosition>,

    annotations: Vec<Annotation>,

  //     (Info.elements
  //     Info.elements = (
  //   DocElements
  //   & Annotation*
  // )
    #[yaserde(child, rename = "implemnts")]
    implements: Vec<Implements>,

    #[yaserde(child, rename = "constructor")]
    constructor: Vec<Function>,

    #[yaserde(child, rename = "method")]
    method: Vec<Function>,

    #[yaserde(child, rename = "function")]
    function: Vec<Function>,

    #[yaserde(child, rename = "virtual-method")]
    virtual_method: Vec<Function>,

    // #[xml(child = "field")]
    // field: Vec<Field>,

    // TODO
    #[yaserde(child, rename = "property")]
    property: Vec<Property>,

    #[yaserde(child, rename = "signal")]
    signal: Vec<Signal>,

    #[yaserde(child, rename = "union")]
    union: Vec<Union>,

    #[yaserde(child, rename = "constant")]
    constant: Vec<Constant>,

    #[yaserde(child, rename = "callback")]
    callback: Vec<Callback>,

  //     # Other elements a class can contain
  //      & Implements*
  //      & Constructor*
  //      & Method*
  //      & Function*
  //      & VirtualMethod*
  //      & Field*
  //      & Property*
  //      & Signal*
  //      & Union*
  //      & Constant*
  //      & Record*
  //      & Callback*)
}

#[derive(Default, Debug, Clone, PartialEq, YaDeserialize)]
#[yaserde(rename ="property")]
struct Property {
    info: InfoAttrs,

    #[yaserde(attribute)]
    name: String,

    #[yaserde(attribute)]
    writable: Option<String>,

    #[yaserde(attribute)]
    readable: Option<String>,

    #[yaserde(attribute)]
    construct: Option<String>,

    #[yaserde(attribute, rename = "construct-only")]
    construct_only: Option<String>,

    #[yaserde(attribute)]
    setter: Option<String>,
    #[yaserde(attribute)]
    getter: Option<String>,


      // TransferOwnership?,
      //
      // # Other elements a property can contain
      // (Info.elements
      //  & AnyType)
}

#[derive(Default, Debug, Clone, PartialEq, YaDeserialize)]
#[yaserde(rename ="Signal")]
struct Signal {
    info: InfoAttrs,

    #[yaserde(attribute)]
    name: String,

    #[yaserde(attribute)]
    detialed: Option<String>,

    #[yaserde(attribute)]
    when: Option<String>,

    #[yaserde(attribute)]
    action: Option<String>,

    #[yaserde(attribute, rename = "no-hooks")]
    no_hooks: Option<String>,

    #[yaserde(attribute, rename = "no-recurse")]
    no_recurse: Option<String>,

    #[yaserde(attribute)]
    emitter: Option<String>,
    //
    //   # Other elements a signal can contain
    //   (Info.elements
    //    & Callable.params?
    //    & Callable.return?)
    // }
}

#[derive(Default, Debug, Clone, PartialEq, YaDeserialize)]
#[yaserde(rename ="implements")]
struct Implements {
    #[yaserde(attribute)]
    name: String,
}

#[derive(Default, Debug, Clone, PartialEq, YaDeserialize)]
#[yaserde(rename ="implements")]
struct Interface {
    info: InfoAttrs,

    #[yaserde(attribute)]
    name: String,
    #[yaserde(attribute, rename = "glib:type-name")]
    glib_type_name: String,
    #[yaserde(attribute, rename = "glib:get-type")]
    glib_get_type: String,
    #[yaserde(attribute, rename = "glib:type-struct")]
    glib_type_struct: Option<String>,

    #[yaserde(attribute, rename = "c:type")]
    ctype: Option<String>,
    #[yaserde(attribute, rename = "c:symbol-prefix")]
    symbol_prefix: Option<String>,
  //
  //     # Other elements an interface can contain
  //     (Info.elements
  //      & Prerequisite*
  //      & Implements*
  //      & Function*
  //      & Constructor?
  //      & Method*
  //      & VirtualMethod*
  //      & Field*
  //      & Property*
  //      & Signal*
  //      & Callback*
  //      & Constant*)
  //   }
}

#[derive(Default, Debug, Clone, PartialEq, YaDeserialize)]
#[yaserde(rename ="record")]
struct Record {
    info: InfoAttrs,

    #[yaserde(attribute)]
    name: String,

    #[yaserde(attribute, rename = "c:type")]
    ctype: Option<String>,
    #[yaserde(attribute, rename = "c:symbol-prefix")]
    symbol_prefix: Option<String>,

    #[yaserde(attribute, rename = "disguised")]
    disguised: Option<String>,

    #[yaserde(attribute, rename = "glib:type-name")]
    glib_type_name: Option<String>,
    #[yaserde(attribute, rename = "glib:get-type")]
    glib_get_type: Option<String>,
    #[yaserde(attribute, rename = "glib:is-gtype-struct-for")]
    is_gtype_struct_for: Option<String>,

    #[yaserde(attribute)]
    foreign: Option<String>,

   //    # Other elements a record can contain
   //      # mandatory 
   //    (Info.elements
   //     & Field*
   //     & Function*
   //     & Union*
   //     & Method*
   //     & Constructor*)
   //  }
}

#[derive(Default, Debug, Clone, PartialEq, YaDeserialize)]
#[yaserde(rename ="record")]
struct Enum {
    info: InfoAttrs,
    // XXX Info.attrs,

    #[yaserde(attribute)]
    name: String,

    #[yaserde(attribute, rename = "c:type")]
    ctype: Option<String>,

    #[yaserde(attribute, rename = "glib:type-name")]
    glib_type_name: Option<String>,
    #[yaserde(attribute, rename = "glib:get-type")]
    glib_get_type: Option<String>,
    #[yaserde(attribute, rename = "glib:error-domain")]
    glib_error_domain: Option<String>,
    
  //     (Info.elements
  //      & Member*
  //      & Function*)
  //   }
  //
}

#[derive(Default, Debug, Clone, PartialEq, YaDeserialize)]
struct Function {
    info: InfoAttrs,
    // Callable.attrs,
    // XXX Info.attrs,

    #[yaserde(attribute)]
    name: String,

    #[yaserde(attribute)]
    identifier: Option<String>,

    #[yaserde(attribute, rename = "shadowed-by")]
    shadowed_by: Option<String>,

    #[yaserde(attribute, rename = "shadows")]
    shadows: Option<String>,

    #[yaserde(attribute, rename = "throws")]
    throws: Option<String>,

    #[yaserde(attribute, rename = "move-to")]
    moved_to: Option<String>,

    // (Callable.params?
    //  & Callable.return?
    //  & DocElements)
}
#[derive(Default, Debug, Clone, PartialEq, YaDeserialize)]
#[yaserde(rename ="union")]
struct Union {
    info: InfoAttrs,

    #[yaserde(attribute)]
    name: Option<String>,

    #[yaserde(attribute, rename = "c:type")]
    ctype: Option<String>,

    #[yaserde(attribute, rename = "c:symbol-prefix")]
    symbol_prefix: Option<String>,

    #[yaserde(attribute, rename = "glib:type-name")]
    glib_type_name: Option<String>,
    #[yaserde(attribute, rename = "glib:get-type")]
    glib_get_type: Option<String>,
      // (Info.elements
      //  & Field*
      //  & Constructor*
      //  & Method*
      //  & Function*
      //  & Record*)
}

#[derive(Default, Debug, Clone, PartialEq, YaDeserialize)]
#[yaserde(rename ="union")]
struct Bitfield {
    info: InfoAttrs,

    #[yaserde(attribute)]
    name: Option<String>,

    #[yaserde(attribute, rename = "c:type")]
    ctype: Option<String>,

    #[yaserde(attribute, rename = "glib:type-name")]
    glib_type_name: Option<String>,
    #[yaserde(attribute, rename = "glib:get-type")]
    glib_get_type: Option<String>,
    // (Info.elements
    //  & Member*
    //  & Function*)
}

#[derive(Default, Debug, Clone, PartialEq, YaDeserialize)]
#[yaserde(rename ="callback")]
struct Callback {
    info: InfoAttrs,

    #[yaserde(attribute "name")]
    name: String,

    #[yaserde(attribute, rename = "c:type")]
    ctype: Option<String>,

    #[yaserde(attribute, rename = "throws")]
    throws: Option<String>,
    
      // (Info.elements
      //  & Callable.params?
      //  & Callable.return?)
}

#[derive(Default, Debug, Clone, PartialEq, YaDeserialize)]
#[yaserde(rename ="constant")]
struct Constant {
    info: InfoAttrs,

    #[yaserde(attribute)]
    name: String,

    #[yaserde(attribute)]
    value: String,

    #[yaserde(attribute, rename = "c:type")]
    ctype: Option<String>,

    #[yaserde(attribute, rename = "c:identifer")]
    identifier: Option<String>,

    // (Info.elements
    //  & AnyType?)
}

#[derive(Default, Debug, Clone, PartialEq, YaDeserialize)]
#[yaserde( rename = "annotation")]
struct Annotation {
    
    #[yaserde(attribute)]
    name: String,

    #[yaserde(attribute)]
    value: String,
}

#[derive(Default, Debug, Clone, PartialEq, YaDeserialize)]
#[yaserde( rename = "glib:boxed")]
struct Boxed {
    info: InfoAttrs,

    #[yaserde(attribute, rename = "glib:name")]
    name: String,

    #[yaserde(attribute, rename = "c:symbol-prefix")]
    symbol_prefix: Option<String>,

    #[yaserde(attribute, rename = "glib:type-name")]
    glib_type_name: Option<String>,
    #[yaserde(attribute, rename = "glib:get-type")]
    glib_get_type: Option<String>,

    //   # Other elements a Boxed type can contain
    //   (Info.elements
    //    & Function*)
    // }
}

   // element namespace {
   //    ## name of the namespace. For example, 'Gtk'
   //    attribute name { xsd:string }?,
   //    ## version number of the namespace
   //    attribute version { xsd:string }?,
   //    ## prefixes to filter out from C identifiers for data structures and types. For example, GtkWindow will be Window. If c:symbol-prefixes is not used, then this element is used for both
   //    attribute c:identifier-prefixes { xsd:string }?,
   //    ## prefixes to filter out from C functions. For example, gtk_window_new will lose gtk_
   //    attribute c:symbol-prefixes { xsd:string }?,
   //    ## Deprecated: the same as c:identifier-prefixes. Only used for backward compatibility 
   //    attribute c:prefix { xsd:string }?,
   //    ## Path to the shared library implementing the namespace. It can be a comma-separated list, with relative path only
   //    attribute shared-library { xsd:string }?,
   //
   //    # Other elements a namespace can contain
   //    (Alias*
   //     & Class*
   //     & Interface*
   //     & Record*
   //     & Enum*
   //     & Function*
   //     & Union*
   //     & BitField*
   //     & Callback*
   //     & Constant*
   //     & Annotation*
   //     & Boxed*)
   //  }





fn main() -> strong_xml::XmlResult<()> {
    // let mut file = File::open("/home/dagle/code/gir-parser2/GMime-3.0.gir");
    let my_str = include_str!("/home/dagle/code/gir-parser2/GMime-3.0.gir");
    // let my_str = include_str!("/home/dagle/code/gir-parser2/Gtk-4.0.gir");
    let repo: Repository = yaserde::de::from_str(my_str).unwrap();
    // let repo = Repository::from_str(my_str)?;
    println!("{:#?}", repo);

    Ok(())
}
