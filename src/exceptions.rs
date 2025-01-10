use crate::tokens::LoxObject;

#[derive(Debug)]
pub struct RuntimeError {}

#[derive(Debug)]
pub struct Return {
    pub value: LoxObject,
}

#[derive(Debug)]
pub struct ParserError {
    pub msg: String,
}

impl ParserError {
    pub fn raise(msg: String) -> Self {
        ParserError { msg }
    }
}
