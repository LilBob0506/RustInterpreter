#![allow(dead_code)]
mod entities;
mod environment;
mod expr;
mod expr2;
mod interpreter;
mod parser;
mod scanner;
mod stmt;
use std::env::args;
use std::io::{self, stdout, BufRead, Write};
mod ast_printer;
use ast_printer::AstPrinter;
use entities::{Token, TokenType};
mod errors;
use errors::LoxError;

static mut HAD_ERROR: bool = false;
pub fn main() {
    let args: Vec<String> = args().collect();

    if args.len() > 2 {
        println!("Usage: rustylox [script]");
        std::process::exit(64);
    } else if args.len() == 2 {
        run_file(&args[1]).expect("Could not run file");
        unsafe {
            if HAD_ERROR {
                std::process::exit(65);
            }
        }
    } else {
        run_prompt();
    }
}

fn run_file(path: &str) -> io::Result<()> {
    let buf = std::fs::read_to_string(path)?;
    if run(buf).is_err() {
        std::process::exit(65)
    }
    Ok(())
}
fn run_prompt() {
    let stdin = io::stdin();
    print!(">");
    let _ = stdout().flush();
    for line in stdin.lock().lines() {
        if let Ok(line) = line {
            if line.is_empty() {
                break;
            }
            let _ = run(line);
        } else {
            break;
        }
        print!(">");
        let _ = stdout().flush();
    }
}

pub fn run(src: String) -> Result<(), LoxError> {
    let mut scan = scanner::Scanner::new(src);
    let tokens = scan.scan()?;
    let mut parser = parser::Parser::new(tokens);

    match parser.parse() {
        None => {}
        Some(expr) => {
            let printer = AstPrinter {};
            println!("AST Printer:\n{}", printer.print(&expr)?);
        }
    }
    Ok(())
}
/*unsafe {
      if HAD_ERROR {
        return;
    }
}


unsafe {
    if HAD_ERROR {
        return;
    }
}
println!("{:#?}", Interpreter::interpret(&parsed[..]));*/

fn error(token: &Token, message: &str) {
    report(
        token.line,
        &format!(
            "at {}",
            if token.token_type == TokenType::EOF {
                "end"
            } else {
                &token.lexeme
            }
        ),
        message,
    );
}
fn error1(line: usize, message: &str) {
    report(line, "", message);
}

fn report(line: usize, loc: &str, message: &str) {
    eprintln!("[line {line}] Error {loc}: {message}");
    unsafe {
        HAD_ERROR = true;
    } // thread safety guaranteed by the lack of threads
}
