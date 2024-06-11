use crate::expr::{self, Iden};
use crate::expr::{
    Expr, Literal, Stmt, UnaryOp, BinaryOp, Unary, Binary
};

use crate::lexer;
use crate::token::{
    Token, TokenType
};

type TType = TokenType;

#[non_exhaustive]
enum ParseErr {
    UnexpectedToken(Token),
    ExpectedExpr{
        ty: TType,
        line: usize,
    },
    InvalidTokenBinExpr {
        ty: TType, 
        line: usize,
    },
    TokenMismatch {
        expected: TType,
        found: TType,
        message: Option<String>,
    },
    // MaxParamsExceeded {
    //     line: usize,
    // }
}

struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens: tokens,
            current: 0,
        }
    } 

    fn parse(&mut self) -> Result<Vec<Stmt>, ParseErr>{
        let mut stmts = vec![];

        while !self.idx_at_end() {
            let stmt = self.declaration()?;
            stmts.push(stmt);
        }
    }

    fn declaration(&mut self) -> Result<Stmt, ParseErr> {
        if self.matches_o(TType::Fun)  {
            return self.
        }

        if self.matches_o(TType::Fun) {

        }
    }

    fn statement(&mut self) -> Result<Stmt, ParseErr> {
        if self.matches_o(TType::If) { return Ok(self.if_stmt()); }
        if self.matches_o(TType::For) { return Ok(self.for_stmt()); }
        if self.matches_o(TType::While) { return Ok(self.while_stmt()); }
        if self.matches_o(TType::Return) { return Ok(self.return_stmt()); }
        if self.matches_o(TType::Print) { return Ok(self.print()); }
    }

    fn var(&mut self) -> Result<Stmt, ParseErr> {
        let iden_token = *self.consume(TType::Identifier, "function contains no identifier")?;
        let fn_iden = expr::Iden {
            line: iden_token.line,
            str: String::from_utf8(iden_token.lexeme).unwrap(),
        };

        let 
    }

    fn fun(&mut self) -> Result<Stmt, ParseErr> {

    }

    fn args(&mut self) -> Result<Vec<Iden>, ParseErr> {
        let mut args = vec![];

        self.consume(TType::LeftParen, "left parenthesis for arguments missing in function signature")?;
        if !self.check(TType::RightParen) {
            loop {
                let arg = self.consume(TType::Identifier, "expected identifier")?; 

                let arg_iden = expr::Iden {
                    line: arg.line,
                    str: String::from_utf8(arg.lexeme).unwrap(),
                };

                args.push(arg_iden);
                
                if !self.matches_o(TType::Comma) { break }
            }
        }

        assert!(self.check(TType::RightParen));
        self.consume(TType::RightParen, "expected ( after arguments")?;

        Ok(args)
    }

    fn fun_body(&mut self) -> Result<Vec<Stmt>, ParseErr> {
        self.consume(TType::LeftBrace, "function has no { before function body");

        let mut stmts = vec![];

        while !self.check(TType::RightBrace) && !self.idx_at_end(){
            stmts.push(self.declaration()?);
        }

        self.consume(TType::RightBrace, "function body isn't terminated with {")

        Ok(stmts)
    }


    //****STATEMENTS */
    fn if_stmt(&mut self) -> Result<Stmt, ParseErr> {
        unimplemented!()
    }

    fn for_stmt(&mut self) -> Result<Stmt, ParseErr> {
        unimplemented!()
    }

    fn while_stmt(&mut self) -> Result<Stmt, ParseErr> {
        unimplemented!()
    }

    fn return_stmt(&mut self) -> Result<Stmt, ParseErr> { 
        unimplemented!()
    }

    fn print_stmt(&mut self) -> Result<Stmt,ParseErr> {
        unimplemented!()
    }
    //**** */

    fn expression(&mut self) -> Result<Expr, ParseErr> {
        // equality()
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, ParseErr> {
        let mut expr = self.comparison()?;

        while self.matches(&[TType::BangEq, TType::EqEq]) {
            let op = Parser::token_to_binary(self.previous().clone())?;
            let rightexpr = self.comparison()?;
            expr = Expr::Binary(op, Box::new((expr ,rightexpr))); 
        }

        Ok(expr)
    }

        fn comparison(&mut self) -> Result<Expr, ParseErr> {
        let mut expr = self.term()?;

        while self.matches(&[TType::Greater, TType::GreaterEq, TType::Less, TType::LessEq]) {
            let op = Parser::token_to_binary(self.previous().clone())?;
            let rightexpr = self.comparison()?;
            expr = Expr::Binary(op, Box::new((expr, rightexpr)));
        }
        
        Ok(expr) 
    }

    fn term(&mut self) -> Result<Expr, ParseErr> {
        let mut expr = self.factor()?;

        while self.matches(&[TType::Minus, TType::Plus]) {
            let op = Parser::token_to_binary(self.previous().clone())?;
            let rightexpr = self.factor()?;
            expr = Expr::Binary(op, Box::new((expr, rightexpr)));
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, ParseErr> {
        let mut expr = self.unary()?;

        while self.matches(&[TType::Slash, TType::Star]) {
            let op = Parser::token_to_binary(self.previous().clone())?;
            let rightexpr = self.unary()?;

            expr = Expr::Binary(op, Box::new((expr, rightexpr))); 
        }

        Ok(expr)
    }



    fn unary(&mut self) -> Result<Expr, ParseErr> {
        if self.matches(&[TType::Bang, TType::Minus]) {
            let op = Parser::token_to_unary(self.previous().clone())?;
            let rightexpr = Box::new(self.unary()?);

            return Ok(Expr::Unary(op, rightexpr));
        }

        // return primary();
        // return Err(ParseErr::UnexpectedToken());
        self.primary()
    }

    /// `primary` → `NUMBER` | `STRING` | `"true"` | `"false"`` | `"nil"``
    /// | `"(" expression ")"` ;
    fn primary(&mut self) -> Result<Expr, ParseErr> {
        use crate::token::Literal;
        // use crate::expr::Literal;
        let line = (*self.peek()).line;

        if self.matches_o(TType::False) {
            return Ok(Expr::Literal(expr::Literal::False))
        }
        if self.matches_o(TType::True) {
            return Ok(Expr::Literal(expr::Literal::True))
        }
        if self.matches_o(TType::Null) {
            return Ok(Expr::Literal(expr::Literal::Null))
        }
        
        if self.matches_o(TType::Number) {
            match &self.previous().literal {
                Some(Literal::Number(n)) => {
                    return Ok(Expr::Literal(expr::Literal::Number(*n)));
                },
                // Some(els)  => panic!("crap"),
                // None => panic!("crap"),
                _ => panic!("crap")
            }
        }

        if self.matches_o(TType::String) {
            match &self.previous().literal {
                Some(Literal::Str(s)) => {
                    return Ok(Expr::Literal(expr::Literal::String(s.to_string())));
                },
                // Some(els)  => panic!("crap"),
                // None => panic!("crap"),
                _ => panic!("crap")
            }
        }

        if self.matches_o(TType::Identifier) {
            match &self.previous().literal {
                Some(Literal::Identifier(i)) => {
                    return Ok(
                        Expr::Variable(Iden{
                            line: line,
                            str: i.clone(), // probably performance hit here
                            // col: self.previous().,
                        })
                    )
                },
                None => (),
                _ => (),
            }
        }

        if self.matches_o(TType::LeftParen) {
            let expr = self.expression()?;
            self.consume(TokenType::RightParen, "expected ) after expression")?;
            return Ok(Expr::Grouping(Box::new(expr)));
        }

        Err(ParseErr::ExpectedExpr { ty: self.peek().ttype, line: line })
    }

    fn token_to_unary(token: Token) -> Result<Unary, ParseErr> {
        let line = token.line;
        match token.ttype {
            TType::Minus => {
                Ok(Unary {
                    line: line,
                    ty: UnaryOp::Minus
                })
            },

            TType::Bang => {
                Ok(Unary {
                    line: line,
                    ty: UnaryOp::Bang
                })
            }

            _ => {
                Err(ParseErr::UnexpectedToken(token))
            }
        }
    }

    fn token_to_binary(token: Token) -> Result<Binary, ParseErr> {
        match token.ttype {
            TokenType::Minus => { Ok( Binary {
                line: token.line,
                ty: BinaryOp::Minus
            })},
            TokenType::Plus => { Ok( Binary {
                line: token.line,
                ty: BinaryOp::Plus
            })},
            TokenType::Slash => { Ok( Binary {
                line: token.line,
                ty: BinaryOp::Div
            })},
            TokenType::Star => { Ok( Binary {
                line: token.line,
                ty: BinaryOp::Multi
            })},
            TokenType::BangEq => { Ok( Binary {
                line: token.line,
                ty: BinaryOp::UnEq
            })},
            TokenType::EqEq => { Ok( Binary {
                line: token.line,
                ty: BinaryOp::Equal
            })},
            TokenType::Greater => { Ok( Binary {
                line: token.line,
                ty: BinaryOp::Greater
            })},
            TokenType::GreaterEq => { Ok( Binary {
                line: token.line,
                ty: BinaryOp::GreaterEq
            })},
            TokenType::Less => { Ok( Binary {
                line: token.line,
                ty: BinaryOp::Less
            })},
            TokenType::LessEq => { Ok( Binary {
                line: token.line,
                ty: BinaryOp::LessEq
            })},
            _ => {
                Err(ParseErr::InvalidTokenBinExpr { ty: token.ttype, line: token.line }) 
            },
        }
    }

    /// does the current token match with any of the given ones 
    fn matches(&mut self, ttypes: &[TType]) -> bool {
        for ttype in ttypes.into_iter() {
            if self.check(*ttype) {
                self.advance();
                return true;
            }
        }

        return false;
    }

    /// same shit as matches but only on one argument
    fn matches_o(&mut self, ttype: TType) -> bool {
        if self.check(ttype) {
            self.advance();
            return true;
        }

        return false;
    }

    fn check(&mut self, ttype: TokenType) -> bool{
        if self.idx_at_end() { 
            return false;
        }
        self.peek().ttype == ttype
    }

    /// increment current index and return token at pre incrementation index
    fn advance(&mut self) -> &Token {
        if !self.idx_at_end() {
            self.current+=1;
        }

        self.previous()
    }

    fn idx_at_end(&mut self) -> bool {
        self.peek().ttype == TokenType::Eof
    }

    /// peek into current index
    fn peek(&mut self) -> &Token {
        &self.tokens[self.current]
    }

    /// peek into previous index
    fn previous(&mut self) -> &Token {
        &self.tokens[self.current-1]
    }

    /// does the current index contain the type? if no return a token mismatch error
    fn consume(&mut self, ttype: TType, err_str: &str) -> Result<&Token, ParseErr> {
        if self.check(ttype) { return Ok(self.advance()); }
        Err(ParseErr::TokenMismatch { expected: ttype, found: self.peek().ttype, message: Some(err_str.into()) })
    }
}

