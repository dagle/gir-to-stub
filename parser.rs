use crate::{
    library::*,
    version::Version,
    xmlparser::{Element, XmlParser},
};
use log::{trace, warn};
use std::{
    path::{Path, PathBuf},
    str::FromStr,
};

const EMPTY_CTYPE: &str = "/*EMPTY*/";

pub fn is_empty_c_type(c_type: &str) -> bool {
    c_type == EMPTY_CTYPE
}

impl Repository {
    pub fn read_file<P: AsRef<Path>>(
        &mut self,
        path: P
    ) -> Result<(), String> {
        let mut parser = XmlParser::from_path(path)?;
        return parser.document(|p, _| {
            p.element_with_name("repository", |sub_parser, _elem| {
                self.read_repository(dirs, sub_parser, libs)
            })
        });
    }

    fn read_repository<P: AsRef<Path>>(
        &mut self,
        path: P,
        parser: &mut XmlParser<'_>,
    ) -> Result<(), String> {
        parser.elements(|parser, elem| match elem.name() {
            "include" => parser.ignore_element(),
            "package" => parser.ignore_element(),
            "namespace" => {
                self.read_namespace(parser, elem)
            }
            "attribute" => parser.ignore_element(),
            _ => Err(parser.unexpected_element(elem)),
        })?;
        Ok(())
    }

    fn read_namespace(
        &mut self,
        parser: &mut XmlParser<'_>,
        elem: &Element,
    ) -> Result<(), String> {
        let ns_name = elem.attr_required("name")?;
        let ns_id = self.add_namespace(ns_name);
        // {
        //     let ns = self.namespace_mut(ns_id);
        //     ns.package_name = package;
        //     ns.c_includes = c_includes;
        //     if let Some(s) = elem.attr("shared-library") {
        //         ns.shared_library = s.split(',').map(String::from).collect();
        //     }
        //     if let Some(s) = elem.attr("identifier-prefixes") {
        //         ns.identifier_prefixes = s.split(',').map(String::from).collect();
        //     }
        //     if let Some(s) = elem.attr("symbol-prefixes") {
        //         ns.symbol_prefixes = s.split(',').map(String::from).collect();
        //     }
        // }
        //
        // trace!(
        //     "Reading {}-{}",
        //     ns_name,
        //     elem.attr("version").unwrap_or("?")
        // );

        parser.elements(|parser, elem| {
            trace!("<{} name={:?}>", elem.name(), elem.attr("name"));
            match elem.name() {
                "class" => {
                    let class = Class::read(parser, elem);
                    self.class.push(class);
                }
                "record" => {
                    self.record.push(Record::read(parser, elem, None, None))
                }
                "union" => {
                    let uni = read_union(parser, elem, None, None);
                    self.unions.push(uni);
                }
                "interface" => {
                    self.interface.push(read_interface(parser, elem))
                }
                "callback" => {
                    self.callback.push(read_function(parser, elem))
                }
                "bitfield" => {
                    self.bitfield.push(read_bitfield(parser, elem))
                }
                "enumeration" => {
                    self.enums.push(read_enumeration(parser, elem))
                }
                "function" => {
                    self.functions.push(read_function(parser, elem))
                }
                "constant" => {
                    self.constants.push(read_constant(parser, elem))
                }
                "alias" => {
                    self.alias.push(read_alias(parser, elem))
                }
                // TODO
                "function-macro" => {
                    parser.ignore_element()
                } 
                // TODO
                "docsection" => {
                    parser.ignore_element()
                }
                _ => {
                    warn!("<{} name={:?}>", elem.name(), elem.attr("name"));
                    parser.ignore_element()
                }
            }
        })?;
        Ok(())
    }
}

