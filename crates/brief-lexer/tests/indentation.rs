use brief_lexer::{lex, TokenKind};
use brief_diagnostic::FileId;

fn lex_kinds(source: &str) -> Vec<TokenKind> {
    let (tokens, _errors) = lex(source, FileId(0));
    tokens.into_iter().map(|t| t.kind).collect()
}

#[test]
fn test_simple_indent() {
    let source = "x\ty";
    let kinds = lex_kinds(source);
    
    assert_eq!(
        kinds,
        vec![
            TokenKind::Identifier("x".to_string()),
            TokenKind::Newline,
            TokenKind::Indent,
            TokenKind::Identifier("y".to_string()),
            TokenKind::Newline,
            TokenKind::Dedent,
            TokenKind::Eof
        ]
    );
}

#[test]
fn test_nested_indent() {
    let source = "x\ty\tz";
    let kinds = lex_kinds(source);
    
    assert_eq!(
        kinds,
        vec![
            TokenKind::Identifier("x".to_string()),
            TokenKind::Newline,
            TokenKind::Indent,
            TokenKind::Identifier("y".to_string()),
            TokenKind::Newline,
            TokenKind::Indent,
            TokenKind::Identifier("z".to_string()),
            TokenKind::Newline,
            TokenKind::Dedent,
            TokenKind::Dedent,
            TokenKind::Eof
        ]
    );
}

#[test]
fn test_dedent() {
    let source = "\tx\ty\nz";
    let kinds = lex_kinds(source);
    
    assert_eq!(
        kinds,
        vec![
            TokenKind::Indent,
            TokenKind::Identifier("x".to_string()),
            TokenKind::Newline,
            TokenKind::Indent,
            TokenKind::Identifier("y".to_string()),
            TokenKind::Newline,
            TokenKind::Dedent,
            TokenKind::Dedent,
            TokenKind::Identifier("z".to_string()),
            TokenKind::Newline,
            TokenKind::Eof
        ]
    );
}

#[test]
fn test_empty_lines_with_indent() {
    let source = "x\n\ty";
    let kinds = lex_kinds(source);
    
    assert_eq!(
        kinds,
        vec![
            TokenKind::Identifier("x".to_string()),
            TokenKind::Newline,
            TokenKind::Indent,
            TokenKind::Identifier("y".to_string()),
            TokenKind::Newline,
            TokenKind::Dedent,
            TokenKind::Eof
        ]
    );
}

#[test]
fn test_comment_lines_ignored_for_indent() {
    let source = "x\n\t// comment\ty";
    let kinds = lex_kinds(source);
    
    // Comment line should be ignored, so y should be at same indent as comment
    assert_eq!(
        kinds,
        vec![
            TokenKind::Identifier("x".to_string()),
            TokenKind::Newline,
            TokenKind::Indent,
            TokenKind::Identifier("y".to_string()),
            TokenKind::Newline,
            TokenKind::Dedent,
            TokenKind::Eof
        ]
    );
}

#[test]
fn test_multiple_dedents() {
    let source = "\t\tx\n\ty\nz";
    let kinds = lex_kinds(source);
    
    assert_eq!(
        kinds,
        vec![
            TokenKind::Indent,
            TokenKind::Indent,
            TokenKind::Identifier("x".to_string()),
            TokenKind::Newline,
            TokenKind::Dedent,
            TokenKind::Identifier("y".to_string()),
            TokenKind::Newline,
            TokenKind::Dedent,
            TokenKind::Identifier("z".to_string()),
            TokenKind::Newline,
            TokenKind::Eof
        ]
    );
}

#[test]
fn test_no_indent_on_same_level() {
    let source = "\tx\n\ty";
    let kinds = lex_kinds(source);
    
    assert_eq!(
        kinds,
        vec![
            TokenKind::Indent,
            TokenKind::Identifier("x".to_string()),
            TokenKind::Newline,
            TokenKind::Identifier("y".to_string()),
            TokenKind::Newline,
            TokenKind::Dedent,
            TokenKind::Eof
        ]
    );
}

