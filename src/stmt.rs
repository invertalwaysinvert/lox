use std::fmt::Display;

use crate::{expr::Expr, tokens::Token};

pub trait StmtVisitor<T> {
    fn visit_expression_stmt(&mut self, stmt: ExpressionStmt) -> T;
    fn visit_print_stmt(&mut self, stmt: PrintStmt) -> T;
    fn visit_var_stmt(&mut self, stmt: VarStmt) -> T;
    fn visit_block_stmt(&mut self, stmt: BlockStmt) -> T;
    fn visit_if_stmt(&mut self, stmt: IfStmt) -> T;
    fn visit_while_stmt(&mut self, stmt: WhileStmt) -> T;
}

pub trait StmtVisitorAcceptor<T> {
    fn accept(&self, visitor: &mut impl StmtVisitor<T>) -> T;
}

#[derive(Clone, Debug)]
pub enum Stmt {
    Expression(ExpressionStmt),
    Print(PrintStmt),
    Var(VarStmt),
    Block(BlockStmt),
    If(IfStmt),
    While(WhileStmt),
}

impl Display for Stmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Expression(x) => write!(f, "{}", x.expression),
            Self::Print(x) => write!(f, "{}", x.expression),
            Self::Var(x) => {
                if let Some(y) = x.initializer.clone() {
                    write!(f, "{}", y)
                } else {
                    write!(f, "")
                }
            } // TODO: This is wrong, fix it
            Self::Block(x) => {
                write!(f, "{:?}", x)
            }
            Self::If(x) => {
                write!(f, "{:?}", x)
            }
            Self::While(x) => {
                write!(f, "{:?}", x)
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct ExpressionStmt {
    pub expression: Expr,
}

impl ExpressionStmt {
    pub fn new(expression: Expr) -> Self {
        ExpressionStmt { expression }
    }
}

impl<T> StmtVisitorAcceptor<T> for ExpressionStmt {
    fn accept(&self, visitor: &mut impl StmtVisitor<T>) -> T {
        visitor.visit_expression_stmt(self.clone())
    }
}

#[derive(Clone, Debug)]
pub struct PrintStmt {
    pub expression: Expr,
}

impl PrintStmt {
    pub fn new(expression: Expr) -> Self {
        PrintStmt { expression }
    }
}

impl<T> StmtVisitorAcceptor<T> for PrintStmt {
    fn accept(&self, visitor: &mut impl StmtVisitor<T>) -> T {
        visitor.visit_print_stmt(self.clone())
    }
}

#[derive(Clone, Debug)]
pub struct VarStmt {
    pub name: Token,
    pub initializer: Option<Expr>,
}

impl VarStmt {
    pub fn new(name: Token, initializer: Option<Expr>) -> Self {
        match initializer {
            Some(x) => VarStmt {
                name,
                initializer: Some(x),
            },
            None => VarStmt {
                name,
                initializer: None,
            },
        }
    }
}

impl<T> StmtVisitorAcceptor<T> for VarStmt {
    fn accept(&self, visitor: &mut impl StmtVisitor<T>) -> T {
        visitor.visit_var_stmt(self.clone())
    }
}

#[derive(Clone, Debug)]
pub struct BlockStmt {
    pub statements: Vec<Stmt>,
}

impl BlockStmt {
    pub fn new(statements: Vec<Stmt>) -> Self {
        BlockStmt { statements }
    }
}

impl<T> StmtVisitorAcceptor<T> for BlockStmt {
    fn accept(&self, visitor: &mut impl StmtVisitor<T>) -> T {
        visitor.visit_block_stmt(self.clone())
    }
}

#[derive(Clone, Debug)]
pub struct IfStmt {
    pub condition: Expr,
    pub then_branch: Box<Stmt>,
    pub else_branch: Box<Option<Stmt>>,
}

impl IfStmt {
    pub fn new(condition: Expr, then_branch: Stmt, else_branch: Option<Stmt>) -> Self {
        IfStmt {
            condition,
            then_branch: Box::new(then_branch),
            else_branch: Box::new(else_branch),
        }
    }
}

impl<T> StmtVisitorAcceptor<T> for IfStmt {
    fn accept(&self, visitor: &mut impl StmtVisitor<T>) -> T {
        visitor.visit_if_stmt(self.clone())
    }
}

#[derive(Clone, Debug)]
pub struct WhileStmt {
    pub condition: Expr,
    pub body: Box<Stmt>,
}

impl WhileStmt {
    pub fn new(condition: Expr, body: Stmt) -> Self {
        WhileStmt {
            condition,
            body: Box::new(body),
        }
    }
}

impl<T> StmtVisitorAcceptor<T> for WhileStmt {
    fn accept(&self, visitor: &mut impl StmtVisitor<T>) -> T {
        visitor.visit_while_stmt(self.clone())
    }
}
