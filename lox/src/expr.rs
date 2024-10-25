use std::{env::args, fs::{write, File}, io};

use crate::{entities::{LiteralValue, LoxValue, Token}, LoxError};

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

/*fn define_ast(output_dir: &String, base_name: &String, &types: &[String]) -> io::Result<()> {
    let path = format!("{output_dir}/{}", base_name.to_ascii_lowercase());
    let mut file = File::create(path)?;

    write(file,  "use crate::error::*;\n")?;
    write(file,  "use crate::entities::*;\n")?;

    for ttype in &mut types {
        let (base_class_name, args) = ttype.split_once(":").unwrap();
        let class_name = format!("{}{}", base_class_name.trim(), base_name);
        let args_split = args.split(",");
        let mut fields = Vec::new();
        for args in args_split {
            fields.push(args.trim().to_string());
        }
        tree_types.push(TreeType {
            base_name,
            class_name,
            fields,
        });
    }

    println!("{:?}", tree_types);

    Ok(())

} 

fn main() -> io::Result<()> {
    let args: Vec<String> = args().collect();

    if args.len() != 2 {
        eprint!("Usage: generate_ast <output directory>");
        std::process::exit(64);
    } 

    let output_dir = args.get(1);

    define_ast(&output_dir, &"expr".to_owned(), &vec![
        "Binary   : Expr left, Token operator, Expr right",
        "Grouping : Expr expression",
        "Literal  : Object value",
        "Unary    : Token operator, Expr right"
    ]);

    Ok(())
}*/