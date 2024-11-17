use std::fmt;
use std::rc::Rc;
use std::time::SystemTime;

use crate::callable::*;
use crate::errors::*;
use crate::interpreter::*;
use crate::lox_class::*;
use crate::entities::*;


#[derive(Clone)]
pub struct LoxNative {
    pub func: Rc<dyn LoxCallable>,
}
impl PartialEq for LoxNative {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(
            Rc::as_ptr(&self.func) as *const (),
            Rc::as_ptr(&other.func) as *const (),
        )
    }
}
impl fmt::Debug for LoxNative {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<Native-Function>")
    }
}
impl fmt::Display for LoxNative {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<native fn>")
    }
}
pub struct NativeClock;

impl LoxCallable for NativeClock {
    fn call(
        &self,
        _terp: &Interpreter,
        _args: Vec<LiteralValue>,
        _klass: Option<Rc<LoxClass>>,
    ) -> Result<LiteralValue, LoxResult> {
        match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(n) => Ok(LiteralValue::Num(n.as_millis() as f64)),
            Err(e) => Err(LoxResult::system_error(&format!(
                "Clock returned invalid duration: {:?}",
                e.duration()
            ))),
        }
    }

    fn arity(&self) -> usize {
        0
    }
}
