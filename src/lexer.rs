#![allow(unused_imports, unused)]
use std::fs::{self, File};
use std::io;
use std::error::Error;
use std::collections::HashMap;

use crate::error_formatting;

use crate::token::{self, Literal};
use token::Token;

use token::TokenType;

#[derive(Clone)]
pub struct Lexer {
    source: Vec<u8>,
    pub tokens: Vec<Token>,

    start: usize,
    current: usize,
    line: usize,

    keywords: HashMap<String, TokenType>,
}

impl Lexer {
    pub fn new() -> Self {
        Lexer {
            source: vec![],
            tokens: vec![],

            start: 0,
            current: 0,
            line: 1,

            keywords: vec![
                ("and", TokenType::And),
                ("class", TokenType::Class),
                ("else", TokenType::Else),
                ("false", TokenType::False),
                ("for", TokenType::For),
                ("fun", TokenType::Fun),
                ("if", TokenType::If),
                ("nil", TokenType::Null),
                ("or", TokenType::Or),
                ("print", TokenType::Print),
                ("return", TokenType::Return),
                ("super", TokenType::Super),
                ("this", TokenType::This),
                ("true", TokenType::True),
                ("var", TokenType::Var),
                ("while", TokenType::While),
                // ("lambda", TokenType::Lambda),
            ].into_iter().map(|(k, v)|(String::from(k), v)).collect(),
        }
    }

    pub fn scan_tokens(&mut self, input: String) {
        self.source = input.into_bytes();

        while !self.source_idx_end() {
            self.start = self.current;
            self.scan_token(); 
        }
    }

    pub fn scan_token(&mut self) {
        
        let byte = self.advance();

        match byte {
            '(' => self.add_token_literal(TokenType::RightParen, None),
            ')' => self.add_token_literal(TokenType::RightParen, None),
            '{' => self.add_token_literal(TokenType::LeftBrace, None),
            '}' => self.add_token_literal(TokenType::RightBrace, None),
            ',' => self.add_token_literal(TokenType::Comma, None),
            '.' => self.add_token_literal(TokenType::Dot, None),
            '-' => self.add_token_literal(TokenType::Minus, None),
            '+' => self.add_token_literal(TokenType::Plus, None),
            ';' => self.add_token_literal(TokenType::Semicolon, None),
            '*' => self.add_token_literal(TokenType::Star, None),

            '!' => {
                if self.double_c_match('=') {
                    self.add_token_literal(TokenType::BangEq, None);
                } else {
                    self.add_token_literal(TokenType::Bang, None);
                }
            }

            '=' => {
                if self.double_c_match('=') {
                    self.add_token_literal(TokenType::EqEq, None);
                } else {
                    self.add_token_literal(TokenType::Eq, None);
                } 
            }

            '<' => {
                if self.double_c_match('=') {
                    self.add_token_literal(TokenType::LessEq, None);
                } else {
                    self.add_token_literal(TokenType::Less, None);
                }  
            }

            '>' => {
                if self.double_c_match('=') {
                    self.add_token_literal(TokenType::GreaterEq, None);
                } else {
                    self.add_token_literal(TokenType::Greater, None);
                }  
            }

            '/' => {
                    // single line
                if self.double_c_match('/') {
                    while self.peek() != '\n' && !self.source_idx_end() {
                        self.advance();
                    } 
                    // multi line comments
                } else if self.double_c_match('*'){
                    while self.peek() != '*' && self.peek_double() != '/' {
                        if self.peek() == '\n' {
                            self.line+=1
                        }
                        self.advance();
                    }
                    // division
                } else {
                    self.add_token_literal(TokenType::Slash, None)
                }
            }

            ' ' | '\r' | '\t' => (),

            '\n' => {
                self.line+=1;
            }

            '"' => {
                while self.peek() != '"' && !self.source_idx_end() {
                    if self.peek() == '\n' {
                            self.line+=1;
                    }
                    self.advance();
                }

                if self.source_idx_end() {
                    eprintln!("unterminated string");
                }

                self.advance();

                self.add_token_literal(
                    TokenType::String, 
                    Some(Literal::Str(
                        String::from_utf8(self.source[self.start + 1..self.current - 1].to_vec()).unwrap()
                    ))
                );
            }

            _ => {
                // number tokenization
                if byte.is_digit(10) {
                    while self.peek().is_digit(10) {
                        self.advance();
                    }

                    if self.peek() == '.' && self.peek_double().is_digit(10) {
                        self.advance();

                        while self.peek().is_digit(10) {
                            self.advance();
                        }
                    }

                    self.add_token_literal(
                        TokenType::Number, 
                        Some(Literal::Number(
                            String::from_utf8(self.source[self.start..self.current].to_vec()).unwrap().parse::<f64>().unwrap()
                        ))
                    );

                    // Identifier tokenization
                } else if byte.is_alphanumeric(){
                    while self.peek().is_alphanumeric() {
                        self.advance();
                    }

                    let iden = String::from_utf8(self.source[self.start..self.current].to_vec()).unwrap();

                    let ttype = match self.keywords.get(&iden) {
                        Some(keyword) => {
                            self.add_token_literal(*keyword, None)
                        },
                        None => {
                            self.add_token_literal(TokenType::Identifier, Some(Literal::Identifier(iden)))
                        },
                    };

                } else {
                    error_formatting::error_print(self.line, "Invalid character sequence in line", ""); 
                }
            },
        }

    }

    fn advance(&mut self) -> char {
        self.current+=1;
        // self.start+=1;

        char::from(self.source[self.current - 1])
    }

    fn add_token_literal(&mut self, ttype: TokenType, literal: Option<Literal>) {
        let token_bytes = self.source[self.start..self.current].to_vec();

        self.tokens.push(
            Token::new(ttype, token_bytes, literal, self.line)
        )
    }

    #[inline(always)]
    fn source_idx_end(&mut self) -> bool {
        self.current >= self.source.len()
    }

    fn double_c_match(&mut self, expected: char) -> bool {
        if self.source_idx_end() { 
            return false; 
        }
        if self.source[self.current] as char != expected { 
            return false; 
        }

        self.current+=1;
        true
    }

    fn peek(&mut self) -> char {
        if self.source_idx_end() {
            return '\0';
        } else {
            return self.source[self.current] as char;
        }
    }

    fn peek_double(&mut self) -> char {
        if self.source_idx_end() {
            return '\0';
        } else {
            return self.source[self.current+1] as char;
        }
    }
}

#[cfg(test)]
mod scantest {
    use super::*;

    #[test]
    fn test_nl() {
        let str = String::from("\n\n\n");

        let mut scanner = Lexer::new();

        scanner.scan_tokens(str);

        assert_eq!(scanner.line, 4);
    }

    #[test]
    fn test_parse() {
        let str = String::from(">=,!=");

        let mut scanner = Lexer::new();

        scanner.scan_tokens(str);

        assert_eq!(scanner.tokens[0].ttype, TokenType::GreaterEq);
        assert_eq!(scanner.tokens[1].ttype, TokenType::Comma);
        assert_eq!(scanner.tokens[2].ttype, TokenType::BangEq);
    }

    #[test]
    fn test_string() {
        let str: Vec<u8> = vec![34, 65, 83, 83, 34];
        // let str = format!("\"string\"");
        // assert_eq!(str.len(), 8)
        let mut scanner = Lexer::new();

        scanner.scan_tokens(String::from_utf8(str).unwrap());

        assert_eq!(scanner.tokens[0].ttype, TokenType::String);
    }
}