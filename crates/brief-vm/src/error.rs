/// Runtime error
#[derive(Debug, Clone, PartialEq)]
pub enum RuntimeError {
    StackUnderflow,
    StackOverflow,
    InvalidRegister(u8),
    InvalidConstantIndex(u8),
    TypeMismatch { expected: String, got: String },
    DivisionByZero,
    UnknownOpcode,
    UndefinedVariable(String),
    CallError(String),
    // Add more error types as needed
}

impl std::fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RuntimeError::StackUnderflow => write!(f, "Stack underflow"),
            RuntimeError::StackOverflow => write!(f, "Stack overflow"),
            RuntimeError::InvalidRegister(reg) => write!(f, "Invalid register: {}", reg),
            RuntimeError::InvalidConstantIndex(idx) => write!(f, "Invalid constant index: {}", idx),
            RuntimeError::TypeMismatch { expected, got } => {
                write!(f, "Type mismatch: expected {}, got {}", expected, got)
            },
            RuntimeError::DivisionByZero => write!(f, "Division by zero"),
            RuntimeError::UnknownOpcode => write!(f, "Unknown opcode"),
            RuntimeError::UndefinedVariable(name) => write!(f, "Undefined variable: {}", name),
            RuntimeError::CallError(msg) => write!(f, "Call error: {}", msg),
        }
    }
}

impl std::error::Error for RuntimeError {}

