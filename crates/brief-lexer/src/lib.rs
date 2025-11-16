pub mod lexer;
pub mod token;

pub use lexer::Lexer;
pub use token::{Token, TokenKind};

use brief_diagnostic::FileId;

/// Lex source code into tokens
pub fn lex(source: &str, file_id: FileId) -> (Vec<Token>, Vec<String>) {
    Lexer::new(source, file_id).lex()
}
