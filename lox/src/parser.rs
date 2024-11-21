use std::rc::Rc;

use crate::entities::*;
use crate::errors::*;
use crate::expr::*;
use crate::stmt::*;

pub struct Parser<'a> {
    tokens: &'a [Token],
    current: usize,
    had_error: bool,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &[Token]) -> Parser {
        Parser {
            tokens,
            current: 0,
            had_error: false,
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Rc<Stmt>>, LoxResult> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            if let Ok(d) = self.declaration() {
                statements.push(d);
            }
        }
        if self.had_error {
            Err(LoxResult::fail())
        } else {
            Ok(statements)
        }
    }

    fn expression(&mut self) -> Result<Expr, LoxResult> {
        self.assignment()
    }

    fn declaration(&mut self) -> Result<Rc<Stmt>, LoxResult> {
        let result = if self.is_match(&[TokenType::CLASS])  {
            self.class_declaration()
        } else if self.is_match(&[TokenType::FUN]) {
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

    fn class_declaration(&mut self) -> Result<Rc<Stmt>, LoxResult> {
        let name = self.consume(TokenType::IDENTIFIER, "Expect class name")?;

        let superclass = if self.is_match(&[TokenType::LESS]) {
            self.consume(TokenType::IDENTIFIER, "Expect superclass name.")?;
            Some(Rc::new(Expr::Variable(Rc::new(VariableExpr {
                name: self.previous().dup(),
            }))))
        } else {
            None
        };
        self.consume(TokenType::LEFT_BRACE, "Expect '{' before class body")?;

        let mut methods = Vec::new();
        while !self.check(TokenType::RIGHT_BRACE) && !self.is_at_end() {
            methods.push(self.function("method")?);
        }

        self.consume(TokenType::RIGHT_BRACE, "Expect '}' after class body")?;

        Ok(Rc::new(Stmt::Class(Rc::new(ClassStmt {
            name, 
            superclass,
            methods: Rc::new(methods),
        }))))
    }

    fn statement(&mut self) -> Result<Rc<Stmt>, LoxResult> {
        if self.is_match(&[TokenType::Break]) {
            let token = self.previous().dup();
            self.consume(TokenType::SEMICOLON, "Expect ';' after break statement.")?;
            return Ok(Rc::new(Stmt::Break(Rc::new(BreakStmt { token }))));
        }
        if self.is_match(&[TokenType::FOR]) {
            return self.for_statement();
        }
        if self.is_match(&[TokenType::IF]) {
            return Ok(Rc::new(self.if_statement()?));
        }

        if self.is_match(&[TokenType::PRINT]) {
            return Ok(Rc::new(self.print_statement()?));
        }

        if self.is_match(&[TokenType::RETURN]) {
            return Ok(Rc::new(self.return_statement()?));
        }

        if self.is_match(&[TokenType::WHILE]) {
            return Ok(Rc::new(self.while_statement()?));
        }

        if self.is_match(&[TokenType::LEFT_BRACE]) {
            return Ok(Rc::new(Stmt::Block(Rc::new(BlockStmt {
                statements: Rc::new(self.block()?),
            }))));
        }

        self.expression_statement()
    }
    fn for_statement(&mut self) -> Result<Rc<Stmt>, LoxResult> {
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
            body = Rc::new(Stmt::Block(Rc::new(BlockStmt {
                statements: Rc::new(vec![
                    body,
                    Rc::new(Stmt::Expression(Rc::new(ExpressionStmt {
                        expression: Rc::new(incr),
            }))),
        ]),
    })));
} 
        


        body = Rc::new(Stmt::While(Rc::new(WhileStmt {
            condition: if let Some(cond) = condition {
                Rc::new(cond)
            } else {
                Rc::new(Expr::Literal(Rc::new(LiteralExpr {
                    value: Some(LiteralValue::Bool(true)),
                })))
            },
            body,
        })));

        if let Some(init) = initializer {
            body = Rc::new(Stmt::Block(Rc::new(BlockStmt {
                statements: Rc::new(vec![init, body]),
            })));
        }

        Ok(body)
    }
    fn if_statement(&mut self) -> Result<Stmt, LoxResult> {
        let _ = self.consume(TokenType::LEFT_PAREN, "Expect '(' after 'if'.");
        let condition = Rc::new(self.expression()?);
        let _ = self.consume(TokenType::RIGHT_PAREN, "Expect ')' after 'if'.");

        let then_branch = self.statement()?;
        let else_branch = if self.is_match(&[TokenType::ELSE]) {
            Some(self.statement()?)
        } else {
            None
        };

        Ok(Stmt::If(Rc::new(IfStmt {
            condition,
            then_branch,
            else_branch,
        })))
    }

    fn print_statement(&mut self) -> Result<Stmt, LoxResult> {
        let value = Rc::new(self.expression()?);
        self.consume(TokenType::SEMICOLON, "Expect ';' after expression.")?;
        Ok(Stmt::Print(Rc::new(PrintStmt { expression: value })))
    }

    fn return_statement(&mut self) -> Result<Stmt, LoxResult> {
        let keyword = self.previous().dup();
        let value = if self.check(TokenType::SEMICOLON) {
            None
        } else {
            Some(Rc::new(self.expression()?))
        };

        self.consume(TokenType::SEMICOLON, "Expect ';' after return value.")?;


        Ok(Stmt::Return(Rc::new(ReturnStmt { keyword, value })))
    }

    fn var_declaration(&mut self) -> Result<Rc<Stmt>, LoxResult> {
        let name = self.consume(TokenType::IDENTIFIER, "Expect variable name")?;

        let initializer = if self.is_match(&[TokenType::EQUAL]) {
            Some(Rc::new(self.expression()?))
        } else {
            None
        };

        self.consume(
            TokenType::SEMICOLON,
            "Expect ';' after variable declaration.",
        )?;
        Ok(Rc::new(Stmt::Var(Rc::new(VarStmt { name, initializer }))))
    }

    fn while_statement(&mut self) -> Result<Stmt, LoxResult> {
        self.consume(TokenType::LEFT_PAREN, "Expect '(' after 'while'.")?;
        let condition = Rc::new(self.expression()?);
        self.consume(TokenType::RIGHT_PAREN, "Expect ')' after 'while'.")?;
        let body = self.statement()?;
        Ok(Stmt::While(Rc::new(WhileStmt { condition, body })))
    }

    fn expression_statement(&mut self) -> Result<Rc<Stmt>, LoxResult> {
        let expr = Rc::new(self.expression()?);
        self.consume(TokenType::SEMICOLON, "Expect ';' after expression.")?;
        Ok(Rc::new(Stmt::Expression(Rc::new(ExpressionStmt {
            expression: expr,
        }))))
    }

    fn function(&mut self, kind: &str) -> Result<Rc<Stmt>, LoxResult> {
        let name = self.consume(TokenType::IDENTIFIER, &format!("Expect {kind} name"))?;

        self.consume(TokenType::LEFT_PAREN, &format!("Expect '(' after {kind} name"))?;
        
        let mut params = Vec::new();
        if !self.check(TokenType::RIGHT_PAREN) {
            params.push(self.consume(TokenType::IDENTIFIER, "Expect parameter name")?);
            while self.is_match(&[TokenType::COMMA]) {
                if params.len() >= 255 && !self.had_error {
                    let peek = self.peek().dup();
                    self.error(&peek, "Can't have more than 255 parameters.");
                }
                params.push(self.consume(TokenType::IDENTIFIER, "Expect parameter name")?);   
            }
        }

        self.consume(TokenType::RIGHT_PAREN, "Expect ')' after parameter name")?;

        self.consume(TokenType::LEFT_BRACE, &format!("Expect '{{' before  {kind} body"))?;
        let body = Rc::new(self.block()?);
        Ok(Rc::new(Stmt::Function(Rc::new(FunctionStmt { 
            name, 
            params: Rc::new(params), 
            body, 
        }))))
    }

    fn block(&mut self) -> Result<Vec<Rc<Stmt>>, LoxResult> {
        let mut statements = Vec::new();

        while !self.check(TokenType::RIGHT_BRACE) && !self.is_at_end() {
            statements.push(self.declaration()?);
        }

        let _ = self.consume(TokenType::RIGHT_BRACE, "Expect '}' after block.");

        Ok(statements)
    }

    fn equality(&mut self) -> Result<Expr, LoxResult> {
        let mut expr = self.comparison()?;

        while self.is_match(&[TokenType::BANG_EQUAL, TokenType::EQUAL_EQUAL]) {
            let operator = self.previous().dup();
            let right = self.comparison()?;
            expr = Expr::Binary(Rc::new(BinaryExpr {
                left: Rc::new(expr),
                operator,
                right: Rc::new(right),
            }));
        }

        Ok(expr)
    }
    fn assignment(&mut self) -> Result<Expr, LoxResult> {
        let expr = self.or()?;

        if self.is_match(&[TokenType::EQUAL]) {
            let equals = self.previous().dup();
            let value = self.assignment()?;

            if let Expr::Variable(expr) = expr {
                return Ok(Expr::Assign(Rc::new(AssignExpr {
                    name: expr.name.dup(),
                    value: Rc::new(value),
                })));
            } else if let Expr::Get(get) = expr {
                return Ok(Expr::Set(Rc::new(SetExpr {
                    literalvalue: Rc::clone(&get.literalvalue),
                    name: get.name.dup(),
                    value: Rc::new(value),
                })))
            }

            self.error(&equals, "Invalid assignment target.");
        }

        Ok(expr)
    }

    fn or(&mut self) -> Result<Expr, LoxResult> {
        let mut expr = self.and()?;

        while self.is_match(&[TokenType::OR]) {
            let operator = self.previous().dup();
            let right = Rc::new(self.and()?);
            expr = Expr::Logical(Rc::new(LogicalExpr {
                left: Rc::new(expr),
                operator,
                right,
            }));
        }

        Ok(expr)
    }

    fn and(&mut self) -> Result<Expr, LoxResult> {
        let mut expr = self.equality()?;

        while self.is_match(&[TokenType::AND]) {
            let operator = self.previous().dup();
            let right = Rc::new(self.equality()?);
            expr = Expr::Logical(Rc::new(LogicalExpr {
                left: Rc::new(expr),
                operator,
                right,
            }));
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
            expr = Expr::Binary(Rc::new(BinaryExpr {
                left: Rc::new(expr),
                operator,
                right: Rc::new(right),
            }));
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, LoxResult> {
        let mut expr = self.factor()?;

        while self.is_match(&[TokenType::MINUS, TokenType::PLUS]) {
            let operator = self.previous().dup();
            let right = self.factor()?;
            expr = Expr::Binary(Rc::new(BinaryExpr {
                left: Rc::new(expr),
                operator,
                right: Rc::new(right),
            }));
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, LoxResult> {
        let mut expr = self.unary()?;

        while self.is_match(&[TokenType::SLASH, TokenType::STAR]) {
            let operator = self.previous().dup();
            let right = self.unary()?;
            expr = Expr::Binary(Rc::new(BinaryExpr {
                left: Rc::new(expr),
                operator,
                right: Rc::new(right),
            }));
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, LoxResult> {
        if self.is_match(&[TokenType::BANG, TokenType::MINUS]) {
            let operator = self.previous().dup();
            let right = self.unary()?;
            return Ok(Expr::Unary(Rc::new(UnaryExpr {
                operator,
                right: Rc::new(right),
            })));
        }

        self.call()
    }

    fn finish_call(&mut self, callee: &Rc<Expr>) -> Result<Expr, LoxResult> {
        let mut arguments = Vec::new();

        if !self.check(TokenType::RIGHT_PAREN) {
            arguments.push(Rc::new(self.expression()?));
            while self.is_match(&[TokenType::COMMA]) {
                if arguments.len() >= 255 {
                    if !self.had_error {
                        let peek = self.peek().dup();
                        return Err(self.error(&peek, "Can't have more than 255 arguments."));
                    }
                } else {
                    arguments.push(Rc::new(self.expression()?));
                }
            }
        }

        let paren = self.consume(TokenType:: RIGHT_PAREN, "Expect ')' after arguments.")?;

        Ok(Expr::Call(Rc::new(CallExpr {
            callee: Rc::clone(callee),
            paren,
            arguments,
        })))
    }

    fn call(&mut self) -> Result<Expr, LoxResult> {
        let mut expr = self.primary()?;

        loop {
            if self.is_match(&[TokenType::LEFT_PAREN]) {
                expr = self.finish_call(&Rc::new(expr))?;
            } else if self.is_match(&[TokenType::DOT]) {
                let name = self.consume(TokenType:: IDENTIFIER, "Expect property name after  '.' .")?;
                expr = Expr::Get(Rc::new(GetExpr { literalvalue: Rc::new(expr), name }))
            } else {
                break;
            }
        }

        Ok(expr)  
    }

    fn primary(&mut self) -> Result<Expr, LoxResult> {
        if self.is_match(&[TokenType::FALSE]) {
            return Ok(Expr::Literal(Rc::new(LiteralExpr {
                value: Some(LiteralValue::Bool(false)),
            })));
        }
        if self.is_match(&[TokenType::TRUE]) {
            return Ok(Expr::Literal(Rc::new(LiteralExpr {
                value: Some(LiteralValue::Bool(true)),
            })));
        }
        if self.is_match(&[TokenType::NIL]) {
            return Ok(Expr::Literal(Rc::new(LiteralExpr {
                value: Some(LiteralValue::Nil),
            })));
        }
        if self.is_match(&[TokenType::NUMBER, TokenType::STRING]) {
            return Ok(Expr::Literal(Rc::new(LiteralExpr {
                value: self.previous().literal.clone(),
            })));
        }
        if self.is_match(&[TokenType::SUPER]) {
            let keyword = self.previous().dup();
            self.consume(TokenType::DOT, "Expect '.' after 'super'.")?;
            let method = self.consume(
                TokenType::IDENTIFIER,
                "Expect superclass method name")?;
            return Ok(Expr::Super(Rc::new(SuperExpr { keyword, method } )));
        }
        if self.is_match(&[TokenType::THIS]) {
            return Ok(Expr::This(Rc::new(ThisExpr {
                keyword: self.previous().dup(),
            })));
        }
        if self.is_match(&[TokenType::IDENTIFIER]) {
            return Ok(Expr::Variable(Rc::new(VariableExpr {
                name: self.previous().clone(),
            })));
        }

        if self.is_match(&[TokenType::LEFT_PAREN]) {
            let expr = self.expression()?;
            self.consume(TokenType::RIGHT_PAREN, "Expect ')' after expression")?;
            return Ok(Expr::Grouping(Rc::new(GroupingExpr {
                expression: Rc::new(expr),
            })));
        }
        let peek = self.peek().dup();
        Err(self.error(&peek, "Expect expression."))
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

