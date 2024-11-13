use std::fmt::Display;

use crate::expr::Expr;

pub trait StmtVisitor<T> {
    fn visit_expression_stmt(&self, stmt: ExpressionStmt) -> T;
    fn visit_print_stmt(&self, stmt: PrintStmt) -> T;
}

pub trait StmtVisitorAcceptor<T> {
    fn accept(&self, visitor: &impl StmtVisitor<T>) -> T;
}

#[derive(Clone, Debug)]
pub enum Stmt {
    Expression(ExpressionStmt),
    Print(PrintStmt),
}

impl Display for Stmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Expression(x) => write!(f, "{}", x.expression),
            Self::Print(x) => write!(f, "{}", x.expression),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ExpressionStmt {
    pub expression: Box<Expr>,
}

impl ExpressionStmt {
    pub fn new(expression: Expr) -> Self {
        ExpressionStmt {
            expression: Box::new(expression),
        }
    }
}

impl<T> StmtVisitorAcceptor<T> for ExpressionStmt {
    fn accept(&self, visitor: &impl StmtVisitor<T>) -> T {
        visitor.visit_expression_stmt(self.clone())
    }
}

#[derive(Clone, Debug)]
pub struct PrintStmt {
    pub expression: Box<Expr>,
}

impl PrintStmt {
    pub fn new(expression: Expr) -> Self {
        PrintStmt {
            expression: Box::new(expression),
        }
    }
}

impl<T> StmtVisitorAcceptor<T> for PrintStmt {
    fn accept(&self, visitor: &impl StmtVisitor<T>) -> T {
        visitor.visit_print_stmt(self.clone())
    }
}
