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
    fn arity(&self) -> u32;
    fn to_string(&self) -> String;
}

#[derive(Debug, Clone)]
pub struct LoxFunction {
    pub declaration: FunStmt,
    pub closure: Environment,
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
    pub fn new(declaration: FunStmt, closure: Environment) -> Self {
        LoxFunction {
            declaration,
            closure,
        }
    }

    pub fn bind(&self, instance: LoxInstance) -> Self {
        let mut environment = Environment::new_with_enclosing(self.closure.clone());
        environment.define("This this ".to_string(), LoxObject::Instance(instance));
        LoxFunction::new(self.declaration.clone(), environment)
    }
}

impl LoxCallable for LoxFunction {
    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<LoxObject>) -> LoxObject {
        let mut environment = Environment::new_with_enclosing(self.closure.clone());
        for i in 0..self.arity() {
            environment.define(
                self.declaration
                    .params
                    .get(i as usize)
                    .unwrap()
                    .lexeme
                    .clone(),
                arguments.get(i as usize).unwrap().clone(),
            )
        }
        match interpreter.execute_block(self.declaration.body.clone(), environment) {
            Ok(_) => LoxObject::None,
            Err(x) => x.value,
        }
    }

    fn arity(&self) -> u32 {
        self.declaration.params.len() as u32
    }

    fn to_string(&self) -> String {
        format!("<fun {}", self.declaration.name.lexeme.clone())
    }
}
