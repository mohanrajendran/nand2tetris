use regex::Regex;

#[derive(Debug, PartialEq)]
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
    tokens: Vec<String>,
    index: Option<usize>,
    rexpr_keyword: Regex,
    rexpr_symbol: Regex,
    rexpr_integer: Regex,
    rexpr_string: Regex,
    rexpr_identifier: Regex
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
        JackTokenizer {
            tokens: tokens,
            index: None,
            rexpr_keyword: Regex::new(RE_KEYWORD).unwrap(),
            rexpr_symbol: Regex::new(RE_SYMBOL).unwrap(),
            rexpr_integer: Regex::new(RE_INTEGER).unwrap(),
            rexpr_string: Regex::new(RE_STRING).unwrap(),
            rexpr_identifier: Regex::new(RE_IDENTIFIER).unwrap()
        }
    }

    pub fn has_more_tokens(&self) -> bool {
        self.index == None || self.index.unwrap() < self.tokens.len()-1
    }

    pub fn advance(&mut self) {
        let temp = self.index.clone();
        self.index = match temp {
            None => Some(0),
            Some(v) => Some(v+1)
        };
    }

    pub fn token_type(&self) -> TokenType {
        let ref token = self.tokens[self.index.unwrap()];

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
        let rExpr = format!(r"{}|{}|{}|{}|{}", RE_KEYWORD, RE_SYMBOL, RE_INTEGER, RE_STRING, RE_IDENTIFIER);
        let re = Regex::new(&rExpr).unwrap();
        re.captures_iter(&buffer)
            .map(|cap| cap.at(0).unwrap().to_string())
            .collect()
    }

    pub fn print(&self) {
        println!("{:?}", self.tokens);
    }
}
