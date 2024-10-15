use std::fmt::Display;

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TokenType {
	// Single-character tokens.
	LEFT_PAREN, RIGHT_PAREN, LEFT_BRACE, RIGHT_BRACE,
	COMMA, DOT, MINUS, PLUS, SEMICOLON, SLASH, STAR,

	// One or two character tokens.
	BANG, BANG_EQUAL,
	EQUAL, EQUAL_EQUAL,
	GREATER, GREATER_EQUAL,
	LESS, LESS_EQUAL,

	// Literals.
	IDENTIFIER, STRING, NUMBER,

	// Keywords.
	AND, CLASS, ELSE, FALSE, FUN, FOR, IF, NIL, OR,
	PRINT, RETURN, SUPER, THIS, TRUE, VAR, WHILE,

	EOF
}

#[derive(Debug)]
pub enum LiteralValue {
	Num(f64),
	Str(String),
	Bool(bool),
	Nil
}

#[derive(Debug)]
pub struct Token<'a> {
	pub token_type: TokenType,
	pub lexeme: &'a str,
	pub line: usize,
	pub literal: Option<LiteralValue>
}

#[derive(PartialEq, Debug)]
pub enum LoxValue {
	Nil,
	Boolean(bool),
	String(String),
	Number(f64)
}
impl Display for LoxValue {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let q;
		write!(f, "{}", match self {
			LoxValue::Nil => "null",
			LoxValue::String(a) => a,
			LoxValue::Boolean(a) => if *a {"true"} else {"false"},
			LoxValue::Number(a) => {q = a.to_string(); &q}
		})
	}
}
impl From<f64> for LoxValue {
	fn from(value: f64) -> Self {
		LoxValue::Number(value)
	}
}
impl From<bool> for LoxValue {
	fn from(value: bool) -> Self {
		LoxValue::Boolean(value)
	}
}

#[derive(Debug)]
pub struct RuntimeError<'a> {
	pub token: &'a Token<'a>,
	pub message: &'a str
}
#[derive(Debug)]
pub struct ParseError<'a> {
	pub token: &'a Token<'a>,
	pub message: &'a str
}
