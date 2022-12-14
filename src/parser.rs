use crate::expr;
use crate::lexer;
use crate::extensions;

use std::fmt;

#[derive(Default)]
struct Parser {
    tokens: Vec<lexer::Token>,
    cursor: usize,
    in_fundec: bool, // in rust, booleans default to false: https://doc.rust-lang.org/std/primitive.bool.html#impl-Default
    extensions: extensions::Extensions,
}

pub enum Error {
    UnexpectedToken(lexer::Token),
    TokenMismatch {
        expected: lexer::TokenType,
        found: lexer::Token,
        maybe_on_err_string: Option<String>,
    },
    MaxParamsExceeded {
        kind: FunctionKind,
        line: usize,
        col: i64,
    },
    ReturnNotInFun {
        line: usize,
        col: i64,
    },
    InvalidAssignment {
        line: usize,
        col: i64,
    },
    TooManyArguments {
        line: usize,
        col: i64,
    },
    ExpectedExpression {
        token_type: lexer::TokenType,
        line: usize,
        col: i64,
    },
    InvalidTokenInUnaryOp {
        token_type: lexer::TokenType,
        line: usize,
        col: i64,
    },
    InvalidTokenInBinaryOp {
        token_type: lexer::TokenType,
        line: usize,
        col: i64,
    },
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Error::UnexpectedToken(tok) => write!(
                f,
                "Unexpected token {:?} at line={},col={}",
                tok.toktype, tok.line, tok.col
            ),
            Error::TokenMismatch {
                maybe_on_err_string,
                expected,
                found,
            } => {
                write!(
                    f,
                    "Expected token {:?} but found {:?} at line={},col={}",
                    expected, found.toktype, found.line, found.col
                )?;
                if let Some(on_err_string) = maybe_on_err_string {
                    write!(f, ": {}", on_err_string)?;
                }
                fmt::Result::Ok(())
            }
            Error::MaxParamsExceeded { kind, line, col } => write!(
                f,
                "Cannot have more than 255 parameters in a {:?} declaration. Line={},col={}",
                kind, line, col
            ),
            Error::ReturnNotInFun { line, col } => write!(
                f,
                "return statement not enclosed in a FunDecl at line={},col={}",
                line, col
            ),
            Error::InvalidAssignment { line, col } => {
                write!(f, "invalid assignment target at line={},col={}", line, col)
            }
            Error::TooManyArguments { line, col } => write!(
                f,
                "Cannot have more than 255 arguments to a function call. Line={},col={}",
                line, col
            ),
            Error::ExpectedExpression {
                token_type,
                line,
                col,
            } => write!(
                f,
                "Expected expression, but found token {:?} at line={},col={}",
                token_type, line, col
            ),
            Error::InvalidTokenInUnaryOp {
                token_type,
                line,
                col,
            } => write!(
                f,
                "invalid token in unary op {:?} at line={},col={}",
                token_type, line, col
            ),
            Error::InvalidTokenInBinaryOp {
                token_type,
                line,
                col,
            } => write!(
                f,
                "invalid token in binary op {:?} at line={},col={}",
                token_type, line, col
            ),
        }
    }
}

#[derive(Debug)]
pub enum FunctionKind {
    Function,
    Method,
    Lambda,
}

pub fn parse(
    extensions: extensions::Extensions,
    tokens: Vec<lexer::Token>,
) -> Result<Vec<expr::Stmt>, Error> {
    let mut p = Parser {
        tokens,
        extensions,
        ..Default::default()
    };
    let stmts_or_err = p.parse();

    match stmts_or_err {
        Ok(stmts_or_err) => {
            if !p.is_end() {
                let tok = &p.tokens[p.cursor];
                Err(Error::UnexpectedToken(tok.clone()))
            } else {
                Ok(stmts_or_err)
            }
        }
        Err(err) => Err(err),
    }
}



