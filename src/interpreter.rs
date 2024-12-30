use crate::{
    environment::Environment,
    expr::{Expr, ExprVisitor, ExprVisitorAcceptor},
    stmt::{Stmt, StmtVisitor, StmtVisitorAcceptor},
    tokens::{TokenLiteral, TokenType},
};

pub struct Interpreter {
    pub environment: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environment: Environment::new(),
        }
    }

    fn evaluate_expr(&mut self, expr: Expr) -> TokenLiteral {
        match expr {
            Expr::Binary(x) => self.evaluate(x),
            Expr::Grouping(x) => self.evaluate(x),
            Expr::Literal(x) => self.evaluate(x),
            Expr::Unary(x) => self.evaluate(x),
            Expr::Variable(x) => self.evaluate(x),
            Expr::Assign(x) => self.evaluate(x),
            Expr::Logical(x) => self.evaluate(x),
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

    fn is_truthy(&self, object: TokenLiteral) -> TokenLiteral {
        match object {
            TokenLiteral::None => TokenLiteral::Bool(false),
            TokenLiteral::Bool(b) => TokenLiteral::Bool(b),
            _ => TokenLiteral::Bool(true),
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

impl Interpreter
where
    Interpreter: ExprVisitor<TokenLiteral>,
{
    pub fn evaluate<A: ExprVisitorAcceptor<TokenLiteral>>(&mut self, expr: A) -> TokenLiteral {
        expr.accept(self)
    }
}

impl ExprVisitor<TokenLiteral> for Interpreter {
    fn visit_assign_expr(&mut self, expr: crate::expr::AssignExpr) -> TokenLiteral {
        let value = self.evaluate_expr(*expr.value);
        self.environment.assign(expr.name, value.clone());
        value
    }

    fn visit_variable_expr(&mut self, expr: crate::expr::VariableExpr) -> TokenLiteral {
        self.environment
            .get(expr.name.lexeme)
            .expect("Undefined variable found")
    }

    fn visit_literal_expr(&mut self, expr: crate::expr::LiteralExpr) -> TokenLiteral {
        expr.value
    }

    fn visit_grouping_expr(&mut self, expr: crate::expr::GroupingExpr) -> TokenLiteral {
        self.evaluate_expr(*expr.expression)
    }

    fn visit_unary_expr(&mut self, expr: crate::expr::UnaryExpr) -> TokenLiteral {
        let right = self.evaluate_expr(*expr.right);

        match expr.operator.token_type {
            // Todo: Negate the bang
            TokenType::Bang => self.is_truthy(right),
            TokenType::Minus => match right {
                TokenLiteral::Number(n) => TokenLiteral::Number(-1.0 * n),
                _ => panic!(),
            },
            _ => panic!(),
        }
    }

    fn visit_binary_expr(&mut self, expr: crate::expr::BinaryExpr) -> TokenLiteral {
        let left = self.evaluate_expr(*expr.left);
        let right = self.evaluate_expr(*expr.right);
        match expr.operator.token_type {
            TokenType::Greater => TokenLiteral::Bool(left > right),
            TokenType::GreaterEqual => TokenLiteral::Bool(left >= right),
            TokenType::Less => TokenLiteral::Bool(left < right),
            TokenType::LessEqual => TokenLiteral::Bool(left <= right),
            TokenType::Minus => left - right,
            TokenType::Plus => left + right,
            TokenType::Slash => left / right,
            TokenType::Star => left * right,
            TokenType::BangEqual => TokenLiteral::Bool(left != right),
            TokenType::EqualEqual => TokenLiteral::Bool(left == right),
            _ => panic!(),
        }
    }

    fn visit_logical_expr(&mut self, expr: crate::expr::LogicalExpr) -> TokenLiteral {
        let left = self.evaluate_expr(*expr.left);
        if expr.operator.token_type == TokenType::Or {
            if let TokenLiteral::Bool(true) = self.is_truthy(left.clone()) {
                return left;
            }
        } else if let TokenLiteral::Bool(false) = self.is_truthy(left.clone()) {
            return left;
        }
        self.evaluate_expr(*expr.right)
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
            None => TokenLiteral::None,
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
            TokenLiteral::Bool(true) => self.execute_stmt(*stmt.then_branch),
            TokenLiteral::Bool(false) => {
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
                TokenLiteral::Bool(true) => self.execute_stmt(*stmt.body.clone()),
                TokenLiteral::Bool(false) => break,
                _ => panic!(),
            }
        }
    }
}
