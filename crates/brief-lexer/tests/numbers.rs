use brief_lexer::{lex, TokenKind};
use brief_diagnostic::FileId;

fn lex_kinds(source: &str) -> Vec<TokenKind> {
    let (tokens, _errors) = lex(source, FileId(0));
    tokens.into_iter().map(|t| t.kind).collect()
}

#[test]
fn test_integers() {
    let kinds = lex_kinds("0 123 456 -789");
    
    assert_eq!(
        kinds,
        vec![
            TokenKind::Integer(0),
            TokenKind::Integer(123),
            TokenKind::Integer(456),
            TokenKind::Minus,  // Negative numbers are Minus + Integer
            TokenKind::Integer(789),
            TokenKind::Newline,
            TokenKind::Eof
        ]
    );
}

#[test]
fn test_doubles() {
    let kinds = lex_kinds("1.0 3.14 0.5 .5");
    
    assert_eq!(
        kinds,
        vec![
            TokenKind::Double(1.0),
            TokenKind::Double(3.14),
            TokenKind::Double(0.5),
            TokenKind::Double(0.5),  // .5 is parsed as 0.5
            TokenKind::Newline,
            TokenKind::Eof
        ]
    );
}

#[test]
fn test_negative_doubles() {
    let kinds = lex_kinds("-1.5 -.5");
    
    assert_eq!(
        kinds,
        vec![
            TokenKind::Minus,
            TokenKind::Double(1.5),
            TokenKind::Minus,
            TokenKind::Double(0.5),
            TokenKind::Newline,
            TokenKind::Eof
        ]
    );
}

#[test]
fn test_number_edge_cases() {
    let kinds = lex_kinds("0 00 000");
    
    // Leading zeros are allowed (just parsed as 0)
    assert_eq!(
        kinds,
        vec![
            TokenKind::Integer(0),
            TokenKind::Integer(0),
            TokenKind::Integer(0),
            TokenKind::Newline,
            TokenKind::Eof
        ]
    );
}

#[test]
fn test_large_numbers() {
    let kinds = lex_kinds("1234567890 999999999");
    
    assert_eq!(
        kinds,
        vec![
            TokenKind::Integer(1234567890),
            TokenKind::Integer(999999999),
            TokenKind::Newline,
            TokenKind::Eof
        ]
    );
}