impl Class {
    pub fn read(
        parser: &mut XmlParser<'_>,
        elem: &Element,
    ) -> Result<Class, String> {
        let name = elem.attr_required("name")?;
        // let c_type = self.read_object_c_type(parser, elem)?;
        let symbol_prefix = elem.attr_required("symbol-prefix").map(ToOwned::to_owned)?;
        let type_struct = elem.attr("type-struct").map(ToOwned::to_owned);

        let version = self.read_version(parser, ns_id, elem)?;
        let deprecated_version = self.read_deprecated_version(parser, ns_id, elem)?;
        let is_fundamental = elem.attr("fundamental").map(|x| x == "1").unwrap_or(false);
        let (ref_fn, unref_fn) = if is_fundamental {
            (
                elem.attr("ref-func").map(ToOwned::to_owned),
                elem.attr("unref-func").map(ToOwned::to_owned),
            )
        } else {
            (None, None)
        };

        let is_abstract = elem.attr("abstract").map(|x| x == "1").unwrap_or(false);

        let parent = elem.attr("parent");

        let mut class = Class::new(
            name, 

            c_type, 
            symbol_prefix, 
            type_struct,

            parent,

            version,
            deprecated_version,

            is_fundamenal,
            is_abstract,

            ref_fn,
            unref_fn,
        );

        parser.elements(|parser, elem| match elem.name() {
            "constructor" => {
                let constr = read_function(parser, elem);
                class.constructor.push(contr)
            }
            "function" => {
                let fun = read_function(parser, elem);
                class.constructor.push(fun)
            }
            "method" => {
                let method = read_function(parser, elem);
                class.constructor.push(method)
            }
            "implements" => {
                class.read_implement(parser, elem)
            }
            "signal" => {
                let signal = read_signal(parser, elem);
                class.signal.push(signal)
            }
            "property" => {
                let prop = read_property(parser, elem);
                class.property.push(prop);
            }
            "field" => {
                let field = read_field(parser, elem);
                class.field.push(field);
            }
            "virtual-method" => {
                let virt = read_virt(parser, elem);
                class.virt.push(virt);
            }
            "doc" => {
                class.doc = parser.text().ok();
            }
            "doc-deprecated" => {
                class.doc_deprecated = parser.text().ok();
            }
            "source-position" => parser.ignore_element(),
            "union" => {
                // add the class ns
                let uni = read_union(parser, elem, None, None);
                class.unions.push(uni);
            }
            "attribute" => parser.ignore_element(),
            _ => Err(parser.unexpected_element(elem)),
        })?;
        Ok(class)
    }
}

impl Record {
    fn read(
        parser: &mut XmlParser<'_>,
        elem: &Element,
        parent_name_prefix: Option<&str>,
        parent_ctype_prefix: Option<&str>,
    ) -> Result<Record, String> {
        let record_name = elem.attr_required("name")?;
        // Records starting with `_` are intended to be private and should not be bound
        if record_name.starts_with('_') {
            parser.ignore_element()?;
            return Ok(None);
        }
        let c_type = elem.attr_required("type")?;
        let symbol_prefix = elem.attr("symbol-prefix").map(ToOwned::to_owned);
        let get_type = elem.attr("get-type").map(ToOwned::to_owned);
        // let gtype_struct_for = elem.attr("is-gtype-struct-for");
        let version = self.read_version(parser, ns_id, elem)?;
        let deprecated_version = self.read_deprecated_version(parser, ns_id, elem)?;
        let disguised = elem.attr_bool("disguised", false);

        let mut record = Record::new(
            name, 

            c_type, 
            symbol_prefix, 
            type_struct,

            version,
            deprecated_version,

            is_fundamenal,
            is_abstract,

            disguised,
        );

        parser.elements(|parser, elem| match elem.name() {
            "constructor" => {
                let constr = read_function(parser, elem);
                record.constructor.push(contr)
            }
            "function" => {
                let fun = read_function(parser, elem);
                record.constructor.push(fun)
            }
            "method" => {
                let method = read_function(parser, elem);
                record.constructor.push(method)
            }
            "field" => {
                let field = read_field(parser, elem);
                record.field.push(field);
            }
            "doc" => {
                record.doc = parser.text().ok();
            }
            "doc-deprecated" => {
                record.doc_deprecated = parser.text().ok();
            }
            "source-position" => parser.ignore_element(),
            "union" => {
                // add record ns
                let uni = read_union(parser, elem);
                record.unions.push(uni);
            }
            _ => Err(parser.unexpected_element(elem)),
        })?;
        Ok(Record)
    }
}

