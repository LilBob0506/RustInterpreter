use std::collections::HashMap;

use crate::entities::*;

pub struct Environment {
    enclosing: Option<Box<Environment>>,
    values: HashMap<String, LoxValue>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            enclosing: None,
            values: HashMap::new(),
        }
    }

    pub fn new_enclosing(enclosing: Option<Box<Environment>>) -> Self {
        Environment {
            values: HashMap::new(),
            enclosing: enclosing,
        }
    }

    pub fn assign(&mut self, name: &Token, value: LoxValue) -> Result<(), RuntimeError> {
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme.clone(), value);
            return Ok(());
        }
        else if let Some(enclosing_env) = &mut self.enclosing {
            enclosing_env.assign(name, value);
            return Ok(());
        }else {
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
            return Ok(crate::environment::LoxValue::Nil);
        }
        if let Some(enclosing_env) = &self.enclosing {
            return enclosing_env.get(name);
        }
        Err(LoxError::error(
            name.line,
            format!("Undefined variable '{}'.", name),
        ))
    }
}
