use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

use jack_tokenizer::JackTokenizer;
use jack_tokenizer::TokenType;
use jack_tokenizer::KeyWord;

use symbol_table::SymbolTable;
use symbol_table::IdentifierKind;

use vm_writer::VMWriter;

pub struct CompilationEngine {
    tokenizer: JackTokenizer,
    vm_writer: VMWriter,
    symbol_table: SymbolTable,
    class_name: String
}

impl CompilationEngine {
    pub fn new(in_path: PathBuf) -> CompilationEngine {
        let mut buffer = String::new();
        let mut in_file = File::open(&in_path).expect("File missing");
        in_file.read_to_string(&mut buffer);

        let mut out_path = in_path.clone();
        out_path.set_extension("vm");
        let mut out_file = File::create(out_path).expect("Unable to create output file.");

        CompilationEngine {
            tokenizer: JackTokenizer::new(buffer),
            vm_writer: VMWriter::new(out_file),
            symbol_table: SymbolTable::new(),
            class_name: "".to_string()
        }
    }

    pub fn compile_class(&mut self) {
        // class
        self.tokenizer.advance();

        // className
        self.class_name = self.tokenizer.identifier();
        self.tokenizer.advance();

        // {
        self.tokenizer.advance();

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
        self.tokenizer.advance();
    }

    fn compile_class_var_dec(&mut self) {
        // static | field
        let var_kind = match self.tokenizer.key_word() {
            KeyWord::STATIC => IdentifierKind::STATIC,
            KeyWord::FIELD => IdentifierKind::FIELD,
            _ => panic!("Invalid class variable")
        };
        self.tokenizer.advance();

        // type
        let var_type = self.tokenizer.identifier();
        self.tokenizer.advance();

        // varName list
        while self.tokenizer.token_type() != TokenType::SYMBOL || self.tokenizer.symbol() != ';' {
            if self.tokenizer.token_type() != TokenType::SYMBOL || self.tokenizer.symbol() != ',' {
                self.symbol_table.define(self.tokenizer.identifier(), var_type.clone(), var_kind.clone());
            }
            self.tokenizer.advance();
        }

        // ;
        self.tokenizer.advance();
    }

    fn compile_subroutine(&mut self) {
        self.symbol_table.start_subroutine();

        // constructor | function | method
        if self.tokenizer.key_word() == KeyWord::METHOD {
                self.symbol_table.define("this".to_string(), 
                self.class_name.clone(), IdentifierKind::ARG);
            }
        self.tokenizer.advance();

        // void | type
        self.tokenizer.advance();

        // subroutineName
        let subroutineName = format!("{}.{}", self.class_name, self.tokenizer.identifier());
        self.tokenizer.advance();

        // (
        self.tokenizer.advance();

        // optional Parameter List
        self.compile_parameter_list();

        // )
        self.tokenizer.advance();

        // {
        self.tokenizer.advance();

        // optional varDec
        while self.tokenizer.token_type() == TokenType::KEYWORD &&
              self.tokenizer.key_word() == KeyWord::VAR {
            self.compile_var_dec();
        }

        // Write function declaration
        self.vm_writer.write_function(subroutineName, self.symbol_table.var_count(IdentifierKind::VAR));

        // statements
        self.compile_statements();

        // }
        self.tokenizer.advance();
    }

    fn compile_parameter_list(&mut self) {
        while self.tokenizer.token_type() != TokenType::SYMBOL || self.tokenizer.symbol() != ')' {
            if self.tokenizer.token_type() != TokenType::SYMBOL || self.tokenizer.symbol() != ',' {
                // type 
                let var_type = self.tokenizer.identifier();
                self.tokenizer.advance();

                // varName
                self.symbol_table.define(self.tokenizer.identifier(), var_type, IdentifierKind::ARG);
            }
            self.tokenizer.advance();
        } 
    }

    fn compile_var_dec(&mut self) {
        // var
        self.tokenizer.advance();
        
        // type
        let var_type = self.tokenizer.identifier();
        self.tokenizer.advance();

        // varName list
        while self.tokenizer.token_type() != TokenType::SYMBOL || self.tokenizer.symbol() != ';' {
            if self.tokenizer.token_type() != TokenType::SYMBOL || self.tokenizer.symbol() != ',' {
                self.symbol_table.define(self.tokenizer.identifier(), var_type.clone(), IdentifierKind::VAR);
            }
            self.tokenizer.advance();
        }

        // ;
        self.tokenizer.advance();
    }

    fn compile_statements(&mut self) {
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
    }

