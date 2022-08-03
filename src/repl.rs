use crate::{expr, Stmt};
use crate::extensions;
use crate::input;
use crate::line_reader;
use crate::parser;
use crate::lexer;
use crate::interpreter;

use std::sync::atomic::Ordering;

pub(crate) fn mk_interpreter() -> interpreter::Interpreter {
    let interpreter: interpreter::Interpreter = Default::default();

    {
        let interrupt_clone = interpreter.interrupted.clone();
        ctrlc::set_handler(move || {
            interrupt_clone.store(true, Ordering::Release);
        })
            .expect("Error setting Ctrl-C handler");
    }

    interpreter
}


pub(crate) fn check_eval_tokens(
    interpreter: &mut interpreter::Interpreter,
    mut tokens: Vec<lexer::Token>,
    recursion_depth: i64,
    extensions: extensions::Extensions,
    line: &str,
) -> bool {
    match parser::parse(extensions, tokens.clone()) {
        Ok(stmts) => {
            let stmts2: Vec<expr::Stmt> = stmts
                .iter()
                .enumerate()
                .map(|(idx, stmt)| match stmt {
                    expr::Stmt::Expr(expr) => {
                        let var_sym = expr::Symbol {
                            // hack!!! we should find a fresh varname from somewhere
                            name: format!("isurehopethisisntusedelsewhere{}", idx),
                            line: 0,
                            col: 0,
                        };
                        let var_expr = expr::Expr::Variable(var_sym.clone());
                        expr::Stmt::Block(vec![
                            expr::Stmt::VarDecl(var_sym, Some(expr.clone())),
                            expr::Stmt::If(
                                expr::Expr::Binary(
                                    Box::new(var_expr.clone()),
                                    expr::BinaryOp {
                                        toktype: expr::BinaryOpTy::NotEqual,
                                        line: 0,
                                        col: 0,
                                    },
                                    Box::new(expr::Expr::Literal(expr::Literal::Nil)),
                                ),
                                Box::new(expr::Stmt::Print(var_expr)),
                                None,
                            ),
                        ])
                    }
                    _ => stmt.clone(),
                })
                .collect();
            match interpreter.interpret(&stmts2) {
                Ok(()) => {}
                Err(err) => {
                    if err[0..40] == "attempting to assign to undeclared variable at".to_string()[0..40] {
                        return true
                    } else {
                        println!(
                            "Runtime error: {}\n\n{}",
                            err,
                            interpreter.format_backtrace());
                    }
                },
            } false
        }
        Err(
            err
            @
            parser::Error::TokenMismatch {
                expected: lexer::TokenType::Semicolon,
                found:
                lexer::Token {
                    toktype: lexer::TokenType::Eof,
                    ..
                },
                ..
            },
        ) => {
            let expected_eof = tokens.pop().unwrap();

            tokens.push(lexer::Token {
                toktype: lexer::TokenType::Semicolon,
                lexing: Vec::new(),
                literal: None,
                line: 0,
                col: -1,
            });
            tokens.push(expected_eof);

            if recursion_depth > 0 {
                println!("Errrr2");
            } else {
                return check_eval_tokens(interpreter, tokens, recursion_depth + 1, extensions, line)
            }
            false
        }
        Err(err) => {
            println!("Errrrr3");
            false
        },
    }
}








pub(crate) fn eval_tokens2(
    interpreter: &mut interpreter::Interpreter,
    mut tokens: Vec<lexer::Token>,
    recursion_depth: i64,
    extensions: extensions::Extensions,
    line: &str,
) {

    match parser::parse_varerr(extensions, tokens.clone()) {
        Ok(stmts) => {
            let stmts2: Vec<expr::Stmt> = stmts
                .iter()
                .enumerate()
                .map(|(idx, stmt)| match stmt {
                    expr::Stmt::Expr(expr) => {
                        let var_sym = expr::Symbol {
                            // hack!!! we should find a fresh varname from somewhere
                            name: format!("isurehopethisisntusedelsewhere{}", idx),
                            line: 0,
                            col: 0,
                        };
                        let var_expr = expr::Expr::Variable(var_sym.clone());
                        expr::Stmt::Block(vec![
                            expr::Stmt::VarDecl(var_sym, Some(expr.clone())),
                            expr::Stmt::If(
                                expr::Expr::Binary(
                                    Box::new(var_expr.clone()),
                                    expr::BinaryOp {
                                        toktype: expr::BinaryOpTy::NotEqual,
                                        line: 0,
                                        col: 0,
                                    },
                                    Box::new(expr::Expr::Literal(expr::Literal::Nil)),
                                ),
                                Box::new(expr::Stmt::Print(var_expr)),
                                None,
                            ),
                        ])
                    }
                    _ => stmt.clone(),
                })
                .collect();
            match interpreter.interpret(&stmts2) {
                Ok(()) => {}
                Err(err) => {
                    if err[0..40] == "attempting to assign to undeclared variable at".to_string()[0..40] {
                        println!("hi");
                    } else {
                    println!(
                    "Runtime error: {}\n\n{}",
                    err,
                    interpreter.format_backtrace())
                    }
                },
            }
        }
        Err(
            err
            @
            parser::Error::TokenMismatch {
                expected: lexer::TokenType::Semicolon,
                found:
                lexer::Token {
                    toktype: lexer::TokenType::Eof,
                    ..
                },
                ..
            },
        ) => {
            let expected_eof = tokens.pop().unwrap();

            tokens.push(lexer::Token {
                toktype: lexer::TokenType::Semicolon,
                lexing: Vec::new(),
                literal: None,
                line: 0,
                col: -1,
            });
            tokens.push(expected_eof);

            if recursion_depth > 0 {
                println!("Errrr2");
            } else {
                eval_tokens2(interpreter, tokens, recursion_depth + 1, extensions, line)
            }
        }
        Err(err) => println!("Errrrr3"),
    }
}