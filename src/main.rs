mod lexer;
mod interpreter;
mod expr;
mod parser;
mod extensions;
mod line_reader;
mod input;
mod repl;

// Todo: python list
// 루프 문에 input실행하고 parsing, mut state 전달
// 모든 함수에 state 전달
//
// 구동할 수 있는 최소 모듈 리팩터
// 1. single quotation marks 커버드 밸류 역시 스트링으로 받을 수 있게함
// 2. 파이썬과 같이 선언문 없이 선언할 수 있는 기능 구현(이후에 파싱 시퀀스 바꾸기)
// 3. 리스트 trait 만들어서 내부 함수들에 파이썬 리스트 함수들 추가하기
//
// RC , REFCEL
// 파이선 리스트 메소드 요소 구현
// append()
// extend()
// insert()
// remove()
// pop()
// clear()
// index()
// count()
// reverse()
// copy()
//
//
// interpreter 구성요소
//                       |   source code
//                 +–––--+-–––-+
// Lexical         |   Lexer   |
// analysis        +–––--+-–––-+
//                       |   Tokens
//                 +–––--+--–––+
// Syntzs          |   Parser  |
// analysis        +–––--+–--––+
//                       |   AST
//                 +–––--+--–––+           | s |
// Semantic        |  Semantic |           | y |
// analysis        |  Analyzer |           | m |
//                 +–––--+–--––+           | b |
//                       |   AST           | o |
//                 +–––--+–--––+           | l |
// Program         |Interpreter|           |   |
// evaluation      +–––--+–-––-+           | t |
//                       |   Program       | a |
//                       |   output        | b |
//                                         | l |
//                                         | e |
//
//
//


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
        Welcome to pyrust! Using tree-walk interpreter.\n\
        References: https://github.com/tdp2110/crafting-interpreters-rs\n\
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