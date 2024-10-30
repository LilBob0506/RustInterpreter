use crate::expr::*;
use crate::entities::*;
use crate::errors::*;
pub struct Parser<'a> {
    tokens: &'a Vec<Token>,
    current: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &Vec<Token>) -> Parser {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Option<Expr> {
        match self.expression() {
            Ok(expr) => Some(expr),
            Err(_) => None,
        }
    }
    fn expression(&mut self) -> Result<Expr, LoxError> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr, LoxError> {
        let expr = self.equality()?;

        if self.is_match(&[TokenType::EQUAL]) {
           let equals = self.previous().dup();
           let value = self.assignment()?;

           if let Expr::Variable(expr) = expr {
                return Ok(Expr::Assign(AssignExpr {
                    name: expr.name.dup(),
                    value: Box::new(value)
                }));
           }

           Err(LoxError::error(0, "Invalid assignment target")); 
        }

        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.comparison()?;

        while self.is_match(&[TokenType::BANG_EQUAL, TokenType::EQUAL]) {
            let operator = self.previous().dup();
            let right = self.comparison()?;
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.term()?;

        while self.is_match(&[
            TokenType::GREATER,
            TokenType::GREATER_EQUAL,
            TokenType::LESS,
            TokenType::LESS_EQUAL,
        ]) {
            let operator = self.previous().dup();
            let right = self.term()?;
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.factor()?;

        while self.is_match(&[TokenType::MINUS, TokenType::PLUS]) {
            let operator = self.previous().dup();
            let right = self.factor()?;
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.unary()?;

        while self.is_match(&[TokenType::SLASH, TokenType::STAR]) {
            let operator = self.previous().dup();
            let right = self.unary()?;
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, LoxError> {
        if self.is_match(&[TokenType::BANG, TokenType::MINUS]) {
            let operator = self.previous().dup();
            let right = self.unary()?;
            return Ok(Expr::Unary(UnaryExpr {
                operator,
                right: Box::new(right),
            }));
        }

        Ok(self.primary()?)
    }

    fn primary(&mut self) -> Result<Expr, LoxError> {
        if self.is_match(&[TokenType::FALSE]) {
            return Ok(Expr::Literal(LiteralExpr {
                value: Some(LiteralValue::Bool(false)),
            }));
        }
        if self.is_match(&[TokenType::TRUE]) {
            return Ok(Expr::Literal(LiteralExpr {
                value: Some(LiteralValue::Bool(false)),
            }));
        }
        if self.is_match(&[TokenType::NIL]) {
            return Ok(Expr::Literal(LiteralExpr {
                value: Some(LiteralValue::Nil),
            }));
        }

        if self.is_match(&[TokenType::NUMBER, TokenType::STRING]) {
            return Ok(Expr::Literal(LiteralExpr {
                value: self.previous().literal.clone(),
            }));
        }

        if self.is_match(&[TokenType::LEFT_PAREN]) {
            let expr = self.expression()?;
            self.consume(TokenType::RIGHT_PAREN, "Expect ')' after expression")?;
            return Ok(Expr::Grouping(GroupingExpr {
                expression: Box::new(expr),
            }));
        }
        Err(LoxError::error(0, "Expect expression."))
    }

    fn consume(&mut self, ttype: TokenType, message: &str) -> Result<Token, LoxError> {
        if self.check(ttype) {
            Ok(self.advance().dup())
        } else {
            Err(Parser::error(self.peek(), message))
        }
    }

    fn error(token: &Token, message: &str) -> LoxError {
        LoxError::parse_error(token, message)
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().is(TokenType::SEMICOLON) {
                return;
            }

            if matches!(
                self.peek().token_type(),
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

            self.advance();
        }
    }

    fn is_match(&mut self, types: &[TokenType]) -> bool {
        for &t in types {
            if self.check(t) {
                self.advance();
                return true;
            }
        }

        false
    }

    fn check(&self, ttype: TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            self.peek().is(ttype)
        }
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().is(TokenType::EOF)
    }

    fn peek(&self) -> &Token {
        self.tokens.get(self.current).unwrap()
    }

    fn previous(&self) -> &Token {
        self.tokens.get(self.current - 1).unwrap()
    }
}

/*pub struct Parser<'a> {
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
    fn consume_block(&mut self) -> Result<Vec<Box<Stmt<'a>>>, ParseError<'a>> {
        let mut statements = Vec::new();

        while self.try_consume(TokenType::RIGHT_BRACE).is_none() {
            match self.consume_declaration() {
                Ok(stmt) => statements.push(stmt),
                Err(err) => return Err(err),
            }
        }

        // Return the collected statements as part of a successful block parse
        Ok(statements)
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
}*/
