use super::ast::Expr;

pub struct ASTPrinter {}


impl ASTPrinter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn print(&mut self, mut expr: Expr) {
        println!("{:?}", expr.accept(self));
    }
}