use std::time::{SystemTime, UNIX_EPOCH};

use crate::{
    callable::LoxCallable,
    environment::Environment,
    expr::{Expr, ExprVisitor, ExprVisitorAcceptor},
    stmt::{Stmt, StmtVisitor, StmtVisitorAcceptor},
    tokens::{LoxObject, TokenType},
};

pub struct Interpreter {
    pub environment: Environment,
    pub globals: Environment,
}

fn get_system_time(_: Vec<LoxObject>) -> LoxObject {
    LoxObject::Number(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as f32,
    )
}

impl Interpreter {
    pub fn new() -> Self {
        let mut environment = Interpreter {
            environment: Environment::new(),
            globals: Environment::new(),
        };

        let clock = LoxObject::Callable(Box::new(LoxCallable::new(
            get_system_time,
            0,
            "<native fn>".to_string(),
        )));
        environment.globals.define("clock".to_string(), clock);

        environment
    }

    fn evaluate_expr(&mut self, expr: Expr) -> LoxObject {
        match expr {
            Expr::Binary(x) => self.evaluate(x),
            Expr::Grouping(x) => self.evaluate(x),
            Expr::Literal(x) => self.evaluate(x),
            Expr::Unary(x) => self.evaluate(x),
            Expr::Variable(x) => self.evaluate(x),
            Expr::Assign(x) => self.evaluate(x),
            Expr::Logical(x) => self.evaluate(x),
            Expr::Call(x) => self.evaluate(x),
        }
    }

    fn execute_stmt(&mut self, statement: Stmt) {
        match statement {
            Stmt::Expression(x) => self.execute(x),
            Stmt::Print(x) => self.execute(x),
            Stmt::Var(x) => self.execute(x),
            Stmt::Block(x) => self.execute(x),
            Stmt::If(x) => self.execute(x),
            Stmt::While(x) => self.execute(x),
        }
    }

    fn is_truthy(&self, object: LoxObject) -> LoxObject {
        match object {
            LoxObject::None => LoxObject::Bool(false),
            LoxObject::Bool(b) => LoxObject::Bool(b),
            _ => LoxObject::Bool(true),
        }
    }
    pub fn interpret(mut self, statements: Vec<Stmt>) {
        for statement in statements {
            self.execute_stmt(statement)
        }
    }

    fn execute_block(&mut self, statements: Vec<Stmt>, environment: Environment) {
        let previous = environment.clone();
        self.environment = environment;
        for stmt in statements {
            self.execute_stmt(stmt);
        }
        self.environment = previous;
    }
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}

impl Interpreter
where
    Interpreter: ExprVisitor<LoxObject>,
{
    pub fn evaluate<A: ExprVisitorAcceptor<LoxObject>>(&mut self, expr: A) -> LoxObject {
        expr.accept(self)
    }
}

impl ExprVisitor<LoxObject> for Interpreter {
    fn visit_assign_expr(&mut self, expr: crate::expr::AssignExpr) -> LoxObject {
        let value = self.evaluate_expr(*expr.value);
        self.environment.assign(expr.name.lexeme, value.clone());
        value
    }

    fn visit_variable_expr(&mut self, expr: crate::expr::VariableExpr) -> LoxObject {
        self.environment
            .get(expr.name.lexeme)
            .expect("Undefined variable found")
    }

    fn visit_literal_expr(&mut self, expr: crate::expr::LiteralExpr) -> LoxObject {
        expr.value
    }

    fn visit_grouping_expr(&mut self, expr: crate::expr::GroupingExpr) -> LoxObject {
        self.evaluate_expr(*expr.expression)
    }

    fn visit_unary_expr(&mut self, expr: crate::expr::UnaryExpr) -> LoxObject {
        let right = self.evaluate_expr(*expr.right);

        match expr.operator.token_type {
            // Todo: Negate the bang
            TokenType::Bang => self.is_truthy(right),
            TokenType::Minus => match right {
                LoxObject::Number(n) => LoxObject::Number(-1.0 * n),
                _ => panic!(),
            },
            _ => panic!(),
        }
    }

