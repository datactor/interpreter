use std::collections::HashMap;
use std::fmt;


// tokernizing부터 구현
#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum TokenType {
    // Single-char tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Percent,        // % 추가

    // One or two char tokens
    Equal,
    Bang,
    BangEqual,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Star,
    StarStar,       // ** 추가

    // Literals
    Identifier,     // 식별자 이름 지정할 때 규칙 만들어 둘 것
    String,
    Number,

    // Keywords
    And,
    True,
    False,
    Def,
    For,
    In,
    If,
    Elif,
    Else,
    Or,
    Print,
    Return,
    While,
    Class,
    Lambda,
    Nil,
    Super,
    This,
    Var,

    Eof,
}

#[derive(Debug, Clone)]
pub enum Literal {
    Identifier(String),
    Str(String),
    Number(f64),        // 숫자는 모두 받을 것이므로 f64
}

#[derive(Clone)]
pub struct Token {
    pub toktype: TokenType,
    pub lexing: Vec<u8>,            // clone해서 사용(1바이트)
    pub literal: Option<Literal>,   // 식별자, 문자, 숫자 중 하나
    pub line: usize,                // 라인은 1부터
    pub col: i64,                   // 칼럼은 -1부터
}

// pub trait Debug {
//     fn fmt(&self, f: &mut Formatter<'_>) -> Result;
// }
impl fmt::Debug for Token {
    fn fmt(&self, form: &mut fmt::Formatter) -> fmt::Result {
        write!(
            form,
            "Token {{ tktype: {:?}, lexing: \"{}\", literal: {:?}, line: {:?}, col: {:?}}}",
            self.toktype,
            String::from_utf8(self.lexing.clone()).unwrap(),
            self.literal,
            self.line,
            self.col
        )
    }
}

pub fn check_tokens(input: String) -> Result<Vec<Token>, Error> {
    let mut lexer: Lexer = Default::default();  // Returns the "default value" for a type.

    lexer.check_tokens(input);

    match lexer.err {
        Some(err) => Err(err),
        None => Ok(lexer.tokens),               // 에러가 없을 시에 Ok 반환함
    }
}

#[derive(Debug)]
pub struct Error {      // Error type 정의해서 lexer struct의 필드 원소 타입으로 넣음
    pub what: String,
    pub line: usize,
    pub col: i64,
}

pub struct Lexer {
    source: Vec<u8>,    // u8로 받는 이유
    tokens: Vec<Token>, // Vec<Token>
    err: Option<Error>,
    start: usize,
    cursor: usize,
    line: usize,
    col: i64,
    keywords: HashMap<String, TokenType>,
}

// keyword type checker
// pub trait Default: Sized {
//     fn default() -> Self;
// }
impl Default for Lexer {
    fn default() -> Lexer {
        Lexer {
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
                ("def", TokenType::Def),
                ("if", TokenType::If),
                ("elif", TokenType::Elif),
                ("else", TokenType::Else),
                ("or", TokenType::Or),
                ("print", TokenType::Print),
                ("class", TokenType::Class),
                ("return", TokenType::Return),
                ("while", TokenType::While),
                ("labmda", TokenType::Lambda),
                ("super", TokenType::Super),
                ("this", TokenType::This),
                ("nil", TokenType::Nil),
                ("var", TokenType::Var),
            ]
                .into_iter()
                .map(|(key, val)| (String::from(key), val))
                .collect(),
        }
    }
}

