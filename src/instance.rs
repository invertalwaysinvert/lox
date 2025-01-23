use std::{cell::RefCell, collections::HashMap, fmt::Display, rc::Rc};

use crate::{
    class::LoxClass,
    tokens::{LoxObject, Token},
};

#[derive(Clone, Debug)]
pub struct LoxInstance {
    pub class: LoxClass,
    pub fields: Rc<RefCell<HashMap<String, LoxObject>>>,
}

impl LoxInstance {
    pub fn new(class: LoxClass) -> Self {
        LoxInstance {
            class,
            fields: Rc::new(RefCell::new(HashMap::new())),
        }
    }

    pub fn get(&self, name: Token) -> LoxObject {
        let fields = self.fields.borrow();
        if let Some(x) = fields.get(&name.lexeme) {
            return x.clone();
        };

        if let Some(method) = self.class.find_methods(&name.lexeme) {
            let method = method.bind(self.clone());
            let method = LoxObject::FunCall(Box::new(method));
            return method;
        }

        panic!("Undefined property {}.", name.lexeme);
    }

    pub fn set(&mut self, name: Token, value: LoxObject) {
        let mut fields = self.fields.borrow_mut();
        fields.insert(name.lexeme, value);
    }
}

impl Display for LoxInstance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{} instance>", self.class.name)
    }
}
