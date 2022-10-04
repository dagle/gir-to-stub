use std::fs::File;
use std::env;
use std::path::PathBuf;

fn main() {
    for arg in env::args().skip(1) {
        let infile = File::open(&arg).expect("Can't read file");

        let mut out_file = PathBuf::from(&arg);
        out_file.set_extension("txt");
        let mut outfile = File::create(out_file).expect("Couldn't open output file");

        let doc = parse::parse_gir(infile).expect("Couldn't parse gir");
        doc.write(&mut outfile).expect("Couldn't write document to output file");
    }
}
