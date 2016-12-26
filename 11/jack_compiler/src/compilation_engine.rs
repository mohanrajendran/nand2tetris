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
    ast_writer: XmlWriter<'a, File>,
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
            ast_writer: XmlWriter::new(ast_file),
        }
    }

    pub fn compile_class(&mut self) {
        self.token_writer.begin_elem("tokens");
        self.ast_writer.begin_elem("class");

        // class
        self.serialize_and_advance();

        // className
        self.serialize_and_advance();

        // {
        self.serialize_and_advance();

        // optional classVarDec
        while self.tokenizer.token_type() == TokenType::KEYWORD &&
              (self.tokenizer.key_word() == KeyWord::STATIC ||
               self.tokenizer.key_word() == KeyWord::FIELD) {
            self.compile_class_var_dec();
        }

        // optional subroutine
        while self.tokenizer.token_type() == TokenType::KEYWORD &&
              (self.tokenizer.key_word() == KeyWord::CONSTRUCTOR ||
               self.tokenizer.key_word() == KeyWord::FUNCTION ||
               self.tokenizer.key_word() == KeyWord::METHOD) {
            self.compile_subroutine();
        }

        // }
        self.serialize_and_advance();

        self.ast_writer.end_elem();
        self.token_writer.end_elem();
    }

    fn compile_class_var_dec(&mut self) {
        self.ast_writer.begin_elem("classVarDec");

        // static | field
        self.serialize_and_advance();
        // type
        self.serialize_and_advance();

        // varName list
        while self.tokenizer.token_type() != TokenType::SYMBOL || self.tokenizer.symbol() != ';' {
            self.serialize_and_advance();
        }

        // ;
        self.serialize_and_advance();

        self.ast_writer.end_elem();
    }

    fn compile_subroutine(&mut self) {
        self.ast_writer.begin_elem("subroutineDec");

        // constructor | function | method
        self.serialize_and_advance();

        // void | type
        self.serialize_and_advance();

        // subroutineName
        self.serialize_and_advance();

        // (
        self.serialize_and_advance();

        // optional Parameter List
        self.compile_parameter_list();

        // )
        self.serialize_and_advance();

        // compile subroutineBody
        self.ast_writer.begin_elem("subroutineBody");

        // {
        self.serialize_and_advance();

        // optional varDec
        while self.tokenizer.token_type() == TokenType::KEYWORD &&
              self.tokenizer.key_word() == KeyWord::VAR {
            self.compile_var_dec();
        }

        // statements
        self.compile_statements();

        // }
        self.serialize_and_advance();

        // End subroutineBody and subroutineDec
        self.ast_writer.end_elem();
        self.ast_writer.end_elem();
    }

    fn compile_parameter_list(&mut self) {
        self.ast_writer.begin_elem("parameterList");

        while self.tokenizer.token_type() != TokenType::SYMBOL || self.tokenizer.symbol() != ')' {
            self.serialize_and_advance();
        }

        self.ast_writer.end_elem();
    }

    fn compile_var_dec(&mut self) {
        self.ast_writer.begin_elem("varDec");

        // var
        self.serialize_and_advance();

        // type
        self.serialize_and_advance();

        // varName list
        while self.tokenizer.token_type() != TokenType::SYMBOL || self.tokenizer.symbol() != ';' {
            self.serialize_and_advance();
        }

        // ;
        self.serialize_and_advance();

        self.ast_writer.end_elem();
    }

    fn compile_statements(&mut self) {
        self.ast_writer.begin_elem("statements");

        while self.tokenizer.token_type() == TokenType::KEYWORD {
            match self.tokenizer.key_word() {
                KeyWord::LET => self.compile_let(),
                KeyWord::IF => self.compile_if(),
                KeyWord::WHILE => self.compile_while(),
                KeyWord::DO => self.compile_do(),
                KeyWord::RETURN => self.compile_return(),
                _ => {
                    panic!("Unknown statement starting with {:?}",
                           self.tokenizer.key_word())
                }
            };
        }

        self.ast_writer.end_elem();
    }

    fn compile_do(&mut self) {
        self.ast_writer.begin_elem("doStatement");

        // do
        self.serialize_and_advance();

        // subroutineName | className
        self.serialize_and_advance();

        // optional .subroutineName
        if self.tokenizer.token_type() == TokenType::SYMBOL && self.tokenizer.symbol() == '.' {
            // .
            self.serialize_and_advance();

            // subroutineName
            self.serialize_and_advance();
        }

        // (
        self.serialize_and_advance();

        // expressionList
        self.compile_expression_list();

        // )
        self.serialize_and_advance();

        // ;
        self.serialize_and_advance();

        self.ast_writer.end_elem();
    }

    fn compile_let(&mut self) {
        self.ast_writer.begin_elem("letStatement");

        // let
        self.serialize_and_advance();

        // varName
        self.serialize_and_advance();

        // optional index
        if self.tokenizer.token_type() == TokenType::SYMBOL && self.tokenizer.symbol() == '[' {
            // [
            self.serialize_and_advance();

            // expression
            self.compile_expression();

            // ]
            self.serialize_and_advance();
        }

        // =
        self.serialize_and_advance();

        // expression
        self.compile_expression();

        // ;
        self.serialize_and_advance();

        self.ast_writer.end_elem();
    }

    fn compile_while(&mut self) {
        self.ast_writer.begin_elem("whileStatement");

        // while
        self.serialize_and_advance();

        // (
        self.serialize_and_advance();

        // expression
        self.compile_expression();

        // )
        self.serialize_and_advance();

        // {
        self.serialize_and_advance();

        // statements
        self.compile_statements();

        // }
        self.serialize_and_advance();

        self.ast_writer.end_elem();
    }

    fn compile_return(&mut self) {
        self.ast_writer.begin_elem("returnStatement");

        // return
        self.serialize_and_advance();

        // optional expression
        if self.tokenizer.token_type() != TokenType::SYMBOL || self.tokenizer.symbol() != ';' {
            self.compile_expression();
        }

        // ;
        self.serialize_and_advance();

        self.ast_writer.end_elem();
    }

    fn compile_if(&mut self) {
        self.ast_writer.begin_elem("ifStatement");

        // if
        self.serialize_and_advance();

        // (
        self.serialize_and_advance();

        // expression
        self.compile_expression();

        // )
        self.serialize_and_advance();

        // {
        self.serialize_and_advance();

        // statements
        self.compile_statements();

        // }
        self.serialize_and_advance();

        // optional else
        if self.tokenizer.token_type() == TokenType::KEYWORD &&
           self.tokenizer.key_word() == KeyWord::ELSE {
            // else
            self.serialize_and_advance();

            // {
            self.serialize_and_advance();

            // statements
            self.compile_statements();

            // }
            self.serialize_and_advance();
        }

        self.ast_writer.end_elem();
    }

    fn compile_expression(&mut self) {
        self.ast_writer.begin_elem("expression");

        // term
        self.compile_term();

        // optional (op term) multiple
        while self.tokenizer.token_type() == TokenType::SYMBOL &&
              (self.tokenizer.symbol() == '+' || self.tokenizer.symbol() == '-' ||
               self.tokenizer.symbol() == '*' || self.tokenizer.symbol() == '/' ||
               self.tokenizer.symbol() == '&' ||
               self.tokenizer.symbol() == '|' ||
               self.tokenizer.symbol() == '<' ||
               self.tokenizer.symbol() == '>' ||
               self.tokenizer.symbol() == '=') {
            // op
            self.serialize_and_advance();

            // term
            self.compile_term();
        }

        self.ast_writer.end_elem();
    }

    fn compile_term(&mut self) {
        self.ast_writer.begin_elem("term");

        match self.tokenizer.token_type() {
            // integerConstant | stringConstant | keywordConstant
            TokenType::INT_CONST |
            TokenType::STRING_CONST |
            TokenType::KEYWORD => {
                self.serialize_and_advance();
            }
            // unaryOp term | (expression)
            TokenType::SYMBOL => {
                // unaryOp term
                if self.tokenizer.symbol() == '-' || self.tokenizer.symbol() == '~' {
                    self.serialize_and_advance();
                    self.compile_term();
                }
                // (expression)
                else {
                    // (
                    self.serialize_and_advance();

                    // expression
                    self.compile_expression();

                    // )
                    self.serialize_and_advance();
                }
            }
            // varName | varName[expression] |
            // subroutineName (expressionList) |
            // className.subroutineName(expressionList)
            TokenType::IDENTIFIER => {
                // varName | subroutineName | className
                self.serialize_and_advance();

                // non-varname
                if self.tokenizer.token_type() == TokenType::SYMBOL {
                    // [expression]
                    if self.tokenizer.symbol() == '[' {
                        // [
                        self.serialize_and_advance();

                        // expression
                        self.compile_expression();

                        // ]
                        self.serialize_and_advance();
                    }
                    // (expressionList)
                    else if self.tokenizer.symbol() == '(' {
                        // (
                        self.serialize_and_advance();

                        // expressionList
                        self.compile_expression_list();

                        // )
                        self.serialize_and_advance();
                    }
                    // .subroutineName(expressionList)
                    else if self.tokenizer.symbol() == '.' {
                        // .
                        self.serialize_and_advance();

                        // subroutineName
                        self.serialize_and_advance();

                        // (
                        self.serialize_and_advance();

                        // expressionList
                        self.compile_expression_list();

                        // )
                        self.serialize_and_advance();
                    }
                }
            }
        }

        self.ast_writer.end_elem();
    }

    fn compile_expression_list(&mut self) {
        self.ast_writer.begin_elem("expressionList");

        if self.tokenizer.token_type() != TokenType::SYMBOL || self.tokenizer.symbol() != ')' {
            // expression
            self.compile_expression();

            // multiple optional , expression
            while self.tokenizer.token_type() == TokenType::SYMBOL &&
                  self.tokenizer.symbol() == ',' {
                // ,
                self.serialize_and_advance();

                // expression
                self.compile_expression();
            }
        }

        self.ast_writer.end_elem();
    }

    fn serialize_and_advance(&mut self) {
        self.tokenizer.serialize_current(&mut self.token_writer);
        self.tokenizer.serialize_current(&mut self.ast_writer);

        self.tokenizer.advance();
    }
}
