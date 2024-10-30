use crate::entities::*;
use crate::errors::*;
use crate::expr::*;
//use crate::expr2::*;
//use crate::stmt::*;
//use crate::environment::*;
pub struct Interpreter {}
impl ExprVisitor<LiteralValue> for Interpreter {
    fn visit_assign_expr(&self, expr: &AssignExpr) -> Result<LiteralValue, LoxError> {
        let value = self.evaluate(&expr.value);
        self.environment
            .borrow_mut()
            .assign(&expr.name, value)?;
        Ok(value)
    }
    fn visit_literal_expr(&self, expr: &LiteralExpr) -> Result<LiteralValue, LoxError> {
        Ok(expr.value.clone().unwrap())
    }
    fn visit_grouping_expr(&self, expr: &GroupingExpr) -> Result<LiteralValue, LoxError> {
        Ok(self.evaluate(&expr.expression)?)
    }
    fn visit_binary_expr(&self, expr: &BinaryExpr) -> Result<LiteralValue, LoxError> {
        let left = self.evaluate(&expr.left)?;
        let right = self.evaluate(&expr.right)?;
        let result = match expr.operator.token_type() {
            TokenType::MINUS => left - right,
            TokenType::SLASH => left / right,
            TokenType::STAR => left * right,
            TokenType::PLUS => left + right,
            TokenType::GREATER => LiteralValue::Bool(left > right),
            TokenType::GREATER_EQUAL => LiteralValue::Bool(left >= right),
            TokenType::LESS => LiteralValue::Bool(left < right),
            TokenType::LESS_EQUAL => LiteralValue::Bool(left <= right),
            TokenType::BANG_EQUAL => LiteralValue::Bool(left != right),
            TokenType::EQUAL => LiteralValue::Bool(left == right),
            _ => {
                todo!("need to work on your code dude");
            }
        };
        if result == LiteralValue::ArithmeticError {
            Err(LoxError::runtime_error(
                &expr.operator,
                "Illegal expression",
            ))
        } else {
            Ok(result)
        }
    }
    fn visit_unary_expr(&self, expr: &UnaryExpr) -> Result<LiteralValue, LoxError> {
        let right = self.evaluate(&expr.right)?;
        match expr.operator.token_type() {
            TokenType::MINUS => match right {
                LiteralValue::Num(n) => return Ok(LiteralValue::Num(-n)),
                _ => return Ok(LiteralValue::Nil),
            },
            TokenType::BANG => Ok(LiteralValue::Bool(!self.is_truthy(&right))),
            _ => Err(LoxError::error(
                expr.operator.line,
                "Unreachable according to Nystrom",
            )),
        }
    }
    fn visit_variable_expr(&self, _expr: &VariableExpr) -> Result<LiteralValue, LoxError> {
        todo!()
    }
}

