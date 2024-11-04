use std::cell::RefCell;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::rc::Rc;
//use crate::entities;
use crate::entities::*;
use crate::errors::*;

#[derive(Debug)]
pub struct Environment {
    values: HashMap<String, LiteralValue>,
    enclosing: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            values: HashMap::new(),
            enclosing: None,
        }
    }

    pub fn new_with_enclosing(enclosing: Rc<RefCell<Environment>>) -> Environment {
        Environment {
            values: HashMap::new(),
            enclosing: Some(enclosing),
        }
    }

    pub fn assign(&mut self, name: &Token, value: LiteralValue) -> Result<(), LoxResult> {
        if let Entry::Occupied(mut object) = self.values.entry(name.as_string().to_string()) {
            object.insert(value);
            Ok(())
        } else if let Some(enclosing) = &self.enclosing {
            enclosing.borrow_mut().assign(name, value)
        } else {
            Err(LoxResult::runtime_error(
                name,
                &format!("Undefined variable '{}'.", name.as_string()),
            ))
        }
    }

    pub fn define(&mut self, name: &str, value: LiteralValue) {
        self.values.insert(name.to_string(), value);
    }

    pub fn get(&self, name: &Token) -> Result<LiteralValue, LoxResult> {
        if let Some(object) = self.values.get(name.as_string()) {
            Ok(object.clone())
        } else if let Some(enclosing) = &self.enclosing {
            enclosing.borrow().get(name)
        } else {
            Err(LoxResult::runtime_error(
                name,
                &format!("Undefined variable '{}'.", name.as_string()),
            ))
        }
    }
}
