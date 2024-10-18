use crate::entities::Token;
use crate::expr::Expr;

pub trait Walker<'a, T> {
    fn walk(s: &'a Stmt<'a>) -> T;
}

pub enum Stmt<'a> {
    Block {
        statements: &'a [Box<Stmt<'a>>],
    },
    Class {
        name: &'a Token,
        superclass: Expr<'a>,         // Always a Variable
        methods: &'a [Box<Stmt<'a>>], // Always Functions
    },
    Expression {
        expression: Expr<'a>,
    },
    Function {
        name: &'a Token,
        params: &'a [&'a Token],
        body: &'a [Box<Stmt<'a>>],
    },
    If {
        condition: Expr<'a>,
        then_branch: Box<Stmt<'a>>,
        else_branch: Box<Stmt<'a>>,
    },
    Print {
        expression: Expr<'a>,
    },
    Return {
        value: Expr<'a>,
    },
    Var {
        name: &'a Token,
        initializer: Option<Expr<'a>>,
    },
    While {
        body: Box<Stmt<'a>>,
    },
}
