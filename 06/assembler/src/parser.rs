use code::Code;

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

    fn has_more_commands(&self) -> bool {
        self.remaining.lines().any(|line| {
            let line = line.trim();
            !(line.is_empty() || line.starts_with("//"))
        })
    }

    fn advance(&mut self) -> () {
        self.current = "";
        while self.current.is_empty() || self.current.starts_with("//") {
            let lines: Vec<&str> = self.remaining.splitn(2, '\n').collect();
            self.current = lines[0].trim();
            self.remaining = lines[1];
        }
    }

    fn command_type(&self) -> CommandType {
        if self.current.starts_with("@") {
            CommandType::ACommand
        } else if self.current.starts_with("(") {
            CommandType::LCommand
        } else {
            CommandType::CCommand
        }
    }

    fn symbol(&self) -> &str {
        match self.command_type() {
            CommandType::ACommand => &self.current[1..],
            CommandType::LCommand => self.current
                .trim_left_matches('(')
                .trim_right_matches(')'),
            _ => panic!()
        }
    }

    fn dest(&self) -> Option<&str> {
        if !self.current.contains("=") {
            None
        } else {
            let temp: Vec<&str> = self.current.split('=').collect();
            Some(temp[0])
        }
    }

    fn comp(&self) -> &str {
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

    fn jump(&self) -> Option<&str> {
        if !self.current.contains(";") {
            None
        } else {
            let temp: Vec<&str> = self.current.split(';').collect();
            Some(temp[1])
        }
    }

    pub fn assemble(&mut self) -> String {
        let mut prog = String::new();

        while self.has_more_commands() {
            self.advance();

            let opcode = match self.command_type() {
                CommandType::ACommand => {
                    let loc = self.symbol().parse::<u16>().unwrap();
                    format!("{:016b}\n", loc)
                },
                CommandType::CCommand => {
                    let comp = Code::comp(self.comp());
                    let dest = Code::dest(self.dest());
                    let jump = Code::jump(self.jump());
                    format!("111{}{}{}\n", comp, dest, jump)
                },
                _ => String::new()
            };

            prog.push_str(&opcode);
        }
        prog
    }

    pub fn print(&self) {
        for line in self.buffer.lines() {
            println!("{}", line);
        }
    }
}
