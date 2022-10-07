use chrono::{DateTime, Utc};
use std::io::prelude::*;

type Result<T> = std::io::Result<T>;

pub fn write_enum<W: Write>(e: &Comp, w: &mut W) -> Result<()> {
    writeln!(w, "{}", e.name)
}

fn create_section<W: Write>(ns: &str, name: &str, w: &mut W) -> Result<()> {
    writeln!(w, "{:=>76}", "=")?;
    let str = format!("*{}.{}*", ns, name); 
    writeln!(w, "{}.{} {:>pad$}", ns, name, str, pad=79-str.len())?;
    Ok(())
}

// namespace
pub fn write<W: Write>(&self, w: &mut W) -> Result<()> {
    let now: DateTime<Utc> = Utc::now();
    let datestr = format!("Last Generated: {}", now.format("%d/%m/%Y %H:%M"));
    writeln!(w, "*{}.txt* {:>pad$}", self.ns, datestr, pad=76-5-self.ns.len())?;

    for classes in self.global.classes.iter() {
        classes.write(&self.ns, w)?;
    }

    if ! self.global.functions.is_empty() {
        writeln!(w, "")?;

        create_section(&self.ns, "Functions", w)?;
        writeln!(w, "")?;
        for function in self.global.functions.iter() {
            function.write(&self.ns, w)?;
        }
    }

    if ! self.global.macros.is_empty() {
        writeln!(w, "")?;

        create_section(&self.ns, "Macros", w)?;
        writeln!(w, "")?;
        for macr in self.global.macros.iter() {
            macr.write(&self.ns, w)?;
        }
    }

    if ! self.global.functions.is_empty() {
        writeln!(w, "")?;

        create_section(&self.ns, "Enums", w)?;
        writeln!(w, "")?;
        for enu in self.global.enums.iter() {
            write_enum(enu, w)?;
        }
    }
    writeln!(w, "")?;
    write!(w, "vim:tw=78:ts=8:noet:ft=help:norl:")?;
    w.flush()?;
    Ok(())
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
