use brief_runtime::*;
use brief_vm::{Value, RuntimeError, BuiltinRuntime};

// Edge case tests

#[test]
fn test_str_cast_from_string_optimization() {
    // Test that str_cast doesn't unnecessarily convert strings
    let original = "hello".to_string();
    let args = vec![Value::Str(original.clone())];
    let result = str_cast(&args);
    assert!(result.is_ok());
    if let Ok(Value::Str(s)) = result {
        assert_eq!(s, "hello");
        // Note: We can't test that it's the same allocation, but we can test correctness
    } else {
        panic!("Expected Str(\"hello\"), got {:?}", result);
    }
}

#[test]
fn test_rt_concat2_with_strings() {
    // Test that rt_concat2 optimizes when both args are strings
    let args = vec![
        Value::Str("a".to_string()),
        Value::Str("b".to_string()),
    ];
    let result = rt_concat2(&args);
    assert!(result.is_ok());
    if let Ok(Value::Str(s)) = result {
        assert_eq!(s, "ab");
    } else {
        panic!("Expected Str(\"ab\"), got {:?}", result);
    }
}

#[test]
fn test_rt_concat2_mixed_types() {
    // Test concatenation with non-string types
    let args = vec![
        Value::Str("Value: ".to_string()),
        Value::Int(42),
    ];
    let result = rt_concat2(&args);
    assert!(result.is_ok());
    if let Ok(Value::Str(s)) = result {
        assert_eq!(s, "Value: 42");
    } else {
        panic!("Expected Str(\"Value: 42\"), got {:?}", result);
    }
}

#[test]
fn test_len_empty_string() {
    let args = vec![Value::Str("".to_string())];
    let result = len(&args);
    assert!(result.is_ok());
    if let Ok(Value::Int(n)) = result {
        assert_eq!(n, 0);
    } else {
        panic!("Expected Int(0), got {:?}", result);
    }
}

#[test]
fn test_len_long_string() {
    let long_string = "a".repeat(1000);
    let args = vec![Value::Str(long_string.clone())];
    let result = len(&args);
    assert!(result.is_ok());
    if let Ok(Value::Int(n)) = result {
        assert_eq!(n, 1000);
    } else {
        panic!("Expected Int(1000), got {:?}", result);
    }
}

#[test]
fn test_int_cast_zero() {
    let args = vec![Value::Int(0)];
    let result = int_cast(&args);
    assert!(result.is_ok());
    if let Ok(Value::Int(n)) = result {
        assert_eq!(n, 0);
    } else {
        panic!("Expected Int(0), got {:?}", result);
    }
}

#[test]
fn test_int_cast_negative() {
    let args = vec![Value::Int(-42)];
    let result = int_cast(&args);
    assert!(result.is_ok());
    if let Ok(Value::Int(n)) = result {
        assert_eq!(n, -42);
    } else {
        panic!("Expected Int(-42), got {:?}", result);
    }
}

#[test]
fn test_dub_cast_negative() {
    let args = vec![Value::Double(-3.14)];
    let result = dub_cast(&args);
    assert!(result.is_ok());
    if let Ok(Value::Double(d)) = result {
        assert!((d - (-3.14)).abs() < f64::EPSILON);
    } else {
        panic!("Expected Double(-3.14), got {:?}", result);
    }
}

#[test]
fn test_dub_cast_zero() {
    let args = vec![Value::Double(0.0)];
    let result = dub_cast(&args);
    assert!(result.is_ok());
    if let Ok(Value::Double(d)) = result {
        assert!((d - 0.0).abs() < f64::EPSILON);
    } else {
        panic!("Expected Double(0.0), got {:?}", result);
    }
}

#[test]
fn test_str_cast_null() {
    let args = vec![Value::Null];
    let result = str_cast(&args);
    assert!(result.is_ok());
    if let Ok(Value::Str(s)) = result {
        assert_eq!(s, "null");
    } else {
        panic!("Expected Str(\"null\"), got {:?}", result);
    }
}

#[test]
fn test_int_cast_from_string_negative() {
    let args = vec![Value::Str("-42".to_string())];
    let result = int_cast(&args);
    assert!(result.is_ok());
    if let Ok(Value::Int(n)) = result {
        assert_eq!(n, -42);
    } else {
        panic!("Expected Int(-42), got {:?}", result);
    }
}

#[test]
fn test_dub_cast_from_string_scientific() {
    let args = vec![Value::Str("1e10".to_string())];
    let result = dub_cast(&args);
    assert!(result.is_ok());
    if let Ok(Value::Double(d)) = result {
        assert!((d - 1e10).abs() < f64::EPSILON);
    } else {
        panic!("Expected Double(1e10), got {:?}", result);
    }
}

