use std::fmt;

/// CLI-specific errors
#[derive(Debug)]
pub enum CliError {
    IoError(std::io::Error),
    LexError,
    ParseError,
    HirError(Vec<brief_hir::HirError>),
    RuntimeError(brief_vm::RuntimeError),
    UsageError(String),
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CliError::IoError(e) => write!(f, "IO error: {}", e),
            CliError::LexError => write!(f, "Lexical analysis failed"),
            CliError::ParseError => write!(f, "Parsing failed"),
            CliError::HirError(errors) => {
                write!(f, "HIR errors:")?;
                for err in errors {
                    write!(f, "\n  {:?}", err)?;
                }
                Ok(())
            },
            CliError::RuntimeError(e) => write!(f, "Runtime error: {}", e),
            CliError::UsageError(msg) => write!(f, "Usage error: {}", msg),
        }
    }
}

impl std::error::Error for CliError {}

impl From<std::io::Error> for CliError {
    fn from(err: std::io::Error) -> Self {
        CliError::IoError(err)
    }
}

impl From<Vec<brief_hir::HirError>> for CliError {
    fn from(errors: Vec<brief_hir::HirError>) -> Self {
        CliError::HirError(errors)
    }
}

impl From<brief_vm::RuntimeError> for CliError {
    fn from(err: brief_vm::RuntimeError) -> Self {
        CliError::RuntimeError(err)
    }
}

impl From<rustyline::error::ReadlineError> for CliError {
    fn from(err: rustyline::error::ReadlineError) -> Self {
        CliError::IoError(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Readline error: {:?}", err),
        ))
    }
}

/// Exit codes for the CLI
pub enum ExitCode {
    Success = 0,
    CompileError = 1,
    RuntimeError = 2,
}

