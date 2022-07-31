mod parser;
mod types;

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
