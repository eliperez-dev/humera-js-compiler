
pub enum Token {

    // Key words
    Let, Const, If, Else, While,
    Function, Return,

    // Delimiters
    LParen, RParen,   // ( )
    LBrace, RBrace,   // { }
    Comma, Semi,      // , ;

    // Operators
    Plus, Minus, Star, Slash, Percent, //  + - * / %
    Eq, EqEQ, Bang, BangEq,            //  = == ! !=
    Lt, LtEq, Gt, GtEq,                //  < <= > >=

    // EOF
    EOF
}



fn main() {
    let contents = std::fs::read_to_string("example.txt").unwrap();
    println!("File Contents:\n{}", contents);
}