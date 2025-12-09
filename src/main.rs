
mod lexer;
mod token;
mod ast;
mod parser;

use crate::lexer::Lexer;
use crate::parser::Parser;

fn main() {
    let input = std::fs::read_to_string("programs\\factorial.js").unwrap();
    println!("File Contents:\n{}", input);

    let lexer = Lexer::new(&input); 
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();
    
    println!("AST:\n{:#?}", program);
} 