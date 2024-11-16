//use std::sync::Arc;

// TODO: (possibly) implement UTF-8 support. currently breaks on non-ascii
// TODO: (possibly) add /* */ multiline comment support (with nesting)
use crate::entities::*;
use crate::errors::*;
//use std::collections::HashMap;
pub struct Scanner {
    source: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}
impl Scanner {
    pub fn new(src: String) -> Scanner {
        Scanner {
            source: src.chars().collect(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }
    pub fn scan(&mut self) -> Result<&Vec<Token>, LoxResult> {
        let mut had_error: Option<LoxResult> = None;
        while !self.is_at_end() {
            self.start = self.current;
            match self.scans() {
                Ok(_) => {}
                Err(e) => {
                    had_error = Some(e);
                }
            }
        }
        self.tokens.push(Token::eof(self.line));

        if let Some(e) = had_error {
            Err(e)
        } else {
            Ok(&self.tokens)
        }
    }
    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
    fn scans(&mut self) -> Result<(), LoxResult> {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LEFT_PAREN),
            ')' => self.add_token(TokenType::RIGHT_PAREN),
            '{' => self.add_token(TokenType::LEFT_BRACE),
            '}' => self.add_token(TokenType::RIGHT_BRACE),
            ',' => self.add_token(TokenType::COMMA),
            '.' => self.add_token(TokenType::DOT),
            '-' => self.add_token(TokenType::MINUS),
            '+' => self.add_token(TokenType::PLUS),
            ';' => self.add_token(TokenType::SEMICOLON),
            '*' => self.add_token(TokenType::STAR),
            '!' => {
                let tk = if self.equal_differentiator('=') {
                    TokenType::BANG_EQUAL
                } else {
                    TokenType::BANG
                };
                self.add_token(tk);
            }

            '=' => {
                let tk = if self.equal_differentiator('=') {
                    TokenType::EQUAL_EQUAL
                } else {
                    TokenType::EQUAL
                };
                self.add_token(tk);
            }
            '<' => {
                let tk = if self.equal_differentiator('=') {
                    TokenType::LESS_EQUAL
                } else {
                    TokenType::LESS
                };
                self.add_token(tk);
            }
            '>' => {
                let tk = if self.equal_differentiator('=') {
                    TokenType::GREATER_EQUAL
                } else {
                    TokenType::GREATER
                };
                self.add_token(tk);
            }
            '/' => {
                if self.equal_differentiator('/') {
                    while let Some(ch) = self.peak() {
                        if ch != '\n' {
                            self.advance();
                        } else {
                            break;
                        }
                    }
                } else if self.equal_differentiator('*') {
                    // block comment start
                    self.scan_comment()?;
                } else {
                    self.add_token(TokenType::SLASH);
                }
            }
            ' ' | '\r' | '\t' => {}
            '\n' => {
                self.line += 1;
            }
            '"' => {
                self.string()?;
            }
            '0'..='9' => {
                self.number();
            }
            _ if c.is_alphabetic() || c == '_' => {
                self.identifier();
            }
            _ => {
                LoxResult::error(self.line, "Unexpected character.");
            }
        };
        Ok(())
    }

    fn scan_comment(&mut self) -> Result<(), LoxResult> {
        loop {
            match self.peak() {
                Some('*') => {
                    self.advance();
                    if self.equal_differentiator('/') {
                        return Ok(());
                    }
                }
                Some('/') => {
                    self.advance();
                    if self.equal_differentiator('*') {
                        self.scan_comment()?;
                    }
                }
                Some('\n') => {
                    self.advance();
                    self.line += 1;
                }
                None => {
                    return Err(LoxResult::error(self.line, "Unterminated comment"));
                }
                _ => {
                    self.advance();
                }
            }
        }
    }
    fn identifier(&mut self) {
        while Scanner::is_alpha_numeric(self.peak()) {
            self.advance();
        }
        let text: String = self.source[self.start..self.current].iter().collect();
        if let Some(token_type) = Scanner::keywords(text.as_str()) {
            self.add_token(token_type);
        }
        else{
        self.add_token(TokenType::IDENTIFIER);
        }
    }
    fn number(&mut self) {
        while Scanner::is_digit(self.peak()) {
            self.advance();
        }
        if let Some(ch) = self.peak() {
            if ch == '.' {
                if Scanner::is_digit(self.peak_next()) {
                    self.advance();

                    while Scanner::is_digit(self.peak()) {
                        self.advance();
                    }
                }
            }
        }
        let value: String = self.source[self.start..self.current].iter().collect();
        let num: f64 = value.parse().unwrap();
        self.add_token_object(TokenType::NUMBER, Some(LiteralValue::Num(num)));
    }
    fn peak_next(&self) -> Option<char> {
        self.source.get(self.current + 1).copied()
    }
    fn is_digit(ch: Option<char>) -> bool {
        if let Some(ch) = ch {
            ch.is_ascii_digit()
        } else {
            false
        }
    }
    fn is_alpha_numeric(ch: Option<char>) -> bool {
        if let Some(ch) = ch {
            ch.is_ascii_alphabetic() || ch == '_'
        } else {
            false
        }
    }
    fn string(&mut self) -> Result<(), LoxResult> {
        while let Some(ch) = self.peak() {
            match ch {
                '"' => {
                    break;
                }
                '\n' => {
                    self.line += 1;
                }
                _ => {}
            }
            self.advance();
        }
        if self.is_at_end() {
            return Err(LoxResult::error(self.line, "Unterminated String."));
        }
        self.advance();

        let value: String = self.source[self.start + 1..self.current - 1]
            .iter()
            .collect();
        self.add_token_object(TokenType::STRING, Some(LiteralValue::Str(value)));
        Ok(())
    }
    fn advance(&mut self) -> char {
        let result = *self.source.get(self.current).unwrap();
        self.current += 1;
        result
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_object(token_type, None);
    }

    fn add_token_object(&mut self, token_type: TokenType, literal: Option<LiteralValue>) {
        let lexeme: String = self.source[self.start..self.current].iter().collect();
        self.tokens
            .push(Token::new(token_type, lexeme, literal, self.line));
    }
    fn equal_differentiator(&mut self, expected: char) -> bool {
        match self.source.get(self.current) {
            Some(ch) if *ch == expected => {
                self.current += 1;
                true
            }
            _ => false,
        }
    }
    fn peak(&self) -> Option<char> {
        self.source.get(self.current).copied()
    }
    fn peek_next(&self) -> Option<char> {
        self.source.get(self.current + 1).copied()
    }
    fn keywords(check: &str) -> Option<TokenType> {
        match check {
            "and" => Some(TokenType::AND),
            "class" => Some(TokenType::CLASS),
            "else" => Some(TokenType::ELSE),
            "false" => Some(TokenType::FALSE),
            "for" => Some(TokenType::FOR),
            "fun" => Some(TokenType::FUN),
            "if" => Some(TokenType::IF),
            "nil" => Some(TokenType::NIL),
            "or" => Some(TokenType::OR),
            "print" => Some(TokenType::PRINT),
            "return" => Some(TokenType::RETURN),
            "super" => Some(TokenType::SUPER),
            "this" => Some(TokenType::THIS),
            "true" => Some(TokenType::TRUE),
            "var" => Some(TokenType::VAR),
            "while" => Some(TokenType::WHILE),
            "break" => Some(TokenType::Break),
            _ => None,
        }
    }
}
