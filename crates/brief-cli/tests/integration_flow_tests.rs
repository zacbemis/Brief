mod common;
use common::run_code;

#[test]
fn test_if_statement() {
    let source = "def test()\n\tif (5 > 3)\n\t\t10\n\telse\n\t\t20\n";
    let result = run_code(source);
    assert!(result.is_ok(), "expected Ok result, got {:?}", result);
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
    assert!(result.is_ok(), "expected Ok result, got {:?}", result);
    if let Ok(brief_vm::Value::Int(n)) = result {
        assert_eq!(n, 3);
    } else {
        panic!("Expected Int(3), got {:?}", result);
    }
}

#[test]
fn test_for_loop() {
    let source = "def test()\n\tx := 0\n\tfor (i := 0; i < 3; i := i + 1)\n\t\tx := x + 1\n\tx\n";
    let result = run_code(source);
    assert!(result.is_ok(), "expected Ok result, got {:?}", result);
    if let Ok(brief_vm::Value::Int(n)) = result {
        assert_eq!(n, 3);
    } else {
        panic!("Expected Int(3), got {:?}", result);
    }
}

