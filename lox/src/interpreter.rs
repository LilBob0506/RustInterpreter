//use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use std::ops::Deref;

use crate::lox_class::*;
use crate::callable::*;
use crate::entities::*;
use crate::environment::*;
use crate::errors::*;
use crate::expr::*;
use crate::lox_function::*;
use crate::stmt::*;
use crate::native_functions::*;
//use crate::lox_function::*;
#[derive()]

pub struct Interpreter {
   pub globals: Rc<RefCell<Environment>>,
    environment: RefCell<Rc<RefCell<Environment>>>,
    //nest: RefCell<usize>,
    locals: RefCell<HashMap<Rc<Expr>, usize>>,
}

impl StmtVisitor<()> for Interpreter {
    fn visit_break_stmt(&self, _: Rc<Stmt>, _stmt: &BreakStmt) -> Result<(), LoxResult> {
        Err(LoxResult::Break)
    }
    fn visit_while_stmt(&self, _: Rc<Stmt>, stmt: &WhileStmt) -> Result<(), LoxResult> {
        while self.is_truthy(&self.evaluate(stmt.condition.clone())?) {
            match self.execute(stmt.body.clone()) {
                Err(LoxResult::Break) => break,
                Err(e) => return Err(e),
                Ok(_) => {}
            }
        }

        Ok(())
    }
    fn visit_expression_stmt(&self, _: Rc<Stmt>, stmt: &ExpressionStmt) -> Result<(), LoxResult> {
        self.evaluate(stmt.expression.clone())?;
        Ok(())
    }
    fn visit_print_stmt(&self, _: Rc<Stmt>, stmt: &PrintStmt) -> Result<(), LoxResult> {
        let value = self.evaluate(stmt.expression.clone())?;
        println!("{value}");
        Ok(())
    }

    fn visit_if_stmt(&self, _: Rc<Stmt>, stmt: &IfStmt) -> Result<(), LoxResult> {
        if self.is_truthy(&self.evaluate(stmt.condition.clone())?) {
            self.execute(stmt.then_branch.clone())
        } else if let Some(else_branch) = stmt.else_branch.clone() {
            self.execute(else_branch)
        } else {
            Ok(())
        }
    }

    fn visit_block_stmt(&self, _: Rc<Stmt>, stmt: &BlockStmt) -> Result<(), LoxResult> {
        let e = Environment::new_with_enclosing(self.environment.borrow().clone());
        self.execute_block(&stmt.statements, e)
    }

    fn visit_var_stmt(&self, _: Rc<Stmt>, stmt: &VarStmt) -> Result<(), LoxResult> {
        let value = if let Some(initializer) = stmt.initializer.clone() {
            self.evaluate(initializer)?
        } else {
            LiteralValue::Nil
        };

        self.environment
            .borrow()
            .borrow_mut()
            .define(&stmt.name.as_string(), value);
        Ok(())
    }

    
    fn visit_function_stmt(&self, _: Rc<Stmt>, stmt: &FunctionStmt) -> Result<(), LoxResult> {
        let function = LoxFunction::new(stmt, self.environment.borrow().deref(), false);
        self.environment
        .borrow()
        .borrow_mut()
        .define(&stmt.name.as_string(), LiteralValue::Func(Rc::new(function)));
        Ok(())
    }
    
    fn visit_return_stmt(&self, _: Rc<Stmt>, stmt: &ReturnStmt) -> Result<(), LoxResult> {
        if let Some(value) = stmt.value.clone() {
            Err(LoxResult::return_value(self.evaluate(value)?))
        } else {
            Err(LoxResult::return_value(LiteralValue::Nil))
        }
    }
    
