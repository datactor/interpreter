mod lexer;
mod interpreter;
mod expr;
mod parser;
mod extensions;
mod line_reader;
mod input;
mod repl;

/// interpreter, python list 구현해보기
/// 루프 문에 input실행하고 parsing, mut state 전달
/// 모든 함수에 state 전달
///
/// 비동기는 handler가 여러개
/// RC , REFCEL
/// 파이선 리스트 메소드 요소 구현
/// append()
/// extend()
/// insert()
/// remove()
/// pop()
/// clear()
/// index()
/// count()
/// reverse()
/// copy()
///
///
/// interpreter 구성요소
///                       |   source code
///                 +–––--+-–––-+
/// Lexical         |   Lexer   |
/// analysis        +–––--+-–––-+
///                       |   Tokens
///                 +–––--+--–––+
/// Syntzs          |   Parser  |
/// analysis        +–––--+–--––+
///                       |   AST
///                 +–––--+--–––+           | s |
/// Semantic        |  Semantic |           | y |
/// analysis        |  Analyzer |           | m |
///                 +–––--+–--––+           | b |
///                       |   AST           | o |
///                 +–––--+–--––+           | l |
/// Program         |Interpreter|           |   |
/// evaluation      +–––--+–-––-+           | t |
///                       |   Program       | a |
///                       |   output        | b |
///                                         | l |
///                                         | e |
///
/// 1. lexer로 소스코드를 token 단위로 분석하고,
/// 2. parser로 우선순위에 맞춰서 Abstract Syntax Tree를 만들어 준 뒤
/// - Abstract Syntax Tree?
/// 3. Sementic Analyzer로 의미 분석을 하고(type check)
/// 4. Interpreter가 연산해서 출력함.
///
///
/// 구동할 수 있는 최소 모듈 카피 리팩터
/// 1. single quotation marks 커버드 밸류 역시 스트링으로 받을 수 있게함
/// 2. 파이썬과 같이 선언문 없이 선언할 수 있는 기능 구현(이후에 파싱 시퀀스 바꾸기)
/// 3. 리스트 append 방식 파이썬처럼 구현

use crate::expr::Stmt;

fn main() {
    let extensions = extensions::Extensions {
        lists: true,
        lambdas: true,
    };
    let mut interpreter = repl::mk_interpreter();
    let mut line_reader = line_reader::LineReader::new(".repl-history.txt", ">>> ");
    println!(
        "===================================================\n\
        Welcome to lox ! Using tree-walk interpreter.\n\
        This is refactored from https://github.com/tdp2110/crafting-interpreters-rs\n\
        ===================================================\n",
    );
    loop {
        let readline = line_reader.readline();
        match readline {
            line_reader::LineReadStatus::Line(line) => match lexer::check_tokens(line.clone()) {
                Ok(tokens) => {
                    let mut need_empty_decl = repl::check_eval_tokens(&mut interpreter, tokens.clone(),
                                                    0, extensions, &line);
                    if need_empty_decl == true {
                        repl::eval_tokens2(&mut interpreter, tokens,0, extensions, &line)
                    }
                },
                Err(err) => println!("Tokernizer Failure")
            },
            line_reader::LineReadStatus::Done => break,
        }
    }
}