use std::{collections::HashMap, fmt, path::Display};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TokenType {
    // Single-character tokens.
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,

    // One or two character tokens.
    BANG,
    BANG_EQUAL,
    EQUAL,
    EQUAL_EQUAL,
    GREATER,
    GREATER_EQUAL,
    LESS,
    LESS_EQUAL,

    // Literals.
    IDENTIFIER,
    STRING,
    NUMBER,

    // Keywords.
    AND,
    CLASS,
    ELSE,
    ELIF,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,

    EOF,
}

pub fn string_to_token(to_convert: &str) -> Result<TokenType, &str> {
    let mut string_to_token_type = HashMap::new();
    string_to_token_type.insert("and", TokenType::AND);
    string_to_token_type.insert("clas", TokenType::CLASS);
    string_to_token_type.insert("else", TokenType::ELSE);
    string_to_token_type.insert("elif", TokenType::ELIF);
    string_to_token_type.insert("false", TokenType::FALSE);
    string_to_token_type.insert("fun", TokenType::FUN);
    string_to_token_type.insert("for", TokenType::FOR);
    string_to_token_type.insert("if", TokenType::IF);
    string_to_token_type.insert("nil", TokenType::NIL);
    string_to_token_type.insert("or", TokenType::OR);
    string_to_token_type.insert("print", TokenType::PRINT);
    string_to_token_type.insert("return", TokenType::RETURN);
    string_to_token_type.insert("super", TokenType::SUPER);
    string_to_token_type.insert("this", TokenType::THIS);
    string_to_token_type.insert("true", TokenType::TRUE);
    string_to_token_type.insert("var", TokenType::VAR);
    string_to_token_type.insert("while", TokenType::WHILE);

    let x = string_to_token_type.get(to_convert);
    match x {
        Some(t) => return Ok(*t),
        _ => return Err("Invalid token")
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub _type: TokenType,
    pub lexme: String,
    pub line: u32
}

impl Token {
    pub fn new(_type: TokenType, lexme: String, line: u32) -> Self {
        Self {
            _type,
            lexme,
            line
        }
    }

    pub fn to_string(&self) -> String {
        return format!("{:?} {}", self._type, self.lexme)
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
