pub struct Parser<'a> {
    buffer: &'a str,
    remaining: &'a str,
    current: &'a str
}

#[derive(Debug, PartialEq)]
pub enum CommandType {
    ACommand,
    CCommand,
    LCommand
}

impl<'a> Parser<'a> {
    pub fn new(buffer: &'a str) -> Self {
        Parser {
            buffer: buffer,
            remaining: buffer,
            current: ""
        }
    }

    pub fn has_more_commands(&self) -> bool {
        self.remaining.lines().any(|line| {
            let line = line.trim();
            !(line.is_empty() || line.starts_with("//"))
        })
    }

    pub fn advance(&mut self) -> () {
        self.current = "";
        while self.current.is_empty() || self.current.starts_with("//") {
            let mut lines = self.remaining.splitn(2, '\n');
            self.current = lines.next().unwrap();
            self.current = self.current.splitn(2, '/').next().unwrap().trim();
            self.remaining = lines.next().unwrap_or("");
        }
    }

    pub fn command_type(&self) -> CommandType {
        if self.current.starts_with("@") {
            CommandType::ACommand
        } else if self.current.starts_with("(") {
            CommandType::LCommand
        } else {
            CommandType::CCommand
        }
    }

    pub fn symbol(&self) -> &'a str {
        match self.command_type() {
            CommandType::ACommand => &self.current[1..],
            CommandType::LCommand => self.current
                .trim_left_matches('(')
                .trim_right_matches(')'),
            _ => panic!()
        }
    }

    pub fn dest(&self) -> Option<&'a str> {
        if !self.current.contains("=") {
            None
        } else {
            let temp: Vec<&str> = self.current.split('=').collect();
            Some(temp[0])
        }
    }

    pub fn comp(&self) -> &'a str {
        let mut comp = self.current;
        if comp.contains("=") {
            let temp: Vec<&str> = comp.split('=').collect();
            comp = temp[1];
        }
        if comp.contains(";") {
            let temp: Vec<&str> = comp.split(';').collect();
            comp = temp[0];
        }
        comp
    }

    pub fn jump(&self) -> Option<&str> {
        if !self.current.contains(";") {
            None
        } else {
            let temp: Vec<&str> = self.current.split(';').collect();
            Some(temp[1])
        }
    }

    pub fn print(&self) {
        for line in self.buffer.lines() {
            println!("{}", line);
        }
    }
}
