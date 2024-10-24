use crate::error::*;
use crate::entities::*;

pub enum expr {
    Binary(Binaryexpr.
    Grouping(Groupingexpr.
    Literal(Literalexpr.
    Unary(Unaryexpr.
}
pub struct Binaryexpr {
    left: Box<Expr>,
    operator: Token,
    right: Box<Expr>,
}

pub struct Groupingexpr {
    expression: Box<Expr>,
}

pub struct Literalexpr {
    value: Object,
}

pub struct Unaryexpr {
    operator: Token,
    right: Box<Expr>,
}

pub trait ExprVisitor<T> {
    fn visit_binary_binaryexpr(&self, expr: &binaryexpr)-> Result<T, Error);
    fn visit_grouping_groupingexpr(&self, expr: &groupingexpr)-> Result<T, Error);
    fn visit_literal_literalexpr(&self, expr: &literalexpr)-> Result<T, Error);
    fn visit_unary_unaryexpr(&self, expr: &unaryexpr)-> Result<T, Error);
}

impl Binaryexpr {
    fn accept<T>(&self,visitor:: &dyn exprVisitor<T>) -> Result<T, LoxError> {
    visitor.visit_binary_expr(self)
    }
}
}

impl Groupingexpr {
    fn accept<T>(&self,visitor:: &dyn exprVisitor<T>) -> Result<T, LoxError> {
    visitor.visit_grouping_expr(self)
    }
}
}

impl Literalexpr {
    fn accept<T>(&self,visitor:: &dyn exprVisitor<T>) -> Result<T, LoxError> {
    visitor.visit_literal_expr(self)
    }
}
}

impl Unaryexpr {
    fn accept<T>(&self,visitor:: &dyn exprVisitor<T>) -> Result<T, LoxError> {
    visitor.visit_unary_expr(self)
    }
}
