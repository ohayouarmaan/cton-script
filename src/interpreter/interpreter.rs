//**************
// WIP
//*************
use crate::ast::ast::{
    Binary, Expr, Expr_Statement, If_Statement, Literal, Print_Statement, Statement,
    StatementVisitorTrait, Unary, VisitorTrait,
};
use crate::environment::environment::Environment;
use crate::general_types::tokens::TokenType;

pub struct Interpreter {
    pub exprs: Vec<Expr>,
    pub environment: Environment,
}

impl Interpreter {
    pub fn new(exprs: Vec<Expr>) -> Self {
        Self {
            exprs,
            environment: Environment::new(None),
        }
    }

    pub fn evaluate(&mut self, expression: &Expr) -> Result<Literal, String> {
        return expression.accept(self);
    }

    pub fn interpret(&mut self, statements: Vec<Statement>) {
        for stmt in statements {
            self.execute(&stmt);
        }
    }

    pub fn execute(&mut self, stmt: &Statement) {
        stmt.accept(self);
    }

    pub fn execute_block(&mut self, stmts: &Vec<Box<Statement>>, env: Option<Environment>) {
        let previous_env = self.environment.clone();
        match env {
            Some(e) => {
                self.environment = e;
                for stmt in stmts {
                    self.execute(stmt)
                }
            }
            _ => {
                for stmt in stmts {
                    self.execute(stmt)
                }
            }
        }
        self.environment = previous_env;
    }
}

