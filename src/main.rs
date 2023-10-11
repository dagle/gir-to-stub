use std::str::FromStr;

use gir_to_stub::lang;

use anyhow::Result;

use clap::Parser;

#[derive(Clone)]
enum Lang {
    // Python,
    Lua,
}

impl FromStr for Lang {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "lua" => Ok(Lang::Lua),
            // "python" => Ok(Lang::Python),
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
    #[clap(required = true)]
    output: String,

    #[clap(required = true)]
    filename: String,

}

fn get_lang(lang: Lang) -> Box<dyn lang::Generator> {
    match lang {
        // Lang::Python => {
        //     Box::new(lang::python::PythonCodeGen::new())
        // }
        Lang::Lua => {
            Box::new(lang::lua::LuaCodegen::new())
        },
    }
}

// a bit much copy-pasty
fn main() -> Result<()>{
    let args = Cli::parse();
    let cg = get_lang(args.lang);
        cg.generate(&args.filename, &args.output)?;
    }
    Ok(())
}
