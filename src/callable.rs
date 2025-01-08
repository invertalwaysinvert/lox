use crate::tokens::LoxObject;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct LoxCallable {}

impl LoxCallable {
    fn call(&mut self, _arguments: Vec<LoxObject>) {
        todo!()
    }
    fn arity(&self) -> i32 {
        todo!();
        1
    }
}
