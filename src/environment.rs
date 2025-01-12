use std::collections::HashMap;

use crate::{
    exceptions::RuntimeError,
    tokens::{LoxObject, Token},
};

#[derive(Clone, Debug)]
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

    pub fn get_at(&mut self, distance: usize, name: String) -> LoxObject {
        match self.ancestor(distance).values.get(&name) {
            Some(x) => x.clone(),
            None => LoxObject::None,
        }
    }

    pub fn assign_at(&mut self, distance: usize, name: Token, value: LoxObject) {
        self.ancestor(distance).values.insert(name.lexeme, value);
    }

    fn ancestor(&mut self, distance: usize) -> &mut Environment {
        let mut result = self;
        for i in 0..distance {
            if let Some(ref mut enclosing) = result.enclosing {
                result = enclosing;
            }
        }
        result
    }
}
