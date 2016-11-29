use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

extern crate regex;

extern crate xml_writer;
use xml_writer::XmlWriter;

mod jack_tokenizer;
use jack_tokenizer::JackTokenizer;
use jack_tokenizer::TokenType;

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

fn translate(in_path: PathBuf) {
    let mut buffer = String::new();
    let mut in_file = File::open(&in_path).expect("File missing");
    in_file.read_to_string(&mut buffer);

    let mut token_path = in_path.clone();
    token_path.set_extension("tml");
    let mut token_file = File::create(token_path).expect("Unable to create file.");

    let mut tokenizer = JackTokenizer::new(buffer);

    let mut xml = XmlWriter::new(token_file);
    xml.begin_elem("tokens");

    while tokenizer.has_more_tokens() {
        tokenizer.advance();

        match tokenizer.token_type() {
            TokenType::KEYWORD => {
                xml.begin_elem("keyword");
                xml.text(&format!(" {} ", tokenizer.identifier()));
            },
            TokenType::SYMBOL => {
                xml.begin_elem("symbol");
                xml.text(&format!(" {} ", tokenizer.symbol()));
            },
            TokenType::INT_CONST => {
                xml.begin_elem("integerConstant");
                xml.text(&format!(" {} ", tokenizer.int_val()));
            },
            TokenType::STRING_CONST => {
                xml.begin_elem("stringConstant");
                xml.text(&format!(" {} ", tokenizer.string_val()));
            },
            TokenType::IDENTIFIER => {
                xml.begin_elem("identifier");
                xml.text(&format!(" {} ", tokenizer.identifier()));
            }
        }
        xml.end_elem();

        //println!("{:?}", tokenizer.token_type());
    }
    xml.close();
    xml.flush();
}
