use crate::ast::{Program, Statement, Expression, BinaryOp, UnaryOp};
use std::collections::HashMap;

pub struct CodeGenerator {
    output: String,
    // Stack of scopes. Each scope maps "JS name" -> ("WASM name", is_const)
    scopes: Vec<HashMap<String, (String, bool)>>, 
    local_counter: usize,
    label_counter: usize,
}

impl Default for CodeGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl CodeGenerator {
    pub fn new() -> Self {
        CodeGenerator {
            output: String::new(),
            scopes: vec![HashMap::new()], // Global scope
            local_counter: 0,
            label_counter: 0,
        }
    }

    fn enter_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn exit_scope(&mut self) {
        self.scopes.pop();
    }

    fn declare_local(&mut self, name: &str, is_const: bool) -> String {
        let wasm_name = format!("${}_{}", name, self.local_counter);
        self.local_counter += 1;
        
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name.to_string(), (wasm_name.clone(), is_const));
        }
        wasm_name
    }

    fn get_local(&self, name: &str) -> Option<(String, bool)> {
        // Search from inner-most scope to outer-most
        for scope in self.scopes.iter().rev() {
            if let Some(info) = scope.get(name) {
                return Some(info.clone());
            }
        }
        None
    }

    fn new_label(&mut self, prefix: &str) -> String {
        let label = format!("${}_{}", prefix, self.label_counter);
        self.label_counter += 1;
        label
    }

    pub fn generate(&mut self, program: &Program) -> String {
        self.output.push_str("(module\n");
        
        // 1. Generate all function declarations first (hoisting)
        for stmt in &program.body {
            if let Statement::FunctionDeclaration { name, params, body } = stmt {
                self.generate_function(name, params, body);
            }
        }

        // 2. Generate the main entry point for top-level code
        self.output.push_str("  (func $main (result i32)\n");
        
        // Pre-pass: Declare locals for top-level code
        // Note: This is a simplification. Real JS `let` is block-scoped.
        // We need to scan the top-level body for variables.
        let locals = self.collect_locals(&program.body);
        for local in locals {
            self.output.push_str(&format!("    (local {} i32)\n", local));
        }

        // Generate code for non-function statements
        let stmts: Vec<&Statement> = program.body.iter()
            .filter(|s| !matches!(s, Statement::FunctionDeclaration { .. }))
            .collect();

        if let Some((last, rest)) = stmts.split_last() {
            for stmt in rest {
                self.generate_statement(stmt);
            }
            
            // Handle the last statement specially
            match last {
                Statement::Expression(expr) => {
                    self.generate_expression(expr);
                    // Do NOT drop. This is our return value.
                }
                _ => {
                    self.generate_statement(last);
                    self.output.push_str("    i32.const 0\n"); // Default return
                }
            }
        } else {
            self.output.push_str("    i32.const 0\n"); // Empty program
        }

        self.output.push_str("  )\n");
        self.output.push_str("  (export \"_start\" (func $main))\n");
        self.output.push_str(")\n");
        
        self.output.clone()
    }

    fn generate_function(&mut self, name: &str, params: &[String], body: &[Statement]) {
        self.output.push_str(&format!("  (func ${} ", name));
        
        self.enter_scope();
        
        // Params
        for param in params {
            let wasm_name = format!("${}", param); // Params don't need unique suffix usually, but let's be safe? 
            // Actually, params are locals too. Let's just use the name directly for params to keep it simple,
            // assuming no collision with keywords.
            self.output.push_str(&format!("(param {} i32) ", wasm_name));
            
            // Add to scope
            if let Some(scope) = self.scopes.last_mut() {
                scope.insert(param.clone(), (wasm_name, false)); // Params are mutable
            }
        }
        self.output.push_str("(result i32)\n");

        // Locals
        let locals = self.collect_locals(body);
        for local in locals {
            self.output.push_str(&format!("    (local {} i32)\n", local));
        }

        // Body
        for stmt in body {
            self.generate_statement(stmt);
        }

        // Default return 0
        self.output.push_str("    i32.const 0\n");
        self.output.push_str("  )\n");
        
        self.exit_scope();
    }

    // Helper to find all `let` declarations in a block (recursively) and assign them unique WASM names
    fn collect_locals(&mut self, stmts: &[Statement]) -> Vec<String> {
        let mut locals = Vec::new();
        for stmt in stmts {
            match stmt {
                Statement::VariableDeclaration { name, is_const, .. } => {
                    let wasm_name = self.declare_local(name, *is_const);
                    locals.push(wasm_name);
                }
                Statement::Block(inner) => {
                    locals.extend(self.collect_locals(inner));
                }
                Statement::If { then_branch, else_branch, .. } => {
                    // We need to peek inside blocks
                    if let Statement::Block(b) = &**then_branch {
                        locals.extend(self.collect_locals(b));
                    }
                    if let Some(else_b) = else_branch
                        && let Statement::Block(b) = &**else_b {
                            locals.extend(self.collect_locals(b));
                        }
                }
                Statement::While { body, .. } => {
                    if let Statement::Block(b) = &**body {
                        locals.extend(self.collect_locals(b));
                    }
                }
                _ => {}
            }
        }
        locals
    }

    fn generate_statement(&mut self, stmt: &Statement) {
        match stmt {
            Statement::VariableDeclaration { name, init, .. } => {
                self.generate_expression(init);
                let (wasm_name, _) = self.get_local(name).expect("Local not found (should be declared in pre-pass)");
                self.output.push_str(&format!("    local.set {}\n", wasm_name));
            }
            Statement::Expression(expr) => {
                self.generate_expression(expr);
                // If expression returns a value, drop it (unless it's the last one, but for now drop to keep stack clean)
                self.output.push_str("    drop\n"); 
            }
            Statement::Return(expr) => {
                if let Some(e) = expr {
                    self.generate_expression(e);
                } else {
                    self.output.push_str("    i32.const 0\n");
                }
                self.output.push_str("    return\n");
            }
            Statement::Block(stmts) => {
                // WASM blocks don't create scope automatically for locals (we handled that with renaming),
                // but they are useful for control flow.
                for s in stmts {
                    self.generate_statement(s);
                }
            }
            Statement::If { condition, then_branch, else_branch } => {
                self.generate_expression(condition);
                self.output.push_str("    (if\n");
                self.output.push_str("      (then\n");
                self.generate_statement(then_branch);
                self.output.push_str("      )\n");
                if let Some(else_b) = else_branch {
                    self.output.push_str("      (else\n");
                    self.generate_statement(else_b);
                    self.output.push_str("      )\n");
                }
                self.output.push_str("    )\n");
            }
            Statement::While { condition, body } => {
                let block_label = self.new_label("break");
                let loop_label = self.new_label("continue");
                
                self.output.push_str(&format!("    (block {}\n", block_label));
                self.output.push_str(&format!("      (loop {}\n", loop_label));
                
                // Condition
                self.generate_expression(condition);
                self.output.push_str("        i32.eqz\n"); // Invert condition for br_if
                self.output.push_str(&format!("        br_if {}\n", block_label));
                
                // Body
                self.generate_statement(body);
                
                // Jump back
                self.output.push_str(&format!("        br {}\n", loop_label));
                
                self.output.push_str("      )\n"); // End loop
                self.output.push_str("    )\n"); // End block
            }
            _ => {}
        }
    }

    fn generate_expression(&mut self, expr: &Expression) {
        match expr {
            Expression::Number(n) => {
                self.output.push_str(&format!("    i32.const {}\n", n));
            }
            Expression::Identifier(name) => {
                let (wasm_name, _) = self.get_local(name).unwrap_or_else(|| panic!("Undefined variable: {}", name));
                self.output.push_str(&format!("    local.get {}\n", wasm_name));
            }
            Expression::Binary(left, op, right) => {
                // Constant Folding Optimization
                if let (Expression::Number(l), Expression::Number(r)) = (&**left, &**right) {
                    let result = match op {
                        BinaryOp::Add => l.wrapping_add(*r),
                        BinaryOp::Sub => l.wrapping_sub(*r),
                        BinaryOp::Mul => l.wrapping_mul(*r),
                        BinaryOp::Div => if *r != 0 { l.wrapping_div(*r) } else { 0 }, // Avoid panic
                        BinaryOp::Mod => if *r != 0 { l.wrapping_rem(*r) } else { 0 },
                        BinaryOp::Eq => if l == r { 1 } else { 0 },
                        BinaryOp::Ne => if l != r { 1 } else { 0 },
                        BinaryOp::Lt => if l < r { 1 } else { 0 },
                        BinaryOp::Gt => if l > r { 1 } else { 0 },
                        BinaryOp::Le => if l <= r { 1 } else { 0 },
                        BinaryOp::Ge => if l >= r { 1 } else { 0 },
                    };
                    self.output.push_str(&format!("    i32.const {}\n", result));
                    return;
                }

                self.generate_expression(left);
                self.generate_expression(right);
                match op {
                    BinaryOp::Add => self.output.push_str("    i32.add\n"),
                    BinaryOp::Sub => self.output.push_str("    i32.sub\n"),
                    BinaryOp::Mul => self.output.push_str("    i32.mul\n"),
                    BinaryOp::Div => self.output.push_str("    i32.div_s\n"), // Signed division
                    BinaryOp::Mod => self.output.push_str("    i32.rem_s\n"),
                    BinaryOp::Eq => self.output.push_str("    i32.eq\n"),
                    BinaryOp::Ne => self.output.push_str("    i32.ne\n"),
                    BinaryOp::Lt => self.output.push_str("    i32.lt_s\n"),
                    BinaryOp::Gt => self.output.push_str("    i32.gt_s\n"),
                    BinaryOp::Le => self.output.push_str("    i32.le_s\n"),
                    BinaryOp::Ge => self.output.push_str("    i32.ge_s\n"),
                }
            }
            Expression::Assignment(name, value) => {
                self.generate_expression(value);
                let (wasm_name, is_const) = self.get_local(name).unwrap_or_else(|| panic!("Undefined variable: {}", name));
                
                if is_const {
                    panic!("Assignment to constant variable '{}'", name);
                }

                self.output.push_str("    local.tee "); // tee sets the local AND leaves value on stack
                self.output.push_str(&wasm_name);
                self.output.push('\n');
            }
            Expression::Call(name, args) => {
                for arg in args {
                    self.generate_expression(arg);
                }
                self.output.push_str(&format!("    call ${}\n", name));
            }
            Expression::Unary(op, right) => {
                match op {
                    UnaryOp::Not => {
                        self.generate_expression(right);
                        self.output.push_str("    i32.eqz\n"); // 0 -> 1, non-zero -> 0
                    }
                    UnaryOp::Neg => {
                        self.output.push_str("    i32.const 0\n");
                        self.generate_expression(right);
                        self.output.push_str("    i32.sub\n"); // 0 - x
                    }
                }
            }
        }
    }
}
