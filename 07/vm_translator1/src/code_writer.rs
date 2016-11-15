use std::io::prelude::*;
use std::fs::File;
use std::convert::AsRef;

use parser::CommandType;

pub struct CodeWriter {
    out_file: File,
    arithmetic_counter: u16
}

impl CodeWriter {
    pub fn new(out_file: File) -> Self {
        CodeWriter {
            out_file: out_file,
            arithmetic_counter: 0
        }
    }

    pub fn set_file_name(&mut self, name: &str) -> () {
        unimplemented!();
    }

    pub fn write_arithmetic(&mut self, command: &str) -> () {
        match command.to_lowercase().as_ref() {
            "add" => self.write_binary_op("M=D+M"),
            "sub" => self.write_binary_op("M=M-D"),
            "neg" => self.write_unary_op("M=-M"),
            "eq"  => self.write_binary_jmp("JEQ"),
            "gt"  => self.write_binary_jmp("JGT"),
            "lt"  => self.write_binary_jmp("JLT"),
            "and" => self.write_binary_op("M=D&M"),
            "or"  => self.write_binary_op("M=D|M"),
            "not" => self.write_unary_op("M=!M"),
            &_ => ()
        }
    }

    fn write_unary_op(&mut self, operation:&str) -> () {
        write!(self.out_file, "@SP \n\
                               A=M-1 \n\
                               {}\n", operation);
    }

    fn write_binary_op(&mut self, operation: &str) -> () {
        write!(self.out_file, "@SP \n\
                               AM=M-1 \n\
                               D=M \n\
                               A=A-1 \n\
                               {}\n", operation);
    }

    fn write_binary_jmp(&mut self, jump: &str) -> () {
        write!(self.out_file, "@SP \n\
                               AM=M-1 \n\
                               D=M \n\
                               A=A-1 \n\
                               D=M-D \n\
                               @FALSE{0} \n\
                               D;{1} \n\
                               @SP \n\
                               A=M-1 \n\
                               M=0 \n\
                               @CONTINUE{0} \n\
                               0;JMP \n\
                               (FALSE{0}) \n\
                               @SP \n\
                               A=M-1 \n\
                               M=-1 \n\
                               (CONTINUE{0})\n", self.arithmetic_counter, jump);
        self.arithmetic_counter = self.arithmetic_counter + 1;
    }

    pub fn write_push_pop(&mut self, command: CommandType, segment: &str, index: u16) -> () {
        if command == CommandType::CPush {
            write!(self.out_file, "@{} \n\
                                   D=A \n\
                                   @SP \n\
                                   A=M \n\
                                   M=D \n\
                                   @SP \n\
                                   M=M+1\n", index);
        }
    }

    pub fn close(&mut self) -> () {
        write!(self.out_file, "(END) \n\
                               @END \n\
                               0;JMP");
    }
}
