use std::{collections::HashMap, fmt::Display};

use crate::{
    callable::{LoxCallable, LoxFunction},
    instance::LoxInstance,
    tokens::LoxObject,
};

#[derive(Clone, Debug)]
pub struct LoxClass {
    pub name: String,
    pub superclass: Box<Option<LoxClass>>,
    pub methods: HashMap<String, LoxFunction>,
}

impl LoxClass {
    pub fn new(
        name: String,
        superclass: Option<LoxClass>,
        methods: HashMap<String, LoxFunction>,
    ) -> Self {
        LoxClass {
            name,
            superclass: Box::new(superclass),
            methods,
        }
    }

    pub fn find_methods(&self, name: &str) -> Option<LoxFunction> {
        match self.methods.get(name) {
            Some(x) => Some(x.clone()),
            None => {
                if let Some(superclass) = *self.superclass.clone() {
                    superclass.find_methods(name)
                } else {
                    None
                }
            }
        }
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
        interpreter: &mut crate::interpreter::Interpreter,
        arguments: Vec<crate::tokens::LoxObject>,
    ) -> crate::tokens::LoxObject {
        let mut instance = LoxObject::Instance(LoxInstance::new(self.clone()));
        if let Some(init) = self.find_methods("init") {
            instance = init
                .bind(LoxInstance::new(self.clone()))
                .call(interpreter, arguments);
        }
        instance
    }

    fn arity(&self) -> u32 {
        if let Some(init) = self.find_methods("init") {
            init.arity()
        } else {
            0
        }
    }

    fn to_string(&self) -> String {
        self.name.to_string()
    }
}
