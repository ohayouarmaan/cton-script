use std::any;

use super::printer::ASTPrinter;
use crate::general_types::tokens::Token;
use crate::general_types::tokens::TokenType;

#[derive(Debug, Clone)]
pub enum Expr {
    Binary(Binary),
    Literal(Literal),
    Unary(Unary),
    Grouping(Box<Expr>),
}

pub trait VisitorTrait<T> {
    fn visitBinaryExpression(&self, expr: Binary) -> T;
    fn visitUnaryExpression(&self, expr: Unary) -> T;
    fn visitLiteral(&self, expr: Literal) -> T;
}

impl Expr {
    pub fn accept<T>(&self, ast_printer: &impl VisitorTrait<T>) -> T {
        match self {
            Expr::Binary(bin) => {
                return ast_printer.visitBinaryExpression(bin.clone());
            }
            Expr::Unary(una) => {
                return ast_printer.visitUnaryExpression(una.clone());
            }
            Expr::Literal(lit) => {
                return ast_printer.visitLiteral(lit.clone());
            }
            Expr::Grouping(group) => {
                return group.accept::<T>(ast_printer);
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum Literal {
    NUMBER(Token),
    STRING(Token),
    TRUE,
    FALSE,
    NIL,
}

#[derive(Debug, Clone)]
pub struct Binary {
    pub left: Box<Expr>,
    pub operator: TokenType,
    pub right: Box<Expr>,
}

impl Binary {
    pub fn accept<T>(&self, ast_printer: &impl VisitorTrait<T>) -> T {
        return ast_printer.visitBinaryExpression(self.clone());
    }
}

#[derive(Debug, Clone)]
pub struct Unary {
    pub operator: TokenType,
    pub value: Box<Expr>,
}

impl Unary {
    pub fn accept<T>(&self, ast_printer: &impl VisitorTrait<T>) -> T {
        return ast_printer.visitUnaryExpression(self.clone());
    }
}
