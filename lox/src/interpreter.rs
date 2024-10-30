use crate::entities::*;
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
    fn visit_binary_expr(&self, _expr: &BinaryExpr) -> Result<LiteralValue, LoxError> {
        Ok(LiteralValue::Nil)
    }
    fn visit_unary_expr(&self, expr: &UnaryExpr) -> Result<LiteralValue, LoxError> {
        let right = self.evaluate(&expr.right)?;
        match expr.operator.token_type() {
            TokenType::MINUS => match right {
                LiteralValue::Num(n) => return Ok(LiteralValue::Num(-n)),
                _ => return Ok(LiteralValue::Nil),
            },
            TokenType::BANG => {
                if self.is_truthy(&right) {
                    Ok(LiteralValue::Bool(false))
                } else {
                    Ok(LiteralValue::Bool(true))
                }
            }
            _ => Err(LoxError::error(
                0,
                "Unreachable accordin to Nystrom".to_string(),
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