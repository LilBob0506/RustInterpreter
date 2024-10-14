#![allow(dead_code)]

mod entities;
mod scanner;
mod expr;
mod parser;
mod interpreter;
mod stmt;

use std::env::args;
use std::fs;
use std::io::Write;

use entities::{Token, TokenType};
use interpreter::Interpreter;

static mut HAD_ERROR: bool = false;
pub fn main() {
	let args: Vec<String> = args().collect();

	match args.len() {
		1 => run_prompt(),
		2 => {
			run_file(&args[1]);
			unsafe {
				if HAD_ERROR {
					std::process::exit(65);
				}
			}
		}
		_ => {
			println!("Usage: rustylox [script]");
			std::process::exit(64);
		}
		
	}
}
fn run_file(path: &str) {
	run(&fs::read_to_string(path).unwrap());
}
fn run_prompt() {
	let mut line = String::new();
	loop {
		print!("> ");
		line.clear();
		std::io::stdout().flush().unwrap();
		std::io::stdin().read_line(&mut line).unwrap();
		if line.is_empty() {
			break;
		}
		run(&line);
		unsafe {
			if HAD_ERROR {
				// TODO: handle error
				HAD_ERROR = false;
			}
		}
	}
	println!(""); // on end-of-input
}
fn run(src: &str) {
	let mut scan = scanner::Scanner::new(src);
	let tokens = scan.scan_tokens();
	unsafe {if HAD_ERROR {
		return;
	}}
	let mut parser = parser::Parser::new(tokens);
	let parsed = parser.parse();
	unsafe {if HAD_ERROR {
		return;
	}}
	println!("{:#?}", Interpreter::interpret(&parsed[..]));
}

fn error(token: &Token, message: &str) {
	report(token.line, &format!("at {}", if token.token_type == TokenType::EOF {"end"} else {token.lexeme}), message);
}
fn error1(line: usize, message: &str) {
	report(line, "", message);
}

fn report(line: usize, loc: &str, message: &str) {
	eprintln!("[line {line}] Error {loc}: {message}");
	unsafe { HAD_ERROR = true; } // thread safety guaranteed by the lack of threads
}
