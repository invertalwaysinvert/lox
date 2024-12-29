use std::fmt::Display;

use crate::tokens::{Token, TokenLiteral};

pub trait ExprVisitor<T> {
    fn visit_binary_expr(&mut self, expr: BinaryExpr) -> T;
    fn visit_grouping_expr(&mut self, expr: GroupingExpr) -> T;
    fn visit_literal_expr(&mut self, expr: LiteralExpr) -> T;
    fn visit_unary_expr(&mut self, expr: UnaryExpr) -> T;
    fn visit_variable_expr(&mut self, expr: VariableExpr) -> T;
    fn visit_assign_expr(&mut self, expr: AssignExpr) -> T;
    fn visit_logical_expr(&mut self, expr: LogicalExpr) -> T;
}

pub trait ExprVisitorAcceptor<T> {
    fn accept(&self, visitor: &mut impl ExprVisitor<T>) -> T;
}

#[derive(Clone, Debug)]
pub enum Expr {
    Binary(BinaryExpr),
    Grouping(GroupingExpr),
    Literal(LiteralExpr),
    Unary(UnaryExpr),
    Variable(VariableExpr),
    Assign(AssignExpr),
    Logical(LogicalExpr),
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Literal(x) => write!(f, "{}", x.value),
            Self::Grouping(x) => write!(f, "( group {} )", x.expression),
            _ => write!(f, ""),
        }
    }
}

#[derive(Clone, Debug)]
pub struct BinaryExpr {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

impl BinaryExpr {
    pub fn new(left: Expr, operator: Token, right: Expr) -> Self {
        BinaryExpr {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }
}

impl<T> ExprVisitorAcceptor<T> for BinaryExpr {
    fn accept(&self, visitor: &mut impl ExprVisitor<T>) -> T {
        visitor.visit_binary_expr(self.clone())
    }
}

#[derive(Clone, Debug)]
pub struct GroupingExpr {
    pub expression: Box<Expr>,
}

impl GroupingExpr {
    pub fn new(expression: Expr) -> Self {
        GroupingExpr {
            expression: Box::new(expression),
        }
    }
}

impl<T> ExprVisitorAcceptor<T> for GroupingExpr {
    fn accept(&self, visitor: &mut impl ExprVisitor<T>) -> T {
        visitor.visit_grouping_expr(self.clone())
    }
}

#[derive(Clone, Debug)]
pub struct LiteralExpr {
    pub value: TokenLiteral,
}

impl LiteralExpr {
    pub fn new(value: TokenLiteral) -> Self {
        LiteralExpr { value }
    }
}

impl<T> ExprVisitorAcceptor<T> for LiteralExpr {
    fn accept(&self, visitor: &mut impl ExprVisitor<T>) -> T {
        visitor.visit_literal_expr(self.clone())
    }
}

#[derive(Clone, Debug)]
pub struct UnaryExpr {
    pub right: Box<Expr>,
    pub operator: Token,
}

impl UnaryExpr {
    pub fn new(right: Expr, operator: Token) -> Self {
        UnaryExpr {
            right: Box::new(right),
            operator,
        }
    }
}

impl<T> ExprVisitorAcceptor<T> for UnaryExpr {
    fn accept(&self, visitor: &mut impl ExprVisitor<T>) -> T {
        visitor.visit_unary_expr(self.clone())
    }
}

#[derive(Debug, Clone)]
pub struct VariableExpr {
    pub name: Token,
}

impl VariableExpr {
    pub fn new(name: Token) -> Self {
        VariableExpr { name }
    }
}

impl<T> ExprVisitorAcceptor<T> for VariableExpr {
    fn accept(&self, visitor: &mut impl ExprVisitor<T>) -> T {
        visitor.visit_variable_expr(self.clone())
    }
}

#[derive(Debug, Clone)]
pub struct AssignExpr {
    pub name: Token,
    pub value: Box<Expr>,
}

impl AssignExpr {
    pub fn new(name: Token, value: Expr) -> Self {
        AssignExpr {
            name,
            value: Box::new(value),
        }
    }
}

impl<T> ExprVisitorAcceptor<T> for AssignExpr {
    fn accept(&self, visitor: &mut impl ExprVisitor<T>) -> T {
        visitor.visit_assign_expr(self.clone())
    }
}

#[derive(Clone, Debug)]
pub struct LogicalExpr {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

impl LogicalExpr {
    pub fn new(left: Expr, operator: Token, right: Expr) -> Self {
        LogicalExpr {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }
}
impl<T> ExprVisitorAcceptor<T> for LogicalExpr {
    fn accept(&self, visitor: &mut impl ExprVisitor<T>) -> T {
        visitor.visit_logical_expr(self.clone())
    }
}
