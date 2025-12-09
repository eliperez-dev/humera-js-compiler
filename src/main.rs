use std::env;
use std::process;
use humera_js_compiler::compile;

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

    let wat = compile(&input);
    
    std::fs::write("output.wat", wat).unwrap();
    println!("Successfully wrote output.wat");
}
