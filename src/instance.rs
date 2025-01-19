use std::{collections::HashMap, fmt::Display};

use crate::{
    class::LoxClass,
    tokens::{LoxObject, Token},
};

#[derive(Clone, Debug)]
pub struct LoxInstance {
    pub class: LoxClass,
    pub fields: HashMap<String, LoxObject>,
}

impl LoxInstance {
    pub fn new(class: LoxClass) -> Self {
        LoxInstance {
            class,
            fields: HashMap::new(),
        }
    }

    pub fn get(&self, name: Token) -> LoxObject {
        if let Some(x) = self.fields.get(&name.lexeme) {
            return x.clone();
        };

        if let Some(method) = self.class.find_methods(&name.lexeme) {
            let method = method.bind(self.clone()); // TODO: Should not be cloning here, methods are now
                                                    // unattached from the instances
            let method = LoxObject::Callable(Box::new(method));
            return method;
        }

        panic!("Undefined property {}.", name.lexeme);
    }

    pub fn set(&mut self, name: Token, value: LoxObject) {
        self.fields.insert(name.lexeme, value);
    }
}

impl Display for LoxInstance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{} instance>", self.class.name)
    }
}
