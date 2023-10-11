use std::{str::FromStr, path::{Path, PathBuf}, fs::{self, File}, io::BufReader};
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
    // genfilepath
    fn generate(&self, filename: &str, output_dir: &str) -> Result<()>;
    // fn generate(&self, filename: Option<&str>, output_dir: Option<&str>) -> Result<()> {
    //     if let Some(filename) = filename {
    //         self.genfile(filename, output_dir)
    //     } else {
    //         let paths = fs::read_dir("/usr/share/gir-1.0/")?;
    //         for path in paths {
    //             let osstr = path.expect("Couldn't read filename").file_name();
    //             let filename = osstr.to_str().expect("Couldn't read filename");
    //             println!("Generating file {}", filename);
    //             self.genfile(filename, output_dir)?;
    //         }
    //         Ok(())
    //     }
    // }
}

fn get_gir(filename: PathBuf) -> Result<PathBuf> {
    if filename.exists() {
        Ok(filename)
    }  else {
        // Path::new("/usr/share/gir-1.0/").join(filename)
        let path = Path::new("/usr/share/gir-1.0/").join(filename);
        if !path.exists() {
            // return Err(anyhow::anyhow!(format!("No gir with name: {} found", filename.to_str())))
            return Err(anyhow::anyhow!(format!("No gir with name: found")))
        }
        Ok(path)
    }
}

/// Try to open the file at path, if it fails,
/// it will try to search for the path in gir directory
fn open_gir<P: AsRef<Path>>(filename: P) -> Result<BufReader<File>> {
    let open = fs::File::open(&filename)?;
    Ok(BufReader::new(open))
}
