use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Sub},
};

use crate::callable::LoxCallable;

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: LoxObject,
    pub line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: LoxObject, line: usize) -> Self {
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

#[derive(Debug, PartialEq, PartialOrd)]
pub enum LoxObject {
    String(String),
    Number(f32),
    Bool(bool),
    None,
    Callable(Box<LoxCallable>),
}

impl Clone for LoxObject {
    fn clone(&self) -> Self {
        match self {
            Self::String(x) => LoxObject::String(x.clone()),
            Self::Number(x) => LoxObject::Number(*x),
            Self::Bool(x) => LoxObject::Bool(*x),
            Self::None => LoxObject::None,
            Self::Callable(x) => LoxObject::Callable(Box::new(*x.clone())),
        }
    }
}

impl Add for LoxObject {
    type Output = LoxObject;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (LoxObject::Number(x), LoxObject::Number(y)) => LoxObject::Number(x + y),
            (LoxObject::String(x), LoxObject::String(y)) => LoxObject::String(x + &y),
            _ => panic!(),
        }
    }
}

impl Sub for LoxObject {
    type Output = LoxObject;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (LoxObject::Number(x), LoxObject::Number(y)) => LoxObject::Number(x - y),
            _ => panic!(),
        }
    }
}

impl Mul for LoxObject {
    type Output = LoxObject;
    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (LoxObject::Number(x), LoxObject::Number(y)) => LoxObject::Number(x * y),
            _ => panic!(),
        }
    }
}

impl Div for LoxObject {
    type Output = LoxObject;
    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (LoxObject::Number(x), LoxObject::Number(y)) => LoxObject::Number(x / y),
            _ => panic!(),
        }
    }
}

impl Display for LoxObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LoxObject::None => write!(f, ""),
            LoxObject::Bool(x) => write!(f, "{}", x),
            LoxObject::Number(x) => write!(f, "{}", x),
            LoxObject::String(x) => write!(f, "{}", x),
            LoxObject::Callable(_x) => write!(f, "<loxFunction>"),
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
