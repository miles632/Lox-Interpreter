use crate::{expr::{Expr, Stmt}, token::{self, Token, TokenType}};
use std::fmt;

use crate::lexer;

#[non_exhaustive]
enum ParseErr {
    UnexpectedToken(Token)
}

struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    fn parse() -> Result<Vec<Stmt>, ParseErr>{
        unimplemented!()
    }

    fn expression() -> Result<Expr, ParseErr> {
        // equality()
        unimplemented!()
    }

    fn equality() -> Result<Expr, ParseErr> {
        // let expr: Expr = comparison();

        // while match()

        unimplemented!()
    }

    fn matches(&mut self, token: Token) -> bool {
        if self.check(token.ttype) {
            self.advance();
        }

        return false;
    }

    fn check(&mut self, ttype: TokenType) -> bool{
        if self.idx_at_end() { 
            return false;
        }
        self.peek().ttype == ttype
    }

    fn advance(&mut self) -> &Token {
        if !self.idx_at_end() {
            self.current+=1;
        }

        self.previous()
    }

    fn idx_at_end(&mut self) -> bool {
        self.peek().ttype == TokenType::Eof
    }

    fn peek(&mut self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&mut self) -> &Token {
        &self.tokens[self.current-1]
    }
}

