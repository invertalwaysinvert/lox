use crate::{
    expr::{BinaryExpr, ExprVisitor, ExprVisitorAcceptor},
    tokens::TokenLiteral,
};

struct PrettyPrinter {}

impl PrettyPrinter {
    fn paranthesize(name: &str, exprs: Vec<&str>) -> String {
        let mut result = format!("( {name} ");
        for expr in exprs {
            result.push_str(expr);
            result.push(' ')
        }
        result.push(')');
        result
    }
}

impl PrettyPrinter
where
    PrettyPrinter: ExprVisitor<String>,
{
    fn print<A: ExprVisitorAcceptor<String>>(&self, expr: A) -> String {
        expr.accept(self)
    }
}

impl ExprVisitor<String> for PrettyPrinter {
    fn visit_binary_expr(&self, expr: BinaryExpr) -> String {
        PrettyPrinter::paranthesize(
            &expr.operator.lexeme,
            vec![&expr.left.to_string(), &expr.right.to_string()],
        )
    }

    fn visit_grouping_expr(&self, expr: crate::expr::GroupingExpr) -> String {
        PrettyPrinter::paranthesize("group", vec![&expr.expression.to_string()])
    }

    fn visit_unary_expr(&self, expr: crate::expr::UnaryExpr) -> String {
        PrettyPrinter::paranthesize(&expr.operator.lexeme, vec![&expr.right.to_string()])
    }

    fn visit_literal_expr(&self, expr: crate::expr::LiteralExpr) -> String {
        match expr.value {
            TokenLiteral::None => "nil".to_string(),
            x => x.to_string(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        expr::{BinaryExpr, Expr, GroupingExpr, LiteralExpr},
        tokens::{Token, TokenLiteral, TokenType},
    };

    use super::PrettyPrinter;

    #[test]
    fn test_binary_expression() {
        let printer = PrettyPrinter {};
        let output = printer.print(BinaryExpr::new(
            Expr::Literal(LiteralExpr::new(TokenLiteral::String("abc".to_string()))),
            Token::new(TokenType::Star, "*".to_string(), TokenLiteral::None, 1),
            Expr::Literal(LiteralExpr::new(TokenLiteral::String("xyz".to_string()))),
        ));
        assert_eq!(output, "( * abc xyz )");
    }

    #[test]
    fn test_binary_expression_full() {
        let printer = PrettyPrinter {};
        let output = printer.print(BinaryExpr::new(
            Expr::Literal(LiteralExpr::new(TokenLiteral::String("abc".to_string()))),
            Token::new(TokenType::Star, "*".to_string(), TokenLiteral::None, 1),
            Expr::Grouping(GroupingExpr::new(Expr::Literal(LiteralExpr::new(
                TokenLiteral::String("xyz".to_string()),
            )))),
        ));
        assert_eq!(output, "( * abc ( group xyz ) )");
    }
}
