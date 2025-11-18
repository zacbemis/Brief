use brief_runtime::Runtime;
use brief_vm::BuiltinRuntime;

#[test]
fn test_builtin_len_string() {
    // Test len() builtin directly
    let runtime = Runtime::new();
    
    // Manually test builtin call
    let args = vec![brief_vm::Value::Str("hello".to_string())];
    let result = runtime.call_builtin("len", &args);
    assert!(result.is_ok());
    if let Ok(brief_vm::Value::Int(n)) = result {
        assert_eq!(n, 5);
    } else {
        panic!("Expected Int(5), got {:?}", result);
    }
}

#[test]
fn test_builtin_int_cast_through_vm() {
    let runtime = Runtime::new();
    
    let args = vec![brief_vm::Value::Double(3.14)];
    let result = runtime.call_builtin("int", &args);
    assert!(result.is_ok());
    if let Ok(brief_vm::Value::Int(n)) = result {
        assert_eq!(n, 3);
    } else {
        panic!("Expected Int(3), got {:?}", result);
    }
}

#[test]
fn test_builtin_dub_cast_through_vm() {
    let runtime = Runtime::new();
    
    let args = vec![brief_vm::Value::Int(42)];
    let result = runtime.call_builtin("dub", &args);
    assert!(result.is_ok());
    if let Ok(brief_vm::Value::Double(d)) = result {
        assert!((d - 42.0).abs() < f64::EPSILON);
    } else {
        panic!("Expected Double(42.0), got {:?}", result);
    }
}

#[test]
fn test_builtin_str_cast_through_vm() {
    let runtime = Runtime::new();
    
    let args = vec![brief_vm::Value::Int(123)];
    let result = runtime.call_builtin("str", &args);
    assert!(result.is_ok());
    if let Ok(brief_vm::Value::Str(s)) = result {
        assert_eq!(s, "123");
    } else {
        panic!("Expected Str(\"123\"), got {:?}", result);
    }
}

#[test]
fn test_builtin_concat_through_vm() {
    let runtime = Runtime::new();
    
    let args = vec![
        brief_vm::Value::Str("Hello".to_string()),
        brief_vm::Value::Str("World".to_string()),
    ];
    let result = runtime.call_builtin("rt_concat2", &args);
    assert!(result.is_ok());
    if let Ok(brief_vm::Value::Str(s)) = result {
        assert_eq!(s, "HelloWorld");
    } else {
        panic!("Expected Str(\"HelloWorld\"), got {:?}", result);
    }
}

#[test]
fn test_unknown_builtin() {
    let runtime = Runtime::new();
    let args = vec![brief_vm::Value::Int(42)];
    let result = runtime.call_builtin("unknown_function", &args);
    assert!(result.is_err());
}

