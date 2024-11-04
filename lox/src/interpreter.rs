use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::Rc;

use crate::callable::Callable;
use crate::callable::LoxCallable;
use crate::entities::*;
use crate::environment::*;
use crate::errors::*;
use crate::expr::*;
use crate::stmt::*;
#[derive()]

pub struct Interpreter {
    globals: Rc<RefCell<Environment>>,
    environment: RefCell<Rc<RefCell<Environment>>>,
    nest: RefCell<usize>,
}

impl StmtVisitor<()> for Interpreter {
    fn visit_break_stmt(&self, stmt: &BreakStmt) -> Result<(), LoxResult> {
        if *self.nest.borrow() == 0 {
            Err(LoxResult::runtime_error(
                &stmt.token,
                "break outside of while/for loop",
            ))
        } else {
            Err(LoxResult::Break)
        }
    }
    fn visit_while_stmt(&self, stmt: &WhileStmt) -> Result<(), LoxResult> {
        *self.nest.borrow_mut() += 1;
        while self.is_truthy(&self.evaluate(&stmt.condition)?) {
            match self.execute(&stmt.body) {
                Err(LoxResult::Break) => break,
                Err(e) => return Err(e),
                Ok(_) => {}
            }
        }
        *self.nest.borrow_mut() -= 1;
        Ok(())
    }
    fn visit_expression_stmt(&self, stmt: &ExpressionStmt) -> Result<(), LoxResult> {
        self.evaluate(&stmt.expression)?;
        Ok(())
    }
    fn visit_print_stmt(&self, stmt: &PrintStmt) -> Result<(), LoxResult> {
        let value = self.evaluate(&stmt.expression)?;
        println!("{value}");
        Ok(())
    }

    fn visit_if_stmt(&self, stmt: &IfStmt) -> Result<(), LoxResult> {
        if self.is_truthy(&self.evaluate(&stmt.condition)?) {
            self.execute(&stmt.then_branch)
        } else if let Some(else_branch) = &stmt.else_branch {
            self.execute(else_branch)
        } else {
            Ok(())
        }
    }

    fn visit_block_stmt(&self, stmt: &BlockStmt) -> Result<(), LoxResult> {
        let e = Environment::new_with_enclosing(self.environment.borrow().clone());
        self.execute_block(&stmt.statements, e)
    }

    fn visit_var_stmt(&self, stmt: &VarStmt) -> Result<(), LoxResult> {
        let value = if let Some(initializer) = &stmt.initializer {
            self.evaluate(initializer)?
        } else {
            LiteralValue::Nil
        };

        self.environment
            .borrow()
            .borrow_mut()
            .define(stmt.name.as_string(), value);
        Ok(())
    }
    
    fn visit_function_stmt(&self, expr: &FunctionStmt) -> Result<(), LoxResult> {
        Ok(())
    }
}

impl ExprVisitor<LiteralValue> for Interpreter {
    fn visit_literal_expr(&self, expr: &LiteralExpr) -> Result<LiteralValue, LoxResult> {
        Ok(expr.value.clone().unwrap())
    }
    fn visit_grouping_expr(&self, expr: &GroupingExpr) -> Result<LiteralValue, LoxResult> {
        Ok(self.evaluate(&expr.expression)?)
    }
    fn visit_binary_expr(&self, expr: &BinaryExpr) -> Result<LiteralValue, LoxResult> {
        let left = self.evaluate(&expr.left)?;
        let right = self.evaluate(&expr.right)?;
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
                TokenType::EQUAL => LiteralValue::Bool(left == right),
                _ => {
                    todo!();
                }
            },

