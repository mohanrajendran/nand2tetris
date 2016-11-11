use std::env;
use std::fs::File;
use std::io::Read;
use std::io::Write;

extern crate getopts;
use getopts::Options;
use getopts::ParsingStyle;

pub mod parser;
use parser::Parser;
use parser::CommandType;

pub mod code;
use code::Code;

pub mod symbol_table;

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
        in_file_name.replace(".asm", ".mhack")
    };

    let mut in_file = File::open(in_file_name).expect("Unable to find file.");
    let mut buffer = String::new();
    in_file.read_to_string(&mut buffer).expect("Unable to read file.");
    let buffer = buffer;

    let assembly = assemble(buffer);
    let mut out_file = File::create(out_file_name).expect("Unable to create file.");
    out_file.write_all(assembly.as_bytes()).expect("Unable to write to file.");
}

fn assemble(buffer: String) -> String {
    let mut parser = Parser::new(&buffer);
    let mut prog = String::new();

    while parser.has_more_commands() {
        parser.advance();

        let opcode = match parser.command_type() {
            CommandType::ACommand => {
                let loc = parser.symbol().parse::<u16>().unwrap();
                format!("{:016b}\n", loc)
            },
            CommandType::CCommand => {
                let comp = Code::comp(parser.comp());
                let dest = Code::dest(parser.dest());
                let jump = Code::jump(parser.jump());
                format!("111{}{}{}\n", comp, dest, jump)
            },
            _ => String::new()
        };
        prog.push_str(&opcode);
    }
    prog
}
