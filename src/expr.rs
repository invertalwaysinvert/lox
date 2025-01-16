use core::fmt;
use std::fmt::Display;

use crate::tokens::{LoxObject, Token};

pub trait ExprVisitor<T> {
    fn visit_binary_expr(&mut self, expr: BinaryExpr) -> T;
    fn visit_grouping_expr(&mut self, expr: GroupingExpr) -> T;
    fn visit_literal_expr(&mut self, expr: LiteralExpr) -> T;
    fn visit_unary_expr(&mut self, expr: UnaryExpr) -> T;
    fn visit_variable_expr(&mut self, expr: VariableExpr) -> T;
    fn visit_assign_expr(&mut self, expr: AssignExpr) -> T;
    fn visit_logical_expr(&mut self, expr: LogicalExpr) -> T;
    fn visit_call_expr(&mut self, expr: CallExpr) -> T;
    fn visit_get_expr(&mut self, expr: GetExpr) -> T;
    fn visit_set_expr(&mut self, expr: SetExpr) -> T;
    fn visit_this_expr(&mut self, expr: ThisExpr) -> T;
    fn visit_super_expr(&mut self, expr: SuperExpr) -> T;
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
    Call(CallExpr),
    Get(GetExpr),
    Set(SetExpr),
    This(ThisExpr),
    Super(SuperExpr),
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Literal(x) => write!(f, "Literal({})", x.value),
            Self::Grouping(x) => write!(f, "Group({})", x.expression),
            Self::Binary(x) => write!(f, "Binary({} {} {})", x.left, x.operator, x.right),
            Self::Unary(x) => write!(f, "Unary({} {})", x.operator, x.right),
            Self::Variable(x) => write!(f, "Var({})", x.name),
            Self::Assign(x) => write!(f, "Assign({} = {})", x.name, x.value),
            Self::Logical(x) => write!(f, "Logical({} {} {})", x.left, x.operator, x.right),
            Self::Get(x) => write!(f, "Get({} {})", x.object, x.name),
            Self::Set(x) => write!(f, "Set({} {} {})", x.object, x.name, x.value),
            Self::This(x) => write!(f, "This({})", x.name),
            Self::Super(x) => write!(f, "Super({} {})", x.keyword, x.method),
            Self::Call(x) => write!(
                f,
                "Call({} ({}))",
                x.callee,
                x.arguments
                    .iter()
                    .map(|arg| arg.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
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
    pub value: LoxObject,
}

impl LiteralExpr {
    pub fn new(value: LoxObject) -> Self {
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

impl fmt::Display for VariableExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "name: {}", self.name.lexeme)
    }
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

impl Display for AssignExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "name: {}, value: {}", self.name, *self.value)
    }
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

#[derive(Clone, Debug)]
pub struct CallExpr {
    pub callee: Box<Expr>,
    pub paren: Token,
    pub arguments: Vec<Expr>,
}

impl CallExpr {
    pub fn new(callee: Expr, paren: Token, arguments: Vec<Expr>) -> Self {
        CallExpr {
            callee: Box::new(callee),
            paren,
            arguments,
        }
    }
}
impl<T> ExprVisitorAcceptor<T> for CallExpr {
    fn accept(&self, visitor: &mut impl ExprVisitor<T>) -> T {
        visitor.visit_call_expr(self.clone())
    }
}

#[derive(Clone, Debug)]
pub struct GetExpr {
    pub object: Box<Expr>,
    pub name: Token,
}

impl GetExpr {
    pub fn new(object: Expr, name: Token) -> Self {
        GetExpr {
            object: Box::new(object),
            name,
        }
    }
}
impl<T> ExprVisitorAcceptor<T> for GetExpr {
    fn accept(&self, visitor: &mut impl ExprVisitor<T>) -> T {
        visitor.visit_get_expr(self.clone())
    }
}

#[derive(Clone, Debug)]
pub struct SetExpr {
    pub object: Box<Expr>,
    pub name: Token,
    pub value: Box<Expr>,
}

impl SetExpr {
    pub fn new(object: Expr, name: Token, value: Expr) -> Self {
        SetExpr {
            object: Box::new(object),
            name,
            value: Box::new(value),
        }
    }
}
impl<T> ExprVisitorAcceptor<T> for SetExpr {
    fn accept(&self, visitor: &mut impl ExprVisitor<T>) -> T {
        visitor.visit_set_expr(self.clone())
    }
}

#[derive(Clone, Debug)]
pub struct ThisExpr {
    pub name: Token,
}

impl ThisExpr {
    pub fn new(name: Token) -> Self {
        ThisExpr { name }
    }
}
impl<T> ExprVisitorAcceptor<T> for ThisExpr {
    fn accept(&self, visitor: &mut impl ExprVisitor<T>) -> T {
        visitor.visit_this_expr(self.clone())
    }
}

impl fmt::Display for ThisExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "name: {}", self.name.lexeme)
    }
}

#[derive(Clone, Debug)]
pub struct SuperExpr {
    pub keyword: Token,
    pub method: Token,
}

impl SuperExpr {
    pub fn new(keyword: Token, method: Token) -> Self {
        SuperExpr { keyword, method }
    }
}
impl<T> ExprVisitorAcceptor<T> for SuperExpr {
    fn accept(&self, visitor: &mut impl ExprVisitor<T>) -> T {
        visitor.visit_super_expr(self.clone())
    }
}
//
// impl fmt::Display for SuperExpr {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "super keyword: {}", self.keyword.lexeme)
//     }
// }
