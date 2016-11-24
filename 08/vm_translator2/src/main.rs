use std::env;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

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

    let in_path = if matches.free.len() == 1 {
        Path::new(&matches.free[0])
    } else {
        let brief = format!("Usage: {} FILE [options]", program);
        println!("{}", opts.usage(&brief));
        return;
    };

    // Set output file
    let out_file = if matches.opt_present("o") {
        matches.opt_str("o").unwrap()
    } else if in_path.is_file() {
        let mut in_path_buf = in_path.to_path_buf();
        in_path_buf.set_extension("asm");
        in_path_buf.to_str().unwrap().to_string()
    } else {
        let dir_name = in_path.file_stem().unwrap();
        let mut in_path_buf = in_path.to_path_buf();
        in_path_buf.push(dir_name);
        in_path_buf.set_extension("asm");
        in_path_buf.to_str().unwrap().to_string()
    };

    // Collect input files
    let input_files = if in_path.is_file() {
        vec!(in_path.to_path_buf())
    } else {
        let in_files = in_path.read_dir().unwrap();

        let in_files:Vec<_> = in_files
            .map(|entry| entry.unwrap().path())
            .filter(|path| path.is_file())
            .filter(|path| path.extension().unwrap() == "vm")
            .collect();

        in_files
    };

    let out_file = File::create(out_file).expect("Unable to create file.");
    let mut code_writer = CodeWriter::new(out_file);
    code_writer.write_init();

    for input_file in input_files {
        translate(&input_file, &mut code_writer);
    }

    code_writer.close();
}

fn translate(input_file: &PathBuf, code_writer: &mut CodeWriter) -> () {
    let mut in_file = File::open(&input_file).expect("Unable to find file.");
    let mut buffer = String::new();
    in_file.read_to_string(&mut buffer).expect("Unable to read file.");
    let in_file_name = input_file.file_stem().unwrap().to_str().unwrap();

    let mut parser = Parser::new(&buffer);
    code_writer.set_file_name(in_file_name);

    while parser.has_more_commands() {
        parser.advance();

        match parser.command_type() {
            CommandType::CPush | CommandType::CPop => {
                code_writer.write_push_pop(parser.command_type(), parser.arg1(), parser.arg2());
            }
            CommandType::CArithmetic => {
                code_writer.write_arithmetic(parser.arg1());
            }
            CommandType::CLabel => {
                code_writer.write_label(parser.arg1());
            }
            CommandType::CGoto => {
                code_writer.write_goto(parser.arg1());
            }
            CommandType::CIf => {
                code_writer.write_if(parser.arg1());
            }
            CommandType::CFunction => {
                code_writer.write_function(parser.arg1(), parser.arg2());
            }
            CommandType::CReturn => {
                code_writer.write_return();
            }
            CommandType::CCall => {
                code_writer.write_call(parser.arg1(), parser.arg2());
            }
        }
    }
}
