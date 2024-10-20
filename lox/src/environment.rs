use std::collections::HashMap;

use crate::entities::*;
use crate::error::*;

pub struct Environment {
    values: HashMap<String, LoxValue>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, value: LoxValue) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &Token) -> Result<LoxValue, LoxError> {
        if let Some(object) = self.values.get(name.to_string()) {
            Ok(object);
        } else {
            Err(LoxError::error(
                &usize,
                format!("Undefined variable '{}'.",),
            ))
        }
    }
}
