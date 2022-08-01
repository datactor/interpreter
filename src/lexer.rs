use std::collections::HashMap;
use std::fmt;


// tokernizing부터 구현
#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum TokenType {
    // Single-char tokens
    LeftParen,      // (
    RightParen,     // )
    LeftBrace,      // {
    RightBrace,     // }
    LeftBracket,    // [
    RightBracket,   // ]
    Comma,          // ,
    Dot,            // .
    Minus,          // -
    Plus,           // +
    Semicolon,      // ;
    Slash,          // /
    Percent,        // %

    // One or two char tokens
    Equal,          // = (+declaration)
    Bang,           // !
    BangEqual,      // !=
    EqualEqual,     // ==
    Greater,        // >
    GreaterEqual,   // >=
    Less,           // <
    LessEqual,      // <=
    Star,           // *
    StarStar,       // **

    // Literals
    Identifier,     // 식별자 이름 지정할 때 규칙 만들어 둘 것
    String,         // is_alphabetic() 사용할 것
    Number,         // is_ascii_digit() 사용할 것

    // Keywords
    And,            // and, &
    True,           // true
    False,          // false
    For,            // for
    In,             // in
    If,             // if
    Elif,           // elif
    Else,           // else
    Or,             // or, |
    Print,          // print
    Return,         // return
    While,          // while
    Eof,            // EOF
}

#[derive(Debug, Clone)]
pub enum Literal {
    Identifier(String),
    Str(String),
    Number(f64),        // 파이썬을 짤 것이기 때문에 일단은 f64로 통일해 놓자
}

#[derive(Clone)]
pub struct Token {
    pub tktype: TokenType,
    pub lexing: Vec<u8>,            // clone해서 사용
    pub literal: Option<Literal>,   // 식별자, 문자, 숫자 중 하나
    pub line: usize,                // 라인은 1부터
    pub col: i64,                   // 칼럼은 -1부터
}

// native 라이브러리의 Debug 트레잇 바운딩
// pub trait Debug {
//     #[stable(feature = "rust1", since = "1.0.0")]
//     fn fmt(&self, f: &mut Formatter<'_>) -> Result;
// }
impl fmt::Debug for Token {
    fn fmt(&self, form: &mut fmt::Formatter) -> fmt::Result {
        write!(
            form,
            "Token {{ tktype: {:?}, lexing: \"{}\", literal: {:?}, line: {:?}, col: {:?}}}",
            self.tktype,
            String::from_utf8(self.lexing.clone()).unwrap(),
            self.literal,
            self.line,
            self.col
        )
    }
}


// pub trait Default: Sized {
//     fn default() -> Self;
// }
pub fn check_tokens(input: String) -> Result<Vec<Token>, Error> {
    let mut scanner: Scanner = Default::default();  // Returns the "default value" for a type.

    scanner.scan_tokens(input);

    match scanner.err {
        Some(err) => Err(err),
        None => Ok(scanner.tokens),                 //에러가 없을 시에 Ok 반환함
    }
}

#[derive(Debug)]
pub struct Error {      // Error type 정의
    pub what: String,
    pub line: usize,
    pub col: i64,
}

pub struct Scanner {
    source: Vec<u8>,
    tokens: Vec<Token>,
    err: Option<Error>,
    start: usize,
    cursor: usize,
    line: usize,
    col: i64,
    keywords: HashMap<String, TokenType>,
}

// keyword type checker
// native trait Default
impl Default for Scanner {
    fn default() -> Scanner {
        Scanner {
            source: Vec::new(),
            tokens: Vec::new(),
            err: None,
            start: 0,
            cursor: 0,
            line: 1,
            col: -1,
            keywords: vec![
                ("and", TokenType::And),
                ("true", TokenType::True),
                ("false", TokenType::False),
                ("in", TokenType::In),
                ("for", TokenType::For),
                ("if", TokenType::If),
                ("elif", TokenType::Elif),
                ("else", TokenType::Else),
                ("or", TokenType::Or),
                ("print", TokenType::Print),
                ("return", TokenType::Return),
                ("while", TokenType::While),
            ].into_iter()   // vec구조 원소별로 iterator type으로 넘기기
                .map(|(key, val)| (String::from(key), val))// to String
                .collect(),
            // | | 클로저 -> 환경을 캡처 할 수 있는 익명 함수. 나중에 실행하기 위해 저장 -> 개념 파악하기
            // collect() chars 같은 컬렉션에서도 빌드 해 iter 할 수 있음.
        }
    }
}

impl Scanner {
    fn scan_tokens(&mut self, input: String) {
        self.source = input.into_bytes();       // into_bytes()

        while !self.done() {
            self.start = self.current;
            self.scan_token();
        }

        match self.err {
            Some(_) => {}
            None => self.tokens.push(Token {
                tktype: TokenType::Eof,
                lexing: Vec::new(),
                literal: None,
                line: self.line,
                col: self.col,
            })
        }
    }

    fn nexting(&mut self) -> char {
        self.cursor += 1;
        self.col += 1;

        char::from(self.source[self.cursor - 1])
    }

    fn scan_token(&mut self) {
        let c = self.nexting();

        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            '[' => self.add_token(TokenType::LeftBracket),
            ']' => self.add_token(TokenType::RightBracket),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '%' => self.add_token(TokenType::Percent),




            _ => {},
        }
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_literal(token_type, None)
    }

    fn add_token_literal(&mut self, token_type: TokenType, literal: Option<Literal>) {
        self.tokens.push(Token {
            tktype: token_type,
            lexing: self.source[self.start..self.cursor].to_vec(),
            literal,            // 인자와 필드네임이 같으면 생략 가능!!!
            line: self.line,
            col: self.col,
        })
    }

    // error or is_end 일 경우
    fn done(&self) -> bool {
        self.err.is_some() || self.is_end()
    }

    // 다 읽었을 때
    fn is_end(&self) -> bool {
        self.cursor >= self.source.len()
    }


}