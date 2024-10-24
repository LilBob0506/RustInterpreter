use std::collections::HashMap;

use crate::entities::*;

pub struct Environment {
    values: HashMap<String, LoxValue>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            values: HashMap::new(),
        }
    }

    pub fn assign(&mut self, name: &Token, value: LoxValue) -> Result<(), RuntimeError> {
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme.clone(), value);
            Ok(())
        } else {
            let runtime_error = RuntimeError {
                token: name,
                message: &format!("Undefined variable '{}'.", name.lexeme),
            };
            Ok(())
        }
    }

    pub fn define(&mut self, name: String, value: LoxValue) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &Token) -> Result<LoxValue, LoxError> {
        if let Some(_object) = self.values.get(&name.to_string()) {
            Ok(crate::environment::LoxValue::Nil)
        } else {
            Err(LoxError::error(
                name.line,
                format!("Undefined variable '{}'.", name),
            ))
        }
    }

}