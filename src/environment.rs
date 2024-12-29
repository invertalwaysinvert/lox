use std::collections::HashMap;

use crate::{
    exceptions::RuntimeError,
    tokens::{Token, TokenLiteral},
};

#[derive(Clone)]
pub struct Environment {
    enclosing: Option<Box<Environment>>,
    pub values: HashMap<String, TokenLiteral>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            enclosing: None,
            values: HashMap::new(),
        }
    }

    pub fn new_with_enclosing(enclosing: Environment) -> Self {
        Environment {
            enclosing: Some(Box::new(enclosing)),
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, value: TokenLiteral) {
        self.values.insert(name, value);
    }

    pub fn assign(&mut self, name: Token, value: TokenLiteral) {
        if let Some(_x) = self.values.get(&name.lexeme) {
            self.values.insert(name.lexeme, value);
            return;
        };

        match &mut self.enclosing {
            Some(x) => {
                (*x).assign(name.clone(), value);
                return;
            }
            None => (),
        };
        panic!("Undefined variable '{}'.", name.lexeme)
    }

    pub fn get(&self, name: String) -> Result<TokenLiteral, RuntimeError> {
        if let Some(x) = self.values.get(&name) {
            return Ok(x.clone());
        };

        match &self.enclosing {
            Some(x) => (*x).get(name),
            None => Err(RuntimeError {}),
        }
    }
}
