use std::rc::Rc;

use crate::interpreter::*;
use crate::stmt::*;
use crate::entities::*;
use crate::callable::*;
use crate::errors::*;
use crate::environment::*;

pub struct LoxFunction {
    declaration: Rc<Stmt>
}

impl LoxFunction {
    pub fn new(declaration: &Stmt) -> Self {
        Self { 
            declaration: Rc::clone(declaration), 
        }
    }
}

impl LoxCallable for LoxFunction {
    fn call(&self, interpreter: &Interpreter, argument: &Vec<LiteralValue>) -> Result<LiteralValue, LoxResult> {
        if let Stmt::Function(FunctionStmt { name, params, body }) = self.declaration.deref() {
            let mut e = Environment::new_with_enclosing(Rc::clone(interpreter.globals));

            for (param, arg) in self.declaration.params.iter().zip(arguments.iter()) {
                e.borrow_mut().define(param.as_string(), arg);
            }

            interpreter.execute_block(self.declaration.body, environment);
            Ok(LiteralValue::NIL)
        } else {
            panic!();
        }
    }

    fn arity(&self) -> usize {
        if let Stmt::Function(FunctionStmt { name, params, body }) = self.declaration.deref() {
            param.len()
        } else {
            panic!();
        }
    }

    fn to_string(&self) -> String {
        if let Stmt::Function(FunctionStmt { name, params, body }) = self.declaration.deref() {
            param.len()
        } else {
            panic!();
        }
    }
}
