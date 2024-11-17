use core::fmt;
use std::rc::Rc;
//use std::ops::*;

use crate::{ lox_class::LoxClass, lox_instance::LoxInstance,native_functions::*};
use crate::lox_function::*;
use std::fmt::Display;
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TokenType {
    // Single-character tokens.
    Break,
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
    Func(Rc<LoxFunction>),
    Class(Rc<LoxClass>),
    Instance(Rc<LoxInstance>),
    Native(Rc<LoxNative>),
    Nil,
    ArithmeticError,
    NumsOrStringsError,
}
impl fmt::Display for LiteralValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LiteralValue::Num(x) => write!(f, "{x}"),
            LiteralValue::Str(x) => write!(f, "{x}"),
            LiteralValue::Nil => write!(f, "nil"),
            LiteralValue::Bool(true) => write!(f, "true"),
            LiteralValue::Bool(false) => write!(f, "false"),
            LiteralValue::Func(func) => write!(f, "{}", func),
            LiteralValue::Class(c) => write!(f, "{}", c),
            LiteralValue::Native(n) => write!(f, "{n}"),
            LiteralValue::Instance(i) => write!(f, "<Instance of {}", i),
             _ => panic!("Should not be trying to print this"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
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
    pub fn as_string(&self) -> String {
        self.lexeme.clone()
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

