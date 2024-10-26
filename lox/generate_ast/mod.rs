use std::{env::args, fs::{write, File}, io};

#[derive(Debug)]
struct TreeType {
    base_class_name: String,
    class_name: String,
    fields: Vec<String>,
}

pub fn generate_ast(output_dir: &str) -> io::Result<()> {
    define_ast(
        output_dir,
        "Expr",
        &["error", "token", "object", "rc"],
        &[
            "Assign   : Token name, Rc<Expr> value",
            "Binary   : Rc<Expr> left, Token operator, Rc<Expr> right",
            "Call     : Rc<Expr> callee, Token paren, Vec<Rc<Expr>> arguments",
            "Get      : Rc<Expr> object, Token name",
            "Grouping : Rc<Expr> expression",
            "Literal  : Option<Object> value",
            "Logical  : Rc<Expr> left, Token operator, Rc<Expr> right",
            "Set      : Rc<Expr> object, Token name, Rc<Expr> value",
            "Super    : Token keyword, Token method",
            "This     : Token keyword",
            "Unary    : Token operator, Rc<Expr> right",
            "Variable : Token name",
        ],
    )?;
    define_ast(
        output_dir,
        "Stmt",
        &["error", "expr", "token", "rc"],
        &[
            "Block      : Rc<Vec<Rc<Stmt>>> statements",
            "Class      : Token name, Option<Rc<Expr>> superclass, Rc<Vec<Rc<Stmt>>> methods",
            "Break      : Token token",
            "Expression : Rc<Expr> expression",
            "Function   : Token name, Rc<Vec<Token>> params, Rc<Vec<Rc<Stmt>>> body",
            "If         : Rc<Expr> condition, Rc<Stmt> then_branch, Option<Rc<Stmt>> else_branch",
            "Print      : Rc<Expr> expression",
            "Return     : Token keyword, Option<Rc<Expr>> value",
            "Var        : Token name, Option<Rc<Expr>> initializer",
            "While      : Rc<Expr> condition, Rc<Stmt> body",
        ],
    )?;
    Ok(())
}

fn define_ast(output_dir: &str, base_name: &str, imports: &[&str], åtypes: &[&str],) -> io::Result<()> {
    let _ = imports;
    let path = format!("{output_dir}/{}", base_name.to_ascii_lowercase());
    let _file = File::create(path)?;
    let mut tree_types = Vec::new();

    

    for ttype in åtypes {
        let (base_class_name, args) = ttype.split_once(":").unwrap();
        let class_name = format!("{}{}", base_class_name.trim(), base_name);
        let args_split = args.split(",");
        let mut fields = Vec::new();
        for args in args_split {
            fields.push(args.trim().to_string());
        }
        let value = TreeType {
            base_class_name: base_class_name.trim().to_string(),
            class_name,
            fields,
        };
        tree_types.push(value);
    }

    println!("{:?}", tree_types);

    Ok(())

} 
