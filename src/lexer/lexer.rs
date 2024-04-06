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
        if !self.is_at_end() {
            let ch: char = self
                .source
                .chars()
                .nth(self.current)
                .expect("something went wrong while advancing the lexer.");
            self.current += 1;
            return ch;
        }
        return '\0';
    }

    fn add_token(&mut self, _type: tokens::TokenType) {
        self.tokens.push(tokens::Token::new(
            _type,
            (&(self.source)[self.start..self.current]).to_owned(),
            self.line,
        ));
    }

    // pub fn create_token(&mut self, _type: tokens::TokenType) -> tokens::Token{
    //     return tokens::Token::new(
    //         _type,
    //         (&(self.source)[self.start..self.current]).to_owned(),
    //         self.line,
    //     );
    // }

    fn _match(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        let c: char = self.peek();
        if c == expected {
            self.advance();
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
                if self._match('=') {
                    self.add_token(tokens::TokenType::BANG_EQUAL);
                } else {
                    self.add_token(tokens::TokenType::BANG);
                }
            }
            '>' => {
                if self._match('=') {
                    self.add_token(tokens::TokenType::GREATER_EQUAL);
                } else {
                    self.add_token(tokens::TokenType::GREATER);
                }
            }
            '<' => {
                if self._match('=') {
                    self.add_token(tokens::TokenType::LESS_EQUAL);
                } else {
                    self.add_token(tokens::TokenType::LESS);
                }
            }
            '=' => {
                if self._match('=') {
                    self.add_token(tokens::TokenType::EQUAL_EQUAL);
                } else {
                    self.add_token(tokens::TokenType::EQUAL);
                }
            }
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
            }

            // whitespaces and other ignore cases
            ' ' | '\t' | '\r' => {}
            '"' => {
                self.build_string()
            }
            '\n' => {
                self.line += 1;
            }
            c => {
                if c.is_alphabetic() {
                    // Keyword
                    self.build_keyword(c);
                } else if c.is_numeric() {
                    self.build_numbers(c);
                } else {
                    panic!("Illegal character: {:?}", c);
                }
            }
        }
    }

    fn build_string(&mut self) {
        let mut to_build_string = "\"".to_string();
        let mut current_character = self.advance();
        while current_character != '"' {
            if current_character == '\\' {
                to_build_string += "\\";
                current_character = self.advance();
            }
            to_build_string += &current_character.to_string();
            current_character = self.advance();
        }
        to_build_string += "\"";
        self.add_token(tokens::TokenType::STRING);
    }

    fn build_numbers(&mut self, mut c: char) {
        let mut lit = c.to_string();
        let mut dot_count = 0;
        while (c.is_numeric() || c == '.') && self.peek() != '\0' && self.peek() != '\n' {
            if c == '.' {
                dot_count += 1;
            }
            lit += &c.to_string();
            c = self.advance();
        }

        if dot_count > 1 {
            panic!("Invalid number.");
        } else {
            self.add_token(tokens::TokenType::NUMBER);
        }
    }

    fn build_keyword(&mut self, first_character: char) {
        let mut keyword = String::new();
        let mut c = first_character;
        loop {
            match c {
                ' ' | '\n' | '\t' | '\0' => break,
                _ => {
                    keyword += &c.to_string();
                    match self.peek() {
                        ' ' | '\n' | '\t' | '\0' => break,
                        _ => {
                            c = self.advance();
                        }
                    }
                }
            }
        }
        let kw = tokens::string_to_token(&keyword);
        match kw {
            Ok(keyword) => self.add_token(keyword),
            Err(_) => {
                // Create identifiers
                self.add_token(tokens::TokenType::IDENTIFIER);
            },
        }
    }

    pub fn lex(&mut self) {
        loop {
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
