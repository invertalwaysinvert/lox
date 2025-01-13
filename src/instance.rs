use std::fmt::Display;

use crate::class::LoxClass;

#[derive(Clone, Debug)]
pub struct LoxInstance {
    pub class: LoxClass,
}

impl LoxInstance {
    pub fn new(class: LoxClass) -> Self {
        LoxInstance { class }
    }
}

impl Display for LoxInstance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{} instance>", self.class.name)
    }
}
