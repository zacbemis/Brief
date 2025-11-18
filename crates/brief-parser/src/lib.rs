mod decl;
mod error;
mod expr;
mod parser;
mod stmt;
mod ty;

pub use error::ParseError;
pub use parser::Parser;

use brief_ast::Program;
use brief_diagnostic::FileId;
use brief_lexer::Token;

/// Parse tokens into an AST
pub fn parse(tokens: Vec<Token>, file_id: FileId) -> (Program, Vec<ParseError>) {
    let mut parser = Parser::new(tokens, file_id);
    let program = parser.parse();
    let errors = parser.get_errors().to_vec();
    (program, errors)
}