    fn compile_do(&mut self) {
        // do
        self.tokenizer.advance();

        // subroutineName | className
        self.tokenizer.advance();

        // optional .subroutineName
        if self.tokenizer.token_type() == TokenType::SYMBOL && self.tokenizer.symbol() == '.' {
            // .
            self.tokenizer.advance();

            // subroutineName
            self.tokenizer.advance();
        }

        // (
        self.tokenizer.advance();

        // expressionList
        self.compile_expression_list();

        // )
        self.tokenizer.advance();

        // ;
        self.tokenizer.advance();
    }

    fn compile_let(&mut self) {
        // let
        self.tokenizer.advance();

        // varName
        self.tokenizer.advance();

        // optional index
        if self.tokenizer.token_type() == TokenType::SYMBOL && self.tokenizer.symbol() == '[' {
            // [
            self.tokenizer.advance();

            // expression
            self.compile_expression();

            // ]
            self.tokenizer.advance();
        }

        // =
        self.tokenizer.advance();

        // expression
        self.compile_expression();

        // ;
        self.tokenizer.advance();
    }

    fn compile_while(&mut self) {
        // while
        self.tokenizer.advance();

        // (
        self.tokenizer.advance();

        // expression
        self.compile_expression();

        // )
        self.tokenizer.advance();

        // {
        self.tokenizer.advance();

        // statements
        self.compile_statements();

        // }
        self.tokenizer.advance();
    }

    fn compile_return(&mut self) {
        // return
        self.tokenizer.advance();

        // optional expression
        if self.tokenizer.token_type() != TokenType::SYMBOL || self.tokenizer.symbol() != ';' {
            self.compile_expression();
        }

        // ;
        self.tokenizer.advance();
    }

    fn compile_if(&mut self) {
        // if
        self.tokenizer.advance();

        // (
        self.tokenizer.advance();

        // expression
        self.compile_expression();

        // )
        self.tokenizer.advance();

        // {
        self.tokenizer.advance();

        // statements
        self.compile_statements();

        // }
        self.tokenizer.advance();

        // optional else
        if self.tokenizer.token_type() == TokenType::KEYWORD &&
           self.tokenizer.key_word() == KeyWord::ELSE {
            // else
            self.tokenizer.advance();

            // {
            self.tokenizer.advance();

            // statements
            self.compile_statements();

            // }
            self.tokenizer.advance();
        }
    }

    fn compile_expression(&mut self) {
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
            self.tokenizer.advance();

            // term
            self.compile_term();
        }
    }

    fn compile_term(&mut self) {
        match self.tokenizer.token_type() {
            // integerConstant | stringConstant | keywordConstant
            TokenType::INT_CONST |
            TokenType::STRING_CONST |
            TokenType::KEYWORD => {
                self.tokenizer.advance();
            }
            // unaryOp term | (expression)
            TokenType::SYMBOL => {
                // unaryOp term
                if self.tokenizer.symbol() == '-' || self.tokenizer.symbol() == '~' {
                    self.tokenizer.advance();
                    self.compile_term();
                }
                // (expression)
                else {
                    // (
                    self.tokenizer.advance();

                    // expression
                    self.compile_expression();

                    // )
                    self.tokenizer.advance();
                }
            }
            // varName | varName[expression] |
            // subroutineName (expressionList) |
            // className.subroutineName(expressionList)
            TokenType::IDENTIFIER => {
                // varName | subroutineName | className
                self.tokenizer.advance();

                // non-varname
                if self.tokenizer.token_type() == TokenType::SYMBOL {
                    // [expression]
                    if self.tokenizer.symbol() == '[' {
                        // [
                        self.tokenizer.advance();

                        // expression
                        self.compile_expression();

                        // ]
                        self.tokenizer.advance();
                    }
                    // (expressionList)
                    else if self.tokenizer.symbol() == '(' {
                        // (
                        self.tokenizer.advance();

                        // expressionList
                        self.compile_expression_list();

                        // )
                        self.tokenizer.advance();
                    }
                    // .subroutineName(expressionList)
                    else if self.tokenizer.symbol() == '.' {
                        // .
                        self.tokenizer.advance();

                        // subroutineName
                        self.tokenizer.advance();

                        // (
                        self.tokenizer.advance();

                        // expressionList
                        self.compile_expression_list();

                        // )
                        self.tokenizer.advance();
                    }
                }
            }
        }
    }

    fn compile_expression_list(&mut self) {
        if self.tokenizer.token_type() != TokenType::SYMBOL || self.tokenizer.symbol() != ')' {
            // expression
            self.compile_expression();

            // multiple optional , expression
            while self.tokenizer.token_type() == TokenType::SYMBOL &&
                  self.tokenizer.symbol() == ',' {
                // ,
                self.tokenizer.advance();

                // expression
                self.compile_expression();
            }
        }
        
    }
}
