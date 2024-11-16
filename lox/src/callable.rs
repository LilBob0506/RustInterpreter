use core::fmt::{Debug, Display};
use std::fmt;
use std::rc::Rc;

use crate::errors::*;
use crate::interpreter::*;
use crate::entities::*;
use crate::lox_class::*;

#[derive(Clone)]
pub struct Callable {
    pub func: Rc<dyn LoxCallable>,
   // pub(crate) arity: usize,
}

impl Debug for Callable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Callable")
    }
}

impl Display for Callable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<Callable>");
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
    fn call(
        &self,
        interpreter: &Interpreter,
        arguments: Vec<LiteralValue>,
        klass: Option<Rc<LoxClass>>,
    ) -> Result<LiteralValue, LoxResult>;
    fn arity(&self) -> usize;
    
}
