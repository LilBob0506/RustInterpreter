use std::rc::Rc;
use std::collections::HashMap;
use std::fmt;

use crate::{callable::LoxCallable, entities::LiteralValue, lox_instance::LoxInstance, Interpreter, LoxResult};

#[derive(Debug, Clone, PartialEq)]
pub struct LoxClass {
    name: String,
    methods: HashMap<String, LiteralValue>,
    superclass: Option<Rc<LoxClass>>,
}

impl LoxClass {
    pub fn new(name: &str, superclass: Option<Rc<LoxClass>>, methods: HashMap<String, LiteralValue>) -> Self {
        Self {
            name: name.to_string(),
            methods,
            superclass,
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
        self.methods.get(name).cloned();

        if let Some(method) = self.methods.get(name) {
            Some(method.clone())
        } else if let Some(superclass) = &self.superclass {
            superclass.find_method(name)
        } else {
            None
        }

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
