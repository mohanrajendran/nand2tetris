use std::io::prelude::*;
use std::fs::File;
use std::convert::AsRef;

use parser::CommandType;

pub struct CodeWriter<'a> {
    out_file: File,
    file_name: &'a str,
    arithmetic_counter: u16
}

impl<'a> CodeWriter<'a> {
    pub fn new(out_file: File) -> Self {
        CodeWriter {
            out_file: out_file,
            file_name: "",
            arithmetic_counter: 0
        }
    }

    pub fn set_file_name(&mut self, name: &'a str) -> () {
        self.file_name = name;
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
            match segment.to_lowercase().as_ref() {
                "argument" => self.write_load_segment("ARG", index),
                "local"    => self.write_load_segment("LCL", index),
                "static"   => {
                    let name = self.file_name.clone();
                    self.write_load_literal(format!("{}.{}", name, index), false)
                },
                "constant" => self.write_load_literal(format!("{}", index), true),
                "this"     => self.write_load_segment("THIS", index),
                "that"     => self.write_load_segment("THAT", index),
                "pointer"  => self.write_load_literal(format!("{}", 3+index), false),
                "temp"     => self.write_load_literal(format!("{}", 5+index), false),
                &_ => ()
            }
            self.out_file.write(b"@SP \n\
                                  A=M \n\
                                  M=D \n\
                                  @SP \n\
                                  M=M+1\n");
        } else {
            match segment.to_lowercase().as_ref() {
                "argument" => self.write_temp_offset("ARG", index),
                "local"    => self.write_temp_offset("LCL", index),
                "this"     => self.write_temp_offset("THIS", index),
                "that"     => self.write_temp_offset("THAT", index),
                &_ => ()
            }
            self.out_file.write(b"@SP \n\
                                  AM=M-1 \n\
                                  D=M \n");
            match segment.to_lowercase().as_ref() {
                "argument" |
                "local"    |
                "this"     |
                "that"     => {
                    self.out_file.write(b"@R13 \n\
                                          A=M \n\
                                          M=D \n");
                },
                "static"   => {
                    let name = self.file_name.clone();
                    self.write_copy_to_temp(format!("{}.{}", name, index));
                },
                "pointer"  => self.write_copy_to_temp(format!("{}", 3+index)),
                "temp"     => self.write_copy_to_temp(format!("{}", 5+index)),
                &_ => ()
            }
        }
    }

    fn write_copy_to_temp(&mut self, location: String) -> () {
        write!(self.out_file, "@{} \n\
                               M=D \n", location);
    }

    fn write_temp_offset(&mut self, segment: &str, index: u16) -> () {
        write!(self.out_file, "@{} \n\
                               D=M \n\
                               @{} \n\
                               D=D+A \n\
                               @R13 \n\
                               M=D \n", segment, index);
    }

    fn write_load_segment(&mut self, segment: &str, index: u16) -> () {
        write!(self.out_file, "@{} \n\
                               D=M \n\
                               @{} \n\
                               A=D+A \n\
                               D=M\n", segment, index);
    }

    fn write_load_literal(&mut self, location: String, direct: bool) -> () {
        write!(self.out_file, "@{}\n", location);

        if direct {
            self.out_file.write(b"D=A\n");
        } else {
            self.out_file.write(b"D=M\n");
        }
    }

    pub fn write_label(&mut self, label: &str) -> () {
        write!(self.out_file, "({})\n", label);
    }

    pub fn write_goto(&mut self, label: &str) -> () {
        write!(self.out_file, "@{} \n\
                               0;JMP\n", label);
    }

    pub fn write_if(&mut self, label: &str) -> () {
        write!(self.out_file, "@SP \n\
                               AM=M-1 \n\
                               D=M \n\
                               @{} \n\
                               D;JNE \n", label);
    }

    pub fn close(&mut self) -> () {
        write!(self.out_file, "(END) \n\
                               @END \n\
                               0;JMP");
    }
}
