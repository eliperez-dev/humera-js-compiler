
mod lexer;
mod token;
mod ast;
mod parser;
mod codegen;

use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::codegen::CodeGenerator;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: cargo run <input_file>");
        process::exit(1);
    }

    let filename = &args[1];
    let input = std::fs::read_to_string(filename).unwrap_or_else(|err| {
        eprintln!("Error reading file {}: {}", filename, err);
        process::exit(1);
    });

    println!("Compiling {}...", filename);

    let lexer = Lexer::new(&input); 
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    let mut codegen = CodeGenerator::new();
    let wat = codegen.generate(&program);
    
    // println!("\nGenerated WAT:\n{}", wat);
    
    std::fs::write("output.wat", wat).unwrap();
    println!("Successfully wrote output.wat");
} 