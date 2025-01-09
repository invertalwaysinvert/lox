use crate::tokens::LoxObject;

#[derive(Debug)]
pub struct RuntimeError {}

#[derive(Debug)]
pub struct Return {
    pub value: LoxObject,
}
