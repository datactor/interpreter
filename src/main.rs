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
            _ => { println!("{:?}", buff) },
        }

        let mut s = buff.split("=");

        let mut var = s.nth(0).unwrap().trim();
        let mut val = "";

        let s = buff.split("=");
        for (i,v) in s.enumerate() {
            if i == 0 {
                var = v.trim()
            } else {
                val = v.trim();
            }
        }
        // println!("var is {}, val is {}", var, val);


        // i8, i32, i64, f64, Str 순서대로 파싱시도 후
        // 리스트에 올리기
        match parsing(val) {
            "i8" => println!("i8"),
            "i32" => println!("i32"),
            "i64" => println!("i64"),
            "f64" => println!("f64"),
            "list" => println!("list"),
            "String" => println!("String"),
            _ => {},
        }

        // 변수 선언이 아니라 int값만 입력하면 파이썬 처럼 프린트하기 others값들을 입력하면
        // 프로그램 종료가 아닌 에러 메시지로 변경하기
    }
}

fn parsing(val: &str) -> &str {
    let parsing_val = val.parse::<i8>();
    match parsing_val {
        Ok(parsing_Val) => return "i8",
        Err(error) => {},
    }
    let parsing_val2 = val.parse::<i32>();
    match parsing_val2 {
        Ok(parsing_val2) => return "i32",
        Err(error) => {},
    }
    let parsing_val3 = val.parse::<i64>();
    match parsing_val3 {
        Ok(parsing_val3) => return "i64",
        Err(error) => {},
    }
    let parsing_val4 = val.parse::<f64>();
    match parsing_val4 {
        Ok(parsing_val4) => return "f64",
        Err(error) => {},
    }
    let len = val.len();
    if val.chars().nth(0) != None {
        if val.chars().nth(0).unwrap() == '[' &&
            val.chars().nth(len - 1).unwrap() == ']' {
            return "list"
        } else { return "String" }
    } return "String"
    // return "String" -> int값이 아니면 error 메시지 출력으로 변경 할 것
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