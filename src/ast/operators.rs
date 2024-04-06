
#[repr(C)]
#[derive(Clone, Copy)]
pub enum Operators {
    PLUS,
    MINUS,
    MULTIPLY,
    DIVIDE
}


#[repr(C)]
#[derive(Clone, Copy)]
pub enum UnaryOperators {
    NEGATION,
    LOGICAL_NOT
}
