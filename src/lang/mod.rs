use std::{str::FromStr, path::Path, fs::{self, File}};

pub mod lua;
pub mod python;

#[derive(Clone)]
pub enum Level {
    Code,
    CodeDoc,
    Full,
}

type Result<T> = std::io::Result<T>;

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

// trait Output {
//    fn output_header() -> Result<()>; 
//    fn output_section() -> Result<()>; 
//    /// How to display like a function 
//    /// declareation
//    fn output_declaration() -> Result<()>; 
//    fn output_documentatino() -> Result<()>;
//
//    // fn output_header() -> Result<()>; 
//    // fn output_header() -> Result<()>; 
//    // fn output_header() -> Result<()>; 
// }

pub trait Generator {
    fn gen(&self, filename: &str) -> Result<()>;
}

fn is_dir(dir: &str) -> bool {
    Path::new(dir).is_dir()
}

fn open_gir(filename: &str) -> Result<File> {
    match fs::File::open(filename) {
        Ok(f) => Ok(f),
        Err(_) => {
            // If we don't find the gir locally, we use the global file
            let path = Path::new("/usr/share/gir-1.0/").join(filename);
            fs::File::open(path)
        }
    }
}
