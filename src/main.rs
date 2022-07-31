/// interpreter, python list 구현해보기
/// 루프 문에 input실행하고 parsing, mut state 전달
/// 모든 함수에 state 전달
///
/// 비동기는 handler가 여러개
/// RC , REFCEL
/// 파이선 리스트 메소드 요소 전부 구현

use std::collections::VecDeque;
use std::boxed::Box;
use std::io::stdin;


fn main() {
    let mut list = List::new();
    // let a = I32 { val: "".to_string()};
    // list.append(Box::new(a));

    loop {
        let mut buffer = String::new();
        stdin().read_line(&mut buffer).expect("input error");
        let buff = buffer.trim();
        match buff {
            _ => {println!("{:?}", buff.trim())},
        }


        if buff.len() > 3 {
            // 변수 선언시 let이나 var같은 선언문을 명시 하지 않으면 파싱할 때 검색할 경우가 너무 많음
            if &buff[..3] == "let" {
                // =가 한번 이상 나오는 경우(""안이나 == => 등) 에 대해서 선순위로 파싱 해야함(미구현)
                let x = buffer.split("=");
                let mut b = "";
                for i in x {
                    b = i.trim();
                }
                let len = b.len();
                if b.chars().nth(0).unwrap().to_string() == "[".to_string() {
                    if b.chars().nth(len - 1).unwrap().to_string() == "]".to_string() {
                        println!("list {}", b);
                    }
                }
            }
        }
    }
}

struct List {
    lane: VecDeque<Box<dyn Element>>,
}

impl List {
    fn new() -> Self {
        List { lane: VecDeque::new() }
    }

    fn append(&mut self, element: Box<dyn Element>) {
        self.lane.push_back(element);
    }

    fn pop(&mut self) {
        self.lane.pop_back().unwrap();
    }
}


pub trait Element {
    fn vals(&self) -> String;
}

struct I32 {
    val: String,
}

impl Element for I32 {
    fn vals(&self) -> String {
        format!("{}", self.val)
    }
}

struct I8 {
    val: String,
}

impl Element for I8 {
    fn vals(&self) -> String {
        format!("{}", self.val)
    }
}

struct I64 {
    val: String,
}

impl Element for I64 {
    fn vals(&self) -> String {
        format!("{}", self.val)
    }
}

struct Str {
    val: String,
}

impl Element for Str {
    fn vals(&self) -> String {
        format!("{}", self.val)
    }
}