use std::rc::Rc;
use std::collections::HashMap;
use std::fmt;

use crate::{callable::LoxCallable, entities::LiteralValue, lox_instance::LoxInstance, Interpreter, LoxResult};

#[derive(Debug, Clone, PartialEq)]
pub struct LoxClass {
    name: String,
    methods: HashMap<String, LiteralValue>,
}

impl LoxClass {
    pub fn new(name: &str, methods: HashMap<String, LiteralValue>) -> Self {
        Self {
            name: name.to_string(),
            methods,
        }
    }

    pub fn instantiate(
        &self,
        interpreter: &Interpreter,
        arguments: Vec<LiteralValue>,
        klass: Rc<LoxClass>,
    ) -> Result<LiteralValue, LoxResult> {
        let instance = LiteralValue::Instance(Rc::new(LoxInstance::new(klass)));
        if let Some(LiteralValue::Func(initializer)) = self.find_method("init") {
            if let LiteralValue::Func(init) = initializer.bind(&instance) {
                init.call(interpreter, arguments,None)?;
            }
        }
        Ok(instance)
    }


    pub fn find_method(&self, name: &str) -> Option<LiteralValue> {
        self.methods.get(name).cloned()
    }
}

impl fmt::Display for LoxClass {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let methods = self
            .methods
            .keys()
            .cloned()
            .collect::<Vec<String>>()
            .join(", ");
        write!(f, "<Class {} {{ {methods} }}>", self.name)
    }
}

impl LoxCallable for LoxClass {
    fn call(
        &self,
        interpreter: &Interpreter,
        arguments: Vec<LiteralValue>,
        klass: Option<Rc<LoxClass>>,
    ) -> Result<LiteralValue, LoxResult> {
        self.instantiate(interpreter, arguments, klass.unwrap())
    }
    fn arity(&self) -> usize {
        if let Some(LiteralValue::Func(initializer)) = self.find_method("init") {
            initializer.arity()
        } else {
            0
        }
    }
}
