#![allow(unused)]

use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum TokenType {
    // single char
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    // LeftBracket,
    // RightBracket,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // one or two char
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // literals
    Identifier,
    String,
    Number,


    // keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    // Lambda,

    Eof,
}

#[derive(Debug, Clone)]
pub enum Literal {
    Identifier(String),
    Str(String),
    Number(f64),
}

// #[derive(Clone)]
pub struct Token {
    pub ttype: TokenType,
    pub lexeme: Vec<u8>,
    pub literal: Option<Literal>,
    pub line: usize,
    // pub col, u64,
}

impl Token {
    pub fn new(ttype: TokenType, lexeme: Vec<u8>, literal: Option<Literal>, line: usize) -> Self {
        Token {
            ttype: ttype,
            lexeme: lexeme,
            literal: literal,
            line: line,
        } 
    }
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "TokenType: {:?} \n",
            self.ttype
        )
    }
}

impl Clone for Token {
    fn clone(&self) -> Self {
        Self { ttype: self.ttype.clone(), lexeme: self.lexeme.clone(), literal: self.literal.clone(), line: self.line.clone() }
    }
}

