mod parser;
mod types;
mod lexer;

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
/// 3. Sementic Analyzer로 의미 분석을 하고(type check)
/// 4. Interpreter가 연산해서 출력함.
///
/// token? 코드의 더이상 분해 될 수 없는 최소 단위의 '의미를 가진 텍스트'.
/// token의 종류 -> 키워드, 식별자, 구분자, 연산자, 문자열 리터럴, 숫자, 상수 등
/// tokernizing부터 해보기

use std::boxed::Box;
use std::io::stdin;


fn main() {
    let mut list = types::List::new();
    loop {
        let mut buffer = String::new();
        stdin().read_line(&mut buffer).expect("input error");
        let buff = buffer.trim();
        match buff {
            _ => { println!("{:?}", buff) },
        }

        let mut var = "";
        let mut val = "";
        for (i, v) in buff.chars().enumerate() {
            if v == '=' {
                var = &buff[0..i].trim();
                val = &buff[i + 1..buff.len()].trim();
                break
            }
        }

        // 리스트에 올리기
        match parser::parsing(val) {
            "i8" => println!("i8"),
            "i32" => println!("i32"),
            "i64" => println!("i64"),
            "f64" => println!("f64"),
            "list" => {
                println!("list");
            },
            "String" => println!("String"),
            _ => {},
        }
        // [변수, 밸류, 리스트] 구조 더 고민해보기
    }
}