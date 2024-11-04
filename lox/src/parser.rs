use std::rc::Rc;

use crate::entities::*;
use crate::errors::*;
use crate::expr::*;
use crate::stmt::*;
use crate::HAD_ERROR;

pub struct Parser<'a> {
    tokens: &'a [Token],
    current: usize,
    had_error: bool,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &Vec<Token>) -> Parser {
        Parser {
            tokens,
            current: 0,
            had_error: false,
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, LoxResult> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            statements.push(self.declaration()?)
        }
        Ok(statements)
    }

    pub fn success(&self) -> bool {
        !self.had_error
    }

    fn expression(&mut self) -> Result<Expr, LoxResult> {
        self.assignment()
    }

    fn declaration(&mut self) -> Result<Stmt, LoxResult> {
        let result = if self.is_match(&[TokenType::FUN]) {
            self.function("function")
        } else if self.is_match(&[TokenType::VAR]) {
            self.var_declaration()
        } else {
            self.statement()
        };

        if result.is_err() {
            self.synchronize();
        }
        result
    }

    fn statement(&mut self) -> Result<Stmt, LoxResult> {
        if self.is_match(&[TokenType::FOR]) {
            return self.for_statement();
        }
        if self.is_match(&[TokenType::IF]) {
            return self.if_statement();
        }

        if self.is_match(&[TokenType::PRINT]) {
            return self.print_statement();
        }

        if self.is_match(&[TokenType::WHILE]) {
            return self.while_statement();
        }

        if self.is_match(&[TokenType::LEFT_BRACE]) {
            return Ok(Stmt::Block(BlockStmt {
                statements: self.block()?,
            }));
        }

        self.expression_statement()
    }
    fn for_statement(&mut self) -> Result<Stmt, LoxResult> {
        self.consume(TokenType::LEFT_PAREN, "Expect '(' after 'for'.")?;

        let initializer = if self.is_match(&[TokenType::SEMICOLON]) {
            None
        } else if self.is_match(&[TokenType::VAR]) {
            Some(self.var_declaration()?)
        } else {
            Some(self.expression_statement()?)
        };

        let condition = if self.check(TokenType::SEMICOLON) {
            None
        } else {
            Some(self.expression()?)
        };

        self.consume(TokenType::SEMICOLON, "Expect ';' after loop condition.")?;

        let increment = if self.check(TokenType::RIGHT_PAREN) {
            None
        } else {
            Some(self.expression()?)
        };

        self.consume(TokenType::RIGHT_PAREN, "Expect ')' after for clauses.")?;

        let mut body = self.statement()?;

        if let Some(incr) = increment {
            body = Stmt::Block(BlockStmt {
                statements: vec![body, Stmt::Expression(ExpressionStmt { expression: incr })],
            });
        }

        body = Stmt::While(WhileStmt {
            condition: if let Some(cond) = condition {
                cond
            } else {
                Expr::Literal(LiteralExpr {
                    value: Some(LiteralValue::Bool(true)),
                })
            },
            body: Box::new(body),
        });

        if let Some(init) = initializer {
            body = Stmt::Block(BlockStmt {
                statements: vec![init, body],
            });
        }

        Ok(body)
    }
    fn if_statement(&mut self) -> Result<Stmt, LoxResult> {
        self.consume(TokenType::LEFT_PAREN, "Expect '(' after 'if'.");
        let condition = self.expression()?;
        self.consume(TokenType::RIGHT_PAREN, "Expect ')' after 'if'.");

        let then_branch = Box::new(self.statement()?);
        let else_branch = if self.is_match(&[TokenType::ELSE]) {
            Some(Box::new(self.statement()?))
        } else {
            None
        };

        Ok(Stmt::If(IfStmt {
            condition,
            then_branch,
            else_branch,
        }))
    }

    fn print_statement(&mut self) -> Result<Stmt, LoxResult> {
        let value = self.expression()?;
        self.consume(TokenType::SEMICOLON, "Expect ';' after value.")?;
        Ok(Stmt::Print(PrintStmt { expression: value }))
    }

    fn var_declaration(&mut self) -> Result<Stmt, LoxResult> {
        let name = self.consume(TokenType::IDENTIFIER, "Expect variable name")?;

        let initializer = if self.is_match(&[TokenType::EQUAL]) {
            Some(self.expression()?)
        } else {
            None
        };

        self.consume(
            TokenType::SEMICOLON,
            "Expect ';' after variable declaration.",
        )?;
        Ok(Stmt::Var(VarStmt { name, initializer }))
    }

    fn while_statement(&mut self) -> Result<Stmt, LoxResult> {
        self.consume(TokenType::LEFT_PAREN, "Expect '(' after 'while'.")?;
        let condition = self.expression()?;
        self.consume(TokenType::RIGHT_PAREN, "Expect ')' after 'while'.")?;
        let body = Box::new(self.statement()?);
        Ok(Stmt::While(WhileStmt { condition, body }))
    }

    fn expression_statement(&mut self) -> Result<Stmt, LoxResult> {
        let expr = self.expression()?;
        self.consume(TokenType::SEMICOLON, "Expect ';' after value.")?;
        Ok(Stmt::Expression(ExpressionStmt { expression: expr }))
    }

    fn function(&mut self, kind: &str) -> Result<Stmt, LoxResult> {
        let name = self.consume(TokenType::IDENTIFIER, &format!("Expect {kind} name"))?;

        self.consume(TokenType::IDENTIFIER, &format!("Expect '(' after {kind} name"))?;
        
        let mut params = Vec::new();
        if !self.check(TokenType::RIGHT_PAREN) {
            parameters.push(self.consume(TokenType::IDENTIFIER, "Expect parameter name"));
            while self.is_match(&[TokenType::COMMA]) {
                if parameters.len() >= 255 {
                    if !self.had_error {
                        let peek = self.peek().dup();
                        self.error(&peek, "Can't have more than 255 parameters");
                    }
                }
                params.push(self.consume(TokenType::IDENTIFIER, "Expect parameter name"));   
            }
        }

        self.consume(TokenType::RIGHT_PAREN, "Expect parameter name")?;

        self.consume(TokenType::LEFT_BRACE, &format!("Expect '{{' before  {kind} body"));
        let body = self.block()?;
        Ok(Stmt::Function(FunctionStmt { name, params: parameters, body, }))
    }

    fn block(&mut self) -> Result<Vec<Stmt>, LoxResult> {
        let mut statements = Vec::new();

        while !self.check(TokenType::RIGHT_BRACE) && !self.is_at_end() {
            statements.push(self.declaration()?);
        }

        self.consume(TokenType::RIGHT_BRACE, "Expect '}' after block.");

        Ok(statements)
    }

    fn equality(&mut self) -> Result<Expr, LoxResult> {
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
    fn assignment(&mut self) -> Result<Expr, LoxResult> {
        let expr = self.or()?;

        if self.is_match(&[TokenType::EQUAL]) {
            let equals = self.previous().dup();
            let value = self.assignment()?;

            if let Expr::Variable(expr) = expr {
                return Ok(Expr::Assign(AssignExpr {
                    name: expr.name.dup(),
                    value: Box::new(value),
                }));
            }

            self.error(&equals, "Invalid assignment target.");
        }

        Ok(expr)
    }

    fn or(&mut self) -> Result<Expr, LoxResult> {
        let mut expr = self.and()?;

        while self.is_match(&[TokenType::OR]) {
            let operator = self.previous().dup();
            let right = Box::new(self.and()?);
            expr = Expr::Logical(LogicalExpr {
                left: Box::new(expr),
                operator,
                right,
            });
        }

        Ok(expr)
    }

    fn and(&mut self) -> Result<Expr, LoxResult> {
        let mut expr = self.equality()?;

        while self.is_match(&[TokenType::AND]) {
            let operator = self.previous().dup();
            let right = Box::new(self.equality()?);
            expr = Expr::Logical(LogicalExpr {
                left: Box::new(expr),
                operator,
                right,
            });
        }

        Ok(expr)
    }
    fn comparison(&mut self) -> Result<Expr, LoxResult> {
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

    fn term(&mut self) -> Result<Expr, LoxResult> {
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

    fn factor(&mut self) -> Result<Expr, LoxResult> {
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

    fn unary(&mut self) -> Result<Expr, LoxResult> {
        if self.is_match(&[TokenType::BANG, TokenType::MINUS]) {
            let operator = self.previous().dup();
            let right = self.unary()?;
            return Ok(Expr::Unary(UnaryExpr {
                operator,
                right: Box::new(right),
            }));
        }

        self.call()
    }

    fn finish_call(&mut self, callee: &Rc<Expr>) -> Result<Expr, LoxResult> {
        let mut arguments = Vec::new();

        if !self.check(TokenType::RIGHT_PAREN) {
            arguments.push(self.expression()?);
            while self.is_match(&[TokenType::COMMA]) {
                if arguments.len() >= 255 {
                    if !self.had_error {
                        let peek = self.peek().dup();
                        LoxResult::runtime_error(&peek, "Can't have more than 255 arguments");
                        self.had_error = true;
                    }
                } else {
                    arguments.push(self.expression()?);
                }
            }
        }

        let paren = self.consume(TokenType:: RIGHT_PAREN, "Expect ')' after arguments.")?;

        Ok(Expr::Call(CallExpr {
            callee: Rc::clone(callee),
            paren,
            arguments,
        }))
    }

    fn call(&mut self) -> Result<Expr, LoxResult> {
        let mut expr = self.primary()?;

        loop {
            if self.is_match(&[TokenType::LEFT_PAREN]) {
                expr = self.finish_call(&RC::new(expr))?;
            } else {
                break;
            }
        }

        Ok(expr)  
    }

    fn primary(&mut self) -> Result<Expr, LoxResult> {
        if self.is_match(&[TokenType::FALSE]) {
            return Ok(Expr::Literal(LiteralExpr {
                value: Some(LiteralValue::Bool(false)),
            }));
        }
        if self.is_match(&[TokenType::TRUE]) {
            return Ok(Expr::Literal(LiteralExpr {
                value: Some(LiteralValue::Bool(true)),
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

        if self.is_match(&[TokenType::IDENTIFIER]) {
            return Ok(Expr::Variable(VariableExpr {
                name: self.previous().clone(),
            }));
        }

        if self.is_match(&[TokenType::LEFT_PAREN]) {
            let expr = self.expression()?;
            self.consume(TokenType::RIGHT_PAREN, "Expect ')' after expression")?;
            return Ok(Expr::Grouping(GroupingExpr {
                expression: Box::new(expr),
            }));
        }
        let peek = self.peek().dup();
        Err(LoxResult::parse_error(&peek, "Expect expression."))
    }

    fn consume(&mut self, ttype: TokenType, message: &str) -> Result<Token, LoxResult> {
        if self.check(ttype) {
            Ok(self.advance().dup())
        } else {
            let peek = self.peek().dup();
            Err(self.error(&peek, message))
        }
    }

    fn error(&mut self, token: &Token, message: &str) -> LoxResult {
        self.had_error = true;
        LoxResult::parse_error(token, message)
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
