use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;

extern crate getopts;
use getopts::Options;
use getopts::ParsingStyle;

mod parser;
use parser::Parser;
use parser::CommandType;

mod code_writer;
use code_writer::CodeWriter;

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
        in_file_name.replace(".vm", ".asm")
    };

    let mut in_file = File::open(&in_file_name).expect("Unable to find file.");
    let mut buffer = String::new();
    in_file.read_to_string(&mut buffer).expect("Unable to read file.");
    let buffer = &buffer;

    let out_file = File::create(out_file_name).expect("Unable to create file.");
    let code_writer = CodeWriter::new(out_file);

    translate(buffer, code_writer, &in_file_name);
}

fn translate<'a>(buffer:&'a str,mut code_writer: CodeWriter<'a>, in_file_name: &'a str) -> () {
    let mut parser = Parser::new(buffer);
    let path = Path::new(in_file_name);
    let file_name = path.file_stem().unwrap().to_str().unwrap();
    code_writer.set_file_name(file_name);

    while parser.has_more_commands() {
        parser.advance();

        match parser.command_type() {
            CommandType::CPush |
            CommandType::CPop => {
                code_writer.write_push_pop(parser.command_type(), parser.arg1(), parser.arg2());
            }
            CommandType::CArithmetic => {
                code_writer.write_arithmetic(parser.arg1())
            }
        }
    }

    code_writer.close();
}
