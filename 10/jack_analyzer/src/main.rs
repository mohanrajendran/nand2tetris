use std::env;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

extern crate regex;

mod jack_tokenizer;
use jack_tokenizer::JackTokenizer;

fn main() {
    let mut args = env::args();
    let program = args.next().unwrap();
    let in_path = args.next().expect(&format!("Usage: {} PATH", program));
    let in_path = Path::new(&in_path).to_path_buf();

    // Collect input files
    let input_files: Vec<PathBuf> = if in_path.is_file() {
        vec!(in_path.to_path_buf())
    } else {
        let in_files = in_path.read_dir().unwrap();

        in_files
            .map(|entry| entry.unwrap().path())
            .filter(|path| path.is_file())
            .filter(|path| path.extension().unwrap() == "jack")
            .collect()
    };

    for in_file in input_files {
        translate(in_file);
    }
}

fn translate(in_file: PathBuf) {
    let mut buffer = String::new();
    let mut file = File::open(in_file).expect("File missing");
    file.read_to_string(&mut buffer);

    let mut tokenizer = JackTokenizer::new(buffer);
    tokenizer.print_tokens();
}