pub fn parse_varerr(
    extensions: extensions::Extensions,
    tokens: Vec<lexer::Token>,
) -> Result<Vec<expr::Stmt>, Error> {
    let mut p = Parser {
        tokens,
        extensions,
        ..Default::default()
    };
    let stmts_or_err = p.parse_varerr();

    match stmts_or_err {
        Ok(stmts_or_err) => {
            if !p.is_end() {
                let tok = &p.tokens[p.cursor];
                Err(Error::UnexpectedToken(tok.clone()))
            } else {
                Ok(stmts_or_err)
            }
        }
        Err(err) => Err(err),
    }
}


impl Parser {
    pub fn parse(&mut self) -> Result<Vec<expr::Stmt>, Error> { // ??????????????? ?????? ??? ????????????????????? ??????
        let mut statements = Vec::new();
        while !self.is_end() {
            let stmt = self.declaration()?;
            statements.push(stmt);
        }
        Ok(statements)
    }


    pub fn parse_varerr(&mut self) -> Result<Vec<expr::Stmt>, Error> {
        let mut statements = Vec::new();
        statements.push(self.declvar()?);
        while !self.is_end() {
            let stmt = self.declaration()?;
            statements.push(stmt);
        }
        Ok(statements)
    }


    pub fn declvar(&mut self) -> Result<expr::Stmt, Error> {        // ????????? var_decl ??????
        return self.var_decl()
    }

    fn declaration(&mut self) -> Result<expr::Stmt, Error> {
        if self.matches(lexer::TokenType::Var) {              // ?????? ?????? ?????? ?????????
            return self.var_decl();
        }

        if self.matches(lexer::TokenType::Def) {
            return Ok(expr::Stmt::FunDecl(self.fun_decl(FunctionKind::Function)?));
        }

        if self.matches(lexer::TokenType::Class) {
            return self.class_decl();
        }

        self.statement()
    }

    fn class_decl(&mut self) -> Result<expr::Stmt, Error> {
        let name_tok = self
            .consume(lexer::TokenType::Identifier, "Expected class name")?
            .clone();

        let class_symbol = expr::Symbol {
            name: String::from_utf8(name_tok.lexing).unwrap(),
            line: name_tok.line,
            col: name_tok.col,
        };

        let superclass_maybe = if self.matches(lexer::TokenType::Less) {
            let superclass_tok =
                self.consume(lexer::TokenType::Identifier, "Expected class name.")?;
            Some(expr::Symbol {
                name: String::from_utf8(superclass_tok.lexing.clone()).unwrap(),
                line: superclass_tok.line,
                col: superclass_tok.col,
            })
        } else {
            None
        };

        self.consume(lexer::TokenType::LeftBrace, "Expected { after class name")?;

        let mut methods = Vec::new();
        while !self.check(lexer::TokenType::RightBrace) && !self.is_end() {
            methods.push(self.fun_decl(FunctionKind::Method)?);
        }
        let methods = methods;

        self.consume(
            lexer::TokenType::RightBrace,
            "Expected } after class body",
        )?;

        Ok(expr::Stmt::ClassDecl(expr::ClassDecl {
            name: class_symbol,
            superclass: superclass_maybe,
            methods,
        }))
    }

    fn fun_decl(&mut self, kind: FunctionKind) -> Result<expr::FunDecl, Error> {
        let name_tok = self
            .consume(
                lexer::TokenType::Identifier,
                format!("Expected {:?} name", kind).as_ref(),
            )?
            .clone();

        let fun_symbol = expr::Symbol {
            name: String::from_utf8(name_tok.lexing).unwrap(),
            line: name_tok.line,
            col: name_tok.col,
        };

        let (parameters, body) = self.params_and_body(kind)?;

        Ok(expr::FunDecl {
            name: fun_symbol,
            params: parameters,
            body,
        })
    }

