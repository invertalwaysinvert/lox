use std::collections::HashMap;

use crate::{
    exceptions::RuntimeError,
    tokens::{Token, TokenLiteral},
};

pub struct Environment {
    pub values: HashMap<String, TokenLiteral>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, value: TokenLiteral) {
        self.values.insert(name, value);
    }

    pub fn assign(&mut self, name: Token, value: TokenLiteral) {
        match self.values.get(&name.lexeme) {
            Some(_x) => self.values.insert(name.lexeme, value),
            None => panic!("Undefined variable '{}'.", name.lexeme),
        };
    }

    pub fn get(&self, name: String) -> Result<TokenLiteral, RuntimeError> {
        match self.values.get(&name) {
            Some(x) => Ok(x.clone()),
            None => Err(RuntimeError {}),
        }
    }
}
