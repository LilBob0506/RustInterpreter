#![allow(dead_code)]
use std::env::args;
use std::io::{self, stdout, BufRead, Write};
mod entities;

mod callable;

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

mod errors;
use errors::*;

static mut HAD_ERROR: bool = false;
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
        if self.run(buf).is_err() {
            // Ignore: error was already reported
            std::process::exit(65);
        }
        Ok(())
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
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan()?;
        let mut parser = Parser::new(tokens);
        let statements = parser.parse()?;

        if self.interpreter.interpret(&statements) {
            Ok(())
        } else {
            Err(LoxResult::error(0, ""))
        }
    }
}
