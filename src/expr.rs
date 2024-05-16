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
    Unary(UnaryOp, Box<Expr>),
    Binary(BinaryOp, Box<(Expr, Expr)>),
    Grouping(Box<Expr>),
}

enum Literal {
    String(String),
    Number(f64),
    True,
    False,
    Null
}

enum UnaryOp {
    Minus, 
    Plus
}

enum BinaryOp {
    EqEq,
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

struct Source { line: usize, col: usize }
