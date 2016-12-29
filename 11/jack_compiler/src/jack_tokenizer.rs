use std::fs;
use std::fs::File;
use std::path::PathBuf;

use regex::Regex;

#[derive(Debug, PartialEq)]
pub enum TokenType {
    KEYWORD,
    SYMBOL,
    IDENTIFIER,
    INT_CONST,
    STRING_CONST,
}

#[derive(Debug, PartialEq)]
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
    THIS,
}

pub struct JackTokenizer {
    tokens: Vec<String>,
    index: usize,
    rexpr_keyword: Regex,
    rexpr_symbol: Regex,
    rexpr_integer: Regex,
    rexpr_string: Regex,
    rexpr_identifier: Regex,
}

static RE_KEYWORD: &'static str = r"class|method|function|constructor|int|boolean|char|void|var|static|field|let|do|if|else|while|return|true|false|null|this";
static RE_SYMBOL: &'static str = r"[\{\}\(\)\[\]\.,;\+\*/&\|<>=~-]";
static RE_INTEGER: &'static str = r"\d+";
static RE_STRING: &'static str = r#""(.+)""#;
static RE_IDENTIFIER: &'static str = r"[:word:]+";

impl JackTokenizer {
    pub fn new(buffer: String) -> JackTokenizer {
        let buffer = JackTokenizer::strip(buffer);
        let tokens = JackTokenizer::extract_tokens(buffer);
        let keyword_specific = "^(".to_string() + RE_KEYWORD + &")$".to_string();
        JackTokenizer {
            tokens: tokens,
            index: 0,
            rexpr_keyword: Regex::new(&keyword_specific).unwrap(),
            rexpr_symbol: Regex::new(RE_SYMBOL).unwrap(),
            rexpr_integer: Regex::new(RE_INTEGER).unwrap(),
            rexpr_string: Regex::new(RE_STRING).unwrap(),
            rexpr_identifier: Regex::new(RE_IDENTIFIER).unwrap(),
        }
    }

    pub fn has_more_tokens(&self) -> bool {
        self.index < self.tokens.len() - 1
    }

    pub fn advance(&mut self) {
        self.index = self.index + 1;
    }

    pub fn token_type(&self) -> TokenType {
        let ref token = self.tokens[self.index];

        if self.rexpr_keyword.is_match(&token) {
            TokenType::KEYWORD
        } else if self.rexpr_symbol.is_match(&token) {
            TokenType::SYMBOL
        } else if self.rexpr_integer.is_match(&token) {
            TokenType::INT_CONST
        } else if self.rexpr_string.is_match(&token) {
            TokenType::STRING_CONST
        } else {
            TokenType::IDENTIFIER
        }
    }

    pub fn key_word(&self) -> KeyWord {
        let ref token = self.tokens[self.index];

        match token.as_str() {
            "class" => KeyWord::CLASS,
            "method" => KeyWord::METHOD,
            "function" => KeyWord::FUNCTION,
            "constructor" => KeyWord::CONSTRUCTOR,
            "boolean" => KeyWord::BOOLEAN,
            "char" => KeyWord::CHAR,
            "void" => KeyWord::VOID,
            "var" => KeyWord::VAR,
            "static" => KeyWord::STATIC,
            "field" => KeyWord::FIELD,
            "let" => KeyWord::LET,
            "do" => KeyWord::DO,
            "if" => KeyWord::IF,
            "else" => KeyWord::ELSE,
            "while" => KeyWord::WHILE,
            "return" => KeyWord::RETURN,
            "true" => KeyWord::TRUE,
            "false" => KeyWord::FALSE,
            "null" => KeyWord::NULL,
            "this" => KeyWord::THIS,
            &_ => panic!(),
        }
    }

    pub fn symbol(&self) -> char {
        let ref token = self.tokens[self.index];

        token.chars().nth(0).unwrap()
    }

    pub fn identifier(&self) -> String {
        let ref token = self.tokens[self.index];

        token.clone()
    }

    pub fn int_val(&self) -> u16 {
        let ref token = self.tokens[self.index];

        token.parse::<u16>().expect("Unable to parase token.")
    }

    pub fn string_val(&self) -> String {
        let ref token = self.tokens[self.index];

        token.trim_matches('"').to_string()
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

    fn extract_tokens(buffer: String) -> Vec<String> {
        let rExpr = format!(r"{}|{}|{}|{}|{}",
                            RE_KEYWORD,
                            RE_SYMBOL,
                            RE_INTEGER,
                            RE_STRING,
                            RE_IDENTIFIER);
        let re = Regex::new(&rExpr).unwrap();
        re.captures_iter(&buffer)
            .map(|cap| cap.at(0).unwrap().to_string())
            .collect()
    }

    pub fn rewind(&mut self) {
        self.index = 0
    }

    pub fn print(&self) {
        println!("{:?}", self.tokens);
    }
}
