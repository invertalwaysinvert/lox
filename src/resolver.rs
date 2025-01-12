use std::collections::HashMap;

use crate::{
    expr::{Expr, ExprVisitor, ExprVisitorAcceptor},
    interpreter::Interpreter,
    stmt::{FunStmt, Stmt, StmtVisitor, StmtVisitorAcceptor},
    tokens::Token,
};

pub struct Resolver<'a> {
    interpreter: &'a mut Interpreter,
    scopes: Vec<HashMap<String, bool>>,
}

impl<'a> Resolver<'a> {
    pub fn new(interpreter: &'a mut Interpreter) -> Resolver {
        Resolver {
            interpreter,
            scopes: Vec::new(),
        }
    }

    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }

    fn declare(&mut self, name: &Token) {
        if self.scopes.is_empty() {
            return;
        }
        if let Some(innermost) = self.scopes.last_mut() {
            innermost.insert(name.lexeme.clone(), false);
        }
    }

    fn define(&mut self, name: &Token) {
        if self.scopes.is_empty() {
            return;
        }
        if let Some(innermost) = self.scopes.last_mut() {
            innermost.insert(name.lexeme.clone(), true);
        }
    }

    fn resolve_local(&mut self, expr: Expr, name: Token) {
        for (i, scope) in self.scopes.clone().into_iter().rev().enumerate() {
            if scope.contains_key(&name.lexeme) {
                self.interpreter
                    .resolve(name.clone(), self.scopes.len() - 1 - i);
                return;
            };
        }
    }

    fn evaluate_expr(&mut self, expr: Expr) {
        match expr {
            Expr::Binary(x) => self.resolve_expr(x),
            Expr::Grouping(x) => self.resolve_expr(x),
            Expr::Literal(x) => self.resolve_expr(x),
            Expr::Unary(x) => self.resolve_expr(x),
            Expr::Variable(x) => self.resolve_expr(x),
            Expr::Assign(x) => self.resolve_expr(x),
            Expr::Logical(x) => self.resolve_expr(x),
            Expr::Call(x) => self.resolve_expr(x),
        }
    }

    fn execute_stmt(&mut self, statement: Stmt) {
        match statement {
            Stmt::Expression(x) => self.resolve_statement(x),
            Stmt::Print(x) => self.resolve_statement(x),
            Stmt::Var(x) => self.resolve_statement(x),
            Stmt::Block(x) => self.resolve_statement(x),
            Stmt::If(x) => self.resolve_statement(x),
            Stmt::While(x) => self.resolve_statement(x),
            Stmt::Fun(x) => self.resolve_statement(x),
            Stmt::Return(x) => self.resolve_statement(x),
        }
    }

    fn resolve_function(&mut self, function: FunStmt) {
        self.begin_scope();
        for param in function.params {
            self.declare(&param);
            self.define(&param);
        }
        self.resolve_statements(function.body);
        self.end_scope();
    }
}

impl<'a> Resolver<'a>
where
    Resolver<'a>: ExprVisitor<()>,
{
    pub fn resolve_expr<A: ExprVisitorAcceptor<()>>(&mut self, expr: A) {
        expr.accept(self)
    }
}

impl<'a> Resolver<'a>
where
    Resolver<'a>: StmtVisitor<()>,
{
    pub fn resolve_statement<A: StmtVisitorAcceptor<()>>(&mut self, statement: A) {
        let _ = statement.accept(self);
    }

    pub fn resolve_statements(&mut self, statements: Vec<Stmt>) {
        for statement in statements {
            self.execute_stmt(statement)
        }
    }
}

impl<'a> StmtVisitor<()> for Resolver<'a> {
    fn visit_block_stmt(
        &mut self,
        stmt: crate::stmt::BlockStmt,
    ) -> Result<(), crate::exceptions::Return> {
        self.begin_scope();
        self.resolve_statements(stmt.statements);
        self.end_scope();
        Ok(())
    }

    fn visit_var_stmt(
        &mut self,
        stmt: crate::stmt::VarStmt,
    ) -> Result<(), crate::exceptions::Return> {
        self.declare(&stmt.name);
        if let Some(expr) = stmt.initializer {
            self.evaluate_expr(expr)
        };
        self.define(&stmt.name);
        Ok(())
    }

    fn visit_fun_stmt(
        &mut self,
        stmt: crate::stmt::FunStmt,
    ) -> Result<(), crate::exceptions::Return> {
        self.declare(&stmt.name);
        self.define(&stmt.name);
        self.resolve_function(stmt);
        Ok(())
    }

    fn visit_expression_stmt(
        &mut self,
        stmt: crate::stmt::ExpressionStmt,
    ) -> Result<(), crate::exceptions::Return> {
        self.evaluate_expr(stmt.expression);
        Ok(())
    }

    fn visit_if_stmt(
        &mut self,
        stmt: crate::stmt::IfStmt,
    ) -> Result<(), crate::exceptions::Return> {
        self.evaluate_expr(stmt.condition);
        self.execute_stmt(*stmt.then_branch);
        if let Some(else_branch) = *stmt.else_branch {
            self.execute_stmt(else_branch);
        }
        Ok(())
    }

    fn visit_print_stmt(
        &mut self,
        stmt: crate::stmt::PrintStmt,
    ) -> Result<(), crate::exceptions::Return> {
        self.evaluate_expr(stmt.expression);
        Ok(())
    }

    fn visit_return_stmt(
        &mut self,
        stmt: crate::stmt::ReturnStmt,
    ) -> Result<(), crate::exceptions::Return> {
        if let Some(value) = *stmt.value {
            self.evaluate_expr(value);
        }
        Ok(())
    }

    fn visit_while_stmt(
        &mut self,
        stmt: crate::stmt::WhileStmt,
    ) -> Result<(), crate::exceptions::Return> {
        self.evaluate_expr(stmt.condition);
        self.execute_stmt(*stmt.body);
        Ok(())
    }
}

impl<'a> ExprVisitor<()> for Resolver<'a> {
    fn visit_variable_expr(&mut self, expr: crate::expr::VariableExpr) {
        if let Some(innermost) = self.scopes.last() {
            if let Some(false) = innermost.get(&expr.name.lexeme) {
                panic!("Can't read local variable in it's own initializer");
            }
        }
        self.resolve_local(Expr::Variable(expr.clone()), expr.name);
    }

    fn visit_assign_expr(&mut self, expr: crate::expr::AssignExpr) {
        self.evaluate_expr(*expr.value.clone());
        self.resolve_local(Expr::Assign(expr.clone()), expr.name);
    }

    fn visit_binary_expr(&mut self, expr: crate::expr::BinaryExpr) {
        self.evaluate_expr(*expr.left);
        self.evaluate_expr(*expr.right);
    }

    fn visit_call_expr(&mut self, expr: crate::expr::CallExpr) {
        self.evaluate_expr(*expr.callee);

        for arg in expr.arguments {
            self.evaluate_expr(arg);
        }
    }

    fn visit_grouping_expr(&mut self, expr: crate::expr::GroupingExpr) {
        self.evaluate_expr(*expr.expression);
    }

    fn visit_literal_expr(&mut self, _expr: crate::expr::LiteralExpr) {}

    fn visit_logical_expr(&mut self, expr: crate::expr::LogicalExpr) {
        self.evaluate_expr(*expr.left);
        self.evaluate_expr(*expr.right);
    }

    fn visit_unary_expr(&mut self, expr: crate::expr::UnaryExpr) {
        self.evaluate_expr(*expr.right);
    }
}
