use crate::{
    callable::{LoxCallable, LoxFunction},
    environment::Environment,
    exceptions::Return,
    expr::{Expr, ExprVisitor, ExprVisitorAcceptor},
    stmt::{Stmt, StmtVisitor, StmtVisitorAcceptor},
    tokens::{LoxObject, TokenType},
};

#[derive(Clone)]
pub struct Interpreter {
    pub environment: Environment,
    pub globals: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            environment: Environment::new(),
            globals: Environment::new(),
        }
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

    fn execute_stmt(&mut self, statement: Stmt) -> Result<LoxObject, Return> {
        match statement {
            Stmt::Expression(x) => self.execute(x),
            Stmt::Print(x) => self.execute(x),
            Stmt::Var(x) => self.execute(x),
            Stmt::Block(x) => self.execute(x),
            Stmt::If(x) => self.execute(x),
            Stmt::While(x) => self.execute(x),
            Stmt::Fun(x) => self.execute(x),
            Stmt::Return(x) => self.execute(x),
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
            let _ = self.execute_stmt(statement);
        }
    }

    pub fn execute_block(
        &mut self,
        statements: Vec<Stmt>,
        environment: Environment,
    ) -> Result<(), Return> {
        let previous = self.environment.clone();
        self.environment = environment;
        let mut response = Ok(());
        for stmt in statements {
            if let Err(x) = self.execute_stmt(stmt) {
                response = Err(x);
                break;
            }
        }
        self.environment = previous;
        response
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

    fn visit_literal_expr(&mut self, expr: crate::expr::LiteralExpr) -> LoxObject {
        expr.value
    }

    fn visit_variable_expr(&mut self, expr: crate::expr::VariableExpr) -> LoxObject {
        self.environment
            .get(expr.name.lexeme.clone())
            .unwrap_or_else(|_| panic!("Undefined variable found: {}", &expr.name.lexeme))
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
            if arguments.len() != function.arity() as usize {
                panic!("Unexpected number of arguments received");
            }
            return function.call(self, arguments);
        } else {
            panic!("Expression not of type LoxCallable")
        }
    }
}

impl Interpreter
where
    Interpreter: StmtVisitor<LoxObject>,
{
    fn execute<A: StmtVisitorAcceptor<LoxObject>>(&mut self, stmt: A) -> Result<LoxObject, Return> {
        stmt.accept(self)
    }
}

impl StmtVisitor<LoxObject> for Interpreter {
    fn visit_expression_stmt(
        &mut self,
        stmt: crate::stmt::ExpressionStmt,
    ) -> Result<LoxObject, Return> {
        Ok(self.evaluate_expr(stmt.expression))
    }

    fn visit_print_stmt(&mut self, stmt: crate::stmt::PrintStmt) -> Result<LoxObject, Return> {
        let value = self.evaluate_expr(stmt.expression);
        println!("{}", value);
        Ok(LoxObject::None)
    }

    fn visit_var_stmt(&mut self, stmt: crate::stmt::VarStmt) -> Result<LoxObject, Return> {
        let value = match stmt.initializer {
            Some(f) => self.evaluate_expr(f),
            None => LoxObject::None,
        };

        self.environment.define(stmt.name.lexeme, value);
        Ok(LoxObject::None)
    }

    fn visit_block_stmt(&mut self, stmt: crate::stmt::BlockStmt) -> Result<LoxObject, Return> {
        self.execute_block(stmt.statements, self.environment.clone())?;
        // TODO: Cloning the environment here is leading to weird behaviour where assign values to
        // variables inside a while block is not being reflected outside it
        Ok(LoxObject::None)
    }

    fn visit_if_stmt(&mut self, stmt: crate::stmt::IfStmt) -> Result<LoxObject, Return> {
        let value = self.evaluate_expr(stmt.condition);
        match self.is_truthy(value) {
            LoxObject::Bool(true) => self.execute_stmt(*stmt.then_branch)?,
            LoxObject::Bool(false) => {
                if let Some(s) = *stmt.else_branch {
                    self.execute_stmt(s)?
                } else {
                    LoxObject::None
                }
            }
            _ => panic!(),
        };
        Ok(LoxObject::None)
    }

    fn visit_while_stmt(&mut self, stmt: crate::stmt::WhileStmt) -> Result<LoxObject, Return> {
        loop {
            let value = self.evaluate_expr(stmt.condition.clone());
            match self.is_truthy(value) {
                LoxObject::Bool(true) => self.execute_stmt(*stmt.body.clone()).unwrap(),
                LoxObject::Bool(false) => break Ok(LoxObject::None),
                _ => panic!(),
            };
        }
    }

    fn visit_fun_stmt(&mut self, stmt: crate::stmt::FunStmt) -> Result<LoxObject, Return> {
        let fun_name = stmt.name.lexeme.clone();
        let function = LoxFunction::new(stmt);
        self.environment
            .define(fun_name, LoxObject::Callable(Box::new(function)));
        Ok(LoxObject::None)
    }

    fn visit_return_stmt(&mut self, stmt: crate::stmt::ReturnStmt) -> Result<LoxObject, Return> {
        let mut output = LoxObject::None;
        if let Some(value) = *stmt.value {
            output = self.evaluate_expr(value)
        }
        Err(Return { value: output })
    }
}
