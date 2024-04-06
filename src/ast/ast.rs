// **********************************************
// WIP
// **********************************************
use crate::general_types::tokens::TokenType;
use crate::general_types::tokens::Token;

#[derive(Debug, Clone)]
pub enum Expr {
    Binary(Binary),
    Literal(Literal),
    Unary(Unary),
    Grouping(Box<Expr>)
}


#[derive(Debug, Clone)]
pub enum Literal {
    NUMBER(Token),
    STRING(Token),
    TRUE,
    FALSE,
    NIL
}

#[derive(Debug, Clone)]
pub struct Binary {
    pub left: Box<Expr>,
    pub operator: TokenType,
    pub right: Box<Expr>
}

#[derive(Debug, Clone)]
pub struct Unary {
    pub operator: TokenType,
    pub value: Box<Expr>
}
