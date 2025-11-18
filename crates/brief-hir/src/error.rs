use brief_diagnostic::Span;

/// HIR-specific errors
#[derive(Debug, Clone, PartialEq)]
pub enum HirError {
    /// Undefined variable
    UndefinedVariable {
        name: String,
        span: Span,
    },
    /// Duplicate symbol definition
    DuplicateSymbol {
        name: String,
        original_span: Span,
        duplicate_span: Span,
    },
    /// Cannot capture variable (e.g., trying to capture a parameter)
    InvalidCapture {
        name: String,
        span: Span,
    },
    /// Other HIR errors
    Other {
        message: String,
        span: Span,
    },
}

impl HirError {
    pub fn span(&self) -> Span {
        match self {
            HirError::UndefinedVariable { span, .. } => *span,
            HirError::DuplicateSymbol { duplicate_span, .. } => *duplicate_span,
            HirError::InvalidCapture { span, .. } => *span,
            HirError::Other { span, .. } => *span,
        }
    }
}

