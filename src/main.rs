use std::str::FromStr;

use gir_to_stub::lang;

use anyhow::Result;

use clap::Parser;

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
    // #[clap(short, long, value_name = "lua|python")]
    #[clap(short, long, value_name = "lua")]
    lang: Lang,

    /// directory to search for filename
    #[clap(short, long, default_value = "/usr/share/gir-1.0/")]
    dir: String,

    /// gir filename, will search /usr/share/gir-1.0/
    #[clap(required = true)]
    gir_file: String,

    /// directory to generate the bindings to
    #[clap(required = true)]
    output: String,
}

impl Cli {
    fn gen(&self) -> Result<()> {
        match self.lang {
            Lang::Lua => {
                let cg = lang::lua::LuaCodegen::new(&self.gir_file, &self.dir, &self.output)?;
                cg.generate()
            }
        }
    }
}

#[derive(Clone)]
enum Lang {
    // Python,
    Lua,
}

// a bit much copy-pasty
fn main() -> Result<()>{
    let args = Cli::parse();
    args.gen()?;
    Ok(())
}
