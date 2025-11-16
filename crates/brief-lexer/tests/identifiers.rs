use brief_lexer::{lex, TokenKind};
use brief_diagnostic::FileId;

fn lex_kinds(source: &str) -> Vec<TokenKind> {
    let (tokens, _errors) = lex(source, FileId(0));
    tokens.into_iter().map(|t| t.kind).collect()
}

#[test]
fn test_simple_identifier() {
    let kinds = lex_kinds("x");
    
    assert_eq!(
        kinds,
        vec![
            TokenKind::Identifier("x".to_string()),
            TokenKind::Newline,
            TokenKind::Eof
        ]
    );
}

#[test]
fn test_identifier_with_underscore() {
    let kinds = lex_kinds("my_var");
    
    assert_eq!(
        kinds,
        vec![
            TokenKind::Identifier("my_var".to_string()),
            TokenKind::Newline,
            TokenKind::Eof
        ]
    );
}

#[test]
fn test_identifier_starting_with_underscore() {
    let kinds = lex_kinds("_private");
    
    assert_eq!(
        kinds,
        vec![
            TokenKind::Identifier("_private".to_string()),
            TokenKind::Newline,
            TokenKind::Eof
        ]
    );
}

#[test]
fn test_identifier_with_numbers() {
    let kinds = lex_kinds("var123");
    
    assert_eq!(
        kinds,
        vec![
            TokenKind::Identifier("var123".to_string()),
            TokenKind::Newline,
            TokenKind::Eof
        ]
    );
}

#[test]
fn test_multiple_identifiers() {
    let kinds = lex_kinds("x y z");
    
    assert_eq!(
        kinds,
        vec![
            TokenKind::Identifier("x".to_string()),
            TokenKind::Identifier("y".to_string()),
            TokenKind::Identifier("z".to_string()),
            TokenKind::Newline,
            TokenKind::Eof
        ]
    );
}

#[test]
fn test_identifier_vs_keyword() {
    let kinds = lex_kinds("int myint");
    
    assert_eq!(
        kinds,
        vec![
            TokenKind::Int,
            TokenKind::Identifier("myint".to_string()),
            TokenKind::Newline,
            TokenKind::Eof
        ]
    );
}

#[test]
fn test_camel_case_identifier() {
    let kinds = lex_kinds("myVariableName");
    
    assert_eq!(
        kinds,
        vec![
            TokenKind::Identifier("myVariableName".to_string()),
            TokenKind::Newline,
            TokenKind::Eof
        ]
    );
}

