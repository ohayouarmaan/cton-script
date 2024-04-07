use crate::general_types::tokens::TokenType;
use crate::general_types::tokens::Token;
use super::printer::ASTPrinter;

#[derive(Debug, Clone)]
pub enum Expr {
    Binary(Binary),
    Literal(Literal),
    Unary(Unary),
    Grouping(Box<Expr>)
}

impl Expr {
    pub fn accept(&mut self, ast_printer: *mut ASTPrinter) -> String {
        match self {
            Expr::Binary(bin) => {
                let formatted_string = format!("( {:?} {} {} )", bin.operator, bin.left.accept(ast_printer), bin.right.accept(ast_printer));
                return formatted_string;
            }
            Expr::Unary(uni) => {
                let formatted_string = format!("( {:?} {} )", uni.operator, uni.value.accept(ast_printer));
                return formatted_string
            }
            Expr::Literal(lit) => {
                match lit {
                    Literal::NUMBER(val) => {
                        let formatted_string = format!(" {:?} ", (*val).to_string());
                        return formatted_string
                    }
                    Literal::STRING(val) => {
                        let formatted_string = format!(" {:?} ", (*val).to_string());
                        return formatted_string
                    }
                    Literal::FALSE => {
                        let formatted_string = format!(" {:?} ", "false");
                        return formatted_string
                    }
                    Literal::TRUE => {
                        let formatted_string = format!(" {:?} ", "true");
                        return formatted_string
                    }
                    Literal::NIL => {
                        let formatted_string = format!(" {:?} ", "NULL");
                        return formatted_string
                    }
                }
            }
            Expr::Grouping(group) => {
                let formatted_string = format!(" {} ", group.accept(ast_printer));
                return formatted_string
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
