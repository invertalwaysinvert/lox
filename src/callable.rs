use crate::tokens::LoxObject;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct LoxCallable {
    pub code: fn(Vec<LoxObject>) -> LoxObject,
    pub arity: usize,
    pub name: String,
}

impl LoxCallable {
    pub fn new(code: fn(Vec<LoxObject>) -> LoxObject, arity: usize, name: String) -> Self {
        LoxCallable { code, arity, name }
    }
}