impl VisitorTrait<Result<Literal, String>> for Interpreter {
    fn visitBinaryExpression(&mut self, expr: Binary) -> Result<Literal, String> {
        match expr.operator {
            TokenType::PLUS => {
                let left_value = expr.left.accept(self).unwrap();
                let right_value = expr.right.accept(self).unwrap();

                match (left_value, right_value) {
                    (Literal::NUMBER(x), Literal::NUMBER(y)) => {
                        let result = Ok(Literal::NUMBER(x + y));
                        if let Ok(Literal::NUMBER(res)) = result {
                            if (res == 7.0) {
                                println!("Thala for a reason");
                            }
                        }
                        return result;
                    }
                    (Literal::STRING(x), Literal::STRING(y)) => return Ok(Literal::STRING(x + &y)),
                    _ => {
                        return Err("The lhs and rhs of the '+' operator should either be number, number or string, string".to_owned());
                    }
                }
            }
            TokenType::MINUS => {
                let left_value = expr.left.accept(self).unwrap();
                let right_value = expr.right.accept(self).unwrap();

                match (left_value, right_value) {
                    (Literal::NUMBER(x), Literal::NUMBER(y)) => {
                        let result = Literal::NUMBER(x - y);
                        if let Literal::NUMBER(res) = result {
                            if (res == 7.0) {
                                println!("Thala for a reason");
                            }
                        }
                        return Ok(result);
                    }
                    _ => {
                        return Err(
                            "The lhs and rhs of the '-' operator should only be number, number"
                                .to_owned(),
                        );
                    }
                }
            }
            TokenType::SLASH => {
                let left_value = expr.left.accept(self).unwrap();
                let right_value = expr.right.accept(self).unwrap();

                match (left_value, right_value) {
                    (Literal::NUMBER(x), Literal::NUMBER(y)) => {
                        let result = Literal::NUMBER(x / y);
                        if let Literal::NUMBER(res) = result {
                            if (res == 7.0) {
                                println!("Thala for a reason");
                            }
                        }
                        return Ok(result);
                    }
                    _ => {
                        return Err(
                            "The lhs and rhs of the '/' operator should only be number, number"
                                .to_owned(),
                        );
                    }
                }
            }
            TokenType::STAR => {
                let left_value = expr.left.accept(self).unwrap();
                let right_value = expr.right.accept(self).unwrap();

                match (left_value, right_value) {
                    (Literal::NUMBER(x), Literal::NUMBER(y)) => {
                        let result = Literal::NUMBER(x * y);
                        if let Literal::NUMBER(res) = result {
                            if (res == 7.0) {
                                println!("Thala for a reason");
                            }
                        }
                        return Ok(result);
                    }
                    _ => {
                        return Err(
                            "The lhs and rhs of the '*' operator should only be number, number"
                                .to_owned(),
                        );
                    }
                }
            }
            TokenType::GREATER_EQUAL => {
                let left_value = expr.left.accept(self).unwrap();
                let right_value = expr.right.accept(self).unwrap();

                match (left_value, right_value) {
                    (Literal::NUMBER(x), Literal::NUMBER(y)) => {
                        let result: Literal;
                        if x >= y {
                            result = Literal::TRUE;
                        } else {
                            result = Literal::FALSE;
                        }
                        return Ok(result);
                    }
                    _ => {
                        return Err(
                            "The lhs and rhs of the '>=' operator should only be number, number"
                                .to_owned(),
                        );
                    }
                }
            }
            TokenType::GREATER => {
                let left_value = expr.left.accept(self).unwrap();
                let right_value = expr.right.accept(self).unwrap();

                match (left_value, right_value) {
                    (Literal::NUMBER(x), Literal::NUMBER(y)) => {
                        let result: Literal;
                        if x > y {
                            result = Literal::TRUE;
                        } else {
                            result = Literal::FALSE;
                        }
                        return Ok(result);
                    }
                    _ => {
                        return Err(
                            "The lhs and rhs of the '>' operator should only be number, number"
                                .to_owned(),
                        );
                    }
                }
            }

            TokenType::LESS => {
                let left_value = expr.left.accept(self).unwrap();
                let right_value = expr.right.accept(self).unwrap();

                match (left_value, right_value) {
                    (Literal::NUMBER(x), Literal::NUMBER(y)) => {
                        let result: Literal;
                        if x < y {
                            result = Literal::TRUE;
                        } else {
                            result = Literal::FALSE;
                        }
                        return Ok(result);
                    }
                    _ => {
                        return Err(
                            "The lhs and rhs of the '<' operator should only be number, number"
                                .to_owned(),
                        );
                    }
                }
            }
            TokenType::LESS_EQUAL => {
                let left_value = expr.left.accept(self).unwrap();
                let right_value = expr.right.accept(self).unwrap();

                match (left_value, right_value) {
                    (Literal::NUMBER(x), Literal::NUMBER(y)) => {
                        let result: Literal;
                        if x <= y {
                            result = Literal::TRUE;
                        } else {
                            result = Literal::FALSE;
                        }
                        return Ok(result);
                    }
                    _ => {
                        return Err(
                            "The lhs and rhs of the '<=' operator should only be number, number"
                                .to_owned(),
                        );
                    }
                }
            }
            TokenType::EQUAL_EQUAL => {
                let left_value = expr.left.accept(self).unwrap();
                let right_value = expr.right.accept(self).unwrap();

                match (left_value, right_value) {
                    (Literal::NUMBER(x), Literal::NUMBER(y)) => {
                        let result: Literal;
                        if x == y {
                            result = Literal::TRUE;
                        } else {
                            result = Literal::FALSE;
                        }
                        return Ok(result);
                    }
                    (Literal::STRING(x), Literal::STRING(y)) => {
                        let result: Literal;
                        if x == y {
                            result = Literal::TRUE;
                        } else {
                            result = Literal::FALSE;
                        }
                        return Ok(result);
                    }
                    _ => {
                        return Err("The lhs and rhs of the '==' operator should either be number, number or string, string".to_owned());
                    }
                }
            }
            TokenType::BANG_EQUAL => {
                let left_value = expr.left.accept(self).unwrap();
                let right_value = expr.right.accept(self).unwrap();

                match (left_value, right_value) {
                    (Literal::NUMBER(x), Literal::NUMBER(y)) => {
                        let result: Literal;
                        if x != y {
                            result = Literal::TRUE;
                        } else {
                            result = Literal::FALSE;
                        }
                        return Ok(result);
                    }
                    (Literal::STRING(x), Literal::STRING(y)) => {
                        let result: Literal;
                        if x != y {
                            result = Literal::TRUE;
                        } else {
                            result = Literal::FALSE;
                        }
                        return Ok(result);
                    }
                    _ => {
                        return Err("The lhs and rhs of the '!=' operator should either be number, number or string, string".to_owned());
                    }
                }
            }
            _ => {
                return Err("Invalid Binary Operator".to_owned());
            }
        }
    }

