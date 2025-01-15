use std::collections::HashMap;

use crate::{
    expr::{Expr, ExprVisitor, ExprVisitorAcceptor},
    interpreter::Interpreter,
    stmt::{FunStmt, Stmt, StmtVisitor, StmtVisitorAcceptor},
    tokens::Token,
};

#[derive(Clone)]
enum FunctionType {
    None,
    Function,
    Method,
    Init,
}

#[derive(Clone)]
enum ClassType {
    None,
    Class,
}

pub struct Resolver<'a> {
    interpreter: &'a mut Interpreter,
    scopes: Vec<HashMap<String, bool>>,
    current_function: FunctionType,
    current_class: ClassType,
}

impl<'a> Resolver<'a> {
    pub fn new(interpreter: &'a mut Interpreter) -> Resolver {
        Resolver {
            interpreter,
            scopes: Vec::new(),
            current_function: FunctionType::None,
            current_class: ClassType::None,
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

    fn resolve_local(&mut self, name: Token) {
        for (i, scope) in self.scopes.clone().into_iter().rev().enumerate() {
            if scope.contains_key(&name.to_string()) {
                self.interpreter.resolve(name, self.scopes.len() - 1 - i);
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
            Expr::Get(x) => self.resolve_expr(x),
            Expr::Set(x) => self.resolve_expr(x),
            Expr::This(x) => self.resolve_expr(x),
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
            Stmt::Class(x) => self.resolve_statement(x),
        }
    }

    fn resolve_function(&mut self, function: FunStmt, function_type: FunctionType) {
        let enclosing_function = self.current_function.clone();
        self.current_function = function_type;
        self.begin_scope();
        for param in function.params {
            self.declare(&param);
            self.define(&param);
        }
        self.resolve_statements(function.body);
        self.end_scope();
        self.current_function = enclosing_function;
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
        self.resolve_function(stmt, FunctionType::Function);
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
        match self.current_function {
            FunctionType::None => panic!("Can't return from top level"),
            FunctionType::Function => (),
            FunctionType::Method => (),
            FunctionType::Init => panic!("Can't return from init"),
        }

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

    fn visit_class_stmt(
        &mut self,
        stmt: crate::stmt::ClassStmt,
    ) -> Result<(), crate::exceptions::Return> {
        let enclosing_class = self.current_class.clone();
        self.current_class = ClassType::Class;

        self.declare(&stmt.name);
        self.define(&stmt.name);

        self.begin_scope();
        self.scopes
            .last_mut()
            .unwrap()
            .insert("This this ".to_string(), true);

        for method in stmt.methods {
            if let Stmt::Fun(stmt) = method {
                let mut declaration = FunctionType::Method;
                if stmt.name.lexeme.eq("init") {
                    declaration = FunctionType::Init;
                }
                self.resolve_function(stmt, declaration)
            } else {
                panic!("Invalid method found!")
            }
        }

        self.end_scope();

        self.current_class = enclosing_class;

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
        self.resolve_local(expr.name);
    }

    fn visit_assign_expr(&mut self, expr: crate::expr::AssignExpr) {
        self.evaluate_expr(*expr.value.clone());
        self.resolve_local(expr.name);
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

    fn visit_get_expr(&mut self, expr: crate::expr::GetExpr) {
        self.evaluate_expr(*expr.object);
    }

    fn visit_set_expr(&mut self, expr: crate::expr::SetExpr) {
        self.evaluate_expr(*expr.value);
        self.evaluate_expr(*expr.object);
    }

    fn visit_this_expr(&mut self, expr: crate::expr::ThisExpr) {
        if let ClassType::None = self.current_class {
            panic!("this not allowed outside a class");
        }
        self.resolve_local(expr.name)
    }
}
