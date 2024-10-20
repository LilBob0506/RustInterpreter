use crate::entities::{LiteralValue, LoxValue, RuntimeError, Token, TokenType};
use crate::environment::*;
use crate::expr::{self, Expr};
use crate::stmt::{self, Stmt};

macro_rules! evaluate {
    ($e: expr) => {
        <Interpreter as expr::Walker<Result<LoxValue, RuntimeError<'a>>>>::walk($e)
    };
}
pub(crate) use evaluate;
macro_rules! execute {
    ($e: expr) => {
        <Interpreter as stmt::Walker<Result<(), RuntimeError>>>::walk($e)
    };
}
pub(crate) use execute;

pub struct Interpreter {
    environment: Environment,
}

impl<'a> expr::Walker<'a, Result<LoxValue, RuntimeError<'a>>> for Interpreter {
    fn walk(e: &Expr<'a>) -> Result<LoxValue, RuntimeError<'a>> {
        match e {
            Expr::Assign { .. } => {
                todo!()
            }
            Expr::Binary {
                operator,
                left,
                right,
            } => {
                let left_val = evaluate!(left)?;
                let right_val = evaluate!(right)?;

                match operator.token_type {
					TokenType::MINUS => {
						let (a, b) = Self::unpack_operands_into_nums(&left_val, &right_val, operator)?;
						Ok((a - b).into())
					},
					TokenType::SLASH => {
						let (a, b) = Self::unpack_operands_into_nums(&left_val, &right_val, operator)?;
						Ok((a / b).into())
					},
					TokenType::STAR => {
						let (a, b) = Self::unpack_operands_into_nums(&left_val, &right_val, operator)?;
						Ok((a * b).into())
					},
					TokenType::PLUS => {
						let nums_res = Self::unpack_operands_into_nums(&left_val, &right_val, operator);
						match nums_res {
							Ok((a, b)) => Ok((a + b).into()),
							Err(_) => {
								if let (LoxValue::String(a), LoxValue::String(b)) = (left_val, right_val) {
									return Ok(LoxValue::String(a + &b));
								}
								Err(RuntimeError{token: operator, message: "Operands must be two strings or two numbers."})
							}
						}
					},
					TokenType::GREATER => {
						let (a, b) = Self::unpack_operands_into_nums(&left_val, &right_val, operator)?;
						Ok((a > b).into())
					},
					TokenType::LESS => {
						let (a, b) = Self::unpack_operands_into_nums(&left_val, &right_val, operator)?;
						Ok((a < b).into())
					},
					TokenType::GREATER_EQUAL => {
						let (a, b) = Self::unpack_operands_into_nums(&left_val, &right_val, operator)?;
						Ok((a >= b).into())
					},
					TokenType::LESS_EQUAL => {
						let (a, b) = Self::unpack_operands_into_nums(&left_val, &right_val, operator)?;
						Ok((a <= b).into())
					},
					TokenType::EQUAL_EQUAL => {
						Ok((left_val == right_val).into())
					},
					TokenType::BANG_EQUAL => {
						Ok((left_val != right_val).into())
					}
					_ => panic!("Internal Error. Token {} was improperly scanned as a binary operator without a valid token_type", operator.lexeme)
				}
                //TODO: Case functions for all of the TokenTypes
            }
            Expr::Call { .. } => {
                todo!()
            }
            Expr::Get { .. } => {
                todo!()
            }
            Expr::Grouping { expression } => {
                evaluate!(expression)
            }
            Expr::Literal { value } => Ok(match value {
                LiteralValue::Bool(a) => (*a).into(),
                LiteralValue::Num(a) => (*a).into(),
                LiteralValue::Str(a) => LoxValue::String(a.to_owned()),
                LiteralValue::Nil => LoxValue::Nil,
            }),
            Expr::Logical { .. } => {
                todo!()
            }
            Expr::Set { .. } => {
                todo!()
            }
            Expr::Super { .. } => {
                todo!()
            }
            Expr::This { .. } => {
                todo!()
            }
            Expr::Unary { operator, right } => {
                let right_val = evaluate!(right)?;
                match operator.token_type {
					TokenType::BANG => {
						Ok((!Self::is_truthy(right_val)).into())
					},
					TokenType::MINUS => {
						Ok((-Self::unpack_operand_into_num(&right_val, operator)?).into())
					},
					_ => panic!("Internal Error. Token {} was improperly scanned as a unary operator without a valid token_type", operator.lexeme)
				}
            }
            Expr::Variable { name } => {
                let value = { environment.get(name) };
                value
            }
        }
    }
}

impl<'a> stmt::Walker<'a, Result<(), RuntimeError<'a>>> for Interpreter {
    fn walk(s: &'a Stmt<'a>) -> Result<(), RuntimeError<'a>> {
        match s {
            Stmt::Expression { expression } => {
                evaluate!(expression)?;
                Ok(())
            }
            Stmt::Print { expression } => {
                let val = evaluate!(expression)?;
                println!("{}", val);
                Ok(())
            }
            Stmt::Var { name, initializer } => {
                let value = if let Some(initializer) = stmt::initializer {
                    evaluate!(initializer);
                } else {
                    ()
                };

                environment.define(stmt::name.as_string(), value);
                Ok(())
            }
            _ => todo!(),
        }
    }
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            environment: Environment::new(),
        }
    }
    pub fn interpret<'a>(stmts: &'a [Box<Stmt>]) -> Result<(), RuntimeError<'a>> {
        for s in stmts {
            execute!(s)?;
        }
        Ok(())
    }
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
