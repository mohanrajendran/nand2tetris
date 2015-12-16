use std::str::*;

pub struct Parser<'a> {
    buffer: &'a String,
    current: &'a str
}

#[derive(Debug, PartialEq)]
pub enum CommandType {
    ACommand,
    CCommand,
    LCommand
}

impl<'a> Parser<'a> {
    pub fn new(buffer: &'a String) -> Self {
        Parser {
            buffer: buffer,
            current: ""
        }
    }

    pub fn print(&self) {
        for line in self.buffer.lines() {
            println!("{}", line);
        }
    }
}
    
