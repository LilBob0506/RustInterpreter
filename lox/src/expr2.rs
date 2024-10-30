use crate::entities::{LiteralValue, Token};


pub trait Walker<'a, T> {
    fn walk(& mut self, e: &Expr<'a>) -> T;
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
