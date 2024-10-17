#![allow(dead_code)]

mod entities;
mod error;
//mod expr;
//mod interpreter;
//mod parser;
mod scanner;
//mod stmt;

use entities::{Token, TokenType};
use error::*;
//use interpreter::Interpreter;
use std::env::args;
use std::fs::{read_to_string, File};
use std::io::{self, stdout, BufRead, BufReader, Read, Write};

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

fn run_file(path: &String) -> io::Result<()> {
    let buf = std::fs::read_to_string(path)?;
    match run(buf) {
        Ok(_) => {}
        Err(_) => {
            std::process::exit(65)
        }
    }
    Ok(())
}
fn run_prompt() {
    let stdin = io::stdin();
    print!(">");
    stdout().flush();
    for line in stdin.lock().lines() {
        if let Ok(line) = line {
            if line.is_empty() {
                break;
            }
            match run(line) {
                Ok(_) => {}
                Err(_) => {}
            }
        } else {
            break;
        }
        print!(">");
        stdout().flush();
    }
}

fn run(src: String) -> Result<(), LoxError> {
    let mut scan = scanner::Scanner::new(src);
    let tokens = scan.scan();

    for token in tokens {
        println!("{:?}", token)
    }
    Ok(())
    /*unsafe {
        if HAD_ERROR {
            return;
        }
    }
    let mut parser = parser::Parser::new(tokens);
    let parsed = parser.parse();
    unsafe {
        if HAD_ERROR {
            return;
        }
    }
    println!("{:#?}", Interpreter::interpret(&parsed[..]));*/
}

/*fn error1(line: usize, message: &str) {
    report(line, "", message);
}*/
