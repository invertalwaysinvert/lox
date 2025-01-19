use std::collections::HashMap;

use crate::{
    logger,
    tokens::{LoxObject, Token, TokenType},
};

pub struct Scanner<'a> {
    source: &'a str,
    tokens: Vec<Token>,
    current: usize,
    start: usize,
    line: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Scanner {
            source,
            tokens: Vec::new(),
            current: 0,
            start: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.tokens.push(Token {
            token_type: TokenType::Eof,
            lexeme: "".to_string(),
            literal: LoxObject::None,
            line: self.line,
            current: self.current,
        });
        self.tokens.clone()
    }

    fn scan_token(&mut self) {
        match self.advance() {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '!' => {
                if self.match_next('=') {
                    self.add_token(TokenType::BangEqual)
                } else {
                    self.add_token(TokenType::Bang)
                }
            }
            '=' => {
                if self.match_next('=') {
                    self.add_token(TokenType::EqualEqual)
                } else {
                    self.add_token(TokenType::Equal)
                }
            }
            '<' => {
                if self.match_next('=') {
                    self.add_token(TokenType::LessEqual)
                } else {
                    self.add_token(TokenType::Less)
                }
            }
            '>' => {
                if self.match_next('=') {
                    self.add_token(TokenType::GreaterEqual)
                } else {
                    self.add_token(TokenType::Greater)
                }
            }
            '/' => {
                if self.match_next('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash);
                }
            }
            '"' => self.string(),
            '\n' => self.line += 1,
            ' ' | '\r' | '\t' => (),
            c => {
                if c.is_ascii_digit() {
                    self.number();
                } else if self.is_alpha(c) {
                    self.identifier();
                } else {
                    logger::error(self.line, "Unexpected character.");
                }
            }
        }
    }

    fn is_alpha(&self, ch: char) -> bool {
        ch.is_ascii_lowercase() || ch.is_ascii_uppercase() || ch == '_'
    }

    fn is_alphanumeric(&self, ch: char) -> bool {
        self.is_alpha(ch) || ch.is_ascii_digit()
    }

    fn identifier(&mut self) {
        while self.is_alphanumeric(self.peek()) {
            self.advance();
        }

        let substring: String = self
            .source
            .chars()
            .skip(self.start)
            .take(self.current - self.start)
            .collect();

        let keywords = self.get_keywords_table();

        match keywords.get(&substring) {
            Some(&keyword) => self.add_token(keyword),
            None => self.add_token(TokenType::Identifier),
        }
    }

    fn get_keywords_table(&self) -> HashMap<String, TokenType> {
        let mut keywords = HashMap::new();
        keywords.insert("and".to_string(), TokenType::And);
        keywords.insert("class".to_string(), TokenType::Class);
        keywords.insert("else".to_string(), TokenType::Else);
        keywords.insert("false".to_string(), TokenType::False);
        keywords.insert("for".to_string(), TokenType::For);
        keywords.insert("fun".to_string(), TokenType::Fun);
        keywords.insert("if".to_string(), TokenType::If);
        keywords.insert("nil".to_string(), TokenType::Nil);
        keywords.insert("or".to_string(), TokenType::Or);
        keywords.insert("print".to_string(), TokenType::Print);
        keywords.insert("return".to_string(), TokenType::Return);
        keywords.insert("super".to_string(), TokenType::Super);
        keywords.insert("this".to_string(), TokenType::This);
        keywords.insert("true".to_string(), TokenType::True);
        keywords.insert("var".to_string(), TokenType::Var);
        keywords.insert("while".to_string(), TokenType::While);
        keywords
    }

    fn number(&mut self) {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance();
            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }
        let substring: String = self
            .source
            .chars()
            .skip(self.start)
            .take(self.current - self.start)
            .collect();
        self.add_token_with_literal(
            TokenType::Number,
            LoxObject::Number(substring.parse::<f32>().unwrap()),
        )
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            logger::error(self.line, "Unterminated string.");
            return;
        }

        self.advance();

        let literal: String = self
            .source
            .chars()
            .skip(self.start + 1)
            .take(self.current - 1 - (self.start + 1))
            .collect();
        self.add_token_with_literal(TokenType::String, LoxObject::String(literal));
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else if let Some(ch) = self.source.chars().nth(self.current) {
            ch
        } else {
            '\0'
        }
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.chars().count() {
            '\0'
        } else if let Some(ch) = self.source.chars().nth(self.current + 1) {
            ch
        } else {
            '\0'
        }
    }

    fn match_next(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if let Some(ch) = self.source.chars().nth(self.current) {
            if ch != expected {
                return false;
            }
        }

        self.current += 1;
        true
    }

    fn advance(&mut self) -> char {
        let ch = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        ch
    }

    fn add_token(&mut self, token: TokenType) {
        let lexeme = self
            .source
            .chars()
            .skip(self.start)
            .take(self.current - self.start)
            .collect();
        self.tokens.push(Token {
            token_type: token,
            lexeme,
            literal: LoxObject::None,
            line: self.line,
            current: self.current,
        })
    }

    fn add_token_with_literal(&mut self, token: TokenType, literal: LoxObject) {
        let lexeme = self
            .source
            .chars()
            .skip(self.start)
            .take(self.current - self.start)
            .collect();
        self.tokens.push(Token {
            token_type: token,
            lexeme,
            literal,
            line: self.line,
            current: self.current,
        })
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}
