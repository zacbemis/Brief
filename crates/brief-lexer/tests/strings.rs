use brief_lexer::{lex, Token, TokenKind};
use brief_diagnostic::FileId;

fn lex_kinds(source: &str) -> Vec<TokenKind> {
    let (tokens, _errors) = lex(source, FileId(0));
    tokens.into_iter().map(|t| t.kind).collect()
}

fn lex_tokens(source: &str) -> Vec<Token> {
    let (tokens, _errors) = lex(source, FileId(0));
    tokens
}

#[test]
fn test_simple_string() {
    let kinds = lex_kinds("\"hello\"");
    
    assert_eq!(
        kinds,
        vec![
            TokenKind::StrPart("hello".to_string()),
            TokenKind::Newline,
            TokenKind::Eof
        ]
    );
}

#[test]
fn test_empty_string() {
    let kinds = lex_kinds("\"\"");
    
    assert_eq!(
        kinds,
        vec![
            TokenKind::StrPart("".to_string()),
            TokenKind::Newline,
            TokenKind::Eof
        ]
    );
}

#[test]
fn test_string_with_interpolation_simple() {
    let kinds = lex_kinds("\"Hello &name!\"");
    
    assert_eq!(
        kinds,
        vec![
            TokenKind::StrPart("Hello ".to_string()),
            TokenKind::InterpIdent("name".to_string()),
            TokenKind::StrPart("!".to_string()),
            TokenKind::Newline,
            TokenKind::Eof
        ]
    );
}

#[test]
fn test_string_with_interpolation_path() {
    let kinds = lex_kinds("\"Hello &obj.name!\"");
    
    assert_eq!(
        kinds,
        vec![
            TokenKind::StrPart("Hello ".to_string()),
            TokenKind::InterpPath("obj.name".to_string()),
            TokenKind::StrPart("!".to_string()),
            TokenKind::Newline,
            TokenKind::Eof
        ]
    );
}

#[test]
fn test_string_with_multiple_interpolations() {
    let kinds = lex_kinds("\"&name is &age years old\"");
    
    assert_eq!(
        kinds,
        vec![
            TokenKind::StrPart("".to_string()),
            TokenKind::InterpIdent("name".to_string()),
            TokenKind::StrPart(" is ".to_string()),
            TokenKind::InterpIdent("age".to_string()),
            TokenKind::StrPart(" years old".to_string()),
            TokenKind::Newline,
            TokenKind::Eof
        ]
    );
}

#[test]
fn test_string_with_escaped_ampersand() {
    let kinds = lex_kinds("\"Hello && world\"");
    
    assert_eq!(
        kinds,
        vec![
            TokenKind::StrPart("Hello & world".to_string()),
            TokenKind::Newline,
            TokenKind::Eof
        ]
    );
}

#[test]
fn test_string_escape_sequences() {
    // The escape sequences should be processed
    let tokens = lex_tokens("\"Hello\\nWorld\\tTab\"");
    if let TokenKind::StrPart(s) = &tokens[0].kind {
        assert!(s.contains('\n'));
        assert!(s.contains('\t'));
    } else {
        panic!("Expected StrPart");
    }
}

#[test]
fn test_string_interpolation_at_start() {
    let kinds = lex_kinds("\"&name here\"");
    
    assert_eq!(
        kinds,
        vec![
            TokenKind::StrPart("".to_string()),
            TokenKind::InterpIdent("name".to_string()),
            TokenKind::StrPart(" here".to_string()),
            TokenKind::Newline,
            TokenKind::Eof
        ]
    );
}

#[test]
fn test_string_interpolation_at_end() {
    let kinds = lex_kinds("\"Hello &name\"");
    
    assert_eq!(
        kinds,
        vec![
            TokenKind::StrPart("Hello ".to_string()),
            TokenKind::InterpIdent("name".to_string()),
            TokenKind::StrPart("".to_string()),
            TokenKind::Newline,
            TokenKind::Eof
        ]
    );
}

#[test]
fn test_string_only_interpolation() {
    let kinds = lex_kinds("\"&name\"");
    
    assert_eq!(
        kinds,
        vec![
            TokenKind::StrPart("".to_string()),
            TokenKind::InterpIdent("name".to_string()),
            TokenKind::StrPart("".to_string()),
            TokenKind::Newline,
            TokenKind::Eof
        ]
    );
}

#[test]
fn test_string_complex_interpolation() {
    let kinds = lex_kinds("\"&obj.field.method()\"");
    
    // This should be parsed as InterpPath with the full path
    assert_eq!(
        kinds,
        vec![
            TokenKind::StrPart("".to_string()),
            TokenKind::InterpPath("obj.field.method()".to_string()),  // Note: includes parens
            TokenKind::StrPart("".to_string()),
            TokenKind::Newline,
            TokenKind::Eof
        ]
    );
}

