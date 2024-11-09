use std::{cell::RefCell, collections::{hash_map::Entry, HashMap}, rc::Rc};

use crate::{entities::{LiteralValue, Token}, lox_class::LoxClass, LoxResult};

#[derive(Debug, Clone, PartialEq)]
pub struct LoxInstance {
    klass: Rc<LoxClass>,
    fields: RefCell<HashMap<String, LiteralValue>>,
}

impl LoxInstance {
    pub fn new(klass: Rc<LoxClass>) -> Self {
        Self {
            klass: Rc::clone(&klass),
            fields: RefCell::new(HashMap::new()),
        }
    }

    pub fn get(&self, name: &Token) -> Result<LiteralValue, LoxResult> {
        if let Entry::Occupied(o) = self.fields.borrow_mut().entry(name.as_string()) {
            Ok(o.get().clone())
        }
        else {
            Err(LoxResult::runtime_error(
                name, 
                &format!("Undefined property '{}'.", name.as_string()),
            ))
        }
    }

    pub fn set(&self, name: &Token, value: LiteralValue) {
        println!("setting {:?} to {:?}", name, value);
        self.fields.borrow_mut().insert(name.as_string(), value);
    }
}

impl std::string::ToString for LoxInstance {
    fn to_string(&self) -> String {
        format!("<Instance of {}", self.klass.to_string())
    }
}