fn read_union(
    parser: &mut XmlParser<'_>,
    elem: &Element,
    parent_name_prefix: Option<&str>,
    parent_ctype_prefix: Option<&str>,
) -> Result<Union, String> {

    let union_name = elem.attr("name").unwrap_or("");
    let c_type = self.read_object_c_type(parser, elem).unwrap_or("");
    let get_type = elem.attr("get-type").map(|s| s.into());
    let symbol_prefix = elem.attr("symbol-prefix").map(ToOwned::to_owned);

    let mut uni = Union::new(
        name,
        ctype,
        symbol_prefix
    );

    parser.elements(|parser, elem| match elem.name() {
        "source-position" => parser.ignore_element(),
        "field" => {
            let field = read_field(parser, elem);
            uni.field.push(field);
        }
        "constructor" => {
            let constr = read_function(parser, elem);
            uni.constructor.push(contr)
        }
        "function" => {
            let fun = read_function(parser, elem);
            uni.function.push(fun)
        }
        "method" => {
            let method = read_function(parser, elem);
            uni.method.push(method)
        }
        "record" => {
            parser.ignore_element();
            // let record = Record::read(parser, elem, parent_name_prefix, parent_ctype_prefix);
        }
        "doc" => { 
            uni.doc = parser.text().ok();
        }
        "attribute" => parser.ignore_element(),
        _ => Err(parser.unexpected_element(elem)),
    })?;

    Ok(uni)
}

