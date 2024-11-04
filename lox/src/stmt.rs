use crate::errors::*;
use crate::expr::*;

pub enum Stmt {
    Block(BlockStmt),
    Expression(ExpressionStmt),
    If(IfStmt),
    Print(PrintStmt),
    Variable(VariableStmt),
}

impl Stmt {
    pub fn accept<T>(&self, stmt_visitor: &dyn StmtVisitor<T>) -> Result<T, LoxError> {
        match self {
            Stmt::Block(v) => v.accept(stmt_visitor),
            Stmt::Expression(v) => v.accept(stmt_visitor),
            Stmt::If(v) => v.accept(stmt_visitor),
            Stmt::Print(v) => v.accept(stmt_visitor),
            Stmt::Variable(v) => v.accept(stmt_visitor),
        }
    }
}

pub struct BlockStmt {
    pub statments: List<Stmt>,
}

pub struct ExpressionStmt {
    pub expression: Expr,
}

pub struct IfStmt {
    pub condition: Expr,
    pub then_branch: Stmt,
    pub else_branch: Option<Box<Stmt>>,
}

pub struct PrintStmt {
    pub expression: Expr,
}

pub struct VariableStmt {
    pub name: Token,
    pub initializer: Option<Expr>,
}

pub trait StmtVisitor<T> {
    fn visit_block_stmt(&self, expr: &BlockStmt) -> Result<T, LoxError>;
    fn visit_expression_stmt(&self, expr: &ExpressionStmt) -> Result<T, LoxError>;
    fn visit_if_stmt(&self, expr: &IfStmt) -> Result<T, LoxError>;
    fn visit_print_stmt(&self, expr: &PrintStmt) -> Result<T, LoxError>;
    fn visit_variable_stmt(&self, expr: &VariableStmt) -> Result<T, LoxError>;
}

impl BlockStmt {
    pub fn accept<T>(&self, visitor: &dyn StmtVisitor<T>) -> Result<T, LoxError> {
        visitor.visit_block_stmt(self)
    }
}

impl ExpressionStmt {
    pub fn accept<T>(&self, visitor: &dyn StmtVisitor<T>) -> Result<T, LoxError> {
        visitor.visit_expression_stmt(self)
    }
}

impl IfStmt {
    pub fn accept<T>(&self, visitor: &dyn StmtVisitor<T>) -> Result<T, LoxError> {
        visitor.visit_if_stmt(self)
    }
}

impl PrintStmt {
    pub fn accept<T>(&self, visitor: &dyn StmtVisitor<T>) -> Result<T, LoxError> {
        visitor.visit_print_stmt(self)
    }
}

impl VariableStmt {
    pub fn accept<T>(&self, visitor: &dyn StmtVisitor<T>) -> Result<T, LoxError> {
        visitor.visit_variable_stmt(self)
    }
}

