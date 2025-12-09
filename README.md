# JS Compiler

A minimal JavaScript to WebAssembly (WAT) compiler written in Rust.

This project compiles a subset of JavaScript (integers, variables, functions, `if`/`while` loops) into WebAssembly Text Format (`.wat`). It was built to demonstrate parsing techniques, AST manipulation, and WebAssembly stack machine code generation.

## Features

*   **Recursive Descent Parser**: Hand-written parser for full control over grammar and error handling.
*   **Scope Management**: Correctly handles block-scoped variables (`let`) by renaming them to unique WebAssembly locals (e.g., `$x_1`, `$x_2`).
*   **Control Flow**: Translates `while` loops and `if/else` statements into WebAssembly's structured control flow (`block`, `loop`, `br`, `br_if`).
*   **Function Hoisting**: Supports top-level function declarations and calls.
*   **Zero Dependencies**: Built using only the Rust standard library.

### Bonus Features

*   **Const Correctness**: The compiler enforces immutability for `const` variables. Reassigning a `const` variable will cause a compile-time error.
*   **Constant Folding**: Simple arithmetic operations on literals (e.g., `2 + 3 * 4`) are evaluated at compile-time, optimizing the generated WebAssembly code.
*   **Enhanced Error Reporting**: The compiler tracks line and column numbers to provide precise error messages (e.g., `Error at line 5, column 10: Expected ';'`).
*   **Integration Tests**: A comprehensive test suite (`cargo test`) verifies the compiler against various language constructs.

## Architecture

The compiler follows a standard 3-stage pipeline:

1.  **Lexer (`src/lexer.rs`)**: Converts raw source code into a stream of `SpannedToken`s. Handles whitespace skipping, multi-character operators (`==`, `<=`), comments, and tracks line/column numbers.
2.  **Parser (`src/parser.rs`)**: Consumes tokens to build an **Abstract Syntax Tree (AST)**. Uses "Precedence Climbing" to correctly handle operator precedence (e.g., `*` before `+`) and reports precise errors.
3.  **Code Generator (`src/codegen.rs`)**: Traverses the AST and emits WebAssembly Text.
    *   **Pass 1**: Scans for variable declarations to define all WASM locals at the top of the function.
    *   **Pass 2**: Emits stack machine instructions. Handles variable shadowing by maintaining a stack of symbol tables.

## Prerequisites

*   **Rust**: [Install Rust](https://www.rust-lang.org/tools/install)
*   **WABT (The WebAssembly Binary Toolkit)**: Required to verify and run the generated code. [Download WABT](https://github.com/WebAssembly/wabt)

## Usage

### 1. Build and Run the Compiler

You can run the compiler using the provided Node.js wrapper (as per requirements) or directly via Cargo.

**Option A: Node.js Wrapper**
```bash
node compiler.js programs/factorial.js > output.wat
```

**Option B: Cargo (Direct)**
```bash
cargo run programs/factorial.js
```

This will generate `output.wat` in the project root.

### 2. Verify and Run the Output

Use `wat2wasm` to convert the text format to binary, and `wasm-interp` to execute it.

```bash
# Convert WAT to WASM
./tools/include/bin/wat2wasm output.wat

# Run the WASM binary
./tools/include/bin/wasm-interp output.wasm --run-all-exports
```

## Test Programs

The compiler has been verified against the following test cases:

### 1. Factorial (Iterative)
Tests `while` loops and variable reassignment.
```bash
cargo run programs/factorial.js
# Expected Output: 120
```

### 2. GCD (Euclidean Algorithm)
Tests `while` loops, modulo arithmetic, and multiple parameters.
```bash
cargo run programs/gcd.js
# Expected Output: 6
```

### 3. Ackermann Function (Recursive)
Tests deep recursion, function calls, and stack management.
```bash
cargo run programs/ackermann.js
# Expected Output: 125
```

## Supported Language Subset

*   **Types**: 32-bit signed integers (`i32`) only.
*   **Variables**: `let` (mutable) and `const` (immutable, enforced).
*   **Control Flow**: `if`, `else`, `while`, `return`.
*   **Functions**: Declarations and calls.
*   **Operators**: `+`, `-`, `*`, `/`, `%`, `==`, `!=`, `<`, `>`, `<=`, `>=`.
