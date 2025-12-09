
mod lexer;
mod token;
mod ast;
mod parser;
mod codegen;

use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::codegen::CodeGenerator;

fn main() {
    let input = std::fs::read_to_string("programs\\factorial.js").unwrap();
    println!("File Contents:\n{}", input);

    let lexer = Lexer::new(&input); 
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();


    let mut codegen = CodeGenerator::new();
    let wat = codegen.generate(&program);
    
    println!("\nGenerated WAT:\n{}", wat);
    
    std::fs::write("output.wat", wat).unwrap();
} 