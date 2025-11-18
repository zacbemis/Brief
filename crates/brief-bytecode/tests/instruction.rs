use brief_bytecode::*;

#[test]
fn test_instruction_encoding() {
    let inst = Instruction::new(Opcode::ADD, 1, 2, 3);
    assert_eq!(inst.opcode(), Opcode::ADD);
    assert_eq!(inst.a(), 1);
    assert_eq!(inst.b(), 2);
    assert_eq!(inst.c(), 3);
}

#[test]
fn test_instruction_2_operands() {
    let inst = Instruction::new2(Opcode::MOVE, 5, 10);
    assert_eq!(inst.opcode(), Opcode::MOVE);
    assert_eq!(inst.a(), 5);
    assert_eq!(inst.b(), 10);
    assert_eq!(inst.c(), 0);
}

#[test]
fn test_instruction_1_operand() {
    let inst = Instruction::new1(Opcode::RET, 7);
    assert_eq!(inst.opcode(), Opcode::RET);
    assert_eq!(inst.a(), 7);
    assert_eq!(inst.b(), 0);
    assert_eq!(inst.c(), 0);
}

#[test]
fn test_jump_offset() {
    let mut inst = Instruction::new(Opcode::JMP, 0, 0, 0);
    inst.set_offset(42);
    assert_eq!(inst.offset(), 42);
    
    inst.set_offset(-10);
    assert_eq!(inst.offset(), -10);
}

#[test]
fn test_chunk_operations() {
    let mut chunk = Chunk::new("test".to_string());
    
    // Add constants
    let idx1 = chunk.add_constant(Constant::Int(42));
    let idx2 = chunk.add_constant(Constant::Str("hello".to_string()));
    
    // Emit instructions
    let ip1 = chunk.emit(Instruction::new2(Opcode::LOADK, 0, idx1));
    let ip2 = chunk.emit(Instruction::new2(Opcode::LOADK, 1, idx2));
    let ip3 = chunk.emit(Instruction::new(Opcode::ADD, 2, 0, 1));
    
    assert_eq!(ip1, 0);
    assert_eq!(ip2, 1);
    assert_eq!(ip3, 2);
    assert_eq!(chunk.code.len(), 3);
    assert_eq!(chunk.constants.len(), 2);
}

#[test]
fn test_constant_deduplication() {
    let mut chunk = Chunk::new("test".to_string());
    
    let idx1 = chunk.add_constant(Constant::Int(42));
    let idx2 = chunk.add_constant(Constant::Int(42)); // Duplicate
    
    assert_eq!(idx1, idx2);
    assert_eq!(chunk.constants.len(), 1);
}

