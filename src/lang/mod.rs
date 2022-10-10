use std::str::FromStr;

pub mod lua;

#[derive(Clone)]
pub enum Level {
    Code,
    CodeDoc,
    Full,
}

impl std::fmt::Display for Level {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Level::Code => { f.write_str("Code")}
            Level::CodeDoc => {f.write_str("CodeDoc")}
            Level::Full => {f.write_str("Full")}
        }
    }
}

impl FromStr for Level {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "code" | "Code" => Ok(Level::Code),
            "codedoc" | "CodeDoc" => Ok(Level::Code),
            "full" | "Full" => Ok(Level::Code),
            level => {
                let ret = format!("{} level not supported", level);
                Err(ret)
            }
        }
    }
}
