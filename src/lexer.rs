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
    Star,           // *

    // One or two char tokens
    Equal,          // = (+declaration)
    Bang,           // !
    BangEqual,      // !=
    EqualEqual,     // ==
    Greater,        // >
    GreaterEqual,   // >=
    Less,           // <
    LessEqual,      // <=

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
    pub lexing: Vec<u8>,
    pub literal: Option<Literal>,
    pub line: usize,
    pub col: i64,
}

// native 라이브러리의 Debug 트레잇 바운딩
//pub trait Debug {
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

pub fn scan_tokens(input: String) -> Result<Vec<Token>, Error> {
    let mut scanner: Scanner = Default::default();

    scanner.scan_tokens(input);

    match scanner.err {
        Some(err) => Err(err),
        None => Ok(scanner.tokens),
    }
}

#[derive(Debug)]
pub struct Error {
    pub what: String,
    pub line: usize,
    pub col: i64,
}

struct Scanner {
    source: Vec<u8>,
    tokens: Vec<Token>,
    err: Option<Error>,
    start: usize,
    cursor: usize,
    line: usize,
    col: i64,
    keywords: HashMap<String, TokenType>,
}
