/// Opcode definitions for Brief bytecode
/// Fixed-size 32-bit instructions: [op(8)][a(8)][b(8)][c(8)]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Opcode {
    // Constants
    LOADK = 0,    // a = register, b = constant index
    LOADKX,       // Extended constant (uses next instruction)

    // Moves
    MOVE,         // a = destination, b = source

    // Arithmetic
    ADD,          // a = b + c
    SUB,          // a = b - c
    MUL,          // a = b * c
    DIVF,         // a = b / c (float)
    DIVI,         // a = b / c (int, truncates)
    MOD,          // a = b % c
    POW,          // a = b ** c

    // Comparisons
    CMP_EQ,       // a = (b == c)
    CMP_NE,       // a = (b != c)
    CMP_LT,       // a = (b < c)
    CMP_LE,       // a = (b <= c)
    CMP_GT,       // a = (b > c)
    CMP_GE,       // a = (b >= c)

    // Unary operations
    NEG,          // a = -b
    NOT,          // a = !b

    // Control flow
    JIF,          // if !a, jump b (signed offset)
    JMP,          // jump a (signed offset)

    // Functions
    CALL,         // a = function(b, c args starting at b+1)
    RET,          // return a

    // Builtins
    PRINT,        // print a

    // Extended opcodes (for future)
    EXT,          // Extended opcode follows
}

impl Opcode {
    /// Get the number of operands this opcode uses
    pub fn operand_count(&self) -> usize {
        match self {
            Opcode::LOADK | Opcode::MOVE | Opcode::JIF | Opcode::JMP | Opcode::RET | Opcode::PRINT => 2,
            Opcode::NEG | Opcode::NOT => 2,
            Opcode::ADD | Opcode::SUB | Opcode::MUL | Opcode::DIVF | Opcode::DIVI | Opcode::MOD | Opcode::POW => 3,
            Opcode::CMP_EQ | Opcode::CMP_NE | Opcode::CMP_LT | Opcode::CMP_LE | Opcode::CMP_GT | Opcode::CMP_GE => 3,
            Opcode::CALL => 3,
            Opcode::LOADKX | Opcode::EXT => 0, // Special cases
        }
    }
}

