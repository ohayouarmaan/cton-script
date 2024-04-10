use crate::general_types::tokens::{Token, TokenType};

// Expression ASTs
#[derive(Debug, Clone)]
pub enum Expr {
    Binary(Binary),
    Assignment(Assignment),
    Variable(Variable),
    Literal(Literal),
    Unary(Unary),
    Grouping(Box<Expr>),
}

pub trait VisitorTrait<T> {
    fn visitBinaryExpression(&mut self, expr: Binary) -> T;
    fn visitUnaryExpression(&mut self, expr: Unary) -> T;
    fn visitAssignment(&mut self, expr: Assignment) -> T;
    fn visitVariable(&mut self, expr: Variable) -> T;
    fn visitLiteral(&mut self, expr: Literal) -> T;
}

impl Expr {
    pub fn accept<T>(&self, visitor: &mut impl VisitorTrait<T>) -> T {
        match self {
            Expr::Binary(bin) => {
                return visitor.visitBinaryExpression(bin.clone());
            }
            Expr::Assignment(ass) => {
                return visitor.visitAssignment(ass.clone());
            }
            Expr::Variable(var) => {
                return visitor.visitVariable(var.clone());
            }
            Expr::Unary(una) => {
                return visitor.visitUnaryExpression(una.clone());
            }
            Expr::Literal(lit) => {
                return visitor.visitLiteral(lit.clone());
            }
            Expr::Grouping(group) => {
                return group.accept::<T>(visitor);
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum Literal {
    NUMBER(f32),
    STRING(String),
    TRUE,
    FALSE,
    NIL,
}

impl Literal {
    pub fn to_string(&self) -> String {
        match self {
            Literal::NUMBER(n) => {
                return format!("{}", n);
            }
            Literal::STRING(n) => {
                return format!("{}", n);
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
}

#[derive(Debug, Clone)]
pub struct Binary {
    pub left: Box<Expr>,
    pub operator: TokenType,
    pub right: Box<Expr>,
}

impl Binary {
    pub fn accept<T>(&self, ast_printer: &mut impl VisitorTrait<T>) -> T {
        return ast_printer.visitBinaryExpression(self.clone());
    }
}

#[derive(Debug, Clone)]
pub struct Unary {
    pub operator: TokenType,
    pub value: Box<Expr>,
}

impl Unary {
    pub fn accept<T>(&self, ast_printer: &mut impl VisitorTrait<T>) -> T {
        return ast_printer.visitUnaryExpression(self.clone());
    }
}

#[derive(Debug, Clone)]
pub struct Assignment {
    pub name: Token,
    pub value: Box<Expr>
}

#[derive(Debug, Clone)]
pub struct Variable {
    pub name: Token,
}

// Statement AST
#[derive(Debug, Clone)]
pub enum Statement {
    Expr_Statement(Expr_Statement),
    Print_Statement(Print_Statement),
    Var_Declaration_Statement(Var_Declaration_Statement),
    Block_Statement(Block_Statement),
    If_Statement(If_Statement)
}

pub trait StatementVisitorTrait<T> {
    fn VisitExprStatement(&mut self, expr: Expr_Statement);
    fn VisitPrintStatement(&mut self, expr: Print_Statement);
    fn VisitVarDecStatement(&mut self, expr: Var_Declaration_Statement);
    fn VisitBlockStatement(&mut self, expr: &Block_Statement);
    fn VisitIfStatement(&mut self, expr: &If_Statement);
}

impl Statement {
    pub fn accept<T>(&self, visitor: &mut impl StatementVisitorTrait<T>) {
        match self {
            Statement::Expr_Statement(exp) => {
                visitor.VisitExprStatement(exp.clone())
            }
            Statement::Print_Statement(pri) => {
                visitor.VisitPrintStatement(pri.clone())
            }
            Statement::Var_Declaration_Statement(var) => {
                visitor.VisitVarDecStatement(var.clone());
            }
            Statement::Block_Statement(block) => {
                visitor.VisitBlockStatement(&block);
            }
            Statement::If_Statement(ifstmt) => {
                visitor.VisitIfStatement(&ifstmt);
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Expr_Statement {
    pub exp: Expr
}

#[derive(Debug, Clone)]
pub struct Print_Statement {
    pub exp: Expr
}

#[derive(Debug, Clone)]
pub struct Var_Declaration_Statement {
    pub id: Token,
    pub value: Option<Expr>
}

#[derive(Debug, Clone)]
pub struct Block_Statement {
    pub statements: Vec<Box<Statement>>
}

#[derive(Debug, Clone)]
pub struct If_Statement {
    pub condition: Expr,
    pub if_block: Box<Statement>,
    pub elif_stmts: Vec<Box<Elif_Statement>>,
    pub else_block: Option<Box<Statement>>
}

#[derive(Debug, Clone)]
pub struct Elif_Statement {
    pub condition: Expr,
    pub block: Box<Statement>
}