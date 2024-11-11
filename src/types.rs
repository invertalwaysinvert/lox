use crate::expr;

#[derive(Clone, Debug)]
pub enum LoxObject {
    Binary(expr::BinaryExpr),
    Grouping(expr::GroupingExpr),
    Literal(expr::LiteralExpr),
    Unary(expr::UnaryExpr),
}
