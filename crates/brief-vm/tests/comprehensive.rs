use std::rc::Rc;
use brief_vm::*;
use brief_bytecode::*;

fn create_test_chunk() -> Chunk {
    let mut chunk = Chunk::new("test".to_string());
    chunk.max_regs = 20;
    chunk
}

fn run_chunk(chunk: Chunk) -> Result<Value, RuntimeError> {
    let mut vm = VM::new();
    vm.push_frame(Rc::new(chunk), 0);
    vm.run()
}

// Double arithmetic tests

#[test]
fn test_add_doubles() {
    let mut chunk = create_test_chunk();
    let idx1 = chunk.add_constant(Constant::Double(3.5));
    let idx2 = chunk.add_constant(Constant::Double(2.5));
    chunk.emit(Instruction::new2(Opcode::LOADK, 0, idx1));
    chunk.emit(Instruction::new2(Opcode::LOADK, 1, idx2));
    chunk.emit(Instruction::new(Opcode::ADD, 2, 0, 1));
    chunk.emit(Instruction::new1(Opcode::RET, 2));
    
    let result = run_chunk(chunk);
    assert!(result.is_ok());
    if let Ok(Value::Double(d)) = result {
        assert!((d - 6.0).abs() < f64::EPSILON);
    } else {
        panic!("Expected Double(6.0), got {:?}", result);
    }
}

#[test]
fn test_mixed_int_double_add() {
    let mut chunk = create_test_chunk();
    let idx1 = chunk.add_constant(Constant::Int(5));
    let idx2 = chunk.add_constant(Constant::Double(2.5));
    chunk.emit(Instruction::new2(Opcode::LOADK, 0, idx1));
    chunk.emit(Instruction::new2(Opcode::LOADK, 1, idx2));
    chunk.emit(Instruction::new(Opcode::ADD, 2, 0, 1));
    chunk.emit(Instruction::new1(Opcode::RET, 2));
    
    let result = run_chunk(chunk);
    assert!(result.is_ok());
    if let Ok(Value::Double(d)) = result {
        assert!((d - 7.5).abs() < f64::EPSILON);
    } else {
        panic!("Expected Double(7.5), got {:?}", result);
    }
}

// String concatenation tests

#[test]
fn test_string_concatenation() {
    let mut chunk = create_test_chunk();
    let idx1 = chunk.add_constant(Constant::Str("Hello, ".to_string()));
    let idx2 = chunk.add_constant(Constant::Str("World!".to_string()));
    chunk.emit(Instruction::new2(Opcode::LOADK, 0, idx1));
    chunk.emit(Instruction::new2(Opcode::LOADK, 1, idx2));
    chunk.emit(Instruction::new(Opcode::ADD, 2, 0, 1));
    chunk.emit(Instruction::new1(Opcode::RET, 2));
    
    let result = run_chunk(chunk);
    assert!(result.is_ok());
    if let Ok(Value::Str(s)) = result {
        assert_eq!(s, "Hello, World!");
    } else {
        panic!("Expected Str(\"Hello, World!\"), got {:?}", result);
    }
}

#[test]
fn test_string_int_concatenation() {
    let mut chunk = create_test_chunk();
    let idx1 = chunk.add_constant(Constant::Str("Value: ".to_string()));
    let idx2 = chunk.add_constant(Constant::Int(42));
    chunk.emit(Instruction::new2(Opcode::LOADK, 0, idx1));
    chunk.emit(Instruction::new2(Opcode::LOADK, 1, idx2));
    chunk.emit(Instruction::new(Opcode::ADD, 2, 0, 1));
    chunk.emit(Instruction::new1(Opcode::RET, 2));
    
    let result = run_chunk(chunk);
    assert!(result.is_ok());
    if let Ok(Value::Str(s)) = result {
        assert_eq!(s, "Value: 42");
    } else {
        panic!("Expected Str(\"Value: 42\"), got {:?}", result);
    }
}

// Division tests

