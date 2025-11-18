use brief_runtime::*;
use brief_vm::{Value, RuntimeError, BuiltinRuntime};

#[test]
fn test_print_builtin() {
    let args = vec![Value::Str("Hello, World!".to_string())];
    let result = print(&args);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Value::Null);
}

#[test]
fn test_print_requires_argument() {
    let args = vec![];
    let result = print(&args);
    assert!(result.is_err());
}

#[test]
fn test_len_string() {
    let args = vec![Value::Str("hello".to_string())];
    let result = len(&args);
    assert!(result.is_ok());
    if let Ok(Value::Int(n)) = result {
        assert_eq!(n, 5);
    } else {
        panic!("Expected Int(5), got {:?}", result);
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
fn test_int_cast_from_int() {
    let args = vec![Value::Int(42)];
    let result = int_cast(&args);
    assert!(result.is_ok());
    if let Ok(Value::Int(n)) = result {
        assert_eq!(n, 42);
    } else {
        panic!("Expected Int(42), got {:?}", result);
    }
}

#[test]
fn test_int_cast_from_double() {
    let args = vec![Value::Double(3.14)];
    let result = int_cast(&args);
    assert!(result.is_ok());
    if let Ok(Value::Int(n)) = result {
        assert_eq!(n, 3); // Truncated
    } else {
        panic!("Expected Int(3), got {:?}", result);
    }
}

#[test]
fn test_int_cast_from_bool() {
    let args = vec![Value::Bool(true)];
    let result = int_cast(&args);
    assert!(result.is_ok());
    if let Ok(Value::Int(n)) = result {
        assert_eq!(n, 1);
    } else {
        panic!("Expected Int(1), got {:?}", result);
    }
    
    let args = vec![Value::Bool(false)];
    let result = int_cast(&args);
    assert!(result.is_ok());
    if let Ok(Value::Int(n)) = result {
        assert_eq!(n, 0);
    } else {
        panic!("Expected Int(0), got {:?}", result);
    }
}

#[test]
fn test_int_cast_from_string() {
    let args = vec![Value::Str("42".to_string())];
    let result = int_cast(&args);
    assert!(result.is_ok());
    if let Ok(Value::Int(n)) = result {
        assert_eq!(n, 42);
    } else {
        panic!("Expected Int(42), got {:?}", result);
    }
}

#[test]
fn test_int_cast_from_string_invalid() {
    let args = vec![Value::Str("not a number".to_string())];
    let result = int_cast(&args);
    assert!(result.is_err());
}

#[test]
fn test_int_cast_from_null() {
    let args = vec![Value::Null];
    let result = int_cast(&args);
    assert!(result.is_err());
}

#[test]
fn test_dub_cast_from_int() {
    let args = vec![Value::Int(42)];
    let result = dub_cast(&args);
    assert!(result.is_ok());
    if let Ok(Value::Double(d)) = result {
        assert!((d - 42.0).abs() < f64::EPSILON);
    } else {
        panic!("Expected Double(42.0), got {:?}", result);
    }
}

#[test]
fn test_dub_cast_from_double() {
    let args = vec![Value::Double(3.14)];
    let result = dub_cast(&args);
    assert!(result.is_ok());
    if let Ok(Value::Double(d)) = result {
        assert!((d - 3.14).abs() < f64::EPSILON);
    } else {
        panic!("Expected Double(3.14), got {:?}", result);
    }
}

#[test]
fn test_dub_cast_from_bool() {
    let args = vec![Value::Bool(true)];
    let result = dub_cast(&args);
    assert!(result.is_ok());
    if let Ok(Value::Double(d)) = result {
        assert!((d - 1.0).abs() < f64::EPSILON);
    } else {
        panic!("Expected Double(1.0), got {:?}", result);
    }
}

#[test]
fn test_dub_cast_from_string() {
    let args = vec![Value::Str("3.14".to_string())];
    let result = dub_cast(&args);
    assert!(result.is_ok());
    if let Ok(Value::Double(d)) = result {
        assert!((d - 3.14).abs() < f64::EPSILON);
    } else {
        panic!("Expected Double(3.14), got {:?}", result);
    }
}

#[test]
fn test_str_cast_from_int() {
    let args = vec![Value::Int(42)];
    let result = str_cast(&args);
    assert!(result.is_ok());
    if let Ok(Value::Str(s)) = result {
        assert_eq!(s, "42");
    } else {
        panic!("Expected Str(\"42\"), got {:?}", result);
    }
}

#[test]
fn test_str_cast_from_double() {
    let args = vec![Value::Double(3.14)];
    let result = str_cast(&args);
    assert!(result.is_ok());
    if let Ok(Value::Str(s)) = result {
        assert_eq!(s, "3.14");
    } else {
        panic!("Expected Str(\"3.14\"), got {:?}", result);
    }
}

#[test]
fn test_str_cast_from_bool() {
    let args = vec![Value::Bool(true)];
    let result = str_cast(&args);
    assert!(result.is_ok());
    if let Ok(Value::Str(s)) = result {
        assert_eq!(s, "true");
    } else {
        panic!("Expected Str(\"true\"), got {:?}", result);
    }
}

#[test]
fn test_str_cast_from_string() {
    let args = vec![Value::Str("hello".to_string())];
    let result = str_cast(&args);
    assert!(result.is_ok());
    if let Ok(Value::Str(s)) = result {
        assert_eq!(s, "hello");
    } else {
        panic!("Expected Str(\"hello\"), got {:?}", result);
    }
}

#[test]
fn test_rt_concat2() {
    let args = vec![
        Value::Str("Hello, ".to_string()),
        Value::Str("World!".to_string()),
    ];
    let result = rt_concat2(&args);
    assert!(result.is_ok());
    if let Ok(Value::Str(s)) = result {
        assert_eq!(s, "Hello, World!");
    } else {
        panic!("Expected Str(\"Hello, World!\"), got {:?}", result);
    }
}

#[test]
fn test_rt_concat3() {
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
fn test_rt_concat4() {
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
fn test_rt_concat5() {
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

#[test]
fn test_runtime_registration() {
    let runtime = Runtime::new();
    
    // Check that builtins are registered
    assert!(runtime.is_builtin("print"));
    assert!(runtime.is_builtin("len"));
    assert!(runtime.is_builtin("int"));
    assert!(runtime.is_builtin("dub"));
    assert!(runtime.is_builtin("str"));
    assert!(runtime.is_builtin("rt_concat2"));
    assert!(runtime.is_builtin("rt_concat3"));
    assert!(runtime.is_builtin("rt_concat4"));
    assert!(runtime.is_builtin("rt_concat5"));
    
    // Check that non-builtins are not registered
    assert!(!runtime.is_builtin("unknown"));
}

#[test]
fn test_runtime_call_builtin() {
    let runtime = Runtime::new();
    let args = vec![Value::Int(42)];
    let result = runtime.call_builtin("int", &args);
    assert!(result.is_ok());
    if let Ok(Value::Int(n)) = result {
        assert_eq!(n, 42);
    } else {
        panic!("Expected Int(42), got {:?}", result);
    }
}

#[test]
fn test_runtime_call_unknown_builtin() {
    let runtime = Runtime::new();
    let args = vec![Value::Int(42)];
    let result = runtime.call_builtin("unknown", &args);
    assert!(result.is_err());
    if let Err(RuntimeError::CallError(msg)) = result {
        assert!(msg.contains("Unknown builtin"));
    } else {
        panic!("Expected CallError, got {:?}", result);
    }
}

