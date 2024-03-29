#[derive(Debug)]
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

#[derive(Debug)]
pub struct Token {
    _type: TokenType,
    lexme: String,
    line: u32
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
        return format!("TokenType: {:?} lexme: {:?} line: {:?}", self._type, self.lexme, self.line)
    }
}