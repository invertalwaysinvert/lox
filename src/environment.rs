use std::{cell::RefCell, collections::HashMap, fmt::Display, rc::Rc};

use crate::{exceptions::RuntimeError, tokens::LoxObject};

#[derive(Debug)]
pub struct Environment {
    pub enclosing: Option<Rc<RefCell<Environment>>>,
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
            enclosing: Some(Rc::new(RefCell::new(enclosing))),
            values: HashMap::new(),
        }
    }

    pub fn new_with_enclosing_rc(enclosing: Rc<RefCell<Environment>>) -> Self {
        Environment {
            enclosing: Some(enclosing),
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, value: LoxObject) {
        self.values.insert(name, value);
    }

    pub fn assign(&mut self, name: String, value: LoxObject) {
        if self.values.get(&name).is_some() {
            self.values.insert(name, value);
            return;
        };

        match &self.enclosing {
            Some(x) => {
                let mut env = x.borrow_mut();
                (*env).assign(name, value);
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
            Some(x) => {
                let env = x.borrow();
                (*env).get(name)
            }
            None => Err(RuntimeError {}),
        }
    }

    pub fn get_at(&self, distance: usize, name: String) -> LoxObject {
        if distance == 0 {
            match self.values.get(&name) {
                Some(x) => x.clone(),
                None => LoxObject::None,
            }
        } else {
            match &self.enclosing {
                Some(env) => self.get_at_helper(Rc::clone(env), distance - 1, name),
                None => panic!("Should have an env here"),
            }
        }
    }

    fn get_at_helper(
        &self,
        env: Rc<RefCell<Environment>>,
        distance: usize,
        name: String,
    ) -> LoxObject {
        if distance > 0 {
            match &env.borrow().enclosing {
                Some(env_ref) => self.get_at_helper(Rc::clone(env_ref), distance - 1, name),
                None => panic!("Should have an env for assigning"),
            }
        } else {
            match env.borrow().values.get(&name) {
                Some(x) => x.clone(),
                None => LoxObject::None,
            }
        }
    }

    pub fn assign_at(&mut self, distance: usize, name: String, value: LoxObject) {
        if distance == 0 {
            self.values.insert(name, value);
        } else {
            match &self.enclosing {
                Some(env) => {
                    self.assign_at_helper(Rc::clone(env), distance - 1, name, value);
                }
                None => panic!("Should have an env here"),
            }
        }
    }

    fn assign_at_helper(
        &self,
        env: Rc<RefCell<Environment>>,
        distance: usize,
        name: String,
        value: LoxObject,
    ) {
        if distance > 0 {
            match &env.borrow().enclosing {
                Some(env_ref) => {
                    self.assign_at_helper(Rc::clone(env_ref), distance - 1, name, value);
                }
                None => panic!("Should have an env for assigning"),
            }
        } else {
            env.borrow_mut().values.insert(name, value);
        }
    }
}

impl Clone for Environment {
    fn clone(&self) -> Self {
        let enclosing = match &self.enclosing {
            Some(env) => Some(Rc::clone(&env)),
            None => None,
        };

        Environment {
            enclosing,
            values: self.values.clone(),
        }
    }
}

impl Display for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let enclosing = match &self.enclosing {
            Some(env) => env.borrow().to_string(),
            None => "None".to_string(),
        };

        write!(
            f,
            "{{keys: {:?}, values: {:?}, enclosing: {} }}",
            self.values.keys(),
            self.values.values(),
            enclosing
        )
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}
