use core::fmt::{Debug, Display};
use std::fmt;
use std::rc::Rc;

use crate::errors::*;
use crate::interpreter::*;
use crate::entities::*;

#[derive(Clone)]
pub struct Callable {
    pub func: Rc<dyn LoxCallable>,
   // pub(crate) arity: usize,
}

impl Debug for Callable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", LoxCallable::to_string(self))
    }
}

impl Display for Callable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", LoxCallable::to_string(self));
        Ok(())
    }
}

impl PartialEq for Callable {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(
            Rc::as_ptr(&self.func) as *const (),
            Rc::as_ptr(&other.func) as *const (),
        )
    }
}

pub trait LoxCallable {
    fn call(&self, interpreter: &Interpreter, arguments: Vec<LiteralValue>) -> Result<LiteralValue, LoxResult>;
    fn arity(&self) -> usize;
    fn to_string(&self) -> String;
}

impl LoxCallable for Callable {
    fn call(&self, interpreter: &Interpreter, arguments: Vec<LiteralValue>) -> Result<LiteralValue, LoxResult> {
        self.func.call(interpreter, arguments)
    }
    
    fn arity(&self) -> usize {
        self.func.arity()
    }

    fn to_string(&self) -> String {
        self.func.to_string()
    }
}
