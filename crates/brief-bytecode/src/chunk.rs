use crate::instruction::Instruction;
use crate::constant::Constant;

/// Code chunk representing a function
#[derive(Debug, Clone)]
pub struct Chunk {
    pub name: String,
    pub code: Vec<Instruction>,
    pub constants: Vec<Constant>,
    pub max_regs: u8,      // Maximum register count
    pub upvalue_count: u8, // Number of upvalues
    pub param_count: u8,   // Number of parameters
}

impl Chunk {
    pub fn new(name: String) -> Self {
        Self {
            name,
            code: Vec::new(),
            constants: Vec::new(),
            max_regs: 0,
            upvalue_count: 0,
            param_count: 0,
        }
    }

    /// Add an instruction to the chunk
    pub fn emit(&mut self, instruction: Instruction) -> usize {
        let ip = self.code.len();
        self.code.push(instruction);
        ip
    }

    /// Add a constant to the constant pool and return its index
    pub fn add_constant(&mut self, constant: Constant) -> u8 {
        // Check if constant already exists (simple deduplication)
        for (idx, existing) in self.constants.iter().enumerate() {
            if existing == &constant {
                return idx as u8;
            }
        }

        let index = self.constants.len();
        if index > 255 {
            panic!("Too many constants in chunk (max 256)");
        }
        self.constants.push(constant);
        index as u8
    }

    /// Get the instruction at the given IP
    pub fn get_instruction(&self, ip: usize) -> Option<&Instruction> {
        self.code.get(ip)
    }

    /// Patch an instruction at the given IP
    pub fn patch(&mut self, ip: usize, instruction: Instruction) {
        if ip < self.code.len() {
            self.code[ip] = instruction;
        }
    }

    /// Get the current instruction pointer (end of code)
    pub fn ip(&self) -> usize {
        self.code.len()
    }
}

impl std::fmt::Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Chunk: {}", self.name)?;
        writeln!(f, "  Parameters: {}", self.param_count)?;
        writeln!(f, "  Max Registers: {}", self.max_regs)?;
        writeln!(f, "  Upvalues: {}", self.upvalue_count)?;
        writeln!(f, "  Constants:")?;
        for (idx, constant) in self.constants.iter().enumerate() {
            writeln!(f, "    [{}] {}", idx, constant)?;
        }
        writeln!(f, "  Code:")?;
        for (ip, instruction) in self.code.iter().enumerate() {
            writeln!(f, "    {:04} {}", ip, instruction)?;
        }
        Ok(())
    }
}

