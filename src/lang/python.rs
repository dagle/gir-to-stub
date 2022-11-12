use std::{fs::File, io::Write};
use std::fs;
use crate::{library::*, parse};
use std::path::Path;
use super::*;

pub struct PythonCodeGen {
    level: Level
}

type Result<T> = std::io::Result<T>;

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

impl PythonCodeGen {
    pub fn new(level: Level) -> PythonCodeGen {
        PythonCodeGen{
            level,
        }
    }
}

impl Generator for PythonCodeGen {
    fn gen(&self, filename: &str) -> Result<()> {
        let types = "gi-stubs";
        if !super::is_dir(types) {
            fs::create_dir(types)?;
        }
        let mut path = Path::new(types).join(filename);
        path.set_extension("py");
        let mut out_file = fs::File::create(path)?;
        let in_file = open_gir(filename)?;
        let repo = parse::parse_gir(in_file).expect("Couldn't parse gir file");
        repo.namespace[0].gen(&mut out_file)?;

        Ok(())
    }
}

// TODO Typehinting
// TODO docstrings inside a class

fn create_section<W: Write>(ns: &str, str: &str, w: &mut W) -> Result<()> {
    writeln!(w, "# {} {}\n", ns, str)
}
