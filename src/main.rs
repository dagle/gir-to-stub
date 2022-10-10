use std::{fs::File, str::FromStr};
use std::env;
use std::path::PathBuf;

mod library;
mod parse;
mod lang;

use clap::Parser;

#[derive(Clone)]
enum Lang {
    Python,
    Lua,
}

impl FromStr for Lang {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(required = true)]
    #[clap(short, long, value_name = "lua|python")]
    lang: Lang,

    #[clap(short, long, value_name = "Code|CodeDoc|Full")]
    // #[clap(default_value_t = String::from("Code"))]
    #[clap(default_value_t = String::from("Code"))]
    level: String,

    #[clap(required = true)]
    filename: PathBuf,
}

fn main() {
    let args = Cli::parse();
    match args.lang {
        Lang::Python => {},
        Lang::Lua => {
            let cg = lang::lua::LuaCodegen::new(lang::lua::Level::Code);
        },
    }
}