// TODO
fn read_field(
    parser: &mut XmlParser<'_>,
    elem: &Element,
) -> Result<Field, String> {
    let field_name = elem.attr_required("name")?;
    let private = elem.attr_bool("private", false);
    let bits = elem.attr("bits").and_then(|s| s.parse().ok());

    let mut typ = None;
    let mut doc = None;

    parser.elements(|parser, elem| match elem.name() {
        "type" | "array" => {
            // if typ.is_some() {
            //     return Err(parser.fail("Too many <type> elements"));
            // }
            read_type(parser, ns_id, elem).map(|t| {
                typ = Some(t);
            })
        }
        "callback" => {
            if typ.is_some() {
                return Err(parser.fail("Too many <type> elements"));
            }
            self.read_function(parser, ns_id, elem.name(), elem)
                .map(|f| {
                    typ = Some((Type::function(self, f), None, None));
                })
        }
        "doc" => parser.text().map(|t| doc = Some(t)),
        "attribute" => parser.ignore_element(),
        _ => Err(parser.unexpected_element(elem)),
    })?;

    if let Some((tid, c_type, array_length)) = typ {
        Ok(Field {
            name: field_name.into(),
            typ: tid,
            c_type,
            private,
            bits,
            array_length,
            doc,
        })
    } else {
        Err(parser.fail("Missing <type> element"))
    }
}

    // fn read_named_callback(
    //     &mut self,
    //     parser: &mut XmlParser<'_>,
    //     ns_id: u16,
    //     elem: &Element,
    // ) -> Result<(), String> {
    //     self.read_function_if_not_moved(parser, ns_id, elem.name(), elem)?
    //         .map(|func| {
    //             let name = func.name.clone();
    //             self.add_type(ns_id, &name, Type::Function(func))
    //         });
    //
    //     Ok(())
    // }

    fn read_interface(
        parser: &mut XmlParser<'_>,
        ns_id: u16,
        elem: &Element,
    ) -> Result<Interface, String> {
        let interface_name = elem.attr_required("name")?;
        let c_type = self.read_object_c_type(parser, elem)?;
        let symbol_prefix = elem.attr_required("symbol-prefix").map(ToOwned::to_owned)?;
        let type_struct = elem.attr("type-struct").map(ToOwned::to_owned);
        let get_type = elem.attr_required("get-type")?;
        let version = self.read_version(parser, ns_id, elem)?;
        let deprecated_version = self.read_deprecated_version(parser, ns_id, elem)?;

        let mut interface = Interface::new(
            interface_name,
            c_type,
            symbol_prefix,
            type_struct,

            version,
            deprecated_version
        );

        parser.elements(|parser, elem| match elem.name() {
            "constructor" => {
                let constr = read_function(parser, elem);
                interface.constructor.push(contr)
            }
            "function" => {
                let fun = read_function(parser, elem);
                interface.constructor.push(fun)
            }
            "method" => {
                let method = read_function(parser, elem);
                interface.constructor.push(method)
            }
            "virtual-method" => {
                let virt = read_virt(parser, elem);
                interface.virt.push(virt);
            }
            "prerequisite" => self.read_type(parser, ns_id, elem).map(|r| {
                prereqs.push(r.0);
            }),
            "signal" => self
                .read_signal(parser, ns_id, elem)
                .map(|s| signals.push(s)),
            "property" => self.read_property(parser, ns_id, elem).map(|p| {
                if let Some(p) = p {
                    properties.push(p);
                }
            }),
            "doc" => {
                interface.doc = parser.text().ok();
            }
            "doc-deprecated" => {
                interface.doc = parser.text().ok();
            }
            "source-position" => {
                parser.ignore_element();
            }
            "attribute" => parser.ignore_element(),
            _ => Err(parser.unexpected_element(elem)),
        })?;

        // let typ = Type::Interface(Interface {
        //     name: interface_name.into(),
        //     c_type: c_type.into(),
        //     type_struct,
        //     c_class_type: None, // this will be resolved during postprocessing
        //     glib_get_type: get_type.into(),
        //     functions: fns,
        //     signals,
        //     properties,
        //     prerequisites: prereqs,
        //     doc,
        //     doc_deprecated,
        //     version,
        //     deprecated_version,
        //     symbol_prefix,
        // });
        // self.add_type(ns_id, interface_name, typ);
        Ok(interface)
    }

    fn read_bitfield(
        parser: &mut XmlParser<'_>,
        ns_id: u16,
        elem: &Element,
    ) -> Result<(), String> {
        let bitfield_name = elem.attr_required("name")?;
        let c_type = self.read_object_c_type(parser, elem)?;
        let symbol_prefix = elem.attr("symbol-prefix").map(ToOwned::to_owned);
        let get_type = elem.attr("get-type").map(|s| s.into());
        let version = self.read_version(parser, ns_id, elem)?;
        let deprecated_version = self.read_deprecated_version(parser, ns_id, elem)?;

        // let mut members = Vec::new();
        // let mut fns = Vec::new();
        // let mut doc = None;
        // let mut doc_deprecated = None;
        let bitfield = Bitfield::new();

        parser.elements(|parser, elem| match elem.name() {
            "member" => self
                .read_member(parser, ns_id, elem)
                .map(|m| members.push(m)),
                "constructor" | "function" | "method" => {
                    self.read_function_to_vec(parser, ns_id, elem, &mut fns)
                }
            "doc" => bitfield.doc = parser.text().ok(),
            "doc-deprecated" => bitfield.doc_deprecated = parser.text().ok(),
            "source-position" => parser.ignore_element(),
            "attribute" => parser.ignore_element(),
            _ => Err(parser.unexpected_element(elem)),
        })?;

        // let typ = Type::Bitfield(Bitfield {
        //     name: bitfield_name.into(),
        //     c_type: c_type.into(),
        //     members,
        //     functions: fns,
        //     version,
        //     deprecated_version,
        //     doc,
        //     doc_deprecated,
        //     glib_get_type: get_type,
        //     symbol_prefix,
        // });
        // self.add_type(ns_id, bitfield_name, typ);
        Ok(bitfield)
    }

    fn read_enumeration(
        parser: &mut XmlParser<'_>,
        ns_id: u16,
        elem: &Element,
    ) -> Result<(), String> {
        let enum_name = elem.attr_required("name")?;
        let c_type = self.read_object_c_type(parser, elem)?;
        let symbol_prefix = elem.attr("symbol-prefix").map(ToOwned::to_owned);
        let get_type = elem.attr("get-type").map(|s| s.into());
        let version = self.read_version(parser, ns_id, elem)?;
        let deprecated_version = self.read_deprecated_version(parser, ns_id, elem)?;
        let error_domain = elem
            .attr("error-domain")
            .map(|s| ErrorDomain::Quark(String::from(s)));

        // let mut members = Vec::new();
        // let mut fns = Vec::new();
        // let mut doc = None;
        // let mut doc_deprecated = None;
        let enu = Enumeration::new(
            enum_name,
            c_type,
            symbol_prefix,

            version,
            deprecated_version,
            );

        parser.elements(|parser, elem| match elem.name() {
            "member" => self
                .read_member(parser, ns_id, elem)
                .map(|m| members.push(m)),
                "constructor" | "function" | "method" => {
                    self.read_function_to_vec(parser, ns_id, elem, &mut fns)
                }
            "doc" => enu.doc = parser.text().ok(),
            "doc-deprecated" => enu.doc_deprecated = parser.text().ok(),
            "source-position" => parser.ignore_element(),
            "attribute" => parser.ignore_element(),
            _ => Err(parser.unexpected_element(elem)),
        })?;

        Ok(enu)
    }

    fn read_constant(
        parser: &mut XmlParser<'_>,
        ns_id: u16,
        elem: &Element,
    ) -> Result<Constant, String> {
        let const_name = elem.attr_required("name")?;
        let c_identifier = elem.attr_required("type")?;
        let value = elem.attr_required("value")?;
        let version = self.read_version(parser, ns_id, elem)?;
        let deprecated_version = self.read_deprecated_version(parser, ns_id, elem)?;

        let mut constant = Constant::new(
            const_name,
            c_identifier,
            
            value,

            version,
            deprecated_version,
            );

        parser.elements(|parser, elem| match elem.name() {
            "type" | "array" => {
                if inner.is_some() {
                    return Err(parser.fail_with_position(
                            "Too many <type> inner elements in <constant> element",
                            elem.position(),
                    ));
                }
                let (typ, c_type, array_length) = self.read_type(parser, ns_id, elem)?;
                if let Some(c_type) = c_type {
                    inner = Some((typ, c_type, array_length));
                } else {
                    return Err(parser.fail_with_position(
                            "Missing <constant> element's c:type",
                            elem.position(),
                    ));
                }
                Ok(())
            }
            "doc" => contant.doc = parser.text().ok(),
            "doc-deprecated" => contant.doc_deprecated = parser.text().ok(),
            "source-position" => parser.ignore_element(),
            "attribute" => parser.ignore_element(),
            _ => Err(parser.unexpected_element(elem)),
        })?;
        Ok(constant)
    }

    fn read_alias(
        parser: &mut XmlParser<'_>,
        ns_id: u16,
        elem: &Element,
    ) -> Result<(), String> {
        let alias_name = elem.attr_required("name")?;
        let c_identifier = elem.attr_required("type")?;

        // let mut inner = None;
        // let mut doc = None;
        // let mut doc_deprecated = None;
        let mut alias = Alias::new(
            alias_name,
            c_identifer,
            );

        parser.elements(|parser, elem| match elem.name() {
            "source-position" => parser.ignore_element(),
            "type" | "array" => {
                if inner.is_some() {
                    return Err(parser.fail_with_position(
                            "Too many <type> inner elements in <alias> element",
                            elem.position(),
                    ));
                }
                let (typ, c_type, array_length) = self.read_type(parser, ns_id, elem)?;
                if let Some(c_type) = c_type {
                    inner = Some((typ, c_type, array_length));
                } else {
                    return Err(parser.fail("Missing <alias> target's c:type"));
                }
                Ok(())
            }
            "doc" => alias.doc = parser.text().ok(),
            "doc-deprecated" => alias.doc_deprecated = parser.text().map(|t| doc_deprecated = Some(t)),
            "attribute" => parser.ignore_element(),
            _ => Err(parser.unexpected_element(elem)),
        })?;
        Ok(alias)
    }

    fn read_member(
        parser: &mut XmlParser<'_>,
        ns_id: u16,
        elem: &Element,
    ) -> Result<Member, String> {
        let member_name = elem.attr_required("name")?;
        let value = elem.attr_required("value")?;
        let c_identifier = elem.attr("identifier").map(|x| x.into());
        let version = self.read_version(parser, ns_id, elem)?;
        let deprecated_version = self.read_deprecated_version(parser, ns_id, elem)?;

        let mut doc = None;
        let mut doc_deprecated = None;

        let mut member = Member::new(
            member_name,
            c_identifier,

            value,

            version,
            deprecated_version,
            );

        parser.elements(|parser, elem| match elem.name() {
            "doc" => member.doc = parser.text().ok(),
            "doc-deprecated" => member.doc_deprecated = parser.text().ok(),
            "attribute" => parser.ignore_element(),
            _ => Err(parser.unexpected_element(elem)),
        })?;

        Ok(member)
    }

    fn read_function(
        parser: &mut XmlParser<'_>,
        elem: &Element,
    ) -> Result<Function, String> {
        let fn_name = elem.attr_required("name")?;
        let c_identifier = elem.attr("identifier").or_else(|| elem.attr("type"));
        // let kind = FunctionKind::from_str(kind_str).map_err(|why| parser.fail(&why))?;
        // let is_method = kind == FunctionKind::Method;
        let version = self.read_version(parser, elem)?;
        let deprecated_version = self.read_deprecated_version(parser, elem)?;

        let mut params = Vec::new();
        let mut ret = None;
        let mut doc = None;
        let mut doc_deprecated = None;

        parser.elements(|parser, elem| match elem.name() {
            "parameters" => { 
                read_parameters(parser, false, is_method)
                .map(|mut ps| params.append(&mut ps))
            }
            "return-value" => {
                if ret.is_some() {
                    return Err(parser.fail_with_position(
                            "Too many <return-value> elements inside <function> element",
                            elem.position(),
                    ));
                }

                ret = Some(self.read_parameter(parser, ns_id, elem, false, is_method)?);
                Ok(())
            }
            "doc" => doc = parser.text().ok(),
            "doc-deprecated" => doc_deprecated = parser.text().ok(),
            "doc-version" => parser.ignore_element(),
            "source-position" => parser.ignore_element(),
            "attribute" => parser.ignore_element(),
            _ => Err(parser.unexpected_element(elem)),
        })?;

        let func = Function::new(
            fn_name,
            c_indentifier,

            params,
            ret,
            version,
            deprecated_version,
            doc,
            doc_deprecated
        );
        Ok(func)
    }

    fn read_signal(
        parser: &mut XmlParser<'_>,
        elem: &Element,
    ) -> Result<Signal, String> {
        let signal_name = elem.attr_required("name")?;
        let is_action = elem.attr_bool("action", false);
        let is_detailed = elem.attr_bool("detailed", false);
        let version = self.read_version(parser, elem)?;
        let deprecated_version = self.read_deprecated_version(parser, elem)?;

        let mut params = Vec::new();
        let mut ret = None;
        let mut doc = None;
        let mut doc_deprecated = None;

        parser.elements(|parser, elem| match elem.name() {
            "parameters" => self
                .read_parameters(parser, ns_id, true, false)
                .map(|mut ps| params.append(&mut ps)),
            "return-value" => {
                if ret.is_some() {
                    return Err(parser.fail_with_position(
                            "Too many <return-value> elements in <signal> element",
                            elem.position(),
                    ));
                }
                self.read_parameter(parser, ns_id, elem, true, false)
                    .map(|p| ret = Some(p))
            }
            "doc" => parser.text().map(|t| doc = Some(t)),
            "doc-deprecated" => parser.text().map(|t| doc_deprecated = Some(t)),
            "attribute" => parser.ignore_element(),
            _ => Err(parser.unexpected_element(elem)),
        })?;
        if let Some(ret) = ret {
            Ok(Signal {
                name: signal_name.into(),
                parameters: params,
                ret,
                is_action,
                is_detailed,
                version,
                deprecated_version,
                doc,
                doc_deprecated,
            })
        } else {
            Err(parser.fail_with_position(
                    "Missing <return-value> element in <signal> element",
                    elem.position(),
            ))
        }
    }

    fn read_parameters(
        parser: &mut XmlParser<'_>,
        allow_no_ctype: bool,
        for_method: bool,
    ) -> Result<Vec<Parameter>, String> {
        parser.elements(|parser, elem| match elem.name() {
            "parameter" | "instance-parameter" => {
                read_parameter(parser, elem, allow_no_ctype, for_method)
            }
            _ => Err(parser.unexpected_element(elem)),
        })
    }

    fn read_parameter(
        parser: &mut XmlParser<'_>,
        elem: &Element,
        allow_no_ctype: bool,
        for_method: bool,
    ) -> Result<Parameter, String> {
        let param_name = elem.attr("name").unwrap_or("");
        let instance_parameter = elem.name() == "instance-parameter";
        let transfer = elem
            .attr_from_str("transfer-ownership")?
            .unwrap_or(Transfer::None);
        let nullable = elem.attr_bool("nullable", false);
        let allow_none = elem.attr_bool("allow-none", false);
        let scope = elem.attr_from_str("scope")?.unwrap_or(ParameterScope::None);
        let closure = elem.attr_from_str("closure")?;
        let destroy = elem.attr_from_str("destroy")?;
        let caller_allocates = elem.attr_bool("caller-allocates", false);
        let direction = if elem.name() == "return-value" {
            Ok(ParameterDirection::Return)
        } else {
            ParameterDirection::from_str(elem.attr("direction").unwrap_or("in"))
                .map_err(|why| parser.fail_with_position(&why, elem.position()))
        }?;

        let mut typ = None;
        let mut varargs = false;
        let mut doc = None;

        parser.elements(|parser, elem| match elem.name() {
            "type" | "array" => {
                if typ.is_some() {
                    return Err(parser.fail_with_position(
                            &format!("Too many <type> elements in <{}> element", elem.name()),
                            elem.position(),
                    ));
                }
                typ = Some(self.read_type(parser, ns_id, elem)?);
                if let Some((tid, None, _)) = typ {
                    if allow_no_ctype {
                        typ = Some((tid, Some(EMPTY_CTYPE.to_owned()), None));
                    } else {
                        return Err(parser.fail_with_position(
                                &format!("Missing c:type attribute in <{}> element", elem.name()),
                                elem.position(),
                        ));
                    }
                }
                Ok(())
            }
            "varargs" => {
                varargs = true;
                parser.ignore_element()
            }
            "doc" => parser.text().map(|t| doc = Some(t)),
            "attribute" => parser.ignore_element(),
            _ => Err(parser.unexpected_element(elem)),
        })?;

        if let Some((tid, c_type, mut array_length)) = typ {
            if for_method {
                array_length = array_length.map(|l| l + 1);
            }
            Ok(Parameter {
                name: param_name.into(),
                typ: tid,
                c_type: c_type.unwrap(),
                instance_parameter,
                direction,
                transfer,
                caller_allocates,
                nullable: Nullable(nullable),
                allow_none,
                array_length,
                is_error: false,
                doc,
                scope,
                closure,
                destroy,
            })
        } else if varargs {
            Ok(Parameter {
                name: "".into(),
                typ: self.find_type(INTERNAL_NAMESPACE, "varargs").unwrap(),
                c_type: "".into(),
                instance_parameter,
                direction: Default::default(),
                transfer: Transfer::None,
                caller_allocates: false,
                nullable: Nullable(false),
                allow_none,
                array_length: None,
                is_error: false,
                doc,
                scope,
                closure,
                destroy,
            })
        } else {
            Err(parser.fail_with_position(
                    &format!("Missing <type> element in <{}> element", elem.name()),
                    elem.position(),
            ))
        }
    }

    fn read_property(
        parser: &mut XmlParser<'_>,
        elem: &Element,
    ) -> Result<Option<Property>, String> {

        let prop_name = elem.attr_required("name")?;
        let readable = elem.attr_bool("readable", true);
        let writable = elem.attr_bool("writable", false);
        let construct = elem.attr_bool("construct", false);
        let construct_only = elem.attr_bool("construct-only", false);
        let transfer = Transfer::from_str(elem.attr("transfer-ownership").unwrap_or("none"))
            .map_err(|why| parser.fail_with_position(&why, elem.position()))?;

        let version = self.read_version(parser, ns_id, elem)?;
        let deprecated_version = self.read_deprecated_version(parser, ns_id, elem)?;
        let mut has_empty_type_tag = false;
        let mut typ = None;
        let mut doc = None;
        let mut doc_deprecated = None;

        parser.elements(|parser, elem| match elem.name() {
            "type" | "array" => {
                if typ.is_some() {
                    return Err(parser.fail_with_position(
                            "Too many <type> elements in <property> element",
                            elem.position(),
                    ));
                }
                if !elem.has_attrs() && elem.name() == "type" {
                    // defend from <type/>
                    has_empty_type_tag = true;
                    return parser.ignore_element();
                }
                typ = Some(self.read_type(parser, ns_id, elem)?);
                if let Some((tid, None, _)) = typ {
                    typ = Some((tid, Some(EMPTY_CTYPE.to_owned()), None));
                }
                Ok(())
            }
            "doc" => parser.text().map(|t| doc = Some(t)),
            "doc-deprecated" => parser.text().map(|t| doc_deprecated = Some(t)),
            "attribute" => parser.ignore_element(),
            _ => Err(parser.unexpected_element(elem)),
        })?;

        if has_empty_type_tag {
            return Ok(None);
        }

        if let Some((tid, c_type, _array_length)) = typ {
            Ok(Some(Property {
                name: prop_name.into(),
                readable,
                writable,
                construct,
                construct_only,
                transfer,
                typ: tid,
                c_type,
                version,
                deprecated_version,
                doc,
                doc_deprecated,
            }))
        } else {
            Err(parser.fail_with_position(
                    "Missing <type> element in <property> element",
                    elem.position(),
            ))
        }
    }

    fn read_type(
        parser: &mut XmlParser<'_>,
        elem: &Element,
    ) -> Result<(TypeId, Option<String>, Option<u32>), String> {
        let type_name = elem
            .attr("name")
            .or_else(|| {
                if elem.name() == "array" {
                    Some("array")
                } else {
                    None
                }
            })
        .ok_or_else(|| {
            parser.fail_with_position(
                "<type> element is missing a name attribute",
                elem.position(),
            )
        })?;
        let c_type = elem.attr("type").map(|s| s.into());
        let array_length = elem.attr("length").and_then(|s| s.parse().ok());

        let inner = parser.elements(|parser, elem| match elem.name() {
            "type" | "array" => self.read_type(parser, ns_id, elem),
            _ => Err(parser.unexpected_element(elem)),
        })?;

        if inner.is_empty() || type_name == "GLib.ByteArray" {
            if type_name == "array" {
                Err(parser.fail_with_position(
                        "<type> element is missing an inner element type",
                        elem.position(),
                ))
            } else if type_name == "gboolean" && c_type.as_deref() == Some("_Bool") {
                Ok((self.find_or_stub_type(ns_id, "bool"), c_type, array_length))
            } else {
                Ok((
                        self.find_or_stub_type(ns_id, type_name),
                        c_type,
                        array_length,
                ))
            }
        } else {
            let tid = if type_name == "array" {
                let inner_type = &inner[0];
                Type::c_array(
                    self,
                    inner_type.0,
                    elem.attr("fixed-size").and_then(|n| n.parse().ok()),
                    inner_type.1.clone(),
                )
            } else {
                let inner = inner.iter().map(|r| r.0).collect();
                Type::container(self, type_name, inner).ok_or_else(|| {
                    parser.fail_with_position("Unknown container type", elem.position())
                })?
            };
            Ok((tid, c_type, array_length))
        }
    }

    fn read_version(
        parser: &XmlParser<'_>,
        elem: &Element,
    ) -> Result<Option<Version>, String> {
        read_version_attribute(parser, elem, "version")
    }

    fn read_deprecated_version(
        parser: &XmlParser<'_>,
        elem: &Element,
    ) -> Result<Option<Version>, String> {
        read_version_attribute(parser, elem, "deprecated-version")
    }

    fn read_version_attribute(
        parser: &XmlParser<'_>,
        elem: &Element,
        attr: &str,
    ) -> Result<Option<Version>, String> {
        if let Some(v) = elem.attr(attr) {
            match v.parse() {
                Ok(v) => {
                    self.register_version(ns_id, v);
                    Ok(Some(v))
                }
                Err(e) => Err(parser.fail(&format!("Invalid `{}` attribute: {}", attr, e))),
            }
        } else {
            Ok(None)
        }
    }

    fn read_object_c_type<'a>(
        parser: &mut XmlParser<'_>,
        elem: &'a Element,
    ) -> Result<&'a str, String> {
        elem.attr("type")
            .or_else(|| elem.attr("type-name"))
            .ok_or_else(|| {
                parser.fail(&format!(
                        "Missing `c:type`/`glib:type-name` attributes on element <{}>",
                        elem.name()
                ))
            })
    }

fn make_file_name(dir: &Path, name: &str) -> PathBuf {
    let mut path = dir.to_path_buf();
    let name = format!("{}.gir", name);
    path.push(name);
    path
}
