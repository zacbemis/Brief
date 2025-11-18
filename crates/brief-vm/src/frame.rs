use std::rc::Rc;
use brief_bytecode::Chunk;
use crate::value::Value;

/// Call frame for function execution
#[derive(Debug)]
pub struct Frame {
    pub chunk: Rc<Chunk>,
    pub ip: usize,              // Instruction pointer
    pub registers: Vec<Value>,  // Register array (size = chunk.max_regs)
    pub base: usize,            // Base register for arguments
}

impl Frame {
    pub fn new(chunk: Rc<Chunk>, base: usize) -> Self {
        let register_count = chunk.max_regs as usize;
        Self {
            chunk,
            ip: 0,
            registers: vec![Value::Null; register_count],
            base,
        }
    }

    /// Get current instruction
    pub fn current_instruction(&self) -> Option<&brief_bytecode::Instruction> {
        self.chunk.code.get(self.ip)
    }

    /// Advance instruction pointer
    pub fn advance(&mut self) {
        self.ip += 1;
    }
}