    fn visit_class_stmt(&self, _: Rc<Stmt>, stmt: &ClassStmt) -> Result<(), LoxResult> {
        let superclass = if let Some(superclass_expr) = &stmt.superclass {
            let superclass = self.evaluate(superclass_expr.clone())?;

            if let LiteralValue::Class(c) = superclass {
                Some(c)
            } else if let Expr::Variable(v) = superclass_expr.deref() {
                return Err(LoxResult::runtime_error(
                    &v.name,
                    "Superclass must be a class",
                ));
            } else {
                panic!();
            }
        } else {
            None
        };

        self.environment
            .borrow()
            .borrow_mut()
            .define(&stmt.name.as_string(), LiteralValue::Nil);

        let enclosing = if let Some(ref s) = superclass {
            let mut e = Environment::new_with_enclosing(self.environment.borrow().clone());
            e.define("super", LiteralValue::Class(s.clone()));
            Some(self.environment.replace(Rc::new(RefCell::new(e))))
        } else {
            None
        };

        let mut methods = HashMap::new();
        for method in stmt.methods.deref() {
            if let Stmt::Function(func) = method.deref() {
                 let is_init = func.name.as_string() == "init";
                let function = LiteralValue::Func(Rc::new(LoxFunction::new(
                    func,
                    &self.environment.borrow(),
                    is_init,
                )));
                methods.insert(func.name.as_string(), function);
            } else {
                panic!("non-function method in class");
            };
        }

        let klass = LiteralValue::Class(Rc::new(LoxClass::new(
            &stmt.name.as_string(), 
            superclass, 
            methods
        )));

        if let Some(previous) = enclosing {
            self.environment.replace(previous);
        }

        self.environment
            .borrow()
            .borrow_mut()
            .assign(&stmt.name, klass)?;

        Ok(())
    }

}
impl ExprVisitor<LiteralValue> for Interpreter {
    fn visit_this_expr(&self, wrapper: Rc<Expr>, expr: &ThisExpr) -> Result<LiteralValue, LoxResult> {
        self.look_up_variable(&expr.keyword, wrapper)
    }

    fn visit_literal_expr(&self, _: Rc<Expr>, expr: &LiteralExpr) -> Result<LiteralValue, LoxResult> {
        Ok(expr.value.clone().unwrap())
    }
    fn visit_grouping_expr(&self, _: Rc<Expr>, expr: &GroupingExpr) -> Result<LiteralValue, LoxResult> {
        self.evaluate(expr.expression.clone())
    }
    fn visit_binary_expr(&self, _: Rc<Expr>, expr: &BinaryExpr) -> Result<LiteralValue, LoxResult> {
        let left = self.evaluate(expr.left.clone())?;
        let right = self.evaluate(expr.right.clone())?;
        let op = expr.operator.token_type();

        let result = match (left, right) {
            (LiteralValue::Num(left), LiteralValue::Num(right)) => match op {
                TokenType::MINUS => LiteralValue::Num(left - right),
                TokenType::SLASH => LiteralValue::Num(left / right),
                TokenType::STAR => LiteralValue::Num(left * right),
                TokenType::PLUS => LiteralValue::Num(left + right),
                TokenType::GREATER => LiteralValue::Bool(left > right),
                TokenType::GREATER_EQUAL => LiteralValue::Bool(left >= right),
                TokenType::LESS => LiteralValue::Bool(left < right),
                TokenType::LESS_EQUAL => LiteralValue::Bool(left <= right),
                TokenType::BANG_EQUAL => LiteralValue::Bool(left != right),
                TokenType::EQUAL_EQUAL => LiteralValue::Bool(left == right),
                _ => {
                    todo!();
                }
            },
            (LiteralValue::Num(left), LiteralValue::Str(right)) => match op {
                TokenType::PLUS => LiteralValue::Str(format!("{left}{right}")),
                TokenType::EQUAL_EQUAL => LiteralValue::Bool(false),
                TokenType::BANG_EQUAL => LiteralValue::Bool(true),
                _ => LiteralValue::ArithmeticError,
            },
            (LiteralValue::Str(left), LiteralValue::Num(right)) => match op {
                TokenType::PLUS => LiteralValue::Str(format!("{left}{right}")),
                TokenType::EQUAL_EQUAL => LiteralValue::Bool(false),
                TokenType::BANG_EQUAL => LiteralValue::Bool(true),
                _ => LiteralValue::ArithmeticError,
            },
            (LiteralValue::Str(left), LiteralValue::Str(right)) => match op {
                TokenType::PLUS => LiteralValue::Str(format!("{left}{right}")),
                TokenType::BANG_EQUAL => LiteralValue::Bool(left != right),
                TokenType::EQUAL_EQUAL => LiteralValue::Bool(left == right),
                _ => LiteralValue::ArithmeticError,
            },
            (LiteralValue::Bool(left), LiteralValue::Bool(right)) => match op {
                TokenType::BANG_EQUAL => LiteralValue::Bool(left != right),
                TokenType::EQUAL_EQUAL => LiteralValue::Bool(left == right),
                _ => LiteralValue::ArithmeticError,
            },
            (LiteralValue::Bool(_), LiteralValue::Str(_)) | (LiteralValue::Str(_), LiteralValue::Bool(_)) => match op {
                TokenType::BANG_EQUAL => LiteralValue::Bool(true),
                TokenType::EQUAL_EQUAL => LiteralValue::Bool(false),
                _ => LiteralValue::NumsOrStringsError,
            },
            (LiteralValue::Nil, LiteralValue::Nil) => match op {
                TokenType::BANG_EQUAL => LiteralValue::Bool(false),
                TokenType::EQUAL_EQUAL => LiteralValue::Bool(true),
                _ => LiteralValue::NumsOrStringsError,
            },
            (LiteralValue::Nil, _) | (_, LiteralValue::Nil) => match op {
                TokenType::EQUAL_EQUAL => LiteralValue::Bool(false),
                TokenType::BANG_EQUAL => LiteralValue::Bool(true),
                _ => LiteralValue::NumsOrStringsError,
            },
            (LiteralValue::Func(a), LiteralValue::Func(b)) => LiteralValue::Bool(Rc::ptr_eq(&a, &b)),
            (LiteralValue::Class(a), LiteralValue::Class(b)) => LiteralValue::Bool(Rc::ptr_eq(&a, &b)),
            _ => match op {
                TokenType::BANG_EQUAL => LiteralValue::Bool(true),
                TokenType::EQUAL_EQUAL => LiteralValue::Bool(false),
                TokenType::PLUS => LiteralValue::NumsOrStringsError,
                _ => LiteralValue::ArithmeticError,
            },
        };


            match result {
                LiteralValue::ArithmeticError => Err(LoxResult::runtime_error(
                &expr.operator,
                "Operands must be numbers.",
            )),
            LiteralValue::NumsOrStringsError => Err(LoxResult::runtime_error(
                &expr.operator,
                "Operands must be two numbers or two strings.",
            )),
            _ => Ok(result),
            
        }
    }
    fn visit_unary_expr(&self, _: Rc<Expr>, expr: &UnaryExpr) -> Result<LiteralValue, LoxResult> {
        let right = self.evaluate(expr.right.clone())?;
        match expr.operator.token_type() {
            TokenType::MINUS => match right {
                LiteralValue::Num(n) => return Ok(LiteralValue::Num(-n)),
                _ => Err(LoxResult::runtime_error(
                    &expr.operator,
                    "Operand must be a number.",
                )),
            },
            TokenType::BANG => Ok(LiteralValue::Bool(!self.is_truthy(&right))),
            _ => Err(LoxResult::error(
                expr.operator.line,
                "Unreachable according to Nystrom",
            )),
        }
    }