            (LiteralValue::Num(left), LiteralValue::Str(right)) => match op {
                TokenType::PLUS => LiteralValue::Str(format!("{left}{right}")),
                _ => LiteralValue::ArithmeticError,
            },
            (LiteralValue::Str(left), LiteralValue::Num(right)) => match op {
                TokenType::PLUS => LiteralValue::Str(format!("{left}{right}")),
                _ => LiteralValue::ArithmeticError,
            },
            (LiteralValue::Str(left), LiteralValue::Str(right)) => match op {
                TokenType::PLUS => LiteralValue::Str(format!("{left}{right}")),
                TokenType::BANG_EQUAL => LiteralValue::Bool(left != right),
                TokenType::EQUAL => LiteralValue::Bool(left == right),
                _ => LiteralValue::ArithmeticError,
            },
            (LiteralValue::Bool(left), LiteralValue::Bool(right)) => match op {
                TokenType::BANG_EQUAL => LiteralValue::Bool(left != right),
                TokenType::EQUAL => LiteralValue::Bool(left == right),
                _ => LiteralValue::ArithmeticError,
            },
            (LiteralValue::Nil, LiteralValue::Nil) => match op {
                TokenType::BANG_EQUAL => LiteralValue::Bool(false),
                TokenType::EQUAL => LiteralValue::Bool(true),
                _ => LiteralValue::ArithmeticError,
            },
            (LiteralValue::Nil, _) => match op {
                TokenType::EQUAL => LiteralValue::Bool(false),
                TokenType::BANG_EQUAL => LiteralValue::Bool(true),
                _ => LiteralValue::ArithmeticError,
            },
            _ => LiteralValue::ArithmeticError,
        };

        if result == LiteralValue::ArithmeticError {
            Err(LoxResult::runtime_error(
                &expr.operator,
                "Illegal expression",
            ))
        } else {
            Ok(result)
        }
    }
    fn visit_unary_expr(&self, expr: &UnaryExpr) -> Result<LiteralValue, LoxResult> {
        let right = self.evaluate(&expr.right)?;
        match expr.operator.token_type() {
            TokenType::MINUS => match right {
                LiteralValue::Num(n) => return Ok(LiteralValue::Num(-n)),
                _ => return Ok(LiteralValue::Nil),
            },
            TokenType::BANG => Ok(LiteralValue::Bool(!self.is_truthy(&right))),
            _ => Err(LoxResult::error(
                expr.operator.line,
                "Unreachable according to Nystrom",
            )),
        }
    }

    fn visit_variable_expr(&self, expr: &VariableExpr) -> Result<LiteralValue, LoxResult> {
        todo!()
    }
    fn visit_logical_expr(&self, expr: &LogicalExpr) -> Result<LiteralValue, LoxResult> {
        let left = self.evaluate(&expr.left)?;

        if expr.operator.is(TokenType::OR) {
            if self.is_truthy(&left) {
                return Ok(left);
            }
        } else if !self.is_truthy(&left) {
            return Ok(left);
        }

        self.evaluate(&expr.right)
    }

    fn visit_assign_expr(&self, expr: &AssignExpr) -> Result<LiteralValue, LoxResult> {
        let value = self.evaluate(&expr.value)?;
        self.environment
            .borrow()
            .borrow_mut()
            .assign(&expr.name, value.clone())?;
        Ok(value)
    }
    
    fn visit_call_expr(&self, expr: &CallExpr) -> Result<LiteralValue, LoxResult> {
        let callee = self.evaluate(&expr.callee);

        let mut arguments = Vec::new();
        for argument in &expr.arguments {
            arguments.push(self.evaluate(argument)?);
        }

        if let Ok(LiteralValue::Func(function)) = callee {
            if arguments.len() != function.func.arity() {
                return Err(LoxResult::runtime_error(
                    &expr.paren, 
                    &format!("Expected {} arguments but got {}.", function.func.arity(), arguments.len()),
                ))
            }
            function.call(self, arguments)
        } else {
            Err(LoxResult::runtime_error(&expr.paren, "Can only call functions and classes."))
        }
    }
}

impl Interpreter {
    pub fn new() -> Interpreter {
        let globals = Rc::new(RefCell::new(Environment::new()));

        globals.borrow_mut().define("clock", LiteralValue::Func(Callable {
            func: Rc::new(NativeClock {}),
            arity: 0,
        }));

        Interpreter {
            globals: Rc::clone(&globals), 
            environment: RefCell::new(Rc::new(RefCell::new(Environment::new()))),
            nest: RefCell::new(0),
        }
    }

    fn evaluate(&self, expr: &Expr) -> Result<LiteralValue, LoxResult> {
        expr.accept(self)
    }

    fn execute(&self, stmt: &Stmt) -> Result<(), LoxResult> {
        stmt.accept(self)
    }

    fn execute_block(
        &self,
        statements: &[Stmt],
        environment: Environment,
    ) -> Result<(), LoxResult> {
        let previous = self.environment.replace(Rc::new(RefCell::new(environment)));

        let result = statements
            .iter()
            .try_for_each(|statement| self.execute(statement));

        self.environment.replace(previous);

        result
    }

    // Anything that is not Nil or False is true
    fn is_truthy(&self, literal_value: &LiteralValue) -> bool {
        !matches!(literal_value, LiteralValue::Nil | LiteralValue::Bool(false))
    }
    pub fn interpret(&self, statements: &[Stmt]) -> bool {
        let mut success = true;
        *self.nest.borrow_mut() = 0;
        for statement in statements {
            if let Err(e) = self.execute(statement) {
                success = false;
                break;
            }
        }
        success
    }
    pub fn print_environment(&self) {
        println!("{:?}", self.environment);
    }
}

