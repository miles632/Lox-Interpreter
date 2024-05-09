#![allow(unused_imports, unused)]
use std::fs::{self, File};
use std::io;
use std::error::Error;

use crate::error_formatting;

use crate::token::{self, Literal};
use token::Token;

use token::TokenType;

#[derive(Clone)]
pub struct Scanner {
    source: Vec<u8>,
    pub tokens: Vec<Token>,

    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new() -> Self {
        Scanner {
            source: vec![],
            tokens: vec![],

            start: 0,
            current: 0,
            line: 1,
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
                    self.add_token_literal(TokenType::BangEqual, None);
                } else {
                    self.add_token_literal(TokenType::Bang, None);
                }
            }

            '=' => {
                if self.double_c_match('=') {
                    self.add_token_literal(TokenType::EqualEqual, None);
                } else {
                    self.add_token_literal(TokenType::Equal, None);
                } 
            }

            '<' => {
                if self.double_c_match('=') {
                    self.add_token_literal(TokenType::LessEqual, None);
                } else {
                    self.add_token_literal(TokenType::Less, None);
                }  
            }

            '>' => {
                if self.double_c_match('=') {
                    self.add_token_literal(TokenType::GreaterEqual, None);
                } else {
                    self.add_token_literal(TokenType::Greater, None);
                }  
            }

            '/' => {
                if self.double_c_match('/') {
                    while self.peek() != '\n' && !self.source_idx_end() {
                        self.advance();
                    } 
                } else {
                    self.add_token_literal(TokenType::Slash, None)
                }
            }

            ' ' | '\r' | '\t' => (),

            '\n' => {
                self.line+=1;
            }

            '"' => {
                while !self.source_idx_end() && self.peek() != '"' {
                    if self.peek() == '\n' {
                            self.line+=1;
                        }
                        self.advance();
                }

                if self.source_idx_end() {
                    todo!()
                }

                self.add_token_literal(
                    TokenType::String, 
                    Some(Literal::Str(
                        String::from_utf8(self.source[self.start + 1..self.current - 1].to_vec()).unwrap()
                    ))
                );
            }

            _ => {
                error_formatting::error_print(self.line, "Invalid single character sequence in line", "");
            },
        }

    }

    fn advance(&mut self) -> char {
        self.current+=1;
        // self.start+=1;

        self.source[self.current - 1] as char
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
}

#[cfg(test)]
mod scantest {
    use super::*;

    #[test]
    fn test_nl() {
        let str = String::from("\n\n\n");

        let mut scanner = Scanner::new();

        scanner.scan_tokens(str);

        assert_eq!(scanner.line, 4);
    }

    #[test]
    fn test_parse() {
        let str = String::from(">=,!=");

        let mut scanner = Scanner::new();

        scanner.scan_tokens(str);

        assert_eq!(scanner.tokens[0].ttype, TokenType::GreaterEqual);
        assert_eq!(scanner.tokens[1].ttype, TokenType::Comma);
        assert_eq!(scanner.tokens[2].ttype, TokenType::BangEqual);
    }
}