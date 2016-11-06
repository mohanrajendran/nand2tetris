use std::env;
use std::fs::File;
use std::io::Read;
use std::io::Write;

pub mod parser;
pub mod code;
use parser::Parser;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: ./assembler FILENAME");
        return;
    }

    let mut in_file = File::open(&args[1]).expect("Unable to find file.");
    let mut buffer = String::new();
    in_file.read_to_string(&mut buffer).expect("Unable to read file.");
    let buffer = buffer;

    let mut parser = Parser::new(&buffer);
    //parser.print();

    let assembly = parser.assemble();
    let mut out_file = File::create(args[1].replace(".asm", ".mhack")).expect("Unable to create file.");
    out_file.write_all(assembly.as_bytes()).expect("Unable to write to file.");
}

