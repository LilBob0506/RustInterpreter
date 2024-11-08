use std::rc::Rc;

use crate::lox_class::LoxClass;

#[derive(Debug, Clone, PartialEq)]
pub struct LoxInstance {
    klass: Rc<LoxClass>
}

impl LoxInstance {
    pub fn new(klass: Rc<LoxClass>) -> Self {
        Self {
            klass: Rc::clone(&klass),
        }
    }
}