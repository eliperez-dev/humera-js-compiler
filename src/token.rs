#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Span {
    pub line: usize,
    pub column: usize,
}

impl Span {
    pub fn new(line: usize, column: usize) -> Self {
        Span { line, column }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SpannedToken {
    pub token: Token,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(i32),
    Identifier(String),

    // Key words
    Let, Const, If, Else, While,
    Function, Return,

    // Delimiters
    LParen, RParen,   // ( )
    LBrace, RBrace,   // { }
    Comma, Semi,      // , ;

    // Operators
    Plus, Minus, Star, Slash, Percent, //  + - * / %
    Eq, EqEq, Bang, BangEq,            //  = == ! !=
    Lt, LtEq, Gt, GtEq,                //  < <= > >=

    // EOF
    EOF
}
