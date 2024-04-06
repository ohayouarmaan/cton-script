// use crate::general_types::tokens;
// **********************************************
// WIP
// **********************************************
use crate::ast::operators;

#[derive(Clone, Copy)]
pub enum ExprType {
    Binary,
    Literal,
    Unary,
    Grouping
}

// #[derive(Clone, Copy)]
pub struct Expr {
    _type: ExprType,
    value: ExprField
}


#[derive(Clone, Copy)]
pub enum Literal {
    NUMBER,
    STRING,
    TRUE,
    FALSE,
    NIL
}

// #[derive(Clone, Copy)]
pub struct Binary {
    left: Box<Expr>,
    operator: operators::Operators,
    right: Box<Expr>
}

// #[derive(Clone, Copy)]
pub struct Unary {
    operator: operators::UnaryOperators,
    value: Box<Expr>
}

// #[derive(Clone, Copy)]
pub union ExprField {
    Bin: std::mem::ManuallyDrop<Binary>,
    Lit: Literal,
    Unary: std::mem::ManuallyDrop<Unary>
}
