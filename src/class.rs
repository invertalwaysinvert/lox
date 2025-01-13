use std::fmt::Display;

use crate::{callable::LoxCallable, instance::LoxInstance, tokens::LoxObject};

#[derive(Clone, Debug)]
pub struct LoxClass {
    pub name: String,
}

impl LoxClass {
    pub fn new(name: String) -> Self {
        LoxClass { name }
    }
}

impl Display for LoxClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl LoxCallable for LoxClass {
    fn call(
        &self,
        _interpreter: &mut crate::interpreter::Interpreter,
        _arguments: Vec<crate::tokens::LoxObject>,
    ) -> crate::tokens::LoxObject {
        LoxObject::Instance(LoxInstance::new(self.clone()))
    }

    fn arity(&self) -> u32 {
        0
    }

    fn to_string(&self) -> String {
        self.name.to_string()
    }
}