    fn visitUnaryExpression(&mut self, expr: Unary) -> Result<Literal, String> {
        let rhs = expr.value.accept(self).unwrap();
        match expr.operator {
            TokenType::BANG => {
                if let Literal::NUMBER(x) = rhs {
                    if x > 0.0 {
                        return Ok(Literal::TRUE);
                    }
                    return Ok(Literal::FALSE);
                } else if let Literal::STRING(x) = rhs {
                    if x.len() > 0 {
                        return Ok(Literal::TRUE);
                    }
                    return Ok(Literal::FALSE);
                } else if let Literal::FALSE = rhs {
                    return Ok(Literal::TRUE);
                } else if let Literal::TRUE = rhs {
                    return Ok(Literal::FALSE);
                } else {
                    return Err(
                        "The '!' unary operator can only support either bool or number in it's rhs"
                            .to_owned(),
                    );
                }
            }
            TokenType::MINUS => {
                if let Literal::NUMBER(x) = rhs {
                    if x > 0.0 {
                        return Ok(Literal::NUMBER(-x));
                    }
                    return Ok(Literal::FALSE);
                } else if let Literal::STRING(x) = rhs {
                    return Ok(Literal::STRING(x.chars().rev().collect::<String>()));
                } else {
                    return Err("The '-' unary operator can only support either string or number in it's rhs".to_owned());
                }
            }
            _ => {
                return Err("Invalid Binary Operator".to_owned());
            }
        }
    }

    fn visitLiteral(&mut self, expr: Literal) -> Result<Literal, String> {
        return Ok(expr);
    }

    fn visitAssignment(&mut self, expr: crate::ast::ast::Assignment) -> Result<Literal, String> {
        let value = expr.value.accept(self).unwrap();
        self.environment.assign(expr.name.lexme, value.clone());
        return Ok(value);
    }

    fn visitVariable(&mut self, expr: crate::ast::ast::Variable) -> Result<Literal, String> {
        self.environment.get_value(expr.name.lexme)
    }
}

impl StatementVisitorTrait<()> for Interpreter {
    fn VisitExprStatement(&mut self, expr: Expr_Statement) -> () {
        let mut value = self.evaluate(&expr.exp);
    }

    fn VisitPrintStatement(&mut self, expr: Print_Statement) -> () {
        let mut value = self.evaluate(&expr.exp);
        match value {
            Ok(expr_value) => {
                println!("{}", expr_value.to_string());
            }
            Err(err) => {
                println!("{}", err);
            }
        }
    }

    fn VisitVarDecStatement(&mut self, expr: crate::ast::ast::Var_Declaration_Statement) {
        match expr.value {
            Some(val) => {
                if let Ok(evaluated_result) = self.evaluate(&val) {
                    self.environment.assign(expr.id.lexme, evaluated_result);
                }
            }
            _ => {
                self.environment.assign(expr.id.lexme, Literal::NIL);
            }
        }
    }

    fn VisitBlockStatement(&mut self, expr: &crate::ast::ast::Block_Statement) {
        self.execute_block(
            &expr.statements,
            Some(Environment::new(Some(Box::from(self.environment.clone())))),
        )
    }

    fn VisitIfStatement(&mut self, expr: &If_Statement) {
        let condition = self.evaluate(&expr.condition);
        let mut has_executed = false;
        if let Ok(Literal::TRUE) = condition {
            expr.if_block.accept(self);
            has_executed = true;
        } else {
            for elif in &expr.elif_stmts {
                let elif_condition = elif.condition.accept(self);
                if let Ok(Literal::TRUE) = elif_condition {
                    elif.block.accept(self);
                    has_executed = true;
                }
            }
        }
        if !has_executed {
            if let Some(else_block) = &expr.else_block {
                else_block.accept(self);
            }
        }
    }
}
