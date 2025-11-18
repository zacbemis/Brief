use std::rc::Rc;
use brief_vm::*;
use brief_bytecode::*;

fn create_test_chunk() -> Chunk {
    let mut chunk = Chunk::new("test".to_string());
    chunk.max_regs = 10;
    chunk
}

#[test]
fn test_load_constant() {
    let mut chunk = create_test_chunk();
    let idx = chunk.add_constant(Constant::Int(42));
    chunk.emit(Instruction::new2(Opcode::LOADK, 0, idx));
    
    let mut vm = VM::new();
    vm.push_frame(Rc::new(chunk), 0);
    
    let result = vm.run();
    assert!(result.is_ok());
    // Frame should be popped after execution
}

#[test]
fn test_add_integers() {
    let mut chunk = create_test_chunk();
    let idx1 = chunk.add_constant(Constant::Int(10));
    let idx2 = chunk.add_constant(Constant::Int(20));
    chunk.emit(Instruction::new2(Opcode::LOADK, 0, idx1));
    chunk.emit(Instruction::new2(Opcode::LOADK, 1, idx2));
    chunk.emit(Instruction::new(Opcode::ADD, 2, 0, 1));
    chunk.emit(Instruction::new1(Opcode::RET, 2));
    
    let mut vm = VM::new();
    vm.push_frame(Rc::new(chunk), 0);
    
    let result = vm.run();
    assert!(result.is_ok());
    if let Ok(Value::Int(n)) = result {
        assert_eq!(n, 30);
    } else {
        panic!("Expected Int(30), got {:?}", result);
    }
}

#[test]
fn test_subtract_integers() {
    let mut chunk = create_test_chunk();
    let idx1 = chunk.add_constant(Constant::Int(20));
    let idx2 = chunk.add_constant(Constant::Int(10));
    chunk.emit(Instruction::new2(Opcode::LOADK, 0, idx1));
    chunk.emit(Instruction::new2(Opcode::LOADK, 1, idx2));
    chunk.emit(Instruction::new(Opcode::SUB, 2, 0, 1));
    chunk.emit(Instruction::new1(Opcode::RET, 2));
    
    let mut vm = VM::new();
    vm.push_frame(Rc::new(chunk), 0);
    
    let result = vm.run();
    assert!(result.is_ok());
    if let Ok(Value::Int(n)) = result {
        assert_eq!(n, 10);
    } else {
        panic!("Expected Int(10), got {:?}", result);
    }
}

#[test]
fn test_multiply_integers() {
    let mut chunk = create_test_chunk();
    let idx1 = chunk.add_constant(Constant::Int(5));
    let idx2 = chunk.add_constant(Constant::Int(6));
    chunk.emit(Instruction::new2(Opcode::LOADK, 0, idx1));
    chunk.emit(Instruction::new2(Opcode::LOADK, 1, idx2));
    chunk.emit(Instruction::new(Opcode::MUL, 2, 0, 1));
    chunk.emit(Instruction::new1(Opcode::RET, 2));
    
    let mut vm = VM::new();
    vm.push_frame(Rc::new(chunk), 0);
    
    let result = vm.run();
    assert!(result.is_ok());
    if let Ok(Value::Int(n)) = result {
        assert_eq!(n, 30);
    } else {
        panic!("Expected Int(30), got {:?}", result);
    }
}

#[test]
fn test_compare_equals() {
    let mut chunk = create_test_chunk();
    let idx1 = chunk.add_constant(Constant::Int(5));
    let idx2 = chunk.add_constant(Constant::Int(5));
    chunk.emit(Instruction::new2(Opcode::LOADK, 0, idx1));
    chunk.emit(Instruction::new2(Opcode::LOADK, 1, idx2));
    chunk.emit(Instruction::new(Opcode::CMP_EQ, 2, 0, 1));
    chunk.emit(Instruction::new1(Opcode::RET, 2));
    
    let mut vm = VM::new();
    vm.push_frame(Rc::new(chunk), 0);
    
    let result = vm.run();
    assert!(result.is_ok());
    if let Ok(Value::Bool(b)) = result {
        assert!(b);
    } else {
        panic!("Expected Bool(true), got {:?}", result);
    }
}

