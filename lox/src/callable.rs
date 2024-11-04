use std::fmt::Debug;

use crate::errors::*;
use crate::interpreter::*;

#[derive(Clone)]
pub struct Callable {
    func: Rc<dyn LoxCallable>
}

impl Debug for Callable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Callable").field("func", &self.func).finish()
    }
}

impl PartialEq for Callable {
    fn eq(&self, other: &Self) -> bool {
        self.func == other.func
    }
}

pub trait LoxCallable {
    fn call(&self, interpreter: &Interpreter, arguments: Vec<Object>) -> Result<Object, LoxResult>;
}

impl LoxCallable for Callable {
    fn call(&self, _interpreter: &Interpreter, _arguments: Vec<Object>) -> Result<Object, LoxResult> {
        self.func.call(interpreter, arguments)
    }
}
