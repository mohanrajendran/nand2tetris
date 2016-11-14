pub struct Parser<'a> {
    remaining: &'a str,
    current_line: &'a str
}

#[derive(Debug, PartialEq)]
pub enum CommandType {
    CArithmetic,
    CPush
}

impl<'a> Parser<'a> {
    pub fn new(buffer: &'a str) -> Self {
        Parser {
            remaining: buffer,
            current_line: ""
        }
    }

    pub fn has_more_commands(&self) -> bool {
        self.remaining.lines().any(|line| {
            let line = line.trim();
            !(line.is_empty() || line.starts_with("//"))
        })
    }

    pub fn advance(&mut self) -> () {
        self.current_line = "";

        while self.current_line.is_empty() || self.current_line.starts_with("//") {
            let mut lines = self.remaining.splitn(2, '\n');

            self.current_line = lines.next().unwrap();
            self.current_line = self.current_line.splitn(2, '/').next().unwrap().trim();

            self.remaining = lines.next().unwrap_or("");
        }
    }

    pub fn command_type(&self) -> CommandType {
        let tokens:Vec<&str> = self.current_line.split(' ').collect();

        if tokens.len() == 1 {
            CommandType::CArithmetic
        } else {
            CommandType::CPush
        }
    }

    pub fn arg1(&self) -> &'a str {
        let mut tokens = self.current_line.split(' ');

        if self.command_type() == CommandType::CArithmetic {
            tokens.next().unwrap()
        } else {
            tokens.nth(1).unwrap()
        }
    }

    pub fn arg2(&self) -> u16 {
        let mut tokens = self.current_line.split(' ');

        tokens.nth(2).unwrap().parse::<u16>().unwrap()
    }

    pub fn print(&self) {
        for line in self.remaining.lines() {
            println!("{}", line);
        }
    }
}