#[test]
fn test_compare_not_equals() {
    let mut chunk = create_test_chunk();
    let idx1 = chunk.add_constant(Constant::Int(5));
    let idx2 = chunk.add_constant(Constant::Int(10));
    chunk.emit(Instruction::new2(Opcode::LOADK, 0, idx1));
    chunk.emit(Instruction::new2(Opcode::LOADK, 1, idx2));
    chunk.emit(Instruction::new(Opcode::CMP_NE, 2, 0, 1));
    chunk.emit(Instruction::new1(Opcode::RET, 2));
    
    let mut vm = VM::new();
    vm.push_frame(Rc::new(chunk), 0);
    
    let result = vm.run();
    assert!(result.is_ok());
    if let Ok(Value::Bool(b)) = result {
        assert!(b);
    } else {
        panic!("Expected Bool(true), got {:?}", result);
    }
}

#[test]
fn test_negate() {
    let mut chunk = create_test_chunk();
    let idx = chunk.add_constant(Constant::Int(42));
    chunk.emit(Instruction::new2(Opcode::LOADK, 0, idx));
    chunk.emit(Instruction::new2(Opcode::NEG, 1, 0));
    chunk.emit(Instruction::new1(Opcode::RET, 1));
    
    let mut vm = VM::new();
    vm.push_frame(Rc::new(chunk), 0);
    
    let result = vm.run();
    assert!(result.is_ok());
    if let Ok(Value::Int(n)) = result {
        assert_eq!(n, -42);
    } else {
        panic!("Expected Int(-42), got {:?}", result);
    }
}

#[test]
fn test_not_operator() {
    let mut chunk = create_test_chunk();
    let idx = chunk.add_constant(Constant::Bool(false));
    chunk.emit(Instruction::new2(Opcode::LOADK, 0, idx));
    chunk.emit(Instruction::new2(Opcode::NOT, 1, 0));
    chunk.emit(Instruction::new1(Opcode::RET, 1));
    
    let mut vm = VM::new();
    vm.push_frame(Rc::new(chunk), 0);
    
    let result = vm.run();
    assert!(result.is_ok());
    if let Ok(Value::Bool(b)) = result {
        assert!(b); // !false == true
    } else {
        panic!("Expected Bool(true), got {:?}", result);
    }
}

#[test]
fn test_jump_if_false() {
    let mut chunk = create_test_chunk();
    let false_idx = chunk.add_constant(Constant::Bool(false));
    let true_idx = chunk.add_constant(Constant::Bool(true));
    
    // Load false into reg 0
    chunk.emit(Instruction::new2(Opcode::LOADK, 0, false_idx));
    // Jump if false (should jump over next 2 instructions)
    let jmp_ip = chunk.ip();
    chunk.emit(Instruction::new2(Opcode::JIF, 0, 0)); // Offset patched later
    // Load true (should be skipped if jump works)
    chunk.emit(Instruction::new2(Opcode::LOADK, 1, true_idx));
    chunk.emit(Instruction::new1(Opcode::RET, 1));
    
    // Patch jump offset to skip to the false return
    let skip_to_ip = chunk.ip(); // After RET 1
    let offset = (skip_to_ip as i16) - (jmp_ip as i16) - 1; // -1 because IP is already advanced
    let mut jmp_inst = chunk.code[jmp_ip];
    jmp_inst.set_offset(offset);
    chunk.code[jmp_ip] = jmp_inst;
    
    // Load false (this is where we jump to)
    chunk.emit(Instruction::new2(Opcode::LOADK, 2, false_idx));
    chunk.emit(Instruction::new1(Opcode::RET, 2));
    
    let mut vm = VM::new();
    vm.push_frame(Rc::new(chunk), 0);
    
    let result = vm.run();
    assert!(result.is_ok());
    if let Ok(Value::Bool(b)) = result {
        assert!(!b); // Should return false (jumped over true)
    } else {
        panic!("Expected Bool(false), got {:?}", result);
    }
}

#[test]
fn test_move_register() {
    let mut chunk = create_test_chunk();
    let idx = chunk.add_constant(Constant::Int(42));
    chunk.emit(Instruction::new2(Opcode::LOADK, 0, idx));
    chunk.emit(Instruction::new2(Opcode::MOVE, 1, 0));
    chunk.emit(Instruction::new1(Opcode::RET, 1));
    
    let mut vm = VM::new();
    vm.push_frame(Rc::new(chunk), 0);
    
    let result = vm.run();
    assert!(result.is_ok());
    if let Ok(Value::Int(n)) = result {
        assert_eq!(n, 42);
    } else {
        panic!("Expected Int(42), got {:?}", result);
    }
}