    fn visit_variable_expr(
        &self,
        wrapper: Rc<Expr>,
        expr: &VariableExpr,
    ) -> Result<LiteralValue, LoxResult> {
        // self.environment.borrow().borrow().get(&expr.name)
        self.look_up_variable(&expr.name, wrapper)
    }
    
    fn visit_logical_expr(&self, _: Rc<Expr>, expr: &LogicalExpr) -> Result<LiteralValue, LoxResult> {
        let left = self.evaluate(expr.left.clone())?;

        if expr.operator.is(TokenType::OR) {
            if self.is_truthy(&left) {
                return Ok(left);
            }
        } else if !self.is_truthy(&left) {
            return Ok(left);
        }

        self.evaluate(expr.right.clone())
    }

    fn visit_assign_expr(&self, wrapper: Rc<Expr>, expr: &AssignExpr) -> Result<LiteralValue, LoxResult> {
        let value = self.evaluate(expr.value.clone())?;
        if let Some(distance) = self.locals.borrow().get(&wrapper) {
            self.environment.borrow().borrow_mut().assign_at(
                *distance,
                &expr.name,
                value.clone(),
            )?;
        } else {
            self.globals
                .borrow_mut()
                .assign(&expr.name, value.clone())?;
        }
        Ok(value)
    }
    
    fn visit_call_expr(&self, _: Rc<Expr>, expr: &CallExpr) -> Result<LiteralValue, LoxResult> {
        let callee = self.evaluate(expr.callee.clone())?;

        let mut arguments = Vec::new();
        for argument in expr.arguments.clone() {
            arguments.push(self.evaluate(argument)?);
        }

        let (callfunc, klass): (Option<Rc<dyn LoxCallable>>, Option<Rc<LoxClass>>) = match callee {
            LiteralValue::Func(f) => (Some(f), None),
            LiteralValue::Native(n) => (Some(n.func.clone()), None),
            LiteralValue::Class(c) => {
                let klass = Rc::clone(&c);
                (Some(c), Some(klass))
            }
            _ => (None, None),
        };
        if let Some(callfunc) = callfunc {
            if arguments.len() != callfunc.arity() {
                return Err(LoxResult::runtime_error(
                    &expr.paren,
                    &format!(
                        "Expected {} arguments but got {}.",
                        callfunc.arity(),
                        arguments.len()
                    ),
                ));
            }
            callfunc.call(self, arguments, klass)
        } else {
            Err(LoxResult::runtime_error(
                &expr.paren,
                "Can only call functions and classes",
            ))
        }
    }
    
