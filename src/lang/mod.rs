use std::{str::FromStr, path::Path, fs::{self, File}, io::BufReader};
use anyhow::Result;

pub mod lua;
// pub mod python;

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

pub trait Generator {
    fn genfile(&self, filename: &str, output_dir: Option<&str>) -> Result<()>;
    fn generate(&self, filename: Option<&str>, output_dir: Option<&str>) -> Result<()> {
        if let Some(filename) = filename {
            self.genfile(filename, output_dir)
        } else {
            let paths = fs::read_dir("/usr/share/gir-1.0/")?;
            for path in paths {
                let osstr = path.expect("Couldn't read filename").file_name();
                let filename = osstr.to_str().expect("Couldn't read filename");
                println!("Generating file {}", filename);
                self.genfile(filename, output_dir)?;
            }
            Ok(())
        }
    }
}

// pub trait Gen {
// }

// fn is_dir(dir: &str) -> bool {
//     Path::new(dir).is_dir()
// }

/// Try to open the file at path, if it fails,
/// it will try to search for the path in gir directory
fn open_gir<P: AsRef<Path>>(filename: P) -> Result<BufReader<File>> {
    match fs::File::open(&filename) {
        Ok(f) => Ok(BufReader::new(f)),
        Err(_) => {
            let path = Path::new("/usr/share/gir-1.0/").join(filename);
            Ok(BufReader::new(fs::File::open(path)?))
        }
    }
}
