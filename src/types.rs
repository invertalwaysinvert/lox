use crate::expressions;

#[derive(Clone, Debug)]
pub enum LoxObject {
    Binary(expressions::BinaryExpr),
    Grouping(expressions::GroupingExpr),
    Literal(expressions::LiteralExpr),
    Unary(expressions::UnaryExpr),
}
