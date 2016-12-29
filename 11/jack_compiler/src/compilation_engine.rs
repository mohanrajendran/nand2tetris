use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

use jack_tokenizer::JackTokenizer;
use jack_tokenizer::TokenType;
use jack_tokenizer::KeyWord;

use symbol_table::SymbolTable;
use symbol_table::IdentifierKind;

use vm_writer::VMWriter;
use vm_writer::Segment;
use vm_writer::Command;

pub struct CompilationEngine {
    tokenizer: JackTokenizer,
    vm_writer: VMWriter,
    symbol_table: SymbolTable,
    class_name: String,
    loop_counter: u16,
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
            class_name: "".to_string(),
            loop_counter: 0,
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
            _ => panic!("Invalid class variable"),
        };
        self.tokenizer.advance();

        // type
        let var_type = self.tokenizer.identifier();
        self.tokenizer.advance();

        // varName list
        while self.tokenizer.token_type() != TokenType::SYMBOL || self.tokenizer.symbol() != ';' {
            if self.tokenizer.token_type() != TokenType::SYMBOL || self.tokenizer.symbol() != ',' {
                self.symbol_table.define(self.tokenizer.identifier(),
                                         var_type.clone(),
                                         var_kind.clone());
            }
            self.tokenizer.advance();
        }

        // ;
        self.tokenizer.advance();
    }

    fn compile_subroutine(&mut self) {
        self.symbol_table.start_subroutine();
        self.loop_counter = 0;

        // constructor | function | method
        let subroutineType = self.tokenizer.key_word();
        self.tokenizer.advance();
        if subroutineType == KeyWord::METHOD {
            self.symbol_table.define("this".to_string(), self.class_name.clone(), IdentifierKind::ARG);
        }

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
        self.vm_writer.write_function(subroutineName,
                                      self.symbol_table.var_count(IdentifierKind::VAR));
        // If constructor, malloc initial
        if subroutineType == KeyWord::CONSTRUCTOR {
            let numField = self.symbol_table.var_count(IdentifierKind::FIELD);
            self.vm_writer.write_push(Segment::CONSTANT, numField);
            self.vm_writer.write_call("Memory.alloc".to_string(), 1);
            self.vm_writer.write_pop(Segment::POINTER, 0);
        }
        // If method, move arg0 to this
        else if subroutineType == KeyWord::METHOD {
            self.vm_writer.write_push(Segment::ARG, 0);
            self.vm_writer.write_pop(Segment::POINTER, 0);
        }

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
                self.symbol_table
                    .define(self.tokenizer.identifier(), var_type, IdentifierKind::ARG);
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
                self.symbol_table.define(self.tokenizer.identifier(),
                                         var_type.clone(),
                                         IdentifierKind::VAR);
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
        let subroutineName = self.tokenizer.identifier();
        self.tokenizer.advance();

        // compile rest of subroutine
        let numArgs = self.compile_subroutine_call(subroutineName);

        // ;
        self.tokenizer.advance();

        // throw away the returned value
        self.vm_writer.write_pop(Segment::TEMP, 0);
    }

    fn compile_let(&mut self) {
        // let
        self.tokenizer.advance();

        // varName
        let varName = self.tokenizer.identifier();
        let mut segment = match self.symbol_table.kind_of(&varName).expect("Record not found") {
            IdentifierKind::ARG => Segment::ARG,
            IdentifierKind::FIELD => Segment::THIS,
            IdentifierKind::STATIC => Segment::STATIC,
            IdentifierKind::VAR => Segment::LOCAL,
        };
        let mut index = self.symbol_table.index_of(&varName).unwrap();
        self.tokenizer.advance();

        // optional index
        if self.tokenizer.token_type() == TokenType::SYMBOL && self.tokenizer.symbol() == '[' {
            // push the right location
            self.vm_writer.write_push(segment, index);

            // [
            self.tokenizer.advance();

            // expression
            self.compile_expression();

            // compute offset
            self.vm_writer.write_arithmetic(Command::ADD);
            self.vm_writer.write_pop(Segment::POINTER, 1);
            segment = Segment::THAT;
            index = 0;

            // ]
            self.tokenizer.advance();
        }

        // =
        self.tokenizer.advance();

        // expression
        self.compile_expression();

        // pop computed expression to variable
        self.vm_writer.write_pop(segment, index);

        // ;
        self.tokenizer.advance();
    }

    fn compile_while(&mut self) {
        // while
        self.tokenizer.advance();
        let counter = self.loop_counter.clone();
        self.loop_counter += 1;

        // initial label
        self.vm_writer.write_label(format!("WHILE_LOOP{}", counter));

        // (
        self.tokenizer.advance();

        // expression
        self.compile_expression();

        // negate and jump to continue if true
        self.vm_writer.write_arithmetic(Command::NOT);
        self.vm_writer.write_if(format!("WHILE_CONTINUE{}", counter));

        // )
        self.tokenizer.advance();

        // {
        self.tokenizer.advance();

        // statements
        self.compile_statements();
        self.vm_writer.write_goto(format!("WHILE_LOOP{}", counter));

        // }
        self.tokenizer.advance();
        self.vm_writer.write_label(format!("WHILE_CONTINUE{}", counter));
    }

    fn compile_return(&mut self) {
        // return
        self.tokenizer.advance();

        // optional expression
        if self.tokenizer.token_type() != TokenType::SYMBOL || self.tokenizer.symbol() != ';' {
            self.compile_expression();
        } else {
            self.vm_writer.write_push(Segment::CONSTANT, 0);
        }

        self.vm_writer.write_return();
        // ;
        self.tokenizer.advance();
    }

    fn compile_if(&mut self) {
        // if
        self.tokenizer.advance();
        let counter = self.loop_counter.clone();
        self.loop_counter += 1;

        // (
        self.tokenizer.advance();

        // expression
        self.compile_expression();

        // negate and jump to else if true(the negation)
        self.vm_writer.write_arithmetic(Command::NOT);
        self.vm_writer.write_if(format!("IF_ELSE{}", counter));

        // )
        self.tokenizer.advance();

        // {
        self.tokenizer.advance();

        // statements
        self.compile_statements();

        // }
        self.tokenizer.advance();

        // when first is done, navigate to continue
        self.vm_writer.write_goto(format!("IF_CONTINUE{}", counter));
        self.vm_writer.write_label(format!("IF_ELSE{}", counter));

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

        self.vm_writer.write_label(format!("IF_CONTINUE{}", counter));
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
            let op = self.tokenizer.symbol();
            self.tokenizer.advance();

            // term
            self.compile_term();

            // execute operation on terms
            match op {
                '+' => self.vm_writer.write_arithmetic(Command::ADD),
                '-' => self.vm_writer.write_arithmetic(Command::SUB),
                '*' => self.vm_writer.write_call("Math.multiply".to_string(), 2),
                '/' => self.vm_writer.write_call("Math.divide".to_string(), 2),
                '&' => self.vm_writer.write_arithmetic(Command::AND),
                '|' => self.vm_writer.write_arithmetic(Command::OR),
                '<' => self.vm_writer.write_arithmetic(Command::LT),
                '>' => self.vm_writer.write_arithmetic(Command::GT),
                '=' => self.vm_writer.write_arithmetic(Command::EQ),
                _ => panic!("Invalid binary operation."),
            }
        }
    }

    fn compile_term(&mut self) {
        match self.tokenizer.token_type() {
            // integerConstant | stringConstant | keywordConstant
            TokenType::INT_CONST => {
                // push constant
                self.vm_writer.write_push(Segment::CONSTANT, self.tokenizer.int_val());
                self.tokenizer.advance();
            },
            TokenType::STRING_CONST => {
                let chars: Vec<u8> = self.tokenizer.string_val().bytes().collect();
                self.vm_writer.write_push(Segment::CONSTANT, chars.len() as u16);
                self.vm_writer.write_call("String.new".to_string(), 1);

                for c in self.tokenizer.string_val().bytes() {
                    self.vm_writer.write_push(Segment::CONSTANT, c as u16);
                    self.vm_writer.write_call("String.appendChar".to_string(), 2);
                }
            
                self.tokenizer.advance();
            },
            TokenType::KEYWORD => {
                // push keyword
                match self.tokenizer.key_word() {
                    KeyWord::TRUE => {
                        self.vm_writer.write_push(Segment::CONSTANT, 0);
                        self.vm_writer.write_arithmetic(Command::NOT)
                    }
                    KeyWord::FALSE | KeyWord::NULL => {
                        self.vm_writer.write_push(Segment::CONSTANT, 0)
                    }
                    KeyWord::THIS => self.vm_writer.write_push(Segment::POINTER, 0),
                    _ => panic!("Invalid keyword term"),
                }
                self.tokenizer.advance();
            },
            // unaryOp term | (expression)
            TokenType::SYMBOL => {
                // unaryOp term
                if self.tokenizer.symbol() == '-' || self.tokenizer.symbol() == '~' {
                    let op = self.tokenizer.symbol();
                    self.tokenizer.advance();
                    self.compile_term();

                    match op {
                        '-' => self.vm_writer.write_arithmetic(Command::NEG),
                        '~' => self.vm_writer.write_arithmetic(Command::NOT),
                        _ => panic!("Invalid unary op"),
                    }
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
            },
            // varName | varName[expression] |
            // subroutineName (expressionList) |
            // className.subroutineName(expressionList)
            TokenType::IDENTIFIER => {
                // varName | subroutineName | className
                let name = self.tokenizer.identifier();
                self.tokenizer.advance();

                if self.tokenizer.token_type() == TokenType::SYMBOL {
                    // (expressionList) or .subroutineName(expressionList)
                    if self.tokenizer.symbol() == '(' || self.tokenizer.symbol() == '.' {
                        self.compile_subroutine_call(name);
                    }
                    // variable name or [expression]
                    else {
                        let segment =
                            match self.symbol_table.kind_of(&name).expect("Record not found") {
                                IdentifierKind::ARG => Segment::ARG,
                                IdentifierKind::FIELD => Segment::THIS,
                                IdentifierKind::STATIC => Segment::STATIC,
                                IdentifierKind::VAR => Segment::LOCAL,
                            };
                        let index = self.symbol_table.index_of(&name).unwrap();
                        self.vm_writer.write_push(segment, index);

                        // [expression]
                        if self.tokenizer.symbol() == '[' {
                        // [
                        self.tokenizer.advance();

                        // expression
                        self.compile_expression();
                        self.vm_writer.write_arithmetic(Command::ADD);
                        self.vm_writer.write_pop(Segment::POINTER, 1);
                        self.vm_writer.write_push(Segment::THAT, 0);

                        // ]
                        self.tokenizer.advance();
                        }
                    }
                }
            }
        }
    }

    fn compile_expression_list(&mut self) -> u16 {
        let mut count = 0;
        if self.tokenizer.token_type() != TokenType::SYMBOL || self.tokenizer.symbol() != ')' {
            // expression
            self.compile_expression();
            count += 1;

            // multiple optional , expression
            while self.tokenizer.token_type() == TokenType::SYMBOL &&
                  self.tokenizer.symbol() == ',' {
                // ,
                self.tokenizer.advance();

                // expression
                self.compile_expression();
                count += 1;
            }
        }

        count
    }

    fn compile_subroutine_call(&mut self, so_far: String) {
        let mut subroutineName = String::new();
        let mut numArgs: u16 = 0;

        // optional .subroutineName
        if self.tokenizer.token_type() == TokenType::SYMBOL && self.tokenizer.symbol() == '.' {
            // function is one of the following:-
            // 1) static ClassName.method
            // 2) InstanceName.method
            // .
            self.tokenizer.advance();

            // subroutineName
            match self.symbol_table.type_of(&so_far) {
                // instance method
                Some(varType) => {
                    let segment = match self.symbol_table.kind_of(&so_far).unwrap() {
                        IdentifierKind::ARG => Segment::ARG,
                        IdentifierKind::FIELD => Segment::THIS,
                        IdentifierKind::STATIC => Segment::STATIC,
                        IdentifierKind::VAR => Segment::LOCAL,
                    };
                    let index = self.symbol_table.index_of(&so_far).unwrap();

                    subroutineName = varType + "." + &self.tokenizer.identifier();
                    self.vm_writer.write_push(segment, index);
                    numArgs = 1;
                }
                // class function
                None => {
                    subroutineName = so_far + "." + &self.tokenizer.identifier();
                }                
            }

            self.tokenizer.advance();
        } else {
            // function on current instance
            subroutineName = self.class_name.clone() + "." + &so_far;
            self.vm_writer.write_push(Segment::POINTER, 0);
            numArgs = 1;
        }

        // (
        self.tokenizer.advance();

        // expressionList
        numArgs += self.compile_expression_list();

        // )
        self.tokenizer.advance();

        self.vm_writer.write_call(subroutineName, numArgs);
    }
}
