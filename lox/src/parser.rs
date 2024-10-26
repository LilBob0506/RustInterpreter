use crate::entities::{ParseError, Token, TokenType};
use crate::expr2::Expr;
use crate::stmt::{self, Stmt};

type ExprBox<'a> = Result<Box<Expr<'a>>, ParseError<'a>>;
type StmtBox<'a> = Result<Box<Stmt<'a>>, ParseError<'a>>;
pub struct Parser<'a> {
    tokens: &'a Vec<Token>,
    index: usize,
}
impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Parser {
        Parser { tokens, index: 0 }
    }
    pub fn parse(&mut self) -> Vec<Box<Stmt>> {
        let mut statements: Vec<Box<Stmt>> = Vec::new();
        while self.tokens[self.index].token_type != TokenType::EOF {
            match self.consume_declaration() {
                Ok(s) => statements.push(s),
                Err(e) => crate::error(e.token, e.message)
            }
        }
        statements
    }
    fn consume_declaration(&mut self) -> StmtBox<'a> {
        if self.tokens[self.index].token_type == TokenType::VAR {
            self.index += 1;
            self.consume_var_declaration()
        } else {
            self.consume_statement()
        }
        .inspect_err(|_| self.synchronize())
    }
    fn consume_var_declaration(&mut self) -> StmtBox<'a> {
        if self.try_consume(TokenType::IDENTIFIER).is_none() {
            return Err(ParseError {
                token: &self.tokens[self.index],
                message: "Expected variable name.",
            });
        }
        let mut initializer = None;
        if self.try_consume(TokenType::EQUAL).is_some() {
            initializer = Some(*self.consume_expression()?);
        }
        if self.try_consume(TokenType::SEMICOLON).is_none() {
            return Err(ParseError {
                token: &self.tokens[self.index],
                message: "Expected ';' after variable declaration.",
            });
        }
        Ok(Box::new(Stmt::Var {
            name: &self.tokens[self.index],
            initializer,
        }))

        /*
          Token name = consume(IDENTIFIER, "Expect variable name.");

          Expr initializer = null;
          if (match(EQUAL)) {
            initializer = expression();
          }

          consume(SEMICOLON, "Expect ';' after variable declaration.");
          return new Stmt.Var(name, initializer);
        } */
    }
    fn consume_statement(&mut self) -> StmtBox<'a> {
        match self.tokens[self.index].token_type {
            TokenType::PRINT => {
                self.index += 1;
                self.consume_print_stmt()
            }
            TokenType::LEFT_BRACE => {
                self.index += 1;
                self.consume_print_stmt()
            }
            _ => self.consume_expression_stmt(),
        }
    }
    fn consume_print_stmt(&mut self) -> StmtBox<'a> {
        let e = self.consume_expression()?;
        if self.try_consume(TokenType::SEMICOLON).is_none() {
            return Err(ParseError {
                token: &self.tokens[self.index],
                message: "Expected ';' after value.",
            });
        }
        Ok(Box::new(Stmt::Print { expression: *e }))
    }
    fn consume_expression_stmt(&mut self) -> StmtBox<'a> {
        let e = self.consume_expression()?;
        if self.try_consume(TokenType::SEMICOLON).is_none() {
            return Err(ParseError {
                token: &self.tokens[self.index],
                message: "Expected ';' after expression.",
            });
        }
        Ok(Box::new(Stmt::Expression { expression: *e }))
    }
    fn consume_block(&mut self) {
        let mut statements = Vec::new();

        while self.try_consume(TokenType::RIGHT_BRACE).is_none() {
            statements.push(self.consume_declaration());
        }

        Err(ParseError {
            token: &self.tokens[self.index],
            message: "Expected ';' after expression.",
        }); 

        statements; 
    }
    fn consume_expression(&mut self) -> ExprBox<'a> {
        self.consume_equality()
    }
    fn consume_equality(&mut self) -> ExprBox<'a> {
        self.consume_left_associative_binary_operators(
            Self::consume_comparison,
            &[TokenType::EQUAL_EQUAL, TokenType::BANG_EQUAL],
        )
    }
    fn consume_comparison(&mut self) -> ExprBox<'a> {
        self.consume_left_associative_binary_operators(
            Self::consume_term,
            &[
                TokenType::LESS,
                TokenType::LESS_EQUAL,
                TokenType::GREATER,
                TokenType::GREATER_EQUAL,
            ],
        )
    }
    fn consume_term(&mut self) -> ExprBox<'a> {
        self.consume_left_associative_binary_operators(
            Self::consume_factor,
            &[TokenType::MINUS, TokenType::PLUS],
        )
    }
    fn consume_factor(&mut self) -> ExprBox<'a> {
        self.consume_left_associative_binary_operators(
            Self::consume_unary,
            &[TokenType::STAR, TokenType::SLASH],
        )
    }
    fn consume_unary(&mut self) -> ExprBox<'a> {
        let operator = &self.tokens[self.index];
        if [TokenType::BANG, TokenType::MINUS].contains(&operator.token_type) {
            let right = self.consume_unary()?;
            return Ok(Box::new(Expr::Unary { operator, right }));
        }
        self.consume_primary()
    }
    fn consume_primary(&mut self) -> ExprBox<'a> {
        let tok = &self.tokens[self.index];
        self.index += 1;
        match tok.token_type {
            TokenType::FALSE
            | TokenType::TRUE
            | TokenType::NUMBER
            | TokenType::STRING
            | TokenType::NIL => Ok(Box::new(Expr::Literal {
                value: tok.literal.as_ref().unwrap(),
            })),
            TokenType::IDENTIFIER => Ok(Box::new(Expr::Variable { name: tok })),
            TokenType::LEFT_PAREN => {
                let group = self.consume_expression()?;
                if self.try_consume(TokenType::RIGHT_PAREN).is_none() {
                    return Err(ParseError {
                        token: &self.tokens[self.index],
                        message: "Expected ')' after expression.",
                    });
                }
                Ok(Box::new(Expr::Grouping { expression: group }))
            }
            _ => Err(ParseError {
                token: &tok,
                message: "Expected expression.",
            }),
        }
    }
    fn consume_left_associative_binary_operators(
        &mut self,
        consumer: fn(&mut Parser<'a>) -> ExprBox<'a>,
        operators: &[TokenType],
    ) -> ExprBox<'a> {
        let mut ret = consumer(self)?;
        loop {
            let operator = &self.tokens[self.index];
            if !operators.contains(&operator.token_type) {
                return Ok(ret);
            }
            self.index += 1;
            let right = consumer(self)?;
            ret = Box::new(Expr::Binary {
                left: ret,
                operator: &operator,
                right,
            });
        }
    }
    fn synchronize(&mut self) {
        while self.tokens[self.index].token_type != TokenType::EOF {
            if self.tokens[self.index].token_type == TokenType::SEMICOLON {
                return;
            }
            self.index += 1;
            if matches!(
                self.tokens[self.index].token_type,
                TokenType::CLASS
                    | TokenType::FUN
                    | TokenType::VAR
                    | TokenType::FOR
                    | TokenType::IF
                    | TokenType::WHILE
                    | TokenType::PRINT
                    | TokenType::RETURN
            ) {
                return;
            }
        }
    }
    fn try_consume(&mut self, token_type: TokenType) -> Option<&Token> {
        let tok = &self.tokens[self.index];
        if tok.token_type == token_type {
            self.index += 1;
            return Some(tok);
        }
        None
    }
}