#[test]
fn test_float_division() {
    let mut chunk = create_test_chunk();
    let idx1 = chunk.add_constant(Constant::Int(10));
    let idx2 = chunk.add_constant(Constant::Int(3));
    chunk.emit(Instruction::new2(Opcode::LOADK, 0, idx1));
    chunk.emit(Instruction::new2(Opcode::LOADK, 1, idx2));
    chunk.emit(Instruction::new(Opcode::DIVF, 2, 0, 1));
    chunk.emit(Instruction::new1(Opcode::RET, 2));
    
    let result = run_chunk(chunk);
    assert!(result.is_ok());
    if let Ok(Value::Double(d)) = result {
        let expected = 10.0 / 3.0;
        assert!((d - expected).abs() < f64::EPSILON);
    } else {
        panic!("Expected Double(10.0/3.0), got {:?}", result);
    }
}

#[test]
fn test_integer_division() {
    let mut chunk = create_test_chunk();
    let idx1 = chunk.add_constant(Constant::Int(10));
    let idx2 = chunk.add_constant(Constant::Int(3));
    chunk.emit(Instruction::new2(Opcode::LOADK, 0, idx1));
    chunk.emit(Instruction::new2(Opcode::LOADK, 1, idx2));
    chunk.emit(Instruction::new(Opcode::DIVI, 2, 0, 1));
    chunk.emit(Instruction::new1(Opcode::RET, 2));
    
    let result = run_chunk(chunk);
    assert!(result.is_ok());
    if let Ok(Value::Int(n)) = result {
        assert_eq!(n, 3); // 10 / 3 = 3 (truncated)
    } else {
        panic!("Expected Int(3), got {:?}", result);
    }
}

#[test]
fn test_division_by_zero_float() {
    let mut chunk = create_test_chunk();
    let idx1 = chunk.add_constant(Constant::Int(10));
    let idx2 = chunk.add_constant(Constant::Int(0));
    chunk.emit(Instruction::new2(Opcode::LOADK, 0, idx1));
    chunk.emit(Instruction::new2(Opcode::LOADK, 1, idx2));
    chunk.emit(Instruction::new(Opcode::DIVF, 2, 0, 1));
    chunk.emit(Instruction::new1(Opcode::RET, 2));
    
    let result = run_chunk(chunk);
    assert!(result.is_err());
    if let Err(RuntimeError::DivisionByZero) = result {
        // Expected
    } else {
        panic!("Expected DivisionByZero error, got {:?}", result);
    }
}

#[test]
fn test_division_by_zero_int() {
    let mut chunk = create_test_chunk();
    let idx1 = chunk.add_constant(Constant::Int(10));
    let idx2 = chunk.add_constant(Constant::Int(0));
    chunk.emit(Instruction::new2(Opcode::LOADK, 0, idx1));
    chunk.emit(Instruction::new2(Opcode::LOADK, 1, idx2));
    chunk.emit(Instruction::new(Opcode::DIVI, 2, 0, 1));
    chunk.emit(Instruction::new1(Opcode::RET, 2));
    
    let result = run_chunk(chunk);
    assert!(result.is_err());
    if let Err(RuntimeError::DivisionByZero) = result {
        // Expected
    } else {
        panic!("Expected DivisionByZero error, got {:?}", result);
    }
}

// Modulo tests

#[test]
fn test_modulo() {
    let mut chunk = create_test_chunk();
    let idx1 = chunk.add_constant(Constant::Int(10));
    let idx2 = chunk.add_constant(Constant::Int(3));
    chunk.emit(Instruction::new2(Opcode::LOADK, 0, idx1));
    chunk.emit(Instruction::new2(Opcode::LOADK, 1, idx2));
    chunk.emit(Instruction::new(Opcode::MOD, 2, 0, 1));
    chunk.emit(Instruction::new1(Opcode::RET, 2));
    
    let result = run_chunk(chunk);
    assert!(result.is_ok());
    if let Ok(Value::Int(n)) = result {
        assert_eq!(n, 1); // 10 % 3 = 1
    } else {
        panic!("Expected Int(1), got {:?}", result);
    }
}

// Power tests

