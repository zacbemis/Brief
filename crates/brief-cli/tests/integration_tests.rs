use brief_lexer::lex;
use brief_parser::parse;
use brief_hir::{lower, emit_bytecode};
use brief_vm::VM;
use brief_runtime::Runtime;
use brief_diagnostic::FileId;
use std::rc::Rc;

/// Helper to run code through the full pipeline
fn run_code(source: &str) -> Result<brief_vm::Value, String> {
    let file_id = FileId(0);
    
    // 1. Lex
    let (tokens, lex_errors) = lex(source, file_id);
    if !lex_errors.is_empty() {
        return Err(format!("Lex errors: {:?}", lex_errors));
    }
    
    // 2. Parse
    let (program, parse_errors) = parse(tokens, file_id);
    if !parse_errors.is_empty() {
        return Err(format!("Parse errors: {:?}", parse_errors));
    }
    
    // 3. Lower to HIR
    let hir_program = match lower(program) {
        Ok(hir) => hir,
        Err(errors) => {
            return Err(format!("HIR errors: {:?}", errors));
        }
    };
    
    // 4. Emit bytecode
    let chunks = emit_bytecode(&hir_program);
    if std::env::var("BRIEF_DEBUG_CHUNK").is_ok() {
        for (idx, chunk) in chunks.iter().enumerate() {
            eprintln!("Emitted chunk #{} - {}", idx, chunk.name);
            for (ip, instr) in chunk.code.iter().enumerate() {
                eprintln!("  {:04}: {}", ip, instr);
            }
        }
    }
    if chunks.is_empty() {
        return Ok(brief_vm::Value::Null);
    }
    
    // 5. Create VM with runtime
    let mut vm = VM::new();
    let runtime = Runtime::new();
    vm.set_runtime(Box::new(runtime));
    
    // 6. Execute
    let main_chunk = Rc::new(chunks[0].clone());
    vm.push_frame(main_chunk, 0);
    
    // 7. Run
    match vm.run() {
        Ok(value) => Ok(value),
        Err(e) => {
            eprintln!("Runtime error: {:?}", e);
        Err(format!("Runtime error: {:?} | chunks: {:?}", e, chunks))
        }
    }
}

#[test]
fn test_simple_arithmetic() {
    let source = "def test()\n\t5 + 3\n";
    let result = run_code(source);
    if let Err(e) = &result {
        eprintln!("Error: {}", e);
    }
    assert!(result.is_ok(), "Result should be OK, got: {:?}", result);
    if let Ok(brief_vm::Value::Int(n)) = result {
        assert_eq!(n, 8);
    } else {
        panic!("Expected Int(8), got {:?}", result);
    }
}

#[test]
fn test_variable_assignment() {
    let source = "def test()\n\tx := 10\n\tx + 5\n";
    let result = run_code(source);
    assert!(result.is_ok(), "Result should be OK, got: {:?}", result);
    if let Ok(brief_vm::Value::Int(n)) = result {
        assert_eq!(n, 15);
    } else {
        panic!("Expected Int(15), got {:?}", result);
    }
}

#[test]
fn test_multiple_operations() {
    let source = "def test()\n\t(5 * 3) + 2\n";
    let result = run_code(source);
    assert!(result.is_ok());
    if let Ok(brief_vm::Value::Int(n)) = result {
        assert_eq!(n, 17); // 5 * 3 + 2 = 17
    } else {
        panic!("Expected Int(17), got {:?}", result);
    }
}

#[test]
fn test_comparison_operators() {
    let source = "def test()\n\t5 == 5\n";
    let result = run_code(source);
    assert!(result.is_ok());
    if let Ok(brief_vm::Value::Bool(b)) = result {
        assert!(b);
    } else {
        panic!("Expected Bool(true), got {:?}", result);
    }
}

#[test]
fn test_if_statement() {
    let source = "def test()\n\tif (5 > 3)\n\t\t10\n\telse\n\t\t20\n";
    let result = run_code(source);
    assert!(result.is_ok());
    if let Ok(brief_vm::Value::Int(n)) = result {
        assert_eq!(n, 10);
    } else {
        panic!("Expected Int(10), got {:?}", result);
    }
}

#[test]
fn test_while_loop() {
    let source = "def test()\n\tx := 0\n\twhile (x < 3)\n\t\tx := x + 1\n\tx\n";
    let result = run_code(source);
    if let Err(e) = &result {
        eprintln!("Error: {}", e);
    }
    assert!(result.is_ok());
    if let Ok(brief_vm::Value::Int(n)) = result {
        assert_eq!(n, 3);
    } else {
        panic!("Expected Int(3), got {:?}", result);
    }
}

