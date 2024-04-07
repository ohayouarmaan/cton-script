//**************
// WIP
//*************
use crate::ast::ast::Expr;

pub struct Interpreter {
    pub exprs: Vec<Expr>
}

impl Interpreter {
    pub fn new(exprs: Vec<Expr>) -> Self {
        Self {
            exprs
        }
    }
    
}