    fn visit_binary_expr(&mut self, expr: crate::expr::BinaryExpr) -> LoxObject {
        let left = self.evaluate_expr(*expr.left);
        let right = self.evaluate_expr(*expr.right);
        match expr.operator.token_type {
            TokenType::Greater => LoxObject::Bool(left > right),
            TokenType::GreaterEqual => LoxObject::Bool(left >= right),
            TokenType::Less => LoxObject::Bool(left < right),
            TokenType::LessEqual => LoxObject::Bool(left <= right),
            TokenType::Minus => left - right,
            TokenType::Plus => left + right,
            TokenType::Slash => left / right,
            TokenType::Star => left * right,
            TokenType::BangEqual => LoxObject::Bool(left != right),
            TokenType::EqualEqual => LoxObject::Bool(left == right),
            _ => panic!(),
        }
    }

    fn visit_logical_expr(&mut self, expr: crate::expr::LogicalExpr) -> LoxObject {
        let left = self.evaluate_expr(*expr.left);
        if expr.operator.token_type == TokenType::Or {
            if let LoxObject::Bool(true) = self.is_truthy(left.clone()) {
                return left;
            }
        } else if let LoxObject::Bool(false) = self.is_truthy(left.clone()) {
            return left;
        }
        self.evaluate_expr(*expr.right)
    }

    fn visit_call_expr(&mut self, expr: crate::expr::CallExpr) -> LoxObject {
        let callee = self.evaluate_expr(*expr.callee);

        let mut arguments = Vec::new();
        for argument in expr.arguments {
            arguments.push(self.evaluate_expr(argument));
        }

        if let LoxObject::Callable(function) = callee {
            if arguments.len() != function.arity {
                panic!("Unexpected number of arguments received");
            }
            (function.code)(arguments.clone());
        } else {
            panic!("Expression not of type LoxCallable")
        }

        LoxObject::None
    }
}

impl Interpreter
where
    Interpreter: StmtVisitor<()>,
{
    fn execute<A: StmtVisitorAcceptor<()>>(&mut self, stmt: A) {
        stmt.accept(self)
    }
}

impl StmtVisitor<()> for Interpreter {
    fn visit_expression_stmt(&mut self, stmt: crate::stmt::ExpressionStmt) {
        self.evaluate_expr(stmt.expression);
    }

    fn visit_print_stmt(&mut self, stmt: crate::stmt::PrintStmt) {
        let value = self.evaluate_expr(stmt.expression);
        println!("{}", value);
    }

    fn visit_var_stmt(&mut self, stmt: crate::stmt::VarStmt) {
        let value = match stmt.initializer {
            Some(f) => self.evaluate_expr(f),
            None => LoxObject::None,
        };

        self.environment.define(stmt.name.lexeme, value);
    }

    fn visit_block_stmt(&mut self, stmt: crate::stmt::BlockStmt) {
        self.execute_block(stmt.statements, self.environment.clone())
        // TODO: Cloning the environment here is leading to weird behaviour where assign values to
        // variables inside a while block is not being reflected outside it
    }

    fn visit_if_stmt(&mut self, stmt: crate::stmt::IfStmt) {
        let value = self.evaluate_expr(stmt.condition);
        match self.is_truthy(value) {
            LoxObject::Bool(true) => self.execute_stmt(*stmt.then_branch),
            LoxObject::Bool(false) => {
                if let Some(s) = *stmt.else_branch {
                    self.execute_stmt(s)
                }
            }
            _ => panic!(),
        }
    }

    fn visit_while_stmt(&mut self, stmt: crate::stmt::WhileStmt) {
        loop {
            let value = self.evaluate_expr(stmt.condition.clone());
            match self.is_truthy(value) {
                LoxObject::Bool(true) => self.execute_stmt(*stmt.body.clone()),
                LoxObject::Bool(false) => break,
                _ => panic!(),
            }
        }
    }
}
