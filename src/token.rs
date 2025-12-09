#[derive(PartialEq)]
#[derive(Debug)]
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