impl Interpreter {
    fn evaluate(&self, expr: &Expr) -> Result<LiteralValue, LoxError> {
        expr.accept(self)
    }
    // Anything that is not Nil or False is true
    fn is_truthy(&self, literal_value: &LiteralValue) -> bool {
        !matches!(literal_value, LiteralValue::Nil | LiteralValue::Bool(false))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::*;
    fn make_literal(o: LiteralValue) -> Box<Expr> {
        Box::new(Expr::Literal(LiteralExpr { value: Some(o) }))
    }
    fn make_literal_string(s: &str) -> Box<Expr> {
        make_literal(LiteralValue::Str(s.to_string()))
    }
    #[test]
    fn test_unary_minus() {
        let terp = Interpreter {};
        let unary_expr = UnaryExpr {
            operator: Token::new(TokenType::MINUS, "-".to_string(), None, 123),
            right: make_literal(LiteralValue::Num(123.0)),
        };
        let result = terp.visit_unary_expr(&unary_expr);
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(LiteralValue::Num(-123.0)));
    }
    #[test]
    fn test_unary_not() {
        let terp = Interpreter {};
        let unary_expr = UnaryExpr {
            operator: Token::new(TokenType::BANG, "!".to_string(), None, 123),
            right: make_literal(LiteralValue::Bool(false)),
        };
        let result = terp.visit_unary_expr(&unary_expr);
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(LiteralValue::Bool(true)));
    }
    #[test]
    fn test_subtraction() {
        let terp = Interpreter {};
        let binary_expr = BinaryExpr {
            left: make_literal(LiteralValue::Num(15.0)),
            operator: Token::new(TokenType::MINUS, "-".to_string(), None, 123),
            right: make_literal(LiteralValue::Num(7.0)),
        };
        let result = terp.visit_binary_expr(&binary_expr);
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(LiteralValue::Num(8.0)));
    }
    #[test]
    fn test_multiplication() {
        let terp = Interpreter {};
        let binary_expr = BinaryExpr {
            left: make_literal(LiteralValue::Num(15.0)),
            operator: Token::new(TokenType::STAR, "*".to_string(), None, 123),
            right: make_literal(LiteralValue::Num(7.0)),
        };
        let result = terp.visit_binary_expr(&binary_expr);
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(LiteralValue::Num(105.0)));
    }
    #[test]
    fn test_division() {
        let terp = Interpreter {};
        let binary_expr = BinaryExpr {
            left: make_literal(LiteralValue::Num(21.0)),
            operator: Token::new(TokenType::SLASH, "/".to_string(), None, 123),
            right: make_literal(LiteralValue::Num(7.0)),
        };
        let result = terp.visit_binary_expr(&binary_expr);
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(LiteralValue::Num(3.0)));
    }
    #[test]
    fn test_addition() {
        let terp = Interpreter {};
        let binary_expr = BinaryExpr {
            left: make_literal(LiteralValue::Num(21.0)),
            operator: Token::new(TokenType::PLUS, "+".to_string(), None, 123),
            right: make_literal(LiteralValue::Num(7.0)),
        };
        let result = terp.visit_binary_expr(&binary_expr);
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(LiteralValue::Num(28.0)));
    }
    #[test]
    fn test_string_concatination() {
        let terp = Interpreter {};
        let binary_expr = BinaryExpr {
            left: make_literal_string("hello, "),
            operator: Token::new(TokenType::PLUS, "+".to_string(), None, 123),
            right: make_literal_string("world!"),
        };
        let result = terp.visit_binary_expr(&binary_expr);
        assert!(result.is_ok());
        assert_eq!(
            result.ok(),
            Some(LiteralValue::Str("hello, world!".to_string()))
        );
    }
    #[test]
    fn test_arithmetic_error_for_subtration() {
        let terp = Interpreter {};
        let binary_expr = BinaryExpr {
            left: make_literal(LiteralValue::Num(15.0)),
            operator: Token::new(TokenType::MINUS, "-".to_string(), None, 123),
            right: make_literal(LiteralValue::Bool(true)),
        };
        let result = terp.visit_binary_expr(&binary_expr);
        assert!(result.is_err());
    }
    #[test]
    fn test_arithmetic_error_for_greater() {
        let terp = Interpreter {}; 
        let binary_expr = BinaryExpr {
            left: make_literal(LiteralValue::Num(15.0)),
            operator: Token::new(TokenType::GREATER, ">".to_string(), None, 123),
            right: make_literal(LiteralValue::Bool(true)),
        };
        let result = terp.visit_binary_expr(&binary_expr);
        assert!(result.is_ok());
    }
    #[test]
    fn test_greater_than_equal_to() {
        let terp = Interpreter {};
        let binary_expr = BinaryExpr {
            left: make_literal(LiteralValue::Num(15.0)),
            operator: Token::new(TokenType::GREATER_EQUAL, ">=".to_string(), None, 123),
            right: make_literal(LiteralValue::Num(15.0)),
        };
        let result = terp.visit_binary_expr(&binary_expr);
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(LiteralValue::Bool(true)));
    }
    #[test]
    fn test_greater_than_true() {
        let terp = Interpreter {};
        let binary_expr = BinaryExpr {
            left: make_literal(LiteralValue::Num(15.0)),
            operator: Token::new(TokenType::GREATER, ">".to_string(), None, 123),
            right: make_literal(LiteralValue::Num(7.0)),
        };
        let result = terp.visit_binary_expr(&binary_expr);
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(LiteralValue::Bool(true)));
    }
    #[test]
    fn test_greater_than_false() {
        let terp = Interpreter {};
        let binary_expr = BinaryExpr {
            left: make_literal(LiteralValue::Num(15.0)),
            operator: Token::new(TokenType::GREATER, ">".to_string(), None, 123),
            right: make_literal(LiteralValue::Num(17.0)),
        };
        let result = terp.visit_binary_expr(&binary_expr);
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(LiteralValue::Bool(false)));
    }
    #[test]
    fn test_equals() {
        let terp = Interpreter {};
        let binary_expr = BinaryExpr {
            left: make_literal(LiteralValue::Num(15.0)),
            operator: Token::new(TokenType::EQUAL, "==".to_string(), None, 123),
            right: make_literal(LiteralValue::Num(15.0)),
        };
        let result = terp.visit_binary_expr(&binary_expr);
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(LiteralValue::Bool(true)));
    }
    #[test]
    fn test_not_equals_string() {
        let terp = Interpreter {};
        let binary_expr = BinaryExpr {
            left: make_literal_string("hello"),
            operator: Token::new(TokenType::EQUAL, "==".to_string(), None, 123),
            right: make_literal_string("hellx"),
        };
        let result = terp.visit_binary_expr(&binary_expr);
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(LiteralValue::Bool(false)));
    }
    #[test]
    fn test_equals_string() {
        let terp = Interpreter {};
        let binary_expr = BinaryExpr {
            left: make_literal_string("world"),
            operator: Token::new(TokenType::EQUAL, "==".to_string(), None, 123),
            right: make_literal_string("world"),
        };
        let result = terp.visit_binary_expr(&binary_expr);
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(LiteralValue::Bool(true)));
    }
    #[test]
    fn test_equals_nil() {
        let terp = Interpreter {};
        let binary_expr = BinaryExpr {
            left: make_literal(LiteralValue::Nil),
            operator: Token::new(TokenType::EQUAL, "==".to_string(), None, 123),
            right: make_literal(LiteralValue::Nil),
        };
        let result = terp.visit_binary_expr(&binary_expr);
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(LiteralValue::Bool(true)));
    }
    #[test]
    fn test_not_equals() {
        let terp = Interpreter {};
        let binary_expr = BinaryExpr {
            left: make_literal(LiteralValue::Num(15.0)),
            operator: Token::new(TokenType::BANG_EQUAL, "!=".to_string(), None, 123),
            right: make_literal(LiteralValue::Num(16.0)),
        };
        let result = terp.visit_binary_expr(&binary_expr);
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(LiteralValue::Bool(true)));
    }
}
// Updated macros to pass `self` as an argument
/*
macro_rules! evaluate {
    (mut $self: ident, $e: expr) => {
        <Interpreter as Walker<Result<LoxValue, RuntimeError<'a>>>>::walk($self, $e)
    };
}
pub(crate) use evaluate;

macro_rules! execute {
    (mut $self: ident, $e: expr) => {
        <Interpreter as Walky<Result<(), RuntimeError>>>::walk($self, $e)
    };
}
pub(crate) use execute;

pub struct Interpreter {
    environment: Environment,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            environment: Environment::new(),
        }
    }

    pub fn interpret<'a>(&mut self, stmts: &'a [Box<Stmt<'a>>]) -> Result<(), RuntimeError<'a>> {
        for stmt in stmts {
            execute!(mut self, stmt)?;  // Executes statements
        }
        Ok(())
    }

    // Helper functions for evaluation and operations
    fn is_truthy(object: LoxValue) -> bool {
        !matches!(object, LoxValue::Nil | LoxValue::Boolean(false))
    }

    fn unpack_operand_into_num<'a>(
        operand: &LoxValue,
        operator: &'a Token,
    ) -> Result<f64, RuntimeError<'a>> {
        if let LoxValue::Number(x) = operand {
            return Ok(*x);
        }
        Err(RuntimeError {
            token: operator,
            message: "Operand must be a number.",
        })
    }

    fn unpack_operands_into_nums<'a>(
        left: &LoxValue,
        right: &LoxValue,
        operator: &'a Token,
    ) -> Result<(f64, f64), RuntimeError<'a>> {
        if let (LoxValue::Number(a), LoxValue::Number(b)) = (left, right) {
            return Ok((*a, *b));
        }
        Err(RuntimeError {
            token: operator,
            message: "Operands must be numbers.",
        })
    }
}

// Implementing trait Walker for Interpreter with Expr
impl<'a> Walker<'a, Result<LoxValue, RuntimeError<'a>>> for Interpreter {
    fn walk(&mut self, e: &Expr<'a>) -> Result<LoxValue, RuntimeError<'a>> {
        match e {
            // Handling each variant for expressions in Expr
            Expr::Binary { operator, left, right } => {
                let left_val = evaluate!(mut self, left)?;
                let right_val = evaluate!(mut self, right)?;

                match operator.token_type {
                    TokenType::PLUS => Ok((Self::unpack_operands_into_nums(&left_val, &right_val, operator)?).into()),
                    _ => unimplemented!(),
                }
            },
            Expr::Literal { value } => Ok(value.clone().into()),
            _ => unimplemented!()
        }
    }
}
*/
