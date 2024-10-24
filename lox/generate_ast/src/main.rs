use std::env::args;
use std::fs::File;
use std::io::{self, Write};

#[derive(Debug)]
struct TreeType {
    base_class_name: String,
    class_name: String,
    fields: Vec<String>,
}

fn main() -> io::Result<()> {
    let args: Vec<String> = args().collect();

    if args.len() != 2 {
        eprint!("Usage: generate_ast <output directory>");
        std::process::exit(64);
    }

    let output_dir = args.get(1).unwrap().to_string();

    define_ast(
        &output_dir,
        &"expr".to_string(),
        &vec![
            "Binary   : Box<Expr> left, Token operator, Box<Expr> right".to_string(),
            "Grouping : Box<Expr> expression".to_string(),
            "Literal  : Object value".to_string(),
            "Unary    : Token operator, Box<Expr> right".to_string(),
        ],
    )?;

    Ok(())
}

fn define_ast(output_dir: &String, base_name: &String, types: &[String]) -> io::Result<()> {
    let path = format!("{output_dir}/{}.rs", base_name.to_lowercase());
    let mut file = File::create(path)?;
    let mut tree_types = Vec::new();

    // Write header imports
    write!(file, "{}", "use crate::error::*;\n")?;
    write!(file, "{}", "use crate::entities::*;\n")?;

    for ttype in types {
        // Safely split the type description
        let (base_class_name, args) = ttype.split_once(":").unwrap();
        let class_name = format!("{}{}", base_class_name.trim(), base_name);
        let args_split = args.split(",");
        let mut fields = Vec::new();

        // Trim and collect the fields
        for arg in args_split {
            println!("splitting '{arg}");
            let (t2type, name) = arg.trim().split_once(" ").unwrap();
            fields.push(format!("{}: {}", name, t2type));
        }

        // Push into tree_types vector
        tree_types.push(TreeType {
            base_class_name: base_class_name.trim().to_string(),
            class_name,
            fields,
        });
    }

    write!(file, "\npub enum {base_name} {{\n")?;
    for t in &tree_types {
        write!(file, "    {}({}.\n", t.base_class_name, t.class_name)?;
    }
    write!(file, "}}\n")?;

    for t in &tree_types {
        write!(file, "pub struct {} {{\n", t.class_name)?;
        for f in &t.fields {
            write!(file, "    {},\n", f)?;
        }
        write!(file, "}}\n\n")?;
    }
    write!(file, "pub trait ExprVisitor<T> {{\n")?;
    for t in &tree_types {
        write!(
            file,
            "    fn visit_{}_{}(&self, expr: &{})-> Result<T, Error);\n",
            t.base_class_name.to_lowercase(),
            t.class_name.to_lowercase(),
            t.class_name.to_lowercase()
        )?;
    }
    for t in &tree_types {
        write!(file, "}}\n\n")?;
        write!(file, "impl {} {{\n", t.class_name)?;
        write!(
            file,
            "    fn accept<T>(&self,visitor:: &dyn {}Visitor<T>) -> Result<T, LoxError> {{\n",
            base_name
        )?;
        write!(
            file,
            "    visitor.visit_{}_{}(self)\n",
            t.base_class_name.to_lowercase(),
            base_name.to_lowercase()
        )?;
        write!(file, "    }}\n")?;
        write!(file, "}}\n")?;
    }

    Ok(())
}
