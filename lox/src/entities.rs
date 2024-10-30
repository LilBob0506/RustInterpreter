use core::fmt;
use std::cmp::*;
//use std::ops::*;

use std::fmt::Display;
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TokenType {
    // Single-character tokens.
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,

    // One or two character tokens.
    BANG,
    BANG_EQUAL,
    EQUAL,
    EQUAL_EQUAL,
    GREATER,
    GREATER_EQUAL,
    LESS,
    LESS_EQUAL,

    // Literals.
    IDENTIFIER,
    STRING,
    NUMBER,

    // Keywords.
    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,

    EOF,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LiteralValue {
    Num(f64),
    Str(String),
    Bool(bool),
    Nil,
    ArithmeticError,
}
impl fmt::Display for LiteralValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LiteralValue::Num(x) => write!(f, "{x}"),
            LiteralValue::Str(x) => write!(f, "{x}"),
            LiteralValue::Nil => write!(f, "nil"),
            LiteralValue::Bool(true) => write!(f, "true"),
            LiteralValue::Bool(false) => write!(f, "false"),
            &LiteralValue::ArithmeticError => panic!("Should not be trying to print this"),
        }
    }
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
    pub literal: Option<LiteralValue>,
}

impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: String,
        literal: Option<LiteralValue>,
        line: usize,
    ) -> Token {
        Token {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
    pub fn is(&self, ttype: TokenType) -> bool {
        self.token_type == ttype
    }
    pub fn token_type(&self) -> TokenType {
        self.token_type
    }
    pub fn as_string(&self) -> &String {
        &self.lexeme
    }
    pub fn dup(&self) -> Token {
        Token {
            token_type: self.token_type,
            lexeme: self.lexeme.to_string(),
            literal: self.literal.clone(),
            line: self.line,
        }
    }
    pub fn eof(line: usize) -> Token {
        Token {
            token_type: TokenType::EOF,
            lexeme: "".to_string(),
            literal: None,
            line,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:?} {} {}",
            self.token_type,
            self.lexeme,
            if let Some(literal) = &self.literal {
                literal.to_string()
            } else {
                "None".to_string()
            }
        )
    }
}
#[derive(PartialEq, Debug, Clone)]
pub enum LoxValue {
    Nil,
    Boolean(bool),
    String(String),
    Number(f64),
}
impl Display for LoxValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let q;
        write!(
            f,
            "{}",
            match self {
                LoxValue::Nil => "null",
                LoxValue::String(a) => a,
                LoxValue::Boolean(a) =>
                    if *a {
                        "true"
                    } else {
                        "false"
                    },
                LoxValue::Number(a) => {
                    q = a.to_string();
                    &q
                }
            }
        )
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

//#[derive(Debug)]
/*pub struct RuntimeError<'a> {
    pub token: &'a Token,
    pub message: &'a str,
}

// #[derive(Debug)]
pub struct ParseError<'a> {
    pub token: &'a Token,
    pub message: &'a str,
}

// #[derive(Debug)]
pub struct LoxError {
    token: Option<Token>,
    line: usize,
    message: String,
}

impl LoxError {
    pub fn error(line: usize, message: &str) -> LoxError {
        let err = LoxError {
            token: None,
            line,
            message: message.to_string(),
        };
        err.report("");
        err
    }
    pub fn parse_error(token: &Token, message: &str) -> LoxError {
        let err = LoxError {
            token: Some(token.dup()),
            line: token.line,
            message : message.to_string(),
        };
        err.report("");
        err
    }

    pub fn report(&self, loc: &str) {
        if let Some(token) = &self.token {
            if token.is(TokenType::EOF) {
                eprintln!("{} at end {}", token.line, self.message);
            } else {
                eprintln!("{} at '{}' {}", token.line, token.as_string(), self.message);
            }
        } else {
            eprintln!("[line {}] Error{}: {}", self.line, loc, self.message);
        }
    }
}
*/