impl Lexer {
    fn check_tokens(&mut self, input: String) {
        self.source = input.into_bytes();       // into_bytes()

        while !self.done() {
            self.start = self.cursor;
            self.check_token();
        }

        match self.err {
            Some(_) => {}
            None => self.tokens.push(Token {
                toktype: TokenType::Eof,
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

    fn check_token(&mut self) {
        let c = self.nexting();

        match c {
            '%' => self.add_token(TokenType::Percent),
            '&' => self.add_token(TokenType::And),
            '|' => self.add_token(TokenType::Or),
            '*' => {
                let matches_eq = self.matches('*');
                self.add_token(if matches_eq {
                    TokenType::StarStar                         // 추가 할 것
                } else {
                    TokenType::Star
                })
            }

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
            '!' => {
                let matches_eq = self.matches('=');
                self.add_token(if matches_eq {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                })
            }
            '=' => {
                let matches_eq = self.matches('=');
                self.add_token(if matches_eq {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                })
            }
            '<' => {
                let matches_eq = self.matches('=');
                self.add_token(if matches_eq {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                })
            }
            '>' => {
                let matches_eq = self.matches('=');
                self.add_token(if matches_eq {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                })
            }
            '/' => {
                if self.matches('/') {
                    while self.peek() != '\n' && !self.is_end() {
                        self.nexting();
                    }
                } else {
                    self.add_token(TokenType::Slash)
                }
            }
            ' ' | '\r' | '\t' => {}
            '\n' => {
                self.line += 1;
                self.col = 0
            }
            '"' => self.string(),
            '\'' => self.string2(),
            _ => {
                if Lexer::is_decimal_digit(c) {
                    self.number()
                } else if Lexer::is_alpha(c) {
                    self.identifier()
                } else {
                    self.err = Some(Error {
                        what: format!("Lexer can't handle {}", c),
                        line: self.line,
                        col: self.col,
                    })
                }
            }
        }
    }

    fn is_alpha(c: char) -> bool {
        c.is_alphabetic()
    }

    fn is_decimal_digit(c: char) -> bool {
        c.is_ascii_digit()
    }

    fn is_alphanumeric(c: char) -> bool {
        Lexer::is_alpha(c) || Lexer::is_decimal_digit(c)
    }

    fn peek(&self) -> char {
        if self.is_end() {
            '\0'
        } else {
            char::from(self.source[self.cursor])
        }
    }

    fn peek_next(&self) -> char {
        if self.cursor + 1 >= self.source.len() {
            '\0'
        } else {
            char::from(self.source[self.cursor + 1])
        }
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_end() {
            if self.peek() == '\n' {
                self.line += 1
            }
            self.nexting();
        }

        if self.is_end() {
            self.err = Some(Error {
                what: "Unterminated string".to_string(),
                line: self.line,
                col: self.col,
            })
        }

        assert_eq!(self.peek(), '"');

        self.nexting();

        self.add_token_literal(
            TokenType::String,
            Some(Literal::Str(
                String::from_utf8(self.source[self.start + 1..self.cursor - 1].to_vec())
                    .unwrap(),
            ))
        )
    }

    // 싱글 쿠테이션 마크도 받을 수 있게 하기
    fn string2(&mut self) {
        while self.peek() != '\'' && !self.is_end() {
            if self.peek() == '\n' {
                self.line += 1
            }
            self.nexting();
        }

        if self.is_end() {
            self.err = Some(Error {
                what: "Unterminated string".to_string(),
                line: self.line,
                col: self.col,
            })
        }

        assert_eq!(self.peek(), '\'');

        self.nexting();

        self.add_token_literal(
            TokenType::String,
            Some(Literal::Str(
                String::from_utf8(self.source[self.start + 1..self.cursor - 1].to_vec())
                    .unwrap(),
            ))
        )
    }

    fn identifier(&mut self) {
        while Lexer::is_alphanumeric(self.peek()) {
            self.nexting();
        }

        let literal_val =
            String::from_utf8(self.source[self.start..self.cursor].to_vec()).unwrap();

        let token_type = match self.keywords.get(&literal_val) {
            Some(tokentype) => *tokentype,
            None => TokenType::Identifier,
        };

        match token_type {
            TokenType::Identifier => self.add_token_literal(
                TokenType::Identifier,
                Some(Literal::Identifier(literal_val)),
            ),
            _ => self.add_token(token_type),
        }
    }


    fn number(&mut self) {
        while Lexer::is_decimal_digit(self.peek()) {
            self.nexting();
        }

        if self.peek() == '.' && Lexer::is_decimal_digit(self.peek_next()) {
            self.nexting();
        }

        while Lexer::is_decimal_digit(self.peek()) {
            self.nexting();
        }

        // 숫자값은 그대로 밸류 생성함
        let val: f64 = String::from_utf8(self.source[self.start..self.cursor].to_vec())
            .unwrap()
            .parse()
            .unwrap();

        self.add_token_literal(TokenType::Number, Some(Literal::Number(val)))
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_literal(token_type, None)
    }

    fn add_token_literal(&mut self, token_type: TokenType, literal: Option<Literal>) {
        self.tokens.push(Token {
            toktype: token_type,
            lexing: self.source[self.start..self.cursor].to_vec(),
            literal,
            line: self.line,
            col: self.col,
        })
    }

    fn matches(&mut self, c: char) -> bool {
        if self.is_end() {
            return true;
        }

        if char::from(self.source[self.cursor]) != c {
            return false;
        }

        self.cursor += 1;
        self.col += 1;
        true
    }

    fn done(&self) -> bool {
        self.err.is_some() || self.is_end()
    }

    fn is_end(&self) -> bool {
        self.cursor >= self.source.len()
    }
}