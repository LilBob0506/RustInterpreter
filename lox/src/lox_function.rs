use std::rc::Rc;

use crate::interpreter::*;
use crate::stmt::*;
use crate::entities::*;
use crate::callable::*;
use crate::errors::*;
use crate::environment::*;

use std::cell::RefCell;
pub struct LoxFunction {
    name: Token,
    params: Rc<Vec<Token>>,
    body: Rc<Vec<Stmt>>,
    closure: Rc<Environment>,

}

impl LoxFunction {
    pub fn new(declaration: &Rc<FunctionStmt>, closure: &Rc<RefCell<Environment>>) -> Self {
        Self { 
            name: declaration.name.dup(),
            params: Rc::clone(&declaration.params),
            body: Rc::clone(declaration.body),
            closure: Rc::clone(closure),
        }
    }
}

impl LoxCallable for LoxFunction {
    fn call(&self, interpreter: &Interpreter, argument: &Vec<LiteralValue>) -> Result<LiteralValue, LoxResult> {
        let mut e = Environment::new_with_enclosing(Rc::clone(&self.closure));

        for (param, arg) in self.params.iter().zip(argument.iter()) {
            e.define(param.as_string(), arg.clone());
        }

        match interpreter.execute_block(&self.body, e) {
            Err(LoxResult::ReturnValue { value }) => Ok(value),
            Err(e) => Err(e),
            Ok(_) => Ok(LiteralValue::Nil)
        }
    }

    fn arity(&self) -> usize {
        self.params.len()
    }

    fn to_string(&self) -> String {
        self.name.as_string().int()
    }
}
