use crate::general_types::tokens::{Token, TokenType};
use super::ast::{Binary, Expr, Unary};
use super::ast::Literal;

pub struct Parser {
    pub tokens: Vec<Token>,
    pub current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn consume(&mut self, _type: TokenType, err_message: &str) -> Option<Token> {
        if self.tokens[self.current]._type == _type {
            return self.advance();
        }
        panic!("{:?}", err_message);
    }

    fn previous(&mut self) -> Token {
        return self.tokens[self.current - 1].clone();
    }

    fn match_token(&mut self, tokens: Vec<TokenType>) -> bool {
            match self.peek() {
                Some(value) => {
                    for n in tokens{
                        if n == value._type {
                            self.advance();
                            return true;
                        }
                    }
                }
                None => {
                    return false;
                }
            }
        return false;
    }

    fn advance(&mut self) -> Option<Token> {
        if !self.is_at_end() {
            self.current += 1;
            let next_token = self.tokens.get(self.current);
            match next_token {
                Some(value) => return Some(value.clone()),
                None => return None,
            }
        }
        return None;
    }

    fn is_at_end(&self) -> bool {
        if self.current >= self.tokens.len() {
            return true;
        }
        return false;
    }

    fn peek(&self) -> Option<Token> {
        if !self.is_at_end() {
            return Some(self.tokens[self.current].clone());
        }
        return None;
    }

    fn create_binary_expr(&mut self, match_tokens: Vec<TokenType>, precedent_function: fn(&mut Self) -> Expr) -> Expr {
        let mut expr = precedent_function(self);
        while self.match_token(match_tokens.clone()) {
            match self.peek() {
                Some(_) => {
                    let operator_type = self.previous()._type;
                    let right_expression = precedent_function(self);
                    expr = Expr::Binary(Binary {
                        left: Box::new(expr),
                        operator: operator_type,
                        right: Box::new(right_expression)
                    })
                }
                _ => {}
            }
        }
        return expr;
    }

    pub fn expr(&mut self) -> Expr {
        return self.equality();
    }

    fn equality(&mut self) -> Expr {
        return self.create_binary_expr(vec![TokenType::BANG_EQUAL, TokenType::EQUAL_EQUAL], Self::comparison)
    }

    fn comparison(&mut self) -> Expr {
        return self.create_binary_expr(vec![TokenType::GREATER, TokenType::GREATER_EQUAL, TokenType::LESS, TokenType::LESS_EQUAL], Self::term)
    }

    fn term(&mut self) -> Expr {
        return self.create_binary_expr(vec![TokenType::MINUS, TokenType::PLUS], Self::factor)
    }

    fn factor(&mut self) -> Expr {
        return self.create_binary_expr(vec![TokenType::SLASH, TokenType::STAR], Self::unary)
    }

    fn unary(&mut self) -> Expr {
        if self.match_token(vec![TokenType::BANG, TokenType::MINUS]) {
            let operator = self.previous();
            return Expr::Unary(Unary{
                operator: operator._type,
                value: Box::from(self.unary())
            })
        }
        return self.primary();
    }

    fn primary(&mut self) -> Expr {
        if self.match_token(vec![TokenType::FALSE]) {
            return Expr::Literal(Literal::FALSE)
        } else if self.match_token(vec![TokenType::TRUE]) {
            return Expr::Literal(Literal::TRUE)
        } else if self.match_token(vec![TokenType::NIL]) {
            return Expr::Literal(Literal::NIL)
        } else if self.match_token(vec![TokenType::NUMBER]) {
            match self.peek() {
                Some(_) => {
                    return Expr::Literal(Literal::NUMBER(self.previous().lexme.parse::<f32>().unwrap()));
                },
                _ => {
                    panic!("expected the expression to have a literal value but can't find it.");
                }
            }
        } else if self.match_token(vec![TokenType::STRING]) {
            match self.peek() {
                Some(_) => {
                    return Expr::Literal(Literal::STRING(self.previous().lexme));
                },
                _ => {
                    panic!("expected the expression to have a literal value but can't find it.");
                }
            }
        } else if self.match_token(vec![TokenType::LEFT_PAREN]) {
            let expr = self.expr();
            self.consume(TokenType::RIGHT_PAREN, "Expected a ')'");
            return Expr::Grouping(Box::from(expr));
        } else {
            panic!("invalid token.");
        }
    }
}
