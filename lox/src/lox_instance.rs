use std::{cell::RefCell, collections::{hash_map::Entry, HashMap}, rc::Rc};
use std::fmt;
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

    pub fn get(&self, name: &Token, this: &Rc<LoxInstance>) -> Result<LiteralValue, LoxResult> {
        if let Entry::Occupied(o) = self.fields.borrow_mut().entry(name.as_string()) {
            Ok(o.get().clone())
        } else if let Some(method) = self.klass.find_method(&name.as_string()) {
            if let LiteralValue::Func(func) = method {
                Ok(func.bind(&LiteralValue::Instance(Rc::clone(this))))
            } else {
                panic!("tried to bind 'this' to a non-function {method:?}");
            }
        } else {
            Err(LoxResult::runtime_error(
                name,
                &format!("Undefined property '{}'.", name.as_string()),
            ))
        }
    }


    pub fn set(&self, name: &Token, value: LiteralValue) {
        self.fields.borrow_mut().insert(name.as_string(), value);
    }
}


impl fmt::Display for LoxInstance {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} instance", self.klass)
    }
}