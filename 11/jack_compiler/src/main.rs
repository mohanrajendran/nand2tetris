use std::env;
use std::fs;
use std::path::{Path, PathBuf};

extern crate regex;
extern crate xml_writer;

mod jack_tokenizer;
mod compilation_engine;
mod symbol_table;

use compilation_engine::CompilationEngine;

fn main() {
    let mut args = env::args();
    let program = args.next().unwrap();
    let in_path = args.next().expect(&format!("Usage: {} PATH", program));
    let in_path = Path::new(&in_path).to_path_buf();

    // Collect input files and create output folder
    let input_files: Vec<PathBuf> = if in_path.is_file() {
        let mut output_path = in_path.clone();
        output_path.pop();
        output_path.push("output");
        fs::create_dir_all(&output_path).expect("Unable to create output directory");

        vec!(in_path.to_path_buf())
    } else {
        let mut output_path = in_path.clone();
        output_path.push("output");
        fs::create_dir_all(&output_path).expect("Unable to create output directory");

        let in_files = in_path.read_dir().unwrap();

        in_files
            .map(|entry| entry.unwrap().path())
            .filter(|path| path.is_file())
            .filter(|path| path.extension().unwrap() == "jack")
            .collect()
    };

    for in_file in input_files {
        let mut engine = CompilationEngine::new(in_file);
        engine.compile_class();
    }
}
