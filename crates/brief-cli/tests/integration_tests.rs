mod common;
use common::run_code;

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

