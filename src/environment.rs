use std::collections::HashMap;

use crate::{exceptions::RuntimeError, tokens::LoxObject};

#[derive(Clone)]
pub struct Environment {
    enclosing: Option<Box<Environment>>,
    pub values: HashMap<String, LoxObject>,
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

    pub fn define(&mut self, name: String, value: LoxObject) {
        self.values.insert(name, value);
    }

    pub fn assign(&mut self, name: String, value: LoxObject) {
        if let Some(_x) = self.values.get(&name) {
            self.values.insert(name, value);
            return;
        };

        match &mut self.enclosing {
            Some(x) => {
                (*x).assign(name, value);
                return;
            }
            None => (),
        };
        panic!("Undefined variable '{}'.", name)
    }

    pub fn get(&self, name: String) -> Result<LoxObject, RuntimeError> {
        if let Some(x) = self.values.get(&name) {
            return Ok(x.clone());
        };

        match &self.enclosing {
            Some(x) => (*x).get(name),
            None => Err(RuntimeError {}),
        }
    }
}