#[test]
fn test_power() {
    let mut chunk = create_test_chunk();
    let idx1 = chunk.add_constant(Constant::Int(2));
    let idx2 = chunk.add_constant(Constant::Int(3));
    chunk.emit(Instruction::new2(Opcode::LOADK, 0, idx1));
    chunk.emit(Instruction::new2(Opcode::LOADK, 1, idx2));
    chunk.emit(Instruction::new(Opcode::POW, 2, 0, 1));
    chunk.emit(Instruction::new1(Opcode::RET, 2));
    
    let result = run_chunk(chunk);
    assert!(result.is_ok());
    if let Ok(Value::Double(d)) = result {
        assert!((d - 8.0).abs() < f64::EPSILON); // 2^3 = 8
    } else {
        panic!("Expected Double(8.0), got {:?}", result);
    }
}

// Comparison tests

#[test]
fn test_compare_less_than() {
    let mut chunk = create_test_chunk();
    let idx1 = chunk.add_constant(Constant::Int(5));
    let idx2 = chunk.add_constant(Constant::Int(10));
    chunk.emit(Instruction::new2(Opcode::LOADK, 0, idx1));
    chunk.emit(Instruction::new2(Opcode::LOADK, 1, idx2));
    chunk.emit(Instruction::new(Opcode::CMP_LT, 2, 0, 1));
    chunk.emit(Instruction::new1(Opcode::RET, 2));
    
    let result = run_chunk(chunk);
    assert!(result.is_ok());
    if let Ok(Value::Bool(b)) = result {
        assert!(b); // 5 < 10
    } else {
        panic!("Expected Bool(true), got {:?}", result);
    }
}

#[test]
fn test_compare_less_equal() {
    let mut chunk = create_test_chunk();
    let idx1 = chunk.add_constant(Constant::Int(5));
    let idx2 = chunk.add_constant(Constant::Int(5));
    chunk.emit(Instruction::new2(Opcode::LOADK, 0, idx1));
    chunk.emit(Instruction::new2(Opcode::LOADK, 1, idx2));
    chunk.emit(Instruction::new(Opcode::CMP_LE, 2, 0, 1));
    chunk.emit(Instruction::new1(Opcode::RET, 2));
    
    let result = run_chunk(chunk);
    assert!(result.is_ok());
    if let Ok(Value::Bool(b)) = result {
        assert!(b); // 5 <= 5
    } else {
        panic!("Expected Bool(true), got {:?}", result);
    }
}

#[test]
fn test_compare_greater_than() {
    let mut chunk = create_test_chunk();
    let idx1 = chunk.add_constant(Constant::Int(10));
    let idx2 = chunk.add_constant(Constant::Int(5));
    chunk.emit(Instruction::new2(Opcode::LOADK, 0, idx1));
    chunk.emit(Instruction::new2(Opcode::LOADK, 1, idx2));
    chunk.emit(Instruction::new(Opcode::CMP_GT, 2, 0, 1));
    chunk.emit(Instruction::new1(Opcode::RET, 2));
    
    let result = run_chunk(chunk);
    assert!(result.is_ok());
    if let Ok(Value::Bool(b)) = result {
        assert!(b); // 10 > 5
    } else {
        panic!("Expected Bool(true), got {:?}", result);
    }
}

#[test]
fn test_compare_greater_equal() {
    let mut chunk = create_test_chunk();
    let idx1 = chunk.add_constant(Constant::Int(10));
    let idx2 = chunk.add_constant(Constant::Int(10));
    chunk.emit(Instruction::new2(Opcode::LOADK, 0, idx1));
    chunk.emit(Instruction::new2(Opcode::LOADK, 1, idx2));
    chunk.emit(Instruction::new(Opcode::CMP_GE, 2, 0, 1));
    chunk.emit(Instruction::new1(Opcode::RET, 2));
    
    let result = run_chunk(chunk);
    assert!(result.is_ok());
    if let Ok(Value::Bool(b)) = result {
        assert!(b); // 10 >= 10
    } else {
        panic!("Expected Bool(true), got {:?}", result);
    }
}

// Truthiness tests

#[test]
fn test_null_truthiness() {
    let mut chunk = create_test_chunk();
    let null_idx = chunk.add_constant(Constant::Null);
    chunk.emit(Instruction::new2(Opcode::LOADK, 0, null_idx));
    chunk.emit(Instruction::new2(Opcode::NOT, 1, 0));
    chunk.emit(Instruction::new1(Opcode::RET, 1));
    
    let result = run_chunk(chunk);
    assert!(result.is_ok());
    if let Ok(Value::Bool(b)) = result {
        assert!(b); // !null == true (null is falsey)
    } else {
        panic!("Expected Bool(true), got {:?}", result);
    }
}

