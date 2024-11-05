use core::fmt;
use std::fmt::Debug;

use crate::errors::*;
use crate::interpreter::*;

#[derive(Clone)]
pub struct Callable {
    pub func: Rc<dyn LoxCallable>,
    pub(crate) arity: usize,
}

impl Debug for Callable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", LoxCallable::to_string(self))
    }
}

impl Display for Callable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", LoxCallable::to_string(&self));
        Ok(())
    }
}

impl PartialEq for Callable {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.func, &other.func)
    }
}

pub trait LoxCallable {
    fn call(&self, interpreter: &Interpreter, arguments: Vec<Object>) -> Result<Object, LoxResult>;
    fn arity(&self) -> usize;
    fn to_string(&self) -> String;
}

impl LoxCallable for Callable {
    fn call(&self, _interpreter: &Interpreter, _arguments: Vec<Object>) -> Result<Object, LoxResult> {
        self.func.call(interpreter, arguments)
    }
    
    fn arity(&self) -> usize {
        self.arity
    }

    fn arity(&self) -> usize {
        self.func.arity()
    }

    fn to_string(&self) -> String {
        self.func.to_string()
    }
}