    fn params_and_body(
        &mut self,
        kind: FunctionKind,
    ) -> Result<(Vec<expr::Symbol>, Vec<expr::Stmt>), Error> {
        self.consume(
            lexer::TokenType::LeftParen,
            format!("Expected ( after {:?} name", kind).as_ref(),
        )?;

        let mut parameters = Vec::new();

        if !self.check(lexer::TokenType::RightParen) {
            loop {
                if parameters.len() >= 255 {
                    let peek_tok = self.peek();
                    return Err(Error::MaxParamsExceeded {
                        kind,
                        line: peek_tok.line,
                        col: peek_tok.col,
                    });
                }

                let tok = self
                    .consume(lexer::TokenType::Identifier, "Expected parameter name")?
                    .clone();

                parameters.push(expr::Symbol {
                    name: String::from_utf8(tok.lexing).unwrap(),
                    line: tok.line,
                    col: tok.col,
                });

                if !self.matches(lexer::TokenType::Comma) {
                    break;
                }
            }
        }
        let parameters = parameters;

        self.consume(
            lexer::TokenType::RightParen,
            "Expected ) after parameter list",
        )?;
        self.consume(
            lexer::TokenType::LeftBrace,
            "Expected { before function body",
        )?;
        let saved_is_in_fundec = self.in_fundec;
        self.in_fundec = true;
        let body = self.block()?;
        self.in_fundec = saved_is_in_fundec;

        Ok((parameters, body))
    }

    fn var_decl(&mut self) -> Result<expr::Stmt, Error> {
        let name_token = self
            .consume(lexer::TokenType::Identifier, "Expected variable name")?
            .clone();

        let maybe_initializer = if self.matches(lexer::TokenType::Equal) {
            Some(self.expression()?)
        } else {
            None
        };

        self.consume(
            lexer::TokenType::Semicolon,
            "Expected ; after variable declaration",
        )?;

        Ok(expr::Stmt::VarDecl(
            expr::Symbol {
                name: String::from_utf8(name_token.lexing).unwrap(),
                line: name_token.line,
                col: name_token.col,
            },
            maybe_initializer,
        ))
    }

    fn statement(&mut self) -> Result<expr::Stmt, Error> {
        if self.matches(lexer::TokenType::Print) {
            return self.print_statement();
        }

        if self.matches(lexer::TokenType::While) {
            return self.while_statement();
        }

        if self.matches(lexer::TokenType::LeftBrace) {
            return Ok(expr::Stmt::Block(self.block()?));
        }

        if self.matches(lexer::TokenType::For) {
            return self.for_statement();
        }

        if self.matches(lexer::TokenType::If) {
            return self.if_statement();
        }

        if self.matches(lexer::TokenType::Return) {
            return self.return_statement();
        }

        self.expression_statement()
    }

    fn return_statement(&mut self) -> Result<expr::Stmt, Error> {
        let prev_tok = self.previous().clone();

        if !self.in_fundec {
            return Err(Error::ReturnNotInFun {
                line: prev_tok.line,
                col: prev_tok.col,
            });
        }

        let maybe_retval = if !self.matches(lexer::TokenType::Semicolon) {
            Some(self.expression()?)
        } else {
            None
        };

        if maybe_retval.is_some() {
            self.consume(
                lexer::TokenType::Semicolon,
                "Expected ; after return value",
            )?;
        }

        Ok(expr::Stmt::Return(
            expr::SourceLocation {
                line: prev_tok.line,
                col: prev_tok.col,
            },
            maybe_retval,
        ))
    }

