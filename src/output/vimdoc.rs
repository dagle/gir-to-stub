use chrono::{DateTime, Utc};
use std::io::prelude::*;

// do not do like this
// this is just for now 
// in the future we should be be able
// generate the stub and vimdoc from 
// the same document

type Result<T> = std::io::Result<T>;

pub struct VimDoc {
    // level: Level
}

pub fn write_enum<W: Write>(e: &Comp, w: &mut W) -> Result<()> {
    writeln!(w, "{}", e.name)
}

fn create_section<W: Write>(ns: &str, name: &str, w: &mut W) -> Result<()> {
    writeln!(w, "{:=>76}", "=")?;
    let str = format!("*{}.{}*", ns, name); 
    writeln!(w, "{}.{} {:>pad$}", ns, name, str, pad=79-str.len())?;
    Ok(())
}

#[macro_export]
macro_rules! section {
    ( $( $w:ident, $name:ident, $section:ident ),* ) => {
        {
            if !self.$x.is_empty() {
                create_section(&$name, "$x", $w)?;
                for section in self.$x.iter() {
                    section.gen($name, $w)?;
                }
            }
        };
    }
}

impl VimDoc {
    pub fn new(/* level: Level */) {
        VimDoc{}
    }
    fn gen(&self, filename: &str) {
        let now: DateTime<Utc> = Utc::now();
        let datestr = format!("Last Generated: {}", now.format("%d/%m/%Y %H:%M"));
        writeln!(w, "*{}.txt* {:>pad$}", self.ns, datestr, pad=76-5-self.ns.len())?;

        section!(w, name, classes);
        if !self.classes.is_empty() {
            create_section(&name, "classes", w)?;
            for class in self.classes.iter() {
                writeln!(w, "{}.{} = {{}}", &name, class.name)?;
                class.gen(&name, w)?;
            }
        }
        if !self.functions.is_empty() {
            create_section(&name, "Function", w)?;
            for function in self.functions.iter() {
                function.gen(&name, &name, w)?;
            }
        }
        if !self.enums.is_empty() {
            create_section(&name, "Enums", w)?;
            for enu in self.enums.iter() {
                enu.gen(&name, w)?;
            }
        }
        if !self.record.is_empty() {
            create_section(&name, "Record", w)?;
            for record in self.record.iter() {
                record.gen(&name, w)?;
            }
        }
        if !self.constant.is_empty() {
            create_section(&name, "Constant", w)?;
            for cons in self.constant.iter() {
                cons.gen(&name, w)?;
            }
        }
        if !self.bitfield.is_empty() {
            create_section(&name, "Bitfield", w)?;
            for bitfield in self.bitfield.iter() {
                bitfield.gen(&name, w)?;
            }
        }
        if !self.alias.is_empty() {
            create_section(&name, "Alias", w)?;
            for alias in self.bitfield.iter() {
                alias.gen(&name, w)?;
            }
        }
        if !self.unions.is_empty() {
            create_section(&name, "Union", w)?;
            for unions in self.bitfield.iter() {
                unions.gen(&name, w)?;
            }
        }

        writeln!(w, "")?;
        write!(w, "vim:tw=78:ts=8:noet:ft=help:norl:")?;
        w.flush()?;
        Ok(())
    }
}


// namespace
pub fn write<W: Write>(&self, w: &mut W) -> Result<()> {
}

// class
pub fn write<W: Write>(&self, ns: &str, w: &mut W) -> Result<()>{
    let static_ns = format!("{}.{}", ns, self.name);

    create_section(ns, &self.name, w)?;
    let local_ns = self.name.to_lowercase();

    if let Some(ref doc) = self.doc {
        writeln!(w, "{}\n", doc)?;
    }

    if !self.fields.is_empty() {
        writeln!(w, "Fields~")?;
        for field in self.fields.iter() {
            let str = format!("{}.{}", static_ns, field.0);
            w.write_all(&str.as_bytes())?;
            writeln!(w, "")?;
        }
    }
    if !self.constructor.is_empty() {
        writeln!(w, "Constructors~")?;
        for constr in self.constructor.iter() {
            constr.write(&static_ns, w)?;
            writeln!(w, "")?;
        }
    }
    if !self.method.is_empty() {
        writeln!(w, "Methods~")?;
        for method in self.method.iter() {
            method.write(&local_ns, w)?;
            writeln!(w, "")?;
        }
    }
    if !self.func.is_empty() {
        writeln!(w, "Functions~")?;
        for func in self.func.iter() {
            func.write(&static_ns, w)?;
            writeln!(w, "")?;
        }
    }
    if !self.virt.is_empty() {
        writeln!(w, "Virtual~")?;
        for virt in self.virt.iter() {
            virt.write(&local_ns, w)?;
            writeln!(w, "")?;
        }
    }
    Ok(())
}

fn get_typeless(args: &Vec<(String, Type)>) -> Vec<String> {
    args.into_iter().map(|x| format!("{{{}}}", x.0)).collect()
}

fn write_doc<W: Write>(doc: &Option<String>, w: &mut W) -> Result<()> {
    if let Some(ref doc) = doc {
        let lines = doc.lines();
        for line in lines.into_iter() {
            writeln!(w, "\t{}", line)?;
        }
        writeln!(w, "")?;
    }
    Ok(())
}

pub fn write<W: Write>(&self, ns: &str, w: &mut W) -> Result<()> {
    let typeless = get_typeless(&self.args);
    let fmt = format!("*{}.{}()*", ns, self.name);
    writeln!(w, "{:>78}", fmt)?;
    writeln!(w, "{}.{}({})\n", ns, self.name, typeless.join(", "))?;

    write_doc(&self.doc, w)?;
    writeln!(w, "\tArguments:~")?;
    for arg in self.args.iter() {
        // writeln!(w, "`{}`: `{}` {}", arg.0, arg.1, arg.2)?;
        writeln!(w, "\t\t{{{}}} `{}`", arg.0, arg.1)?;
    }
    writeln!(w, "\tReturns:~\n\t\t`{}`", self.ret.join(", "))?;
    Ok(())
}
