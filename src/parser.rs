use crate::{
    exceptions::ParserError,
    expr::{
        BinaryExpr, CallExpr, Expr, GetExpr, GroupingExpr, LiteralExpr, LogicalExpr, ThisExpr,
        UnaryExpr, VariableExpr,
    },
    logger::error_token,
    stmt::{
        BlockStmt, ClassStmt, ExpressionStmt, FunStmt, IfStmt, PrintStmt, ReturnStmt, Stmt,
        VarStmt, WhileStmt,
    },
    tokens::{LoxObject, Token, TokenType},
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
                Err(e) => {
                    if let Some(token) = self.tokens.get(self.current) {
                        println!("----\nLine {}\n{}\n\n", token, e.msg);
                        self.synchronize();
                    }
                }
            }
        }
        Ok(result)
    }

    fn declaration(&mut self) -> Result<Stmt, ParserError> {
        if self.match_token(vec![TokenType::Class]) {
            self.class_declaration()
        } else if self.match_token(vec![TokenType::Fun]) {
            self.function("function")
        } else if self.match_token(vec![TokenType::Var]) {
            self.variable_declaration()
        } else {
            self.statement()
        }
    }

    fn class_declaration(&mut self) -> Result<Stmt, ParserError> {
        let name = self.consume(TokenType::Identifier, "Expect class name.")?;
        self.consume(TokenType::LeftBrace, "Expect '{' before class body.")?;

        let mut methods = Vec::new();
        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            methods.push(self.function("function")?)
        }
        self.consume(TokenType::RightBrace, "Expect '}' after class body.")?;
        Ok(Stmt::Class(ClassStmt::new(name, methods)))
    }

    fn function(&mut self, kind: &str) -> Result<Stmt, ParserError> {
        let name = self.consume(TokenType::Identifier, &format!("Expect {} name.", kind))?;
        self.consume(
            TokenType::LeftParen,
            &format!("Expect '(' after {} name.", kind),
        )?;
        let mut parameters = Vec::new();
        if !self.check(TokenType::RightParen) {
            loop {
                parameters.push(self.consume(TokenType::Identifier, "Expect parameter name.")?);
                if !self.match_token(vec![TokenType::Comma]) {
                    break;
                }
            }
        }
        self.consume(TokenType::RightParen, "Expect ')' after parameters.")?;
        self.consume(TokenType::LeftBrace, "Expect '{' before body")?;
        let body = self.block()?;
        Ok(Stmt::Fun(FunStmt::new(name, parameters, body)))
    }

    fn statement(&mut self) -> Result<Stmt, ParserError> {
        if self.match_token(vec![TokenType::For]) {
            return self.for_statement();
        }
        if self.match_token(vec![TokenType::If]) {
            return self.if_statement();
        }
        if self.match_token(vec![TokenType::Print]) {
            return self.print_statement();
        }
        if self.match_token(vec![TokenType::Return]) {
            return self.return_statement();
        }
        if self.match_token(vec![TokenType::While]) {
            return self.while_statement();
        }
        if self.match_token(vec![TokenType::LeftBrace]) {
            return Ok(Stmt::Block(BlockStmt::new(self.block()?)));
        }
        self.expression_statement()
    }

    fn return_statement(&mut self) -> Result<Stmt, ParserError> {
        let keyword = self.previous();
        let mut value = None;
        if !self.check(TokenType::Semicolon) {
            value = Some(self.expression()?);
        }
        self.consume(TokenType::Semicolon, "Expect ';' after return.")?;
        Ok(Stmt::Return(ReturnStmt::new(keyword, value)))
    }

    fn for_statement(&mut self) -> Result<Stmt, ParserError> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'for'")?;

        // Initializer
        let initializer;
        if self.match_token(vec![TokenType::Semicolon]) {
            initializer = None;
        } else if self.match_token(vec![TokenType::Var]) {
            initializer = Some(self.variable_declaration()?);
        } else {
            initializer = Some(self.expression_statement()?);
        }

        // Condition
        let mut condition = None;
        if !self.check(TokenType::Semicolon) {
            condition = Some(self.expression()?);
        }
        self.consume(TokenType::Semicolon, "Expect ';' after loop condition")?;

        // Increment
        let mut increment = None;
        if !self.check(TokenType::RightParen) {
            increment = Some(self.expression()?);
        }
        self.consume(TokenType::RightParen, "Expect ')' after for clauses")?;
        let mut body = self.statement()?;

        if let Some(expression) = increment {
            body = Stmt::Block(BlockStmt::new(vec![
                body,
                Stmt::Expression(ExpressionStmt::new(expression)),
            ]))
        }

        let condition = match condition {
            Some(x) => x,
            None => Expr::Literal(LiteralExpr::new(LoxObject::Bool(true))),
        };
        body = Stmt::While(WhileStmt::new(condition, body));

        if let Some(statement) = initializer {
            body = Stmt::Block(BlockStmt::new(vec![statement, body]))
        }

        Ok(body)
    }

    fn while_statement(&mut self) -> Result<Stmt, ParserError> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'while'")?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "Expect ')' after condition")?;
        let body = self.statement()?;
        Ok(Stmt::While(WhileStmt::new(condition, body)))
    }

    fn if_statement(&mut self) -> Result<Stmt, ParserError> {
        self.consume(TokenType::LeftParen, "Expect '(' after if")?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "Expect ')' after if")?;

        let then_branch = self.statement()?;
        let else_branch = if self.match_token(vec![TokenType::Else]) {
            Some(self.statement()?)
        } else {
            None
        };
        Ok(Stmt::If(IfStmt::new(condition, then_branch, else_branch)))
    }

    fn block(&mut self) -> Result<Vec<Stmt>, ParserError> {
        let mut statments: Vec<Stmt> = Vec::new();
        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            statments.push(self.declaration()?);
        }
        self.consume(TokenType::RightBrace, "Expect '}' after body")?;
        Ok(statments)
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
        )?;
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
        let expr = self.or();
        if self.match_token(vec![TokenType::Equal]) {
            let _equals = self.previous();
            let value = self.assignment()?;

            if let Ok(y) = expr {
                return match y {
                    Expr::Variable(x) => Ok(Expr::Assign(crate::expr::AssignExpr {
                        name: x.name,
                        value: Box::new(value),
                    })),
                    Expr::Get(x) => Ok(Expr::Set(crate::expr::SetExpr::new(
                        *x.object, x.name, value,
                    ))),
                    _ => Err(ParserError::raise(String::from(
                        "Don't know what I'm doing here",
                    ))),
                };
            };
        }

        expr
    }

    fn or(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.and()?;

        while self.match_token(vec![TokenType::Or]) {
            let operator = self.previous();
            let right = self.and()?;
            expr = Expr::Logical(LogicalExpr::new(expr, operator, right));
        }
        Ok(expr)
    }

    fn and(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.equality()?;

        while self.match_token(vec![TokenType::And]) {
            let operator = self.previous();
            let right = self.equality()?;
            expr = Expr::Logical(LogicalExpr::new(expr, operator, right));
        }
        Ok(expr)
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
        self.call()
    }

    fn call(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.primary()?;

        loop {
            if self.match_token(vec![TokenType::LeftParen]) {
                expr = self.finish_call(expr)?;
            } else if self.match_token(vec![TokenType::Dot]) {
                let name = self.consume(TokenType::Identifier, "Expect property name after '.'")?;
                expr = Expr::Get(GetExpr::new(expr, name))
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn finish_call(&mut self, callee: Expr) -> Result<Expr, ParserError> {
        let mut arguments = Vec::new();
        if !self.check(TokenType::RightParen) {
            loop {
                arguments.push(self.expression()?);
                if !self.match_token(vec![TokenType::Comma]) {
                    break;
                }
            }
        }

        let paren = self.consume(TokenType::RightParen, "Expect ')' after arguments")?;

        Ok(Expr::Call(CallExpr::new(callee, paren, arguments)))
    }

    fn primary(&mut self) -> Result<Expr, ParserError> {
        if self.match_token(vec![TokenType::False]) {
            return Ok(Expr::Literal(LiteralExpr::new(LoxObject::Bool(false))));
        }
        if self.match_token(vec![TokenType::True]) {
            return Ok(Expr::Literal(LiteralExpr::new(LoxObject::Bool(true))));
        }
        if self.match_token(vec![TokenType::Nil]) {
            return Ok(Expr::Literal(LiteralExpr::new(LoxObject::None)));
        }
        if self.match_token(vec![TokenType::Number, TokenType::String]) {
            return Ok(Expr::Literal(LiteralExpr::new(self.previous().literal)));
        }
        if self.match_token(vec![TokenType::This]) {
            return Ok(Expr::This(ThisExpr::new(self.previous())));
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
        Err(ParserError::raise("Invalid primary expression".to_string()))
    }

    fn consume(&mut self, token_type: TokenType, _message: &str) -> Result<Token, ParserError> {
        if self.check(token_type) {
            Ok(self.advance())
        } else {
            Err(ParserError::raise(_message.to_string()))
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
            self.advance();
        }
    }
}
