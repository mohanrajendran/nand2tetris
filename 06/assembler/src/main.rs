use std::env;
use std::vec;
use std::fs::File;
use std::io::Read;

pub mod parser;
use parser::Parser;

fn main() {
    let args: Vec<_> = env::args().collect();
  
    if args.len() != 2
    {
        panic!("Please enter file name");
    }
    
    let mut file = File::open(&args[1]).unwrap();
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();

    let parser = Parser::new(&buffer);
    parser.print();
}

