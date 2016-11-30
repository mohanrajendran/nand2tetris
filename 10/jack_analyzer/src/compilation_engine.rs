use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use xml_writer::XmlWriter;

use jack_tokenizer::JackTokenizer;
use jack_tokenizer::TokenType;
use jack_tokenizer::KeyWord;

pub struct CompilationEngine<'a> {
    tokenizer: JackTokenizer,
    token_writer: XmlWriter<'a, File>,
    ast_writer: XmlWriter<'a, File>
}

impl<'a> CompilationEngine<'a> {
    pub fn new(in_path: PathBuf) -> CompilationEngine<'a> {
        let mut buffer = String::new();
        let mut in_file = File::open(&in_path).expect("File missing");
        in_file.read_to_string(&mut buffer);

        let file_name = in_path.file_stem().unwrap();
        let mut output_folder = in_path.clone();
        output_folder.pop();
        output_folder.push("output");

        let mut token_path = output_folder.clone();
        token_path.push(file_name.to_str().unwrap().to_string() + "T");
        token_path.set_extension("xml");
        let mut token_file = File::create(token_path).expect("Unable to create file.");

        let mut ast_path = output_folder.clone();
        ast_path.push(file_name);
        ast_path.set_extension("xml");
        let mut ast_file = File::create(ast_path).expect("Unable to create file.");

        CompilationEngine {
            tokenizer: JackTokenizer::new(buffer),
            token_writer: XmlWriter::new(token_file),
            ast_writer: XmlWriter::new(ast_file)
        }
    }

    pub fn compile_class(&mut self) {
        self.token_writer.begin_elem("tokens");
        self.ast_writer.begin_elem("class");
        self.tokenizer.advance();

        // Class name
        self.serialize_to_both();
        self.tokenizer.advance();

        // {
        self.serialize_to_both();
        self.tokenizer.advance();

        // optional classVarDec
        while
            self.tokenizer.token_type() == TokenType::KEYWORD &&
            (self.tokenizer.key_word() == KeyWord::STATIC ||
             self.tokenizer.key_word() == KeyWord::FIELD) {
            self.compile_class_var_dec();
        }

        while
            self.tokenizer.token_type() == TokenType::KEYWORD &&
            (self.tokenizer.key_word() == KeyWord::CONSTRUCTOR ||
             self.tokenizer.key_word() == KeyWord::FUNCTION ||
             self.tokenizer.key_word() == KeyWord::METHOD) {
            self.compile_subroutine();
        }

        self.ast_writer.end_elem();
        self.token_writer.end_elem();
    }

    fn compile_class_var_dec(&mut self) {
        self.ast_writer.begin_elem("classVarDec");

        // static | field
        self.serialize_to_both();
        self.tokenizer.advance();
        // type
        self.serialize_to_both();
        self.tokenizer.advance();
        // var name
        self.serialize_to_both();
        self.tokenizer.advance();
        // class name
        self.serialize_to_both();
        self.tokenizer.advance();

        self.ast_writer.end_elem();
    }

    fn compile_subroutine(&mut self) {
        self.ast_writer.begin_elem("subroutineDec");

        // constructor | function | method
        self.serialize_to_both();
        self.tokenizer.advance();

        // void | type
        self.serialize_to_both();
        self.tokenizer.advance();

        // subroutineName
        self.serialize_to_both();
        self.tokenizer.advance();

        // (
        self.serialize_to_both();
        self.tokenizer.advance();



        // )
        self.serialize_to_both();
        self.tokenizer.advance();

        self.ast_writer.end_elem();
    }

    fn serialize_to_both(&mut self) {
        self.tokenizer.serialize_current(&mut self.token_writer);
        self.tokenizer.serialize_current(&mut self.ast_writer);
    }
}
