use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Sub},
};

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: TokenLiteral,
    pub line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: TokenLiteral, line: usize) -> Self {
        Token {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {} {}", self.token_type, self.lexeme, self.literal)
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum TokenLiteral {
    String(String),
    Number(f32),
    Bool(bool),
    None,
}

impl Add for TokenLiteral {
    type Output = TokenLiteral;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (TokenLiteral::Number(x), TokenLiteral::Number(y)) => TokenLiteral::Number(x + y),
            (TokenLiteral::String(x), TokenLiteral::String(y)) => TokenLiteral::String(x + &y),
            _ => panic!(),
        }
    }
}

impl Sub for TokenLiteral {
    type Output = TokenLiteral;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (TokenLiteral::Number(x), TokenLiteral::Number(y)) => TokenLiteral::Number(x - y),
            _ => panic!(),
        }
    }
}

impl Mul for TokenLiteral {
    type Output = TokenLiteral;
    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (TokenLiteral::Number(x), TokenLiteral::Number(y)) => TokenLiteral::Number(x * y),
            _ => panic!(),
        }
    }
}

impl Div for TokenLiteral {
    type Output = TokenLiteral;
    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (TokenLiteral::Number(x), TokenLiteral::Number(y)) => TokenLiteral::Number(x / y),
            _ => panic!(),
        }
    }
}

impl Display for TokenLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenLiteral::None => write!(f, ""),
            x => write!(f, "{}", x),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenType {
    // Single character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals
    Identifier,
    String,
    Number,

    // Keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}
