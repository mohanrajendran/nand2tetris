use std::env;
use std::fs::File;
use std::io::Read;
use std::io::Write;

extern crate getopts;
use getopts::Options;
use getopts::ParsingStyle;

pub mod parser;
pub mod code;
use parser::Parser;

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("o", "", "set output file name", "OUTFILE");
    opts.parsing_style(ParsingStyle::FloatingFrees);

    let matches = opts.parse(&args[1..]).expect("Unable to parse arguments.");

    let in_file_name = if matches.free.len() == 1 {
        matches.free[0].clone()
    } else {
        let brief = format!("Usage: {} FILE [options]", program);
        println!("{}", opts.usage(&brief));
        return;
    };

    let out_file_name = if matches.opt_present("o") {
        matches.opt_str("o").unwrap()
    } else {
        in_file_name.replace(".asm", ".hack")
    };

    let mut in_file = File::open(in_file_name).expect("Unable to find file.");
    let mut buffer = String::new();
    in_file.read_to_string(&mut buffer).expect("Unable to read file.");
    let buffer = buffer;

    let mut parser = Parser::new(&buffer);
    //parser.print();

    let assembly = parser.assemble();
    let mut out_file = File::create(out_file_name).expect("Unable to create file.");
    out_file.write_all(assembly.as_bytes()).expect("Unable to write to file.");
}

