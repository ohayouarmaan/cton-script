//**************
// WIP
//*************
use std::any;
use crate::ast::ast::{Expr, VisitorTrait, Binary, Unary, Literal};
use crate::general_types::tokens::{TokenType, Token};

pub struct Interpreter {
    pub exprs: Vec<Expr>
}

impl Interpreter {
    pub fn new(exprs: Vec<Expr>) -> Self {
        Self {
            exprs
        }
    }

    pub fn evaluate(&mut self) {
        println!("{:?}", self.exprs[0].accept(self));
    }
}

impl VisitorTrait<Literal> for Interpreter {
    fn visitBinaryExpression(&self, expr: Binary) -> Literal {
        match expr.operator {
            TokenType::PLUS => {
                let left_value = expr.left.accept(self);
                let right_value = expr.right.accept(self);
                
                match (left_value, right_value) {
                    (Literal::NUMBER(x), Literal::NUMBER(y)) => {
                        return Literal::NUMBER(
                            x + y
                        )
                    }
                    (Literal::STRING(x), Literal::STRING(y)) => {
                        return Literal::STRING(
                            x + &y
                        )
                    }
                    _ => {
                        todo!();
                    }
                }

            }
            TokenType::MINUS => {
                let left_value = expr.left.accept(self);
                let right_value = expr.right.accept(self);
                
                match (left_value, right_value) {
                    (Literal::NUMBER(x), Literal::NUMBER(y)) => {
                        return Literal::NUMBER(
                            x - y
                        )
                    }
                    _ => {
                        panic!("wtf")
                    }
                }
            }
            TokenType::SLASH => {
                let left_value = expr.left.accept(self);
                let right_value = expr.right.accept(self);
                
                match (left_value, right_value) {
                    (Literal::NUMBER(x), Literal::NUMBER(y)) => {
                        return Literal::NUMBER(
                            x / y
                        )
                    }
                    _ => {
                        panic!("wtf")
                    }
                }
            }
            TokenType::STAR => {
                let left_value = expr.left.accept(self);
                let right_value = expr.right.accept(self);
                
                match (left_value, right_value) {
                    (Literal::NUMBER(x), Literal::NUMBER(y)) => {
                        return Literal::NUMBER(
                            x * y
                        )
                    }
                    _ => {
                        panic!("wtf")
                    }
                }
            }
            _ => panic!("wtf")
        }
    }

    fn visitUnaryExpression(&self, expr: Unary) -> Literal {
        todo!()
    }

    fn visitLiteral(&self, expr: Literal) -> Literal {
        return expr;
    }
}