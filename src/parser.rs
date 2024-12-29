use crate::{
    expr::{BinaryExpr, Expr, GroupingExpr, LiteralExpr, UnaryExpr, VariableExpr},
    logger::error_token,
    stmt::{ExpressionStmt, PrintStmt, Stmt, VarStmt},
    tokens::{Token, TokenLiteral, TokenType},
};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, ParserError> {
        let mut result = Vec::new();
        while !self.is_at_end() {
            match self.declaration() {
                Ok(stmt) => result.push(stmt),
                Err(_) => self.synchronize(),
            }
        }
        Ok(result)
    }

    fn declaration(&mut self) -> Result<Stmt, ParserError> {
        if self.match_token(vec![TokenType::Var]) {
            return self.variable_declaration();
        } else {
            return self.statement();
        }
    }

    fn statement(&mut self) -> Result<Stmt, ParserError> {
        if self.match_token(vec![TokenType::Print]) {
            return self.print_statement();
        }
        self.expression_statement()
    }

    fn print_statement(&mut self) -> Result<Stmt, ParserError> {
        let value = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after value.")?;
        Ok(Stmt::Print(PrintStmt::new(value)))
    }

    fn variable_declaration(&mut self) -> Result<Stmt, ParserError> {
        let name = self.consume(TokenType::Identifier, "Expect variable name.")?;
        let mut initializer = None;
        if self.match_token(vec![TokenType::Equal]) {
            initializer = Some(self.expression()?)
        }

        self.consume(
            TokenType::Semicolon,
            "Expect ';' after variable declaration.",
        );
        Ok(Stmt::Var(VarStmt::new(name, initializer)))
    }

    fn expression_statement(&mut self) -> Result<Stmt, ParserError> {
        let value = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after expression.")?;
        Ok(Stmt::Expression(ExpressionStmt::new(value)))
    }

    fn expression(&mut self) -> Result<Expr, ParserError> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr, ParserError> {
        let expr = self.equality();
        if self.match_token(vec![TokenType::Equal]) {
            let equals = self.previous();
            let value = self.assignment()?;

            if let Ok(y) = expr {
                return match y {
                    Expr::Variable(x) => Ok(Expr::Assign(crate::expr::AssignExpr {
                        name: x.name,
                        value: Box::new(value),
                    })),
                    _ => Err(ParserError {}),
                };
            };
        }

        return expr;
    }

    fn equality(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.comparison()?;
        while self.match_token(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison()?;
            expr = Expr::Binary(BinaryExpr::new(expr, operator, right));
        }
        Ok(expr)
    }

    fn match_token(&mut self, types: Vec<TokenType>) -> bool {
        for item in types {
            if self.check(item) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, token: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().token_type == token
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    fn peek(&self) -> Token {
        self.tokens.get(self.current).unwrap().clone()
    }

    fn previous(&self) -> Token {
        self.tokens.get(self.current - 1).unwrap().clone()
    }

    fn comparison(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.term()?;

        while self.match_token(vec![
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous();
            let right = self.term()?;
            expr = Expr::Binary(BinaryExpr::new(expr, operator, right));
        }
        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.factor()?;

        while self.match_token(vec![TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous();
            let right = self.factor()?;
            expr = Expr::Binary(BinaryExpr::new(expr, operator, right));
        }
        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.unary()?;

        while self.match_token(vec![TokenType::Slash, TokenType::Star]) {
            let operator = self.previous();
            let right = self.unary()?;
            expr = Expr::Binary(BinaryExpr::new(expr, operator, right));
        }
        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, ParserError> {
        if self.match_token(vec![TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let right = self.unary()?;
            return Ok(Expr::Unary(UnaryExpr::new(right, operator)));
        }
        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, ParserError> {
        if self.match_token(vec![TokenType::False]) {
            return Ok(Expr::Literal(LiteralExpr::new(TokenLiteral::Bool(false))));
        }
        if self.match_token(vec![TokenType::True]) {
            return Ok(Expr::Literal(LiteralExpr::new(TokenLiteral::Bool(true))));
        }
        if self.match_token(vec![TokenType::Nil]) {
            return Ok(Expr::Literal(LiteralExpr::new(TokenLiteral::None)));
        }
        if self.match_token(vec![TokenType::Number, TokenType::String]) {
            return Ok(Expr::Literal(LiteralExpr::new(self.previous().literal)));
        }
        if self.match_token(vec![TokenType::Identifier]) {
            return Ok(Expr::Variable(VariableExpr::new(self.previous())));
        }
        if self.match_token(vec![TokenType::LeftParen]) {
            let expr = self.expression()?;
            match self.consume(TokenType::RightParen, "Expect ')' after expression.") {
                Ok(_token) => return Ok(Expr::Grouping(GroupingExpr::new(expr))),
                Err(err) => return Err(err),
            };
        };
        Err(ParserError {})
    }

    fn consume(&mut self, token_type: TokenType, _message: &str) -> Result<Token, ParserError> {
        if self.check(token_type) {
            Ok(self.advance())
        } else {
            Err(ParserError {})
        }
    }

    fn _throw_error(&self, token: Token, message: &str) {
        error_token(token, message);
    }

    fn synchronize(&mut self) {
        self.advance();
        while !self.is_at_end() {
            if self.previous().token_type == TokenType::Semicolon {
                return;
            }

            match self.peek().token_type {
                TokenType::Class
                | TokenType::For
                | TokenType::Fun
                | TokenType::If
                | TokenType::Print
                | TokenType::Return
                | TokenType::Var
                | TokenType::While => return,
                _ => (),
            }
        }
    }
}

#[derive(Debug)]
pub struct ParserError {}
