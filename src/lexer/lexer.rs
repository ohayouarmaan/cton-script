use core::panic;

use crate::general_types::tokens;

#[derive(Debug)]
pub struct Lexer {
    source: String,
    current: usize,
    start: usize,
    line: u32,
    pub tokens: Vec<tokens::Token>,
}

impl Lexer {
    pub fn new(src: String) -> Self {
        Self {
            source: src,
            current: 0,
            start: 0,
            line: 1,
            tokens: vec![],
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        let ch: char = self
            .source
            .chars()
            .nth(self.current)
            .expect("something went wrong while advancing the lexer.");
        self.current += 1;
        return ch;
    }

    fn add_token(&mut self, _type: tokens::TokenType) {
        self.tokens.push(tokens::Token::new(
            _type,
            (&(self.source)[self.start..self.current]).to_owned(),
            self.line,
        ));
    }

    fn _match(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        let c: char = self.advance();
        if c == expected {
            return true;
        }
        return false;
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        return self
            .source
            .chars()
            .nth(self.current)
            .expect("Something went wrong");
    }

    fn scan(&mut self) {
        let c: char = self.advance();
        match c {
            '(' => self.add_token(tokens::TokenType::LEFT_PAREN),
            ')' => self.add_token(tokens::TokenType::RIGHT_PAREN),
            '{' => self.add_token(tokens::TokenType::LEFT_BRACE),
            '}' => self.add_token(tokens::TokenType::RIGHT_BRACE),
            ',' => self.add_token(tokens::TokenType::COMMA),
            '.' => self.add_token(tokens::TokenType::DOT),
            '-' => self.add_token(tokens::TokenType::MINUS),
            '+' => self.add_token(tokens::TokenType::PLUS),
            ';' => self.add_token(tokens::TokenType::SEMICOLON),
            '*' => self.add_token(tokens::TokenType::STAR),
            '!' => {
                println!("found!");
                if self._match('=') {
                    println!("running if");
                    self.add_token(tokens::TokenType::BANG_EQUAL);
                } else {
                    println!("running else");
                    self.add_token(tokens::TokenType::BANG);
                }
            },
            '>' => {
                if self._match('=') {
                    self.add_token(tokens::TokenType::GREATER_EQUAL);
                } else {
                    self.add_token(tokens::TokenType::GREATER);
                }
            },
            '<' => {
                if self._match('=') {
                    self.add_token(tokens::TokenType::LESS_EQUAL);
                } else {
                    self.add_token(tokens::TokenType::LESS);
                }
            },
            '=' => {
                if self._match('=') {
                    self.add_token(tokens::TokenType::EQUAL_EQUAL);
                } else {
                    self.add_token(tokens::TokenType::EQUAL);
                }
            },
            '/' => {
                if self._match('/') {
                    loop {
                        if self.peek() == '\n' || self.peek() == '\0' {
                            break;
                        }
                        self.advance();
                    }
                } else {
                    self.add_token(tokens::TokenType::SLASH)
                }
            },

            // whitespaces and other ignore cases
            ' ' | '\t' | '\r' => {}

            '\n' => {
                self.line += 1;
            },
            _ => {
                panic!("Illegal character: {:?}", c);
            }
        }
    }

    pub fn lex(&mut self) {
        loop {
            println!("c: {:?}", self.current);
            if self.is_at_end() {
                break;
            }
            self.start = self.current;
            self.scan();
        }
        self.tokens.push(tokens::Token::new(
            tokens::TokenType::EOF,
            "".to_owned(),
            self.line,
        ));
    }
}
