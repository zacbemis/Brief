use brief_lexer::{lex, Token, TokenKind};
use brief_diagnostic::FileId;

/// Helper function to lex source and return just the token kinds (ignoring spans)
pub fn lex_kinds(source: &str) -> Vec<TokenKind> {
    let (tokens, _errors) = lex(source, FileId(0));
    tokens.into_iter().map(|t| t.kind).collect()
}

/// Helper function to lex source and return tokens with kinds
pub fn lex_tokens(source: &str) -> Vec<Token> {
    let (tokens, _errors) = lex(source, FileId(0));
    tokens
}

/// Helper function to lex source and return errors
pub fn lex_errors(source: &str) -> Vec<String> {
    let (_tokens, errors) = lex(source, FileId(0));
    errors
}

