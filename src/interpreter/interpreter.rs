//**************
// WIP
//*************
use crate::ast::ast::{Expr, VisitorTrait, Binary, Unary, Literal};
use crate::general_types::tokens::TokenType;

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
                        let result = Literal::NUMBER(
                            x + y
                        );
                        if let Literal::NUMBER(res) = result {
                            if(res == 7.0) {
                                println!("Thala for a reason");
                            }
                        }
                        return result;
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
                        let result = Literal::NUMBER(
                            x - y
                        );
                        if let Literal::NUMBER(res) = result {
                            if(res == 7.0) {
                                println!("Thala for a reason");
                            }
                        }
                        return result;
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
                        let result = Literal::NUMBER(
                            x / y
                        );
                        if let Literal::NUMBER(res) = result {
                            if(res == 7.0) {
                                println!("Thala for a reason");
                            }
                        }
                        return result;
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
                        let result = Literal::NUMBER(
                            x * y
                        );
                        if let Literal::NUMBER(res) = result {
                            if(res == 7.0) {
                                println!("Thala for a reason");
                            }
                        }
                        return result;
                    }
                    _ => {
                        panic!("wtf")
                    }
                }
            }
            TokenType::GREATER_EQUAL => {
                let left_value = expr.left.accept(self);
                let right_value = expr.right.accept(self);
                
                match (left_value, right_value) {
                    (Literal::NUMBER(x), Literal::NUMBER(y)) => {
                        let mut result: Literal;
                        if x >= y {
                            result = Literal::TRUE;
                        } else {
                            result = Literal::FALSE;
                        }
                        return result;
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
        let rhs = expr.value.accept(self);
        match expr.operator {
            TokenType::BANG => {
                if let Literal::NUMBER(x) = rhs {
                    if x > 0.0 {
                        return Literal::TRUE;
                    }
                    return Literal::FALSE;
                } else if let Literal::STRING(x) = rhs {
                    if x.len() > 0 {
                        return Literal::TRUE
                    }
                    return Literal::FALSE
                } else if let Literal::FALSE = rhs {
                    return Literal::TRUE
                } else if let Literal::TRUE = rhs {
                    return Literal::FALSE
                } else {
                    panic!("Invalid unary value");
                }
            }
            TokenType::MINUS => {
                if let Literal::NUMBER(x) = rhs {
                    if x > 0.0 {
                        return Literal::NUMBER(-x);
                    }
                    return Literal::FALSE;
                } else if let Literal::STRING(x) = rhs {
                    return Literal::STRING(x.chars().rev().collect::<String>());
                } else {
                    panic!("wow");
                }
            }
            _ => todo!()
        }
    }

    fn visitLiteral(&self, expr: Literal) -> Literal {
        return expr;
    }
}