use crate::entities::{LiteralValue, Token};


pub trait Walker<'a, T> {
    fn walk(&self, e: &Expr<'a>) -> T;
}
pub enum Expr<'a> {
    Assign {
        name: &'a Token,
        value: Box<Expr<'a>>,
    },
    Binary {
        left: Box<Expr<'a>>,
        operator: &'a Token,
        right: Box<Expr<'a>>,
    },
    Call {
        callee: Box<Expr<'a>>,
        paren: &'a Token,
        arguments: &'a [Box<Expr<'a>>],
    },
    Get {
        object: Box<Expr<'a>>,
        name: &'a Token,
    },
    Grouping {
        expression: Box<Expr<'a>>,
    },
    Literal {
        value: &'a LiteralValue,
    },
    Logical {
        left: Box<Expr<'a>>,
        operator: &'a Token,
        right: Box<Expr<'a>>,
    },
    Set {
        object: Box<Expr<'a>>,
        name: &'a Token,
        value: Box<Expr<'a>>,
    },
    Super {
        keyword: &'a Token,
        method: &'a Token,
    },
    This {
        keyword: &'a Token,
    },
    Unary {
        operator: &'a Token,
        right: Box<Expr<'a>>,
    },
    Variable {
        name: &'a Token,
    },
}

/*
   For testing and example purposes:
*/
/*pub struct AstPrinter;
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
}*/

/*

pub enum Expr {
    Binary(BinaryExpr),
    Grouping(GroupingExpr),
    Literal(LiteralValue),
    Unary(UnaryExpr),
}

pub struct BinaryExpr {
    left: Box<Expr>,
    operator: Token,
    right: Box<Expr>,
}

pub struct GroupingExpr {
    expression: Box<Expr>,
}

pub struct LiteralExpr {
    value: LoxValue,
}

pub struct UnaryExpr {
    operator: Token,
    right: Box<Expr>,
}

pub trait ExprVisitor<T> {
    fn visit_binary_expr(&self, expr: &BinaryExpr) -> Result<T, LoxError>;
    fn visit_grouping_expr(&self, expr: &GroupingExpr) -> Result<T, LoxError>;
    fn visit_literal_expr(&self, expr: &LiteralExpr) -> Result<T, LoxError>;
    fn visit_unary_expr(&self, expr: &UnaryExpr) -> Result<T, LoxError>;
}

impl BinaryExpr {
    fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> Result<T, LoxError> {
        visitor.visit_binary_expr(self)
    }
}

impl GroupingExpr {
    fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> Result<T, LoxError> {
        visitor.visit_grouping_expr(self)
    }
}

impl LiteralExpr {
    fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> Result<T, LoxError> {
        visitor.visit_literal_expr(self)
    }
}

impl UnaryExpr {
    fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> Result<T, LoxError> {
        visitor.visit_unary_expr(self)
    }
}

*/