#[test]
fn test_builtin_print() {
    // Note: Can't easily test stdout, but we can test it doesn't error
    let source = "def test()\n\tprint(\"Hello\")\n";
    let result = run_code(source);
    assert!(result.is_ok());
    // Print returns null
    if let Ok(brief_vm::Value::Null) = result {
        // Expected
    } else {
        panic!("Expected Null, got {:?}", result);
    }
}

#[test]
fn test_builtin_int_cast() {
    let source = "def test()\n\tint(3.14)\n";
    let result = run_code(source);
    if let Err(e) = &result {
        eprintln!("Error: {}", e);
    }
    assert!(result.is_ok());
    if let Ok(brief_vm::Value::Int(n)) = result {
        assert_eq!(n, 3);
    } else {
        panic!("Expected Int(3), got {:?}", result);
    }
}

#[test]
fn test_builtin_str_cast() {
    let source = "def test()\n\tstr(42)\n";
    let result = run_code(source);
    if let Err(e) = &result {
        eprintln!("Error: {}", e);
    }
    assert!(result.is_ok());
    if let Ok(brief_vm::Value::Str(s)) = result {
        assert_eq!(s, "42");
    } else {
        panic!("Expected Str(\"42\"), got {:?}", result);
    }
}

#[test]
fn test_string_concatenation() {
    let source = "def test()\n\t\"Hello\" + \" \" + \"World\"\n";
    let result = run_code(source);
    assert!(result.is_ok());
    if let Ok(brief_vm::Value::Str(s)) = result {
        assert_eq!(s, "Hello World");
    } else {
        panic!("Expected Str(\"Hello World\"), got {:?}", result);
    }
}

#[test]
fn test_nested_expressions() {
    let source = "def test()\n\t(5 + 3) * 2\n";
    let result = run_code(source);
    assert!(result.is_ok());
    if let Ok(brief_vm::Value::Int(n)) = result {
        assert_eq!(n, 16);
    } else {
        panic!("Expected Int(16), got {:?}", result);
    }
}

#[test]
fn test_boolean_operations() {
    let source = "def test()\n\ttrue && false\n";
    let result = run_code(source);
    assert!(result.is_ok());
    if let Ok(brief_vm::Value::Bool(b)) = result {
        assert!(!b);
    } else {
        panic!("Expected Bool(false), got {:?}", result);
    }
}

#[test]
fn test_unary_negation() {
    let source = "def test()\n\t-5\n";
    let result = run_code(source);
    assert!(result.is_ok());
    if let Ok(brief_vm::Value::Int(n)) = result {
        assert_eq!(n, -5);
    } else {
        panic!("Expected Int(-5), got {:?}", result);
    }
}

#[test]
fn test_division() {
    let source = "def test()\n\t10 / 2\n";
    let result = run_code(source);
    assert!(result.is_ok());
    if let Ok(brief_vm::Value::Double(d)) = result {
        assert!((d - 5.0).abs() < f64::EPSILON);
    } else {
        panic!("Expected Double(5.0), got {:?}", result);
    }
}

#[test]
fn test_modulo() {
    let source = "def test()\n\t10 % 3\n";
    let result = run_code(source);
    assert!(result.is_ok());
    if let Ok(brief_vm::Value::Int(n)) = result {
        assert_eq!(n, 1);
    } else {
        panic!("Expected Int(1), got {:?}", result);
    }
}

#[test]
fn test_power() {
    let source = "def test()\n\t2 ** 3\n";
    let result = run_code(source);
    assert!(result.is_ok());
    if let Ok(brief_vm::Value::Double(d)) = result {
        assert!((d - 8.0).abs() < f64::EPSILON);
    } else {
        panic!("Expected Double(8.0), got {:?}", result);
    }
}

#[test]
fn test_for_loop() {
    let source = "def test()\n\tx := 0\n\tfor (i := 0; i < 3; i := i + 1)\n\t\tx := x + 1\n\tx\n";
    let result = run_code(source);
    if let Err(e) = &result {
        eprintln!("Error: {}", e);
    }
    assert!(result.is_ok());
    if let Ok(brief_vm::Value::Int(n)) = result {
        assert_eq!(n, 3);
    } else {
        panic!("Expected Int(3), got {:?}", result);
    }
}

#[test]
fn test_function_with_parameters() {
    let source = "def add(x, y)\n\tx + y\n\ndef test()\n\tadd(5, 3)\n";
    let result = run_code(source);
    // Function calls aren't implemented yet, so we should see an error rather than hang forever
    assert!(
        result.is_err(),
        "Function calls not yet supported; expected error but got {:?}",
        result
    );
}

#[test]
fn test_complex_expression() {
    let source = "def test()\n\t((10 + 5) * 2) - 3\n";
    let result = run_code(source);
    assert!(result.is_ok());
    if let Ok(brief_vm::Value::Int(n)) = result {
        assert_eq!(n, 27); // (10 + 5) * 2 - 3 = 27
    } else {
        panic!("Expected Int(27), got {:?}", result);
    }
}

