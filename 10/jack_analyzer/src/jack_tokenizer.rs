use regex;
use regex::Regex;

pub enum TokenType {
    KEYWORD,
    SYMBOL,
    IDENTIFIER,
    INT_CONST,
    STRING_CONST
}

pub enum KeyWord {
    CLASS,
    METHOD,
    FUNCTION,
    CONSTRUCTOR,
    INT,
    BOOLEAN,
    CHAR,
    VOID,
    VAR,
    STATIC,
    FIELD,
    LET,
    DO,
    IF,
    ELSE,
    WHILE,
    RETURN,
    TRUE,
    FALSE,
    NULL,
    THIS
}

pub struct JackTokenizer {
    buffer: String
}

static RE_KEYWORD: &'static str = r"class|method|function|constructor|int|boolean|char|void|var|static|field|let|do|if|else|while|return|true|false|null|this";
static RE_SYMBOL: &'static str = r"[\{\}\(\)\[\]\.,;\+\*/&\|<>=~-]";
static RE_INTEGER: &'static str = r"\d+";
static RE_STRING: &'static str = r#""(.+)""#;
static RE_IDENTIFIER: &'static str = r"[:word:]+";

impl JackTokenizer {
    pub fn new(buffer: String) -> JackTokenizer {
        let buffer = JackTokenizer::strip(buffer);
        JackTokenizer {
            buffer: buffer
        }
    }

    fn strip(buffer: String) -> String {
        // Single-line comments
        let re = Regex::new(r"//[^\n]*\n").unwrap();
        let buffer = re.replace_all(&buffer, "");

        // Multi-line comments
        let re = Regex::new(r"(?ms)/\*.*?\*/").unwrap();
        let buffer = re.replace_all(&buffer, "");

        buffer
    }

    pub fn print_tokens(&self) {
        let rExpr = format!(r"{}|{}|{}|{}|{}", RE_KEYWORD, RE_SYMBOL, RE_INTEGER, RE_STRING, RE_IDENTIFIER);
        let re = Regex::new(&rExpr).unwrap();
        for caps in re.captures_iter(&self.buffer) {
            println!("{:?}", caps.at(0));
        }
    }

    pub fn print(&self) {
        println!("{}", self.buffer);
    }
}
