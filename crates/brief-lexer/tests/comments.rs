use brief_lexer::{lex, TokenKind};
use brief_diagnostic::FileId;

fn lex_kinds(source: &str) -> Vec<TokenKind> {
    let (tokens, _errors) = lex(source, FileId(0));
    tokens.into_iter().map(|t| t.kind).collect()
}

#[test]
fn test_line_comment() {
    let kinds = lex_kinds("x // this is a comment\ny");
    
    assert_eq!(
        kinds,
        vec![
            TokenKind::Identifier("x".to_string()),
            TokenKind::Identifier("y".to_string()),
            TokenKind::Newline,
            TokenKind::Eof
        ]
    );
}

#[test]
fn test_line_comment_at_start() {
    let kinds = lex_kinds("// comment only\nx");
    
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
fn test_block_comment() {
    let kinds = lex_kinds("x /* block comment */ y");
    
    assert_eq!(
        kinds,
        vec![
            TokenKind::Identifier("x".to_string()),
            TokenKind::Identifier("y".to_string()),
            TokenKind::Newline,
            TokenKind::Eof
        ]
    );
}

#[test]
fn test_block_comment_multiline() {
    let kinds = lex_kinds("x /* line 1\nline 2 */ y");
    
    assert_eq!(
        kinds,
        vec![
            TokenKind::Identifier("x".to_string()),
            TokenKind::Identifier("y".to_string()),
            TokenKind::Newline,
            TokenKind::Eof
        ]
    );
}

#[test]
fn test_nested_comments() {
    // Nested block comments should work
    let kinds = lex_kinds("x /* outer /* inner */ still outer */ y");
    
    assert_eq!(
        kinds,
        vec![
            TokenKind::Identifier("x".to_string()),
            TokenKind::Identifier("y".to_string()),
            TokenKind::Newline,
            TokenKind::Eof
        ]
    );
}

#[test]
fn test_comment_with_code_after() {
    let kinds = lex_kinds("x // comment\ny // another");
    
    assert_eq!(
        kinds,
        vec![
            TokenKind::Identifier("x".to_string()),
            TokenKind::Identifier("y".to_string()),
            TokenKind::Newline,
            TokenKind::Eof
        ]
    );
}

