use crate::ast::ast::Variable;
use crate::general_types::tokens::{Token, TokenType};
use super::ast::{Assignment, Binary, Block_Statement, Expr, Expr_Statement, Print_Statement, Statement, Unary, Var_Declaration_Statement};
use super::ast::Literal;

pub struct Parser {
    pub tokens: Vec<Token>,
    pub current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn reset(&mut self) {
        self.current = 0;
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

    pub fn parse(&mut self) -> Vec<Statement> {
        let mut stmts: Vec<Statement> = vec![];
        while !self.is_at_end() {
            stmts.push(self.declaration());
        }
        return stmts;
    }

    pub fn declaration(&mut self) -> Statement {
        if self.match_token(vec![TokenType::VAR]) {
            return self.var_declaration();
        }
        return self.statement();
    }

    pub fn var_declaration(&mut self) -> Statement {
        let name = self.consume(TokenType::IDENTIFIER, "Expected an identifier").unwrap();
        let initializer: Option<Expr>;
        if self.match_token(vec![TokenType::EQUAL]) {
            initializer = Some(self.expr());
        } else {
            initializer = None;
        }
        self.consume(TokenType::SEMICOLON, &format!("Expected ';' after a statement line: {}", self.peek().line));
        return Statement::Var_Declaration_Statement(Var_Declaration_Statement{
            id: name,
            value: initializer
        });
    }

    pub fn statement(&mut self) -> Statement {
        if self.match_token(vec![TokenType::PRINT]) {
            return self.print_statement();
        }
        if self.match_token(vec![TokenType::LEFT_BRACE]) {
            return self.block_statement();
        }
        return self.expr_statement();
    }

    pub fn expr_statement(&mut self) -> Statement {
        let mut value: Expr = self.expr();
        self.consume(TokenType::SEMICOLON, &format!("Expected ';' after a statement line: {}", self.peek().line));
        return Statement::Expr_Statement(Expr_Statement{exp: value})
    }

    pub fn block_statement(&mut self) -> Statement {
        let mut stmts = vec![];
        while self.peek()._type != TokenType::RIGHT_BRACE && !self.is_at_end() {
            stmts.push(Box::from(self.declaration()));
        }
        self.consume(TokenType::RIGHT_BRACE, "Expected a '{' after a block statement");
        return Statement::Block_Statement(Block_Statement{ statements: stmts });
    }

    pub fn print_statement(&mut self) -> Statement {
        let mut value: Expr = self.expr();
        self.consume(TokenType::SEMICOLON, &format!("Expected ';' after a statement line: {}", self.peek().line));
        return Statement::Print_Statement(Print_Statement{exp: value})
    }

    fn match_token(&mut self, tokens: Vec<TokenType>) -> bool {
        for n in tokens{
            if n == self.peek()._type {
                self.advance();
                return true;
            }
        }
        return false;
    }

    fn advance(&mut self) -> Option<Token> {
        if !self.is_at_end() {
            let next_token = self.tokens.get(self.current);
            self.current += 1;
            match next_token {
                Some(value) => return Some(value.clone()),
                None => return None,
            }
        }
        return None;
    }

    fn is_at_end(&self) -> bool {
        match self.peek()._type {
            TokenType::EOF => {
                return true;
            }
            _ => {
                return false;
            }
        }
    }

    fn peek(&self) -> Token {
        return self.tokens[self.current].clone();
    }

    fn create_binary_expr(&mut self, match_tokens: Vec<TokenType>, precedent_function: fn(&mut Self) -> Expr) -> Expr {
        let mut expr = precedent_function(self);
        while self.match_token(match_tokens.clone()) {
        let operator_type = self.previous()._type;
        let right_expression = precedent_function(self);
        expr = Expr::Binary(Binary {
            left: Box::new(expr),
            operator: operator_type,
            right: Box::new(right_expression)
        })
        }
        return expr;
    }

    pub fn expr(&mut self) -> Expr {
        return self.assignment();
    }

    pub fn assignment(&mut self) -> Expr {
        let exp = self.equality();
        if self.match_token(vec![TokenType::EQUAL]) {
            let value = self.assignment();
            if let Expr::Variable(x) = exp.clone() {
                let name = x.name;
                return Expr::Assignment(Assignment{
                    name,
                    value: Box::from(value)
                })
            }
        }
        return exp;
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
            return Expr::Literal(Literal::NUMBER(self.previous().lexme.parse::<f32>().unwrap()));
        } else if self.match_token(vec![TokenType::STRING]) {
            return Expr::Literal(Literal::STRING(self.previous().lexme));
        } else if self.match_token(vec![TokenType::LEFT_PAREN]) {
            let expr = self.expr();
            self.consume(TokenType::RIGHT_PAREN, "Expected a ')'");
            return Expr::Grouping(Box::from(expr));
        } else if self.match_token(vec![TokenType::IDENTIFIER]){
            return Expr::Variable(Variable{
                name: self.previous()
            })
        } else {
            panic!("invalid token: '{}'", self.peek().to_string());
        }
    }
}
