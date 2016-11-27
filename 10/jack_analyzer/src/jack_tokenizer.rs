use regex::Regex;

pub struct JackTokenizer {
    buffer: String
}

impl JackTokenizer {
    pub fn new(buffer: String) -> JackTokenizer {
        let buffer = JackTokenizer::strip_comments(buffer);
        JackTokenizer {
            buffer: buffer
        }
    }

    fn strip_comments(buffer: String) -> String {
        // Single-line comments
        let re = Regex::new(r"//[^\n]*\n").unwrap();
        let buffer = re.replace_all(&buffer, "");

        // Multi-line comments
        let re = Regex::new(r"(?ms)/\*.*?\*/").unwrap();
        let buffer = re.replace_all(&buffer, "");
        buffer
    }

    pub fn print(&self) {
        println!("{}", self.buffer);
    }
}
