use std::rc::Rc;

use crate::{callable::LoxCallable, entities::LiteralValue, lox_instance::LoxInstance, Interpreter, LoxResult};

#[derive(Debug, Clone, PartialEq)]
pub struct LoxClass {
    name: String,
    myref: Option<Rc<LoxClass>>
}

impl LoxClass {
    pub fn new(name: &String) -> Self {
        Self {
            name: name.clone(),
            myref: None,
        }
    }

    pub fn instantiate(
        &self, 
        _interpreter: &Interpreter, 
        _arguments: Vec<LiteralValue>, 
        klass: Rc<LoxClass>) -> Result<LiteralValue, LoxResult> {
            Ok(LiteralValue::Instance(Rc::new(LoxInstance::new(klass))))
    }

    pub fn set_ref(&mut self, myref: Rc<LoxClass>) {
        self.myref = Some(myref)
    }
}

impl std::string::ToString for LoxClass {
    fn to_string(&self) -> String {
        self.name.clone()
    }
}

impl LoxCallable for LoxClass {
    fn call(&self, _interpreter: &Interpreter, _arguments: Vec<LiteralValue>) -> Result<LiteralValue, LoxResult> {
        todo!()
    }
    fn arity(&self) -> usize {
        0
    }
}