    fn visit_get_expr(&self, _: Rc<Expr>, expr: &GetExpr) -> Result<LiteralValue, LoxResult> {
        let literalvalue = self.evaluate(expr.literalvalue.clone())?;
        if let LiteralValue::Instance(inst) = literalvalue {
             Ok(inst.get(&expr.name, &inst)?)
        } else {
            Err(LoxResult::runtime_error(&expr.name, "Only instances have properties."))
        }
    }
    
    fn visit_set_expr(&self, _: Rc<Expr>, expr: &SetExpr) -> Result<LiteralValue, LoxResult> {
        let literalvalue = self.evaluate(expr.literalvalue.clone())?;
        if let LiteralValue::Instance(inst) = literalvalue {
            let value = self.evaluate(expr.value.clone())?;
            inst.set(&expr.name, value.clone());
            Ok(value)
        } else {
            Err(LoxResult::runtime_error(
                &expr.name,
                 "Only instances have fields.",
            ))
        }
    }
    
    fn visit_super_expr(&self, wrapper: Rc<Expr>, expr: &SuperExpr) -> Result<LiteralValue, LoxResult> {
        let distance = *self.locals.borrow().get(&wrapper).unwrap();
        let superclass = if let Ok(LiteralValue::Class(superclass)) =
            self.environment.borrow().borrow().get_at(distance, "super")
            {
                superclass
            } else {
                panic!();
            };

        let literalvalue = self 
            .environment
            .borrow()
            .borrow()
            .get_at(distance - 1, "this")
            .ok()
            .unwrap();

        if let Some(method) = superclass.find_method(&expr.method.as_string()) {
            if let LiteralValue::Func(func) = method {
                Ok(func.bind(&literalvalue))
            } else {
                panic!("method was not a function");
            }
        } else {
            Err(LoxResult::runtime_error(
            &expr.method,
            &format!("Undefined property '{}'.", expr.method.as_string()),
            ))
        
        }
        
    }
}

impl Interpreter {
    pub fn new() -> Interpreter {
        let globals = Rc::new(RefCell::new(Environment::new()));

        globals.borrow_mut().define(
            "clock",
            LiteralValue::Native(Rc::new(LoxNative {
                func: Rc::new(NativeClock {}),
            })),
        );

        Interpreter {
            globals: Rc::clone(&globals), 
            environment: RefCell::new(Rc::clone(&globals)),
            locals: RefCell::new(HashMap::new()),
        }
    }

    fn evaluate(&self, expr: Rc<Expr>) -> Result<LiteralValue, LoxResult> {
        expr.accept(expr.clone(), self)
    }

    fn execute(&self, stmt: Rc<Stmt>) -> Result<(), LoxResult> {
        stmt.accept(stmt.clone(), self)
    }

    pub fn execute_block(
        &self,
        statements: &Rc<Vec<Rc<Stmt>>>,
        environment: Environment,
    ) -> Result<(), LoxResult> {
        let previous = self.environment.replace(Rc::new(RefCell::new(environment)));
        
        let result = statements
            .iter()
            .try_for_each(|statement| self.execute(statement.clone()));

        self.environment.replace(previous);

        result
    }

    // Anything that is not Nil or False is true
    fn is_truthy(&self, literal_value: &LiteralValue) -> bool {
        !matches!(literal_value, LiteralValue::Nil | LiteralValue::Bool(false))
    }
    pub fn interpret(&self, statements: &[Rc<Stmt>]) -> Result<(), LoxResult> {
        for statement in statements {
            self.execute(statement.clone())?;
        }
        Ok(())
    }
    pub fn print_environment(&self) {
        println!("{:?}", self.environment);
    }
    pub fn resolve(&self, expr: Rc<Expr>, depth: usize) {
        self.locals.borrow_mut().insert(expr, depth);
    }

    fn look_up_variable(&self, name: &Token, expr: Rc<Expr>) -> Result<LiteralValue, LoxResult> {
        if let Some(distance) = self.locals.borrow().get(&expr) {
            self.environment
                .borrow()
                .borrow()
                .get_at(*distance, &name.as_string())
        } else {
            self.globals.borrow().get(name)
        }
}
}

