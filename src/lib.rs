pub mod lexer;
pub mod token;
pub mod ast;
pub mod parser;
pub mod codegen;

use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::codegen::CodeGenerator;

pub fn compile(input: &str) -> String {
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    let mut codegen = CodeGenerator::new();
    codegen.generate(&program)
}