#[test]
fn test_false_truthiness() {
    let mut chunk = create_test_chunk();
    let false_idx = chunk.add_constant(Constant::Bool(false));
    chunk.emit(Instruction::new2(Opcode::LOADK, 0, false_idx));
    chunk.emit(Instruction::new2(Opcode::NOT, 1, 0));
    chunk.emit(Instruction::new1(Opcode::RET, 1));
    
    let result = run_chunk(chunk);
    assert!(result.is_ok());
    if let Ok(Value::Bool(b)) = result {
        assert!(b); // !false == true
    } else {
        panic!("Expected Bool(true), got {:?}", result);
    }
}

#[test]
fn test_true_truthiness() {
    let mut chunk = create_test_chunk();
    let true_idx = chunk.add_constant(Constant::Bool(true));
    chunk.emit(Instruction::new2(Opcode::LOADK, 0, true_idx));
    chunk.emit(Instruction::new2(Opcode::NOT, 1, 0));
    chunk.emit(Instruction::new1(Opcode::RET, 1));
    
    let result = run_chunk(chunk);
    assert!(result.is_ok());
    if let Ok(Value::Bool(b)) = result {
        assert!(!b); // !true == false
    } else {
        panic!("Expected Bool(false), got {:?}", result);
    }
}

// Error handling tests

#[test]
fn test_invalid_register() {
    let mut chunk = create_test_chunk();
    chunk.max_regs = 5; // Only 5 registers (0-4)
    let idx = chunk.add_constant(Constant::Int(42));
    chunk.emit(Instruction::new2(Opcode::LOADK, 10, idx)); // Invalid register
    
    let result = run_chunk(chunk);
    assert!(result.is_err());
    if let Err(RuntimeError::InvalidRegister(10)) = result {
        // Expected
    } else {
        panic!("Expected InvalidRegister(10), got {:?}", result);
    }
}

#[test]
fn test_invalid_constant_index() {
    let mut chunk = create_test_chunk();
    // Don't add any constants, but try to load one
    chunk.emit(Instruction::new2(Opcode::LOADK, 0, 5)); // Invalid constant index
    
    let result = run_chunk(chunk);
    assert!(result.is_err());
    if let Err(RuntimeError::InvalidConstantIndex(5)) = result {
        // Expected
    } else {
        panic!("Expected InvalidConstantIndex(5), got {:?}", result);
    }
}

#[test]
fn test_type_mismatch_subtract() {
    let mut chunk = create_test_chunk();
    let idx1 = chunk.add_constant(Constant::Str("hello".to_string()));
    let idx2 = chunk.add_constant(Constant::Int(5));
    chunk.emit(Instruction::new2(Opcode::LOADK, 0, idx1));
    chunk.emit(Instruction::new2(Opcode::LOADK, 1, idx2));
    chunk.emit(Instruction::new(Opcode::SUB, 2, 0, 1)); // Str - Int should fail
    chunk.emit(Instruction::new1(Opcode::RET, 2));
    
    let result = run_chunk(chunk);
    assert!(result.is_err());
    if let Err(RuntimeError::TypeMismatch { .. }) = result {
        // Expected
    } else {
        panic!("Expected TypeMismatch error, got {:?}", result);
    }
}

// Double negation test

#[test]
fn test_double_negate() {
    let mut chunk = create_test_chunk();
    let idx = chunk.add_constant(Constant::Int(42));
    chunk.emit(Instruction::new2(Opcode::LOADK, 0, idx));
    chunk.emit(Instruction::new2(Opcode::NEG, 1, 0));
    chunk.emit(Instruction::new2(Opcode::NEG, 2, 1));
    chunk.emit(Instruction::new1(Opcode::RET, 2));
    
    let result = run_chunk(chunk);
    assert!(result.is_ok());
    if let Ok(Value::Int(n)) = result {
        assert_eq!(n, 42); // -(-42) == 42
    } else {
        panic!("Expected Int(42), got {:?}", result);
    }
}

