use std::{env::args, fs::File, io::{self, Write}};

use crate::{entities::{LiteralValue, LoxValue, Token}, LoxError};

pub enum Expr {
    Binary(BinaryExpr),
    Grouping(GroupingExpr),
    Literal(LiteralExpr),
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

pub trait ExprVisitor {
    fn visit_binary_expr(&self, expr: &BinaryExpr) -> Result<(), LoxError>;
    fn visit_grouping_expr(&self, expr: &GroupingExpr) -> Result<(), LoxError>;
    fn visit_literal_expr(&self, expr: &LiteralExpr) -> Result<(), LoxError>;
    fn visit_unary_expr(&self, expr: &UnaryExpr) -> Result<(), LoxError>;
}

impl BinaryExpr {
    fn accept<T>(&self, visitor: &dyn ExprVisitor) -> Result<T, LoxError> {
        visitor.visit_binary_expr(self).map_err(|e| e.into())
    }
}

impl GroupingExpr {
    fn accept<T>(&self, visitor: &dyn ExprVisitor) -> Result<T, LoxError> {
        visitor.visit_grouping_expr(self).map_err(|e| e.into())
    }
}

impl LiteralExpr {
    fn accept<T>(&self, visitor: &dyn ExprVisitor) -> Result<T, LoxError> {
        visitor.visit_literal_expr(self).map_err(|e| e.into())
    }
}

impl UnaryExpr {
    fn accept<T>(&self, visitor: &dyn ExprVisitor) -> Result<T, LoxError> {
        visitor.visit_unary_expr(self).map_err(|e| e.into())
    }
}

fn main() -> io::Result<()> {
    let args: Vec<String> = args().collect();

    if args.len() != 2 {
        eprintln!("Usage: generate_ast <output directory>");
        std::process::exit(64);
    } 

    let output_dir = &args[1];

    define_ast(output_dir, "Expr", &vec![
        "Binary   : Expr left, Token operator, Expr right",
        "Grouping : Expr expression",
        "Literal  : LoxValue value",
        "Unary    : Token operator, Expr right"
    ])?;

    Ok(())
}

fn define_ast(output_dir: &str, base_name: &str, types: &[&str]) -> io::Result<()> {
    let path = format!("{}/{}.rs", output_dir, base_name.to_ascii_lowercase());
    let mut file = File::create(path)?;

    writeln!(file, "use crate::error::*;")?;
    writeln!(file, "use crate::entities::*;")?;

    for ttype in types {
        let (base_class_name, args) = ttype.split_once(":").unwrap();
        let class_name = format!("{}{}", base_class_name.trim(), base_name);
        let args_split = args.split(',');
        let fields: Vec<String> = args_split.map(|arg| arg.trim().to_string()).collect();

        // Here you might want to generate actual Rust structs or other AST code
        writeln!(file, "pub struct {} {{", class_name)?;
        for field in fields {
            writeln!(file, "    pub {},", field)?;
        }
        writeln!(file, "}}")?;
    }

    Ok(())
}