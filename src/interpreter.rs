use crate::{
    expr::{Expr, ExprVisitor, ExprVisitorAcceptor},
    stmt::{Stmt, StmtVisitor, StmtVisitorAcceptor},
    tokens::{TokenLiteral, TokenType},
};

#[derive(Default)]
pub struct Interpreter {}

impl Interpreter {
    pub fn new() -> Self {
        Default::default()
    }

    fn is_truthy(&self, object: TokenLiteral) -> TokenLiteral {
        match object {
            TokenLiteral::None => TokenLiteral::Bool(false),
            TokenLiteral::Bool(b) => TokenLiteral::Bool(b),
            _ => TokenLiteral::Bool(true),
        }
    }
    pub fn interpret(self, statements: Vec<Stmt>) {
        for statement in statements {
            match statement {
                Stmt::Expression(x) => self.execute(x),
                Stmt::Print(x) => self.execute(x),
            }
        }
    }
}

impl Interpreter
where
    Interpreter: ExprVisitor<TokenLiteral>,
{
    pub fn evaluate<A: ExprVisitorAcceptor<TokenLiteral>>(&self, expr: A) -> TokenLiteral {
        expr.accept(self)
    }
}

impl ExprVisitor<TokenLiteral> for Interpreter {
    fn visit_literal_expr(&self, expr: crate::expr::LiteralExpr) -> TokenLiteral {
        expr.value
    }

    fn visit_grouping_expr(&self, expr: crate::expr::GroupingExpr) -> TokenLiteral {
        match *expr.expression {
            Expr::Binary(x) => self.evaluate(x),
            Expr::Grouping(x) => self.evaluate(x),
            Expr::Literal(x) => self.evaluate(x),
            Expr::Unary(x) => self.evaluate(x),
        }
    }

    fn visit_unary_expr(&self, expr: crate::expr::UnaryExpr) -> TokenLiteral {
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
                TokenLiteral::Number(n) => TokenLiteral::Number(-1.0 * n),
                _ => panic!(),
            },
            _ => panic!(),
        }
    }

    fn visit_binary_expr(&self, expr: crate::expr::BinaryExpr) -> TokenLiteral {
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

impl Interpreter
where
    Interpreter: StmtVisitor<()>,
{
    fn execute<A: StmtVisitorAcceptor<()>>(&self, stmt: A) {
        stmt.accept(self)
    }
}

impl StmtVisitor<()> for Interpreter {
    fn visit_expression_stmt(&self, stmt: crate::stmt::ExpressionStmt) {
        match *stmt.expression {
            Expr::Binary(x) => self.evaluate(x),
            Expr::Grouping(x) => self.evaluate(x),
            Expr::Literal(x) => self.evaluate(x),
            Expr::Unary(x) => self.evaluate(x),
        };
    }

    fn visit_print_stmt(&self, stmt: crate::stmt::PrintStmt) {
        let value = match *stmt.expression {
            Expr::Binary(x) => self.evaluate(x),
            Expr::Grouping(x) => self.evaluate(x),
            Expr::Literal(x) => self.evaluate(x),
            Expr::Unary(x) => self.evaluate(x),
        };
        println!("{}", value);
    }
}
