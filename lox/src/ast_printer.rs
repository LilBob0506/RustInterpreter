
//use crate::entities::{LiteralValue, Token};
use crate::expr2::*;

pub trait Walker<'a, T> {
    fn walk(e: &Expr<'a>) -> T;
}

pub struct AstPrinter;
impl Walker<'_, String> for AstPrinter {
    fn walk(e: &Expr) -> String {
        match e {
            Expr::Assign { value, name } => {
                format!("(= {} {})", name.lexeme, Self::walk(value))
            }
            Expr::Binary {
                operator,
                left,
                right,
            } => {
                format!(
                    "({} {} {})",
                    operator.lexeme,
                    Self::walk(left),
                    Self::walk(right)
                )
            }
            Expr::Call { .. } => {
                format!("")
            }
            Expr::Get { .. } => {
                format!("")
            }
            Expr::Grouping { expression } => {
                format!("({})", Self::walk(expression))
            }
            Expr::Literal { value } => {
                format!("{:?}", value)
            }
            Expr::Logical { .. } => {
                format!("")
            }
            Expr::Set { .. } => {
                format!("")
            }
            Expr::Super { .. } => {
                format!("")
            }
            Expr::This { .. } => {
                format!("")
            }
            Expr::Unary { operator, right } => {
                format!("({}({}))", operator.lexeme, Self::walk(right))
            }
            Expr::Variable { .. } => {
                format!("")
            }
        }
    }
}