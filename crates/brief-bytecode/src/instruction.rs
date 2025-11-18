use crate::opcode::Opcode;

/// Fixed-size 32-bit instruction
/// Layout: [op(8)][a(8)][b(8)][c(8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Instruction(pub u32);

impl Instruction {
    /// Create a new instruction
    /// - `op`: Opcode (8 bits)
    /// - `a`: First operand (8 bits)
    /// - `b`: Second operand (8 bits)
    /// - `c`: Third operand (8 bits)
    pub fn new(op: Opcode, a: u8, b: u8, c: u8) -> Self {
        Self(
            (op as u32)
            | ((a as u32) << 8)
            | ((b as u32) << 16)
            | ((c as u32) << 24)
        )
    }

    /// Create instruction with 2 operands (c = 0)
    pub fn new2(op: Opcode, a: u8, b: u8) -> Self {
        Self::new(op, a, b, 0)
    }

    /// Create instruction with 1 operand (b = 0, c = 0)
    pub fn new1(op: Opcode, a: u8) -> Self {
        Self::new(op, a, 0, 0)
    }

    /// Get the opcode
    pub fn opcode(&self) -> Opcode {
        // Safety: We only create opcodes from valid u8 values
        unsafe { std::mem::transmute((self.0 & 0xFF) as u8) }
    }

    /// Get operand A
    pub fn a(&self) -> u8 {
        ((self.0 >> 8) & 0xFF) as u8
    }

    /// Get operand B
    pub fn b(&self) -> u8 {
        ((self.0 >> 16) & 0xFF) as u8
    }

    /// Get operand C
    pub fn c(&self) -> u8 {
        ((self.0 >> 24) & 0xFF) as u8
    }

    /// Get B and C as a 16-bit signed offset (for jumps)
    pub fn offset(&self) -> i16 {
        let b = self.b() as u16;
        let c = self.c() as u16;
        let combined = (c << 8) | b;
        combined as i16
    }

    /// Set B and C from a 16-bit signed offset (for jumps)
    pub fn set_offset(&mut self, offset: i16) {
        let offset = offset as u16;
        let b = (offset & 0xFF) as u8;
        let c = ((offset >> 8) & 0xFF) as u8;
        self.0 = (self.0 & 0x0000FFFF) | ((b as u32) << 16) | ((c as u32) << 24);
    }
}

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} a={} b={} c={}", self.opcode(), self.a(), self.b(), self.c())
    }
}