#[test]
fn test_print_multiple_calls() {
    // Test that print can be called multiple times
    let args1 = vec![Value::Str("First".to_string())];
    let result1 = print(&args1);
    assert!(result1.is_ok());
    
    let args2 = vec![Value::Int(42)];
    let result2 = print(&args2);
    assert!(result2.is_ok());
}

#[test]
fn test_rt_concat3_all_strings() {
    // Test optimization when all args are strings
    let args = vec![
        Value::Str("a".to_string()),
        Value::Str("b".to_string()),
        Value::Str("c".to_string()),
    ];
    let result = rt_concat3(&args);
    assert!(result.is_ok());
    if let Ok(Value::Str(s)) = result {
        assert_eq!(s, "abc");
    } else {
        panic!("Expected Str(\"abc\"), got {:?}", result);
    }
}

#[test]
fn test_rt_concat4_all_strings() {
    let args = vec![
        Value::Str("a".to_string()),
        Value::Str("b".to_string()),
        Value::Str("c".to_string()),
        Value::Str("d".to_string()),
    ];
    let result = rt_concat4(&args);
    assert!(result.is_ok());
    if let Ok(Value::Str(s)) = result {
        assert_eq!(s, "abcd");
    } else {
        panic!("Expected Str(\"abcd\"), got {:?}", result);
    }
}

#[test]
fn test_rt_concat5_all_strings() {
    let args = vec![
        Value::Str("a".to_string()),
        Value::Str("b".to_string()),
        Value::Str("c".to_string()),
        Value::Str("d".to_string()),
        Value::Str("e".to_string()),
    ];
    let result = rt_concat5(&args);
    assert!(result.is_ok());
    if let Ok(Value::Str(s)) = result {
        assert_eq!(s, "abcde");
    } else {
        panic!("Expected Str(\"abcde\"), got {:?}", result);
    }
}

// Error handling tests

#[test]
fn test_len_wrong_type() {
    let args = vec![Value::Int(42)];
    let result = len(&args);
    assert!(result.is_err());
    if let Err(RuntimeError::TypeMismatch { .. }) = result {
        // Expected
    } else {
        panic!("Expected TypeMismatch error, got {:?}", result);
    }
}

#[test]
fn test_int_cast_invalid_string() {
    let args = vec![Value::Str("not a number".to_string())];
    let result = int_cast(&args);
    assert!(result.is_err());
    if let Err(RuntimeError::CallError(msg)) = result {
        assert!(msg.contains("Cannot convert"));
    } else {
        panic!("Expected CallError, got {:?}", result);
    }
}

#[test]
fn test_dub_cast_invalid_string() {
    let args = vec![Value::Str("not a number".to_string())];
    let result = dub_cast(&args);
    assert!(result.is_err());
    if let Err(RuntimeError::CallError(msg)) = result {
        assert!(msg.contains("Cannot convert"));
    } else {
        panic!("Expected CallError, got {:?}", result);
    }
}

#[test]
fn test_rt_concat2_insufficient_args() {
    let args = vec![Value::Str("a".to_string())];
    let result = rt_concat2(&args);
    assert!(result.is_err());
}

#[test]
fn test_rt_concat3_insufficient_args() {
    let args = vec![Value::Str("a".to_string()), Value::Str("b".to_string())];
    let result = rt_concat3(&args);
    assert!(result.is_err());
}

// Integration-style tests

#[test]
fn test_runtime_all_builtins_registered() {
    let runtime = Runtime::new();
    
    // Verify all expected builtins are present
    let expected_builtins = vec![
        "print", "len", "int", "dub", "str",
        "rt_concat2", "rt_concat3", "rt_concat4", "rt_concat5",
    ];
    
    for builtin in expected_builtins {
        assert!(runtime.is_builtin(builtin), "Builtin '{}' should be registered", builtin);
    }
}

#[test]
fn test_runtime_call_chain() {
    // Test calling multiple builtins in sequence
    let runtime = Runtime::new();
    
    // int(42) -> should return Int(42)
    let args1 = vec![Value::Int(42)];
    let result1 = runtime.call_builtin("int", &args1);
    assert!(result1.is_ok());
    
    // str(int(42)) -> should return "42"
    let args2 = vec![result1.unwrap()];
    let result2 = runtime.call_builtin("str", &args2);
    assert!(result2.is_ok());
    if let Ok(Value::Str(s)) = result2 {
        assert_eq!(s, "42");
    } else {
        panic!("Expected Str(\"42\"), got {:?}", result2);
    }
}



