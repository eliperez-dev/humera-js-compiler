use humera_js_compiler::compile;

fn assert_contains(output: &str, pattern: &str) {
    assert!(output.contains(pattern), "Output did not contain '{}'.\nOutput:\n{}", pattern, output);
}

#[test]
fn test_variable_declaration() {
    let input = "let x = 10;";
    let output = compile(input);
    
    assert_contains(&output, "(local $x_0 i32)");
    assert_contains(&output, "i32.const 10");
    assert_contains(&output, "local.set $x_0");
}

#[test]
fn test_arithmetic() {
    let input = "let x = 1 + 2 * 3;";
    let output = compile(input);
    
    // 2 * 3 is folded to 6
    // 1 + 6 is NOT folded because our folder is shallow (codegen-time only)
    assert_contains(&output, "i32.const 1");
    assert_contains(&output, "i32.const 6");
    assert_contains(&output, "i32.add");
}

#[test]
fn test_arithmetic_variables() {
    let input = "let a = 10; let b = 20; let c = a + b;";
    let output = compile(input);
    
    assert_contains(&output, "local.get $a_0");
    assert_contains(&output, "local.get $b_1");
    assert_contains(&output, "i32.add");
}

#[test]
fn test_if_statement() {
    let input = "
        let x = 10;
        if (x > 5) {
            x = 1;
        } else {
            x = 0;
        }
    ";
    let output = compile(input);
    
    assert_contains(&output, "if");
    assert_contains(&output, "then");
    assert_contains(&output, "else");
}

#[test]
fn test_while_loop() {
    let input = "
        let i = 0;
        while (i < 10) {
            i = i + 1;
        }
    ";
    let output = compile(input);
    
    assert_contains(&output, "loop");
    assert_contains(&output, "br_if");
}

#[test]
fn test_function_declaration_and_call() {
    let input = "
        function add(a, b) {
            return a + b;
        }
        let result = add(1, 2);
    ";
    let output = compile(input);
    
    assert_contains(&output, "(func $add");
    assert_contains(&output, "(param $a i32)");
    assert_contains(&output, "(param $b i32)");
    assert_contains(&output, "call $add");
}

#[test]
#[should_panic]
fn test_const_reassignment() {
    let input = "
        const x = 10;
        x = 20;
    ";
    compile(input);
}
