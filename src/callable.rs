use std::cmp::Ordering;

use crate::{
    environment::Environment, instance::LoxInstance, interpreter::Interpreter, stmt::FunStmt,
    tokens::LoxObject, utils::Clock,
};

pub enum LoxCallableType {
    Fun(LoxFunction),
    Clock(Clock),
}

pub trait LoxCallable: std::fmt::Debug {
    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<LoxObject>) -> LoxObject;
    fn arity(&self) -> usize;
    fn to_string(&self) -> String;
}

#[derive(Debug, Clone)]
pub struct LoxFunction {
    pub declaration: FunStmt,
    pub closure: Environment,
    pub is_init: bool,
}

impl PartialOrd for LoxFunction {
    fn partial_cmp(&self, _other: &Self) -> Option<std::cmp::Ordering> {
        Some(Ordering::Equal)
    }
}

impl PartialEq for LoxFunction {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl LoxFunction {
    pub fn new(declaration: FunStmt, closure: Environment, is_init: bool) -> Self {
        LoxFunction {
            declaration,
            closure,
            is_init,
        }
    }

    pub fn bind(&self, instance: LoxInstance) -> Self {
        let mut environment = self.closure.clone();
        environment.define("this".to_string(), LoxObject::Instance(instance));
        LoxFunction::new(self.declaration.clone(), environment, self.is_init)
    }

    pub fn arity(&self) -> usize {
        self.declaration.params.len()
    }
}

impl LoxCallable for LoxFunction {
    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<LoxObject>) -> LoxObject {
        let mut environment = Environment::new_with_enclosing(self.closure.clone());
        for i in 0..self.arity() {
            environment.define(
                self.declaration.params.get(i).unwrap().lexeme.clone(),
                arguments.get(i).unwrap().clone(),
            )
        }
        let value =
            match interpreter.execute_block(self.declaration.body.clone(), Some(environment)) {
                Ok(_) => LoxObject::None,
                Err(x) => {
                    if self.is_init {
                        return self.closure.get("this".to_string()).unwrap();
                    }
                    x.value
                }
            };
        if self.is_init {
            return self.closure.get("this".to_string()).unwrap();
        }
        value
    }

    fn arity(&self) -> usize {
        self.arity()
    }

    fn to_string(&self) -> String {
        format!("<fun {}", self.declaration.name.lexeme.clone())
    }
}