/*#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::*;

    fn make_literal(o: Object) -> Box<Expr> {
        Box::new(Expr::Literal(LiteralExpr { value: Some(o) }))
    }

    fn make_literal_string(s: &str) -> Box<Expr> {
        make_literal(Object::Str(s.to_string()))
    }

    #[test]
    fn test_unary_minus() {
        let terp = Interpreter::new();
        let unary_expr = UnaryExpr {
            operator: Token::new(TokenType::Minus, "-".to_string(), None, 123),
            right: make_literal(Object::Num(123.0)),
        };
        let result = terp.visit_unary_expr(&unary_expr);
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Object::Num(-123.0)));
    }

    #[test]
    fn test_unary_not() {
        let terp = Interpreter::new();
        let unary_expr = UnaryExpr {
            operator: Token::new(TokenType::Bang, "!".to_string(), None, 123),
            right: make_literal(Object::Bool(false)),
        };
        let result = terp.visit_unary_expr(&unary_expr);
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Object::Bool(true)));
    }

    #[test]
    fn test_subtraction() {
        let terp = Interpreter::new();
        let binary_expr = BinaryExpr {
            left: make_literal(Object::Num(15.0)),
            operator: Token::new(TokenType::Minus, "-".to_string(), None, 123),
            right: make_literal(Object::Num(7.0)),
        };
        let result = terp.visit_binary_expr(&binary_expr);
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Object::Num(8.0)));
    }

    #[test]
    fn test_multiplication() {
        let terp = Interpreter::new();
        let binary_expr = BinaryExpr {
            left: make_literal(Object::Num(15.0)),
            operator: Token::new(TokenType::Star, "*".to_string(), None, 123),
            right: make_literal(Object::Num(7.0)),
        };
        let result = terp.visit_binary_expr(&binary_expr);
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Object::Num(105.0)));
    }

    #[test]
    fn test_division() {
        let terp = Interpreter::new();
        let binary_expr = BinaryExpr {
            left: make_literal(Object::Num(21.0)),
            operator: Token::new(TokenType::Slash, "/".to_string(), None, 123),
            right: make_literal(Object::Num(7.0)),
        };
        let result = terp.visit_binary_expr(&binary_expr);
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Object::Num(3.0)));
    }

    #[test]
    fn test_addition() {
        let terp = Interpreter::new();
        let binary_expr = BinaryExpr {
            left: make_literal(Object::Num(21.0)),
            operator: Token::new(TokenType::Plus, "+".to_string(), None, 123),
            right: make_literal(Object::Num(7.0)),
        };
        let result = terp.visit_binary_expr(&binary_expr);
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Object::Num(28.0)));
    }

    #[test]
    fn test_string_concatination() {
        let terp = Interpreter::new();
        let binary_expr = BinaryExpr {
            left: make_literal_string("hello, "),
            operator: Token::new(TokenType::Plus, "+".to_string(), None, 123),
            right: make_literal_string("world!"),
        };
        let result = terp.visit_binary_expr(&binary_expr);
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Object::Str("hello, world!".to_string())));
    }

    #[test]
    fn test_arithmetic_error_for_subtration() {
        let terp = Interpreter::new();
        let binary_expr = BinaryExpr {
            left: make_literal(Object::Num(15.0)),
            operator: Token::new(TokenType::Minus, "-".to_string(), None, 123),
            right: make_literal(Object::Bool(true)),
        };
        let result = terp.visit_binary_expr(&binary_expr);
        assert!(result.is_err());
    }

    #[test]
    fn test_arithmetic_error_for_greater() {
        let terp = Interpreter::new();
        let binary_expr = BinaryExpr {
            left: make_literal(Object::Num(15.0)),
            operator: Token::new(TokenType::Greater, ">".to_string(), None, 123),
            right: make_literal(Object::Bool(true)),
        };
        let result = terp.visit_binary_expr(&binary_expr);
        assert!(result.is_err());
    }

    fn run_comparison_test(tok: &Token, cmps: Vec<bool>) {
        let nums = vec![14.0, 15.0, 16.0];
        let terp = Interpreter::new();

        for (c, nums) in cmps.iter().zip(nums) {
            let binary_expr = BinaryExpr {
                left: make_literal(Object::Num(nums)),
                operator: tok.dup(),
                right: make_literal(Object::Num(15.0)),
            };
            let result = terp.visit_binary_expr(&binary_expr);
            assert!(result.is_ok());
            assert_eq!(
                result.ok(),
                Some(Object::Bool(*c)),
                "Testing {} {} 15.0",
                nums,
                tok.as_string()
            );
        }
    }

    #[test]
    fn test_less_than() {
        run_comparison_test(
            &Token::new(TokenType::Less, "<".to_string(), None, 123),
            vec![true, false, false],
        );
    }

    #[test]
    fn test_less_than_or_equal_to() {
        run_comparison_test(
            &Token::new(TokenType::LessEqual, "<=".to_string(), None, 123),
            vec![true, true, false],
        );
    }

    #[test]
    fn test_greater_than() {
        run_comparison_test(
            &Token::new(TokenType::Greater, ">".to_string(), None, 123),
            vec![false, false, true],
        );
    }

    #[test]
    fn test_greater_than_or_equal_to() {
        run_comparison_test(
            &Token::new(TokenType::GreaterEqual, ">=".to_string(), None, 123),
            vec![false, true, true],
        );
    }

    #[test]
    fn test_equals_nums() {
        run_comparison_test(
            &Token::new(TokenType::Equals, "==".to_string(), None, 123),
            vec![false, true, false],
        );
    }

    #[test]
    fn test_not_equals_nums() {
        run_comparison_test(
            &Token::new(TokenType::BangEqual, "!=".to_string(), None, 123),
            vec![true, false, true],
        );
    }

    #[test]
    fn test_not_equals_string() {
        let terp = Interpreter::new();
        let binary_expr = BinaryExpr {
            left: make_literal_string("hello"),
            operator: Token::new(TokenType::Equals, "==".to_string(), None, 123),
            right: make_literal_string("hellx"),
        };
        let result = terp.visit_binary_expr(&binary_expr);
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Object::Bool(false)));
    }

    #[test]
    fn test_equals_string() {
        let terp = Interpreter::new();
        let binary_expr = BinaryExpr {
            left: make_literal_string("world"),
            operator: Token::new(TokenType::Equals, "==".to_string(), None, 123),
            right: make_literal_string("world"),
        };
        let result = terp.visit_binary_expr(&binary_expr);
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Object::Bool(true)));
    }

    #[test]
    fn test_equals_nil() {
        let terp = Interpreter::new();
        let binary_expr = BinaryExpr {
            left: make_literal(Object::Nil),
            operator: Token::new(TokenType::Equals, "==".to_string(), None, 123),
            right: make_literal(Object::Nil),
        };
        let result = terp.visit_binary_expr(&binary_expr);
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Object::Bool(true)));
    }

    #[test]
    fn test_var_stmt_defined() {
        let terp = Interpreter::new();
        let name = Token::new(TokenType::Identifier, "foo".to_string(), None, 123);
        let var_stmt = VarStmt {
            name: name.dup(),
            initializer: Some(*make_literal(Object::Num(23.0))),
        };
        assert!(terp.visit_var_stmt(&var_stmt).is_ok());
        assert_eq!(
            terp.environment.borrow().get(&name).unwrap(),
            Object::Num(23.0)
        );
    }

    #[test]
    fn test_var_stmt_undefined() {
        let terp = Interpreter::new();
        let name = Token::new(TokenType::Identifier, "foo".to_string(), None, 123);
        let var_stmt = VarStmt {
            name: name.dup(),
            initializer: None,
        };
        assert!(terp.visit_var_stmt(&var_stmt).is_ok());
        assert_eq!(terp.environment.borrow().get(&name).unwrap(), Object::Nil);
    }

    #[test]
    fn test_variable_expr() {
        let terp = Interpreter::new();
        let name = Token::new(TokenType::Identifier, "foo".to_string(), None, 123);
        let var_stmt = VarStmt {
            name: name.dup(),
            initializer: Some(*make_literal(Object::Num(23.0))),
        };
        assert!(terp.visit_var_stmt(&var_stmt).is_ok());

        let var_expr = VariableExpr { name: name.dup() };
        assert_eq!(
            terp.visit_variable_expr(&var_expr).unwrap(),
            Object::Num(23.0)
        );
    }

    #[test]
    fn test_undefined_variable_expr() {
        let terp = Interpreter::new();
        let name = Token::new(TokenType::Identifier, "foo".to_string(), None, 123);
        let var_expr = VariableExpr { name: name.dup() };
        assert!(terp.visit_variable_expr(&var_expr).is_err());
    }
}*/
