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
use symbol_table::SymbolTable;

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

    let mut symbol_table = predefined_symbol_table();
    let assembly = assemble(buffer);

    let mut out_file = File::create(out_file_name).expect("Unable to create file.");
    out_file.write_all(assembly.as_bytes()).expect("Unable to write to file.");
}

fn predefined_symbol_table<'a>() -> SymbolTable<'a> {
    let mut symbol_table = SymbolTable::new();

    symbol_table.add_entry("SP", 0);
    symbol_table.add_entry("LCL", 1);
    symbol_table.add_entry("ARG", 2);
    symbol_table.add_entry("THIS", 3);
    symbol_table.add_entry("THAT", 4);
    symbol_table.add_entry("R0", 0);
    symbol_table.add_entry("R1", 1);
    symbol_table.add_entry("R2", 2);
    symbol_table.add_entry("R3", 3);
    symbol_table.add_entry("R4", 4);
    symbol_table.add_entry("R5", 5);
    symbol_table.add_entry("R6", 6);
    symbol_table.add_entry("R7", 7);
    symbol_table.add_entry("R8", 8);
    symbol_table.add_entry("R9", 9);
    symbol_table.add_entry("R10", 10);
    symbol_table.add_entry("R11", 11);
    symbol_table.add_entry("R12", 12);
    symbol_table.add_entry("R13", 13);
    symbol_table.add_entry("R14", 14);
    symbol_table.add_entry("R15", 15);
    symbol_table.add_entry("SCREEN", 16384);
    symbol_table.add_entry("KBD", 24576);

    symbol_table
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
