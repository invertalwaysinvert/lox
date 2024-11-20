use std::collections::HashMap;

use crate::{exceptions::RuntimeError, tokens::TokenLiteral};

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

    pub fn get(&self, name: String) -> Result<TokenLiteral, RuntimeError> {
        match self.values.get(&name) {
            Some(x) => Ok(x.clone()),
            None => Err(RuntimeError {}),
        }
    }
}
