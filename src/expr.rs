#[non_exhaustive]
pub enum Stmt {
    Expr(Expr),
    FnDecl(String , Vec<Literal>, Vec<Stmt>),
    If(Expr, Box<Stmt>, Option<Box<Stmt>>),
    Let(String, Option<Expr>),
    Scope(Vec<Stmt>),
    Return(Source, Option<Expr>),
}

// #[derive(Debug, Clone)]
#[non_exhaustive]
pub enum Expr {
    Literal(Literal),
    Unary(Unary, Box<Expr>),
    Binary(Binary, Box<(Expr, Expr)>),
    Grouping(Box<Expr>),
    Variable(Iden),
    Assignment(Iden, Box<Expr>),
    List(Vec<Box<Expr>>),
    
}

pub enum Literal {
    String(String),
    Number(f64),
    True,
    False,
    Null
}

pub enum UnaryOp {
    Minus, 
    Bang
}

pub struct Unary {
    pub line: usize,
    pub ty: UnaryOp
}

pub enum BinaryOp {
    Equal,
    UnEq, 
    Less,
    LessEq,
    Greater,
    GreaterEq,
    Plus,
    Minus,
    Multi,
    Div, 
}

pub struct Binary {
    pub line: usize,
    pub ty: BinaryOp
}


pub struct Source { line: usize, col: usize }

pub struct Iden { 
    pub line: usize, 
    // col: usize, 
    pub str: String
}