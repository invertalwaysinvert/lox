use crate::{
    expressions::{Expr, Visitor, VisitorAcceptor},
    tokens::{TokenLiteral, TokenType},
};

pub struct Interpreter {}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {}
    }

    fn is_truthy(&self, object: TokenLiteral) -> TokenLiteral {
        match object {
            TokenLiteral::None => TokenLiteral::Bool(false),
            TokenLiteral::Bool(b) => TokenLiteral::Bool(b),
            _ => TokenLiteral::Bool(true),
        }
    }
}

impl Interpreter
where
    Interpreter: Visitor<TokenLiteral>,
{
    pub fn interpret(self, expression: Expr) {
        let value = self.evaluate(expression);
        dbg!(value);
    }

    pub fn evaluate<A: VisitorAcceptor<TokenLiteral, TokenLiteral>>(
        &self,
        expr: A,
    ) -> TokenLiteral {
        expr.accept(self)
    }
}

impl Visitor<TokenLiteral> for Interpreter {
    fn visit_literal_expr(&self, expr: crate::expressions::LiteralExpr) -> TokenLiteral {
        expr.value
    }

    fn visit_grouping_expr(&self, expr: crate::expressions::GroupingExpr) -> TokenLiteral {
        match *expr.expression {
            Expr::Binary(x) => self.evaluate(x),
            Expr::Grouping(x) => self.evaluate(x),
            Expr::Literal(x) => self.evaluate(x),
            Expr::Unary(x) => self.evaluate(x),
        }
    }

    fn visit_unary_expr(&self, expr: crate::expressions::UnaryExpr) -> TokenLiteral {
        let right = match *expr.right {
            Expr::Binary(x) => self.evaluate(x),
            Expr::Grouping(x) => self.evaluate(x),
            Expr::Literal(x) => self.evaluate(x),
            Expr::Unary(x) => self.evaluate(x),
        };

        match expr.operator.token_type {
            // Todo: Negate the bang
            TokenType::Bang => self.is_truthy(right),
            TokenType::Minus => match right {
                TokenLiteral::Number(x) => TokenLiteral::Number(-1.0 * x),
                _ => panic!(),
            },
            _ => panic!(),
        }
    }

    fn visit_binary_expr(&self, expr: crate::expressions::BinaryExpr) -> TokenLiteral {
        let left = match *expr.left {
            Expr::Binary(x) => self.evaluate(x),
            Expr::Grouping(x) => self.evaluate(x),
            Expr::Literal(x) => self.evaluate(x),
            Expr::Unary(x) => self.evaluate(x),
        };
        let right = match *expr.right {
            Expr::Binary(x) => self.evaluate(x),
            Expr::Grouping(x) => self.evaluate(x),
            Expr::Literal(x) => self.evaluate(x),
            Expr::Unary(x) => self.evaluate(x),
        };

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
}