    fn for_statement(&mut self) -> Result<expr::Stmt, Error> {
        self.consume(lexer::TokenType::LeftParen, "Expected ( after for.")?;

        let mut maybe_initializer: Option<expr::Stmt> = None;
        if self.matches(lexer::TokenType::Semicolon) {
        } else if self.matches(lexer::TokenType::Equal) {
            // statements.push(self.declvar());
            maybe_initializer = Some(self.var_decl()?)
            // maybe_initializer = Some(Ok);
        } else {
            maybe_initializer = Some(self.expression_statement()?)
        }
        let maybe_initializer = maybe_initializer;

        let mut maybe_condition: Option<expr::Expr> = None;
        if !self.check(lexer::TokenType::Semicolon) {
            maybe_condition = Some(self.expression()?)
        }
        let maybe_condition = maybe_condition;

        self.consume(
            lexer::TokenType::Semicolon,
            "Expected ; after loop condition",
        )?;

        let maybe_increment = if !self.check(lexer::TokenType::RightParen) {
            Some(self.expression()?)
        } else {
            None
        };

        self.consume(
            lexer::TokenType::RightParen,
            "Expected ) after for clauses",
        )?;

        let mut body = self.statement()?;

        if let Some(increment) = maybe_increment {
            body = expr::Stmt::Block(vec![body, expr::Stmt::Expr(increment)])
        }

        let condition = match maybe_condition {
            Some(cond) => cond,
            None => expr::Expr::Literal(expr::Literal::True),
        };
        body = expr::Stmt::While(condition, Box::new(body));

        if let Some(initializer) = maybe_initializer {
            body = expr::Stmt::Block(vec![initializer, body])
        }
        let body = body;

        Ok(body)
    }

    fn while_statement(&mut self) -> Result<expr::Stmt, Error> {
        self.consume(lexer::TokenType::LeftParen, "Expected ( after while")?;
        let cond = self.expression()?;
        self.consume(
            lexer::TokenType::RightParen,
            "Expected ) after while condition",
        )?;
        let body = Box::new(self.statement()?);
        Ok(expr::Stmt::While(cond, body))
    }

    fn if_statement(&mut self) -> Result<expr::Stmt, Error> {
        self.consume(lexer::TokenType::LeftParen, "Expected ( after if.")?;
        let cond = self.expression()?;
        self.consume(
            lexer::TokenType::RightParen,
            "Expected ) after if condition.",
        )?;
        let then_branch = Box::new(self.statement()?);
        let maybe_else_branch = if self.matches(lexer::TokenType::Else) {
            Some(Box::new(self.statement()?))
        } else {
            None
        };

        Ok(expr::Stmt::If(cond, then_branch, maybe_else_branch))
    }

    fn block(&mut self) -> Result<Vec<expr::Stmt>, Error> {
        let mut stmts = Vec::new();

        while !self.check(lexer::TokenType::RightBrace) && !self.is_end() {
            stmts.push(self.declaration()?)
        }

        self.consume(lexer::TokenType::RightBrace, "Expected } after block.")?;

        Ok(stmts)
    }

    fn print_statement(&mut self) -> Result<expr::Stmt, Error> {
        let expr = self.expression()?;
        self.consume(lexer::TokenType::Semicolon, "Expected ; after value")?;
        Ok(expr::Stmt::Print(expr))
    }

    fn expression_statement(&mut self) -> Result<expr::Stmt, Error> {
        let expr = self.expression()?;
        self.consume(lexer::TokenType::Semicolon, "Expected ; after value")?;
        Ok(expr::Stmt::Expr(expr))
    }

    fn expression(&mut self) -> Result<expr::Expr, Error> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<expr::Expr, Error> {
        let expr = self.or()?;

        if self.matches(lexer::TokenType::Equal) {
            let equals = self.previous().clone();
            let new_value = self.assignment()?;

            if let expr::Expr::Variable(sym) = &expr {
                return Ok(expr::Expr::Assign(sym.clone(), Box::new(new_value)));
            } else if let expr::Expr::Get(e, attr) = expr {
                return Ok(expr::Expr::Set(e, attr, Box::new(new_value)));
            }
            if let expr::Expr::Subscript {
                value,
                slice,
                source_location,
            } = expr
            {
                return Ok(expr::Expr::SetItem {
                    lhs: value,
                    slice,
                    rhs: Box::new(new_value),
                    source_location,
                });
            } else {
                return Err(Error::InvalidAssignment {
                    line: equals.line,
                    col: equals.col,
                });
            }
        }

        Ok(expr)
    }

