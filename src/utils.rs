use std::time::{SystemTime, UNIX_EPOCH};

use crate::{callable::LoxCallable, tokens::LoxObject};

#[derive(Debug)]
pub struct Clock {}

impl LoxCallable for Clock {
    fn arity(&self) -> u32 {
        0
    }

    fn call(
        &self,
        _interpreter: &mut crate::interpreter::Interpreter,
        _arguments: Vec<crate::tokens::LoxObject>,
    ) -> crate::tokens::LoxObject {
        LoxObject::Number(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as f32,
        )
    }

    fn to_string(&self) -> String {
        "<native fn>".to_string()
    }
}
