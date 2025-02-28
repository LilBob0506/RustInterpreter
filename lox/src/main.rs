#![allow(dead_code)]
use std::env::args;
use std::io::{self, stdout, BufRead, Write};
mod entities;
use std::rc::Rc;

mod lox_instance;
mod lox_class;

mod callable;
mod lox_function;

mod environment;
//use environment::*;

mod expr;
//use expr::*;
//mod expr2;
mod interpreter;
use interpreter::*;

mod parser;
use parser::*;

mod scanner;
use scanner::*;

mod stmt;
//use stmt::*;

//mod ast_printer;
mod resolver;
use resolver::*;

mod errors;
use errors::*;

mod native_functions;


//static mut HAD_ERROR: bool = false;
pub fn main() {
    let args: Vec<String> = args().collect();
    let lox = Lox::new();
    match args.len() {
        1 => lox.run_prompt(),
        2 => lox.run_file(&args[1]).expect("Could not run file"),
        _ => {
            println!("Usage: lox-ast [script]");
            std::process::exit(64);
        }
    }
}

struct Lox {
    interpreter: Interpreter,
}
impl Lox {
    pub fn new() -> Lox {
        Lox {
            interpreter: Interpreter::new(),
        }
    }

    pub fn run_file(&self, path: &str) -> io::Result<()> {
        let buf = std::fs::read_to_string(path)?;
        match self.run(buf) {
            Ok(_) => std::process::exit(0),
            Err(LoxResult::RuntimeError { .. }) => std::process::exit(70),
            _ => std::process::exit(65),
        }

    }

    pub fn run_prompt(&self) {
        let stdin = io::stdin();
        print!("> ");
        let _ = stdout().flush();
        for line in stdin.lock().lines() {
            if let Ok(line) = line {
                if line.is_empty() {
                    break;
                }
                let _ = self.run(line);
            } else {
                break;
            }
            print!(">");
            let _ = stdout().flush();
        }
    }

    fn run(&self, source: String) -> Result<(), LoxResult> {
        if source == "@" {
            self.interpreter.print_environment();
            return Ok(());
        }
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan()?;
        let mut parser = Parser::new(tokens);
        let statements = parser.parse()?;

        let resolver = Resolver::new(&self.interpreter);
        let s = Rc::new(statements);
        resolver.resolve(&Rc::clone(&s))?;
        if resolver.success() {
            self.interpreter.interpret(&Rc::clone(&s))?;
        } else {
            std::process::exit(65);
        }

        Ok(())
    }
}