    fn or(&mut self) -> Result<expr::Expr, Error> {
        let mut expr = self.and()?;

        while self.matches(lexer::TokenType::Or) {
            let right = self.and()?;
            expr = expr::Expr::Logical(Box::new(expr), expr::LogicalOp::Or, Box::new(right));
        }

        Ok(expr)
    }

    fn and(&mut self) -> Result<expr::Expr, Error> {
        let mut expr = self.equality()?;

        while self.matches(lexer::TokenType::And) {
            let right = self.equality()?;
            expr = expr::Expr::Logical(Box::new(expr), expr::LogicalOp::And, Box::new(right));
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<expr::Expr, Error> {
        let mut expr = self.addition()?;

        while self.match_one_of(vec![
            lexer::TokenType::Greater,
            lexer::TokenType::GreaterEqual,
            lexer::TokenType::Less,
            lexer::TokenType::LessEqual,
        ]) {
            let operator_token = self.previous().clone();
            let right = Box::new(self.addition()?);
            let binop_maybe = Parser::op_token_to_binop(&operator_token);

            match binop_maybe {
                Ok(binop) => {
                    let left = Box::new(expr);
                    expr = expr::Expr::Binary(left, binop, right);
                }
                Err(err) => return Err(err),
            }
        }
        Ok(expr)
    }

    fn addition(&mut self) -> Result<expr::Expr, Error> {
        let mut expr = self.multiplication()?;

        while self.match_one_of(vec![lexer::TokenType::Minus, lexer::TokenType::Plus]) {
            let operator_token = self.previous().clone();
            let right = Box::new(self.multiplication()?);
            let binop_maybe = Parser::op_token_to_binop(&operator_token);

            match binop_maybe {
                Ok(binop) => {
                    let left = Box::new(expr);
                    expr = expr::Expr::Binary(left, binop, right);
                }
                Err(err) => return Err(err),
            }
        }
        Ok(expr)
    }

    fn multiplication(&mut self) -> Result<expr::Expr, Error> {
        let mut expr = self.unary()?;

        while self.match_one_of(vec![lexer::TokenType::Slash, lexer::TokenType::Star]) {
            let operator_token = self.previous().clone();
            let right = Box::new(self.unary()?);
            let binop_maybe = Parser::op_token_to_binop(&operator_token);

            match binop_maybe {
                Ok(binop) => {
                    let left = Box::new(expr);
                    expr = expr::Expr::Binary(left, binop, right);
                }
                Err(err) => return Err(err),
            }
        }
        Ok(expr)
    }

    fn unary(&mut self) -> Result<expr::Expr, Error> {
        if self.match_one_of(vec![lexer::TokenType::Bang, lexer::TokenType::Minus]) {
            let operator_token = self.previous().clone();
            let right = Box::new(self.unary()?);
            let unary_op_maybe = Parser::op_token_to_unary_op(&operator_token);

            return match unary_op_maybe {
                Ok(unary_op) => Ok(expr::Expr::Unary(unary_op, right)),
                Err(err) => Err(err),
            };
        }
        self.call()
    }

    fn call(&mut self) -> Result<expr::Expr, Error> {
        let mut expr = self.primary()?;

        loop {
            if self.matches(lexer::TokenType::LeftParen) {
                expr = self.finish_call(expr)?;
            }



            // else if self.matches(lexer::TokenType::Dot) {
            //     let name_tok = self
            //         .consume(
            //             lexer::TokenType::Identifier,
            //             "Expected property name after '.'.",
            //         )?
            //         .clone();
            //     expr = expr::Expr::Get(
            //         Box::new(expr),
            //         expr::Symbol {
            //             name: String::from_utf8(name_tok.lexing).unwrap(),
            //             line: name_tok.line,
            //             col: name_tok.col,
            //         },
            //     );
            // }



            else if self.extensions.lists && self.matches(lexer::TokenType::LeftBracket) {
                let slice_expr = self.expression()?;
                let token = self.consume(
                    lexer::TokenType::RightBracket,
                    "Expected ] after subscript",
                )?;
                expr = expr::Expr::Subscript {
                    value: Box::new(expr),
                    slice: Box::new(slice_expr),
                    source_location: expr::SourceLocation {
                        line: token.line,
                        col: token.col,
                    },
                };
                // } else {
                //     break;
                // }

////////////////////////////////////



            // dot append test
            } else if self.extensions.lists && self.matches(lexer::TokenType::Dot) {
                println!("hhhh");
                if self.matches(lexer::TokenType::Append) {
                    if self.matches(lexer::TokenType::LeftParen) {
                        // let mut list_elements: Vec<expr> = Vec::new();
                        let slice_expr = self.expression()?;
                        let token = self.consume(
                            lexer::TokenType::RightParen,
                            "Expected ] after subscript",
                        )?;
                        expr = expr::Expr::Subscript {
                            value: Box::new(expr),
                            slice: Box::new(slice_expr),
                            source_location: expr::SourceLocation {
                                line: token.line,
                                col: token.col,
                            },
                        };
                    }
                }





                println!("hi");
                // let mut list_elements = Vec::new();
                // println!("dot");
                //
                // if self.check(lexer::TokenType::Append) {
                //     if self.check(lexer::TokenType::LeftParen) {
                //         if !self.check(lexer::TokenType::RightParen) {
                //             loop {
                //                 list_elements.push(self.expression()?);
                //             }
                //         }
                //     }
                // }
            } else {
                break;
            }











////////////////////////////////////////////////
        }
        Ok(expr)
    }

    fn finish_call(&mut self, callee: expr::Expr) -> Result<expr::Expr, Error> {
        let mut arguments = Vec::new();

        if !self.check(lexer::TokenType::RightParen) {
            loop {
                if arguments.len() >= 255 {
                    let peek_tok = self.peek();
                    return Err(Error::TooManyArguments {
                        line: peek_tok.line,
                        col: peek_tok.col,
                    });
                }
                arguments.push(self.expression()?);
                if !self.matches(lexer::TokenType::Comma) {
                    break;
                }
            }
        }

        let token = self.consume(
            lexer::TokenType::RightParen,
            "Expected ) after arguments.",
        )?;

        Ok(expr::Expr::Call(
            Box::new(callee),
            expr::SourceLocation {
                line: token.line,
                col: token.col,
            },
            arguments,
        ))
    }

    fn primary(&mut self) -> Result<expr::Expr, Error> {
        if self.matches(lexer::TokenType::False) {
            return Ok(expr::Expr::Literal(expr::Literal::False));
        }
        if self.matches(lexer::TokenType::True) {
            return Ok(expr::Expr::Literal(expr::Literal::True));
        }
        if self.matches(lexer::TokenType::Nil) {
            return Ok(expr::Expr::Literal(expr::Literal::Nil));
        }
        if self.matches(lexer::TokenType::Super) {
            let super_tok = self.previous().clone();
            self.consume(lexer::TokenType::Dot, "Expected '.' after 'super'.")?;
            let method_tok = self.consume(
                lexer::TokenType::Identifier,
                "Expected superclass method name.",
            )?;
            return Ok(expr::Expr::Super(
                expr::SourceLocation {
                    line: super_tok.line,
                    col: super_tok.col,
                },
                expr::Symbol {
                    name: String::from_utf8(method_tok.lexing.clone()).unwrap(),
                    line: method_tok.line,
                    col: method_tok.col,
                },
            ));
        }
        if self.matches(lexer::TokenType::Number) {
            match &self.previous().literal {
                Some(lexer::Literal::Number(n)) => {
                    return Ok(expr::Expr::Literal(expr::Literal::Number(*n)))
                }
                Some(l) => panic!(
                    "internal error in parser: when parsing number, found literal {:?}",
                    l
                ),
                None => panic!("internal error in parser: when parsing number, found no literal"),
            }
        }
        if self.matches(lexer::TokenType::String) {
            match &self.previous().literal {
                Some(lexer::Literal::Str(s)) => {
                    return Ok(expr::Expr::Literal(expr::Literal::String(s.clone())))
                }
                Some(l) => panic!(
                    "internal error in parser: when parsing string, found literal {:?}",
                    l
                ),
                None => panic!("internal error in parser: when parsing string, found no literal"),
            }
        }
        if self.matches(lexer::TokenType::This) {
            let prev = self.previous();
            return Ok(expr::Expr::This(expr::SourceLocation {
                line: prev.line,
                col: prev.col,
            }));
        }
        if self.matches(lexer::TokenType::Identifier) {
            match &self.previous().literal {
                Some(lexer::Literal::Identifier(s)) => {
                    return Ok(expr::Expr::Variable(expr::Symbol {
                        name: s.clone(),
                        line: self.previous().line,
                        col: self.previous().col,
                    }))
                }
                Some(l) => panic!(
                    "internal error in parser: when parsing identifier, found literal {:?}",
                    l
                ),
                None => {
                    panic!("internal error in parser: when parsing identifier, found no literal")
                }
            }
        }
        if self.matches(lexer::TokenType::LeftParen) {
            let expr = Box::new(self.expression()?);
            if let Err(err) = self.consume(
                lexer::TokenType::RightParen,
                "Expected ')' after expression.",
            ) {
                return Err(err);
            }
            return Ok(expr::Expr::Grouping(expr));
        }
        if self.extensions.lists && self.matches(lexer::TokenType::LeftBracket) {
            let mut list_elements = Vec::new();

            if !self.check(lexer::TokenType::RightBracket) {
                loop {
                    list_elements.push(self.expression()?);
                    if !self.matches(lexer::TokenType::Comma) {
                        break;
                    }
                }
            }

            self.consume(lexer::TokenType::RightBracket, "Expected ].")?;

            return Ok(expr::Expr::List(list_elements));
        }
        if self.extensions.lambdas && self.matches(lexer::TokenType::Lambda) {
            let (params, body) = self.params_and_body(FunctionKind::Lambda)?;
            return Ok(expr::Expr::Lambda(expr::LambdaDecl { params, body }));
        }

        Err(Error::ExpectedExpression {
            token_type: self.peek().toktype,
            line: self.peek().line,
            col: self.peek().col,
        })
    }

    fn consume(
        &mut self,
        tok: lexer::TokenType,
        on_err_str: &str,
    ) -> Result<&lexer::Token, Error> {
        if self.check(tok) {
            return Ok(self.nexting());
        }
        Err(Error::TokenMismatch {
            expected: tok,
            found: self.peek().clone(),
            maybe_on_err_string: Some(on_err_str.into()),
        })
    }

    fn op_token_to_unary_op(tok: &lexer::Token) -> Result<expr::UnaryOp, Error> {
        match tok.toktype {
            lexer::TokenType::Minus => Ok(expr::UnaryOp {
                toktype: expr::UnaryOpTy::Minus,
                line: tok.line,
                col: tok.col,
            }),
            lexer::TokenType::Bang => Ok(expr::UnaryOp {
                toktype: expr::UnaryOpTy::Bang,
                line: tok.line,
                col: tok.col,
            }),
            _ => Err(Error::InvalidTokenInUnaryOp {
                token_type: tok.toktype,
                line: tok.line,
                col: tok.col,
            }),
        }
    }

    fn equality(&mut self) -> Result<expr::Expr, Error> {
        let mut expr = self.comparison()?;

        while self.match_one_of(vec![
            lexer::TokenType::BangEqual,
            lexer::TokenType::EqualEqual,
        ]) {
            let operator_token = self.previous().clone();
            let right = Box::new(self.comparison()?);

            let binop_maybe = Parser::op_token_to_binop(&operator_token);

            match binop_maybe {
                Ok(binop) => {
                    let left = Box::new(expr);
                    expr = expr::Expr::Binary(left, binop, right);
                }
                Err(err) => return Err(err),
            }
        }
        Ok(expr)
    }

    fn op_token_to_binop(tok: &lexer::Token) -> Result<expr::BinaryOp, Error> {
        match tok.toktype {
            lexer::TokenType::EqualEqual => Ok(expr::BinaryOp {
                toktype: expr::BinaryOpTy::EqualEqual,
                line: tok.line,
                col: tok.col,
            }),
            lexer::TokenType::BangEqual => Ok(expr::BinaryOp {
                toktype: expr::BinaryOpTy::NotEqual,
                line: tok.line,
                col: tok.col,
            }),
            lexer::TokenType::Less => Ok(expr::BinaryOp {
                toktype: expr::BinaryOpTy::Less,
                line: tok.line,
                col: tok.col,
            }),
            lexer::TokenType::LessEqual => Ok(expr::BinaryOp {
                toktype: expr::BinaryOpTy::LessEqual,
                line: tok.line,
                col: tok.col,
            }),
            lexer::TokenType::Greater => Ok(expr::BinaryOp {
                toktype: expr::BinaryOpTy::Greater,
                line: tok.line,
                col: tok.col,
            }),
            lexer::TokenType::GreaterEqual => Ok(expr::BinaryOp {
                toktype: expr::BinaryOpTy::GreaterEqual,
                line: tok.line,
                col: tok.col,
            }),
            lexer::TokenType::Plus => Ok(expr::BinaryOp {
                toktype: expr::BinaryOpTy::Plus,
                line: tok.line,
                col: tok.col,
            }),
            lexer::TokenType::Minus => Ok(expr::BinaryOp {
                toktype: expr::BinaryOpTy::Minus,
                line: tok.line,
                col: tok.col,
            }),
            lexer::TokenType::Star => Ok(expr::BinaryOp {
                toktype: expr::BinaryOpTy::Star,
                line: tok.line,
                col: tok.col,
            }),
            lexer::TokenType::Slash => Ok(expr::BinaryOp {
                toktype: expr::BinaryOpTy::Slash,
                line: tok.line,
                col: tok.col,
            }),
            _ => Err(Error::InvalidTokenInBinaryOp {
                token_type: tok.toktype,
                line: tok.line,
                col: tok.col,
            }),
        }
    }

    fn match_one_of(&mut self, types: Vec<lexer::TokenType>) -> bool {
        for toktype in types.iter() {
            if self.matches(*toktype) {
                return true;
            }
        }
        false
    }

    fn matches(&mut self, toktype: lexer::TokenType) -> bool {
        if self.check(toktype) {
            self.nexting();
            return true;
        }
        false
    }

    fn check(&self, toktype: lexer::TokenType) -> bool {
        if self.is_end() {
            return false;
        }

        self.peek().toktype == toktype
    }

    fn nexting(&mut self) -> &lexer::Token {
        if !self.is_end() {
            self.cursor += 1
        }

        self.previous()
    }

    fn is_end(&self) -> bool {
        self.peek().toktype == lexer::TokenType::Eof
    }

    fn is_equal(&self) -> bool {                                // is_equal ?????? ??????
        self.peek().toktype == lexer::TokenType::Equal
    }

    fn peek(&self) -> &lexer::Token {
        &self.tokens[self.cursor]
    }

    fn previous(&self) -> &lexer::Token {
        &self.tokens[self.cursor - 1]
    }
}
