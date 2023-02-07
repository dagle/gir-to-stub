use std::str::FromStr;
use std::fs;

mod library;
mod parse;
mod lang;

use anyhow::Result;

use clap::Parser;

#[derive(Clone)]
enum Lang {
    Python,
    Lua,
}

impl FromStr for Lang {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "lua" => Ok(Lang::Lua),
            "python" => Ok(Lang::Python),
            lang => {
                let ret = format!("{} not supported", lang);
                Err(ret)
            }
        }
    }
}


#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(required = true)]
    #[clap(short, long, value_name = "lua|python")]
    lang: Lang,

    // #[clap(long, value_name = "Code|CodeDoc|Full")]
    #[clap(long, value_name = "Code")]
    #[clap(default_value_t = lang::Level::Code)]
    level: lang::Level,

    // generates all files found in "/usr/share/gir-1.0/" but overrides if file is 
    // found locally.
    #[clap(long)]
    #[clap(default_value_t = false)]
    gen_all: bool,

    #[clap(short, long)]
    output: Option<String>,

    // #[clap(required = true)]
    filename: Option<String>,

}

fn get_lang(lang: Lang) -> Box<dyn lang::Generator> {
    match lang {
        Lang::Python => {
            Box::new(lang::python::PythonCodeGen::new())
        }
        Lang::Lua => {
            Box::new(lang::lua::LuaCodegen::new())
        },
    }
}

// a bit much copy-pasty
fn main() -> Result<()>{
    let args = Cli::parse();
    let cg = get_lang(args.lang);
    if args.gen_all {
        cg.generate(None, args.output.as_deref())?;
    } else {
        let filename = args.filename.expect("Missing filename");
        cg.generate(Some(&filename), args.output.as_deref())?;
    }
    Ok(())
}
