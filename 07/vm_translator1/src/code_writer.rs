use std::io::prelude::*;
use std::fs::File;

use parser::CommandType;

pub struct CodeWriter {
    out_file: File
}

impl CodeWriter {
    pub fn new(out_file: File) -> Self {
        CodeWriter {
            out_file: out_file
        }
    }

    pub fn set_file_name(&mut self, name: &str) -> () {
        unimplemented!();
    }

    pub fn write_arithmetic(&mut self, command: &str) -> () {
        if command.to_lowercase() == "add" {
            self.out_file.write(b"@SP\nA=M\nA=A-1\nD=M\nA=A-1\nM=D+M\n@SP\nM=M-1\n");
        }
    }

    pub fn write_push_pop(&mut self, command: CommandType, segment: &str, index: u16) -> () {
        if command == CommandType::CPush {
            write!(self.out_file, "@{}\nD=A\n@SP\nA=M\nM=D\n@SP\nM=M+1\n", index);
        }
    }

    pub fn close(&mut self) -> () {
        self.out_file.write(b"(END)\n@END\n0;JMP");
    }
}
