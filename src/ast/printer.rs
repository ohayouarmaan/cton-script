use crate::general_types::tokens::TokenType;

use super::ast::{Binary, Expr, Literal, Unary, VisitorTrait};

pub struct ASTPrinter {}

impl VisitorTrait<String> for ASTPrinter {
    fn visitBinaryExpression(&mut self, expr: Binary) -> String {
        return self.parthesize(expr.operator, vec![*expr.left, *expr.right]);
    }

    fn visitUnaryExpression(&mut self, expr: Unary) -> String {
        return self.parthesize(expr.operator, vec![*expr.value]);
    }

    fn visitLiteral(&mut self, expr: Literal) -> String {
        match expr {
            Literal::NUMBER(num) => {
                return num.to_string();
            }
            Literal::STRING(string) => {
                return string.to_string();
            }
            Literal::FALSE => {
                return "false".to_owned();
            }
            Literal::TRUE => {
                return "true".to_owned();
            }
            Literal::NIL => {
                return "nil".to_owned();
            }
        }
    }
    
    fn visitAssignment(&mut self, expr: super::ast::Assignment) -> String {
        todo!()
    }
    
    fn visitVariable(&mut self, expr: super::ast::Variable) -> String {
        todo!()
    }
}

impl ASTPrinter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn print(&mut self, expr: Expr) {
        println!("'{}'", expr.accept::<String>(self));
    }

    pub fn parthesize(&mut self, operator: TokenType, exprs: Vec<Expr>) -> String {
        let mut formatted_string = String::from("( ");
        formatted_string += &format!(" {:?} ", operator);
        for expr in exprs {
            formatted_string += &expr.accept::<String>(self);
            formatted_string += " ";
        }
        formatted_string += " ) ";
        return formatted_string;
    }
}
