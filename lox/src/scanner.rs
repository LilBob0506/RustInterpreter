// TODO: (possibly) implement UTF-8 support. currently breaks on non-ascii
// TODO: (possibly) add /* */ multiline comment support (with nesting)
use crate::entities::*;
use crate::error::*;

pub struct Scanner {
    //keywords: std::collections::HashMap<&'static str, TokenType>,
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}
impl Scanner {
    pub fn new(src: String) -> Scanner {
        Scanner {
            source: src,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }
    pub fn scan(&mut self) -> Result<&Vec<Token>, LoxError> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan();
        }
        self.tokens.push(Token::eof(self.line));
        Ok(&self.tokens)
    }
    pub fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
    pub fn scan(&self) {}
}
/*  assert!(
            source.is_ascii(),
            "Source code contains non ASCII characters. Aborting."
        );
        let ret = Scanner {
            keywords: std::collections::HashMap::from([
                ("and", TokenType::AND),
                ("class", TokenType::CLASS),
                ("else", TokenType::ELSE),
                ("false", TokenType::FALSE),
                ("for", TokenType::FOR),
                ("fun", TokenType::FUN),
                ("if", TokenType::IF),
                ("nil", TokenType::NIL),
                ("or", TokenType::OR),
                ("print", TokenType::PRINT),
                ("return", TokenType::RETURN),
                ("super", TokenType::SUPER),
                ("this", TokenType::THIS),
                ("true", TokenType::TRUE),
                ("var", TokenType::VAR),
                ("while", TokenType::WHILE),
            ]),
            source: source.as_bytes(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        };
        ret
    }
    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while self.current != self.source.len() {
            // We are at the beginning of the next lexeme.
            self.start = self.current;
            self.scan_token();
        }
        self.tokens.push(Token {
            token_type: TokenType::EOF,
            lexeme: "",
            line: self.line,
            literal: None,
        });
        &self.tokens
    }
    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            b'(' => self.add_token(TokenType::LEFT_PAREN),
            b')' => self.add_token(TokenType::RIGHT_PAREN),
            b'{' => self.add_token(TokenType::LEFT_BRACE),
            b'}' => self.add_token(TokenType::RIGHT_BRACE),
            b',' => self.add_token(TokenType::COMMA),
            b'.' => self.add_token(TokenType::DOT),
            b'-' => self.add_token(TokenType::MINUS),
            b'+' => self.add_token(TokenType::PLUS),
            b';' => self.add_token(TokenType::SEMICOLON),
            b'*' => self.add_token(TokenType::STAR),
            b'!' => self.equal_differentiator(TokenType::BANG_EQUAL, TokenType::BANG),
            b'=' => self.equal_differentiator(TokenType::EQUAL_EQUAL, TokenType::EQUAL),
            b'<' => self.equal_differentiator(TokenType::LESS_EQUAL, TokenType::LESS),
            b'>' => self.equal_differentiator(TokenType::GREATER_EQUAL, TokenType::GREATER),
            b'/' => {
                if self.current != self.source.len() && self.source[self.current] == b'/' {
                    self.consume_while(&|x| *x != b'\n');
                } else {
                    self.add_token(TokenType::SLASH);
                }
            }
            b' ' | b'\r' | b'\t' => (),
            b'\n' => self.line += 1,
            // Here down could probably more cleanly be expressed with a regexset
            b'"' => {
                loop {
                    if self.current == self.source.len() {
                        crate::error1(self.line, "Unterminated string.");
                    }
                    match self.advance() {
                        b'\n' => self.line += 1,
                        b'"' => break,
                        _ => (),
                    };
                }
                self.start += 1;
                self.current -= 1;
                self.add_token(TokenType::STRING);
                self.start -= 1;
                self.current += 1;
            }
            c => {
                if c.is_ascii_digit() {
                    self.consume_while(&u8::is_ascii_digit);
                    if self.current + 1 < self.source.len()
                        && self.source[self.current] == b'.'
                        && self.source[self.current + 1].is_ascii_digit()
                    {
                        self.advance();
                        self.consume_while(&u8::is_ascii_digit);
                    };
                    self.add_token(TokenType::NUMBER);
                } else if c.is_ascii_alphabetic() || c == b'_' {
                    self.consume_while(&|x| x.is_ascii_alphanumeric() || *x == b'_');
                    let keyword = self
                        .keywords
                        .get(&std::str::from_utf8(&self.source[self.start..self.current]).unwrap());
                    self.add_token(*keyword.or(Some(&TokenType::IDENTIFIER)).unwrap());
                } else {
                    crate::error1(self.line, "Unexpected character.")
                };
            }
        }
    }
    fn consume_while(&mut self, predicate: &dyn Fn(&u8) -> bool) {
        while self.current != self.source.len() && predicate(&self.source[self.current]) {
            self.current += 1;
        }
    }
    fn equal_differentiator(&mut self, longer: TokenType, shorter: TokenType) {
        if self.current == self.source.len() || self.source[self.current] != b'=' {
            self.add_token(shorter);
        } else {
            self.current += 1;
            self.add_token(longer);
        }
    }
    fn advance(&mut self) -> u8 {
        self.current += 1;
        self.source[self.current - 1]
    }
    fn add_token(&mut self, token_type: TokenType) {
        let lexeme;
        unsafe {
            lexeme = std::str::from_utf8_unchecked(&self.source[self.start..self.current]);
            // already asserted to be valid ascii
        }
        self.tokens.push(Token {
            token_type,
            lexeme,
            line: self.line,
            literal: match token_type {
                TokenType::STRING => Some(LiteralValue::Str(String::from(lexeme))),
                TokenType::FALSE => Some(LiteralValue::Bool(false)),
                TokenType::TRUE => Some(LiteralValue::Bool(true)),
                TokenType::NIL => Some(LiteralValue::Nil),
                TokenType::NUMBER => Some(LiteralValue::Num(lexeme.parse().unwrap_or_else(|_| {
                    crate::error1(self.line, &format!("Invalid numeric constant: {}", lexeme));
                    0.0
                }))),
                _ => None,
            },
        });
    }
}*/
