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

fn lex_errors(source: &str) -> Vec<String> {
    let (_tokens, errors) = lex(source, FileId(0));
    errors
}

#[test]
fn test_empty_source() {
    let kinds = lex_kinds("");
    
    assert_eq!(
        kinds,
        vec![
            TokenKind::Newline,
            TokenKind::Eof
        ]
    );
}

#[test]
fn test_whitespace_only() {
    let kinds = lex_kinds("   \t  ");
    
    assert_eq!(
        kinds,
        vec![
            TokenKind::Newline,
            TokenKind::Eof
        ]
    );
}

#[test]
fn test_unterminated_string() {
    let errors = lex_errors("\"hello");
    
    assert!(!errors.is_empty());
    assert!(errors[0].contains("unterminated"));
}

#[test]
fn test_unterminated_character() {
    let errors = lex_errors("'a");
    
    assert!(!errors.is_empty());
}

#[test]
fn test_unterminated_block_comment() {
    // This should not error (just continue), but let's test it doesn't break
    let kinds = lex_kinds("x /* unclosed");
    
    // Should at least get x
    assert!(kinds.contains(&TokenKind::Identifier("x".to_string())));
}

#[test]
fn test_spaces_in_indentation() {
    let errors = lex_errors("\tx\n y");
    
    // Should error about spaces in indentation
    assert!(!errors.is_empty());
    assert!(errors.iter().any(|e| e.contains("spaces") || e.contains("indentation")));
}

#[test]
fn test_mixed_operators() {
    let kinds = lex_kinds("x += 1");
    
    assert_eq!(
        kinds,
        vec![
            TokenKind::Identifier("x".to_string()),
            TokenKind::PlusAssign,
            TokenKind::Integer(1),
            TokenKind::Newline,
            TokenKind::Eof
        ]
    );
}

#[test]
fn test_complex_expression() {
    let kinds = lex_kinds("x = y + z * 2");
    
    assert_eq!(
        kinds,
        vec![
            TokenKind::Identifier("x".to_string()),
            TokenKind::Assign,
            TokenKind::Identifier("y".to_string()),
            TokenKind::Plus,
            TokenKind::Identifier("z".to_string()),
            TokenKind::Star,
            TokenKind::Integer(2),
            TokenKind::Newline,
            TokenKind::Eof
        ]
    );
}

#[test]
fn test_string_with_quotes() {
    let tokens = lex_tokens("\"He said \\\"hello\\\"\"");
    
    if let TokenKind::StrPart(s) = &tokens[0].kind {
        assert!(s.contains('"'));
    } else {
        panic!("Expected StrPart");
    }
}

#[test]
fn test_multiple_statements() {
    let kinds = lex_kinds("int x\nx = 1\nx += 2");
    
    assert_eq!(
        kinds,
        vec![
            TokenKind::Int,
            TokenKind::Identifier("x".to_string()),
            TokenKind::Newline,
            TokenKind::Identifier("x".to_string()),
            TokenKind::Assign,
            TokenKind::Integer(1),
            TokenKind::Newline,
            TokenKind::Identifier("x".to_string()),
            TokenKind::PlusAssign,
            TokenKind::Integer(2),
            TokenKind::Newline,
            TokenKind::Eof
        ]
    );
}

#[test]
fn test_real_world_example() {
    let source = "int x\nx := 10\nif (x > 5)\n\tret x";
    let kinds = lex_kinds(source);
    
    assert!(kinds.contains(&TokenKind::Int));
    assert!(kinds.contains(&TokenKind::InitAssign));
    assert!(kinds.contains(&TokenKind::Integer(10)));
    assert!(kinds.contains(&TokenKind::If));
    assert!(kinds.contains(&TokenKind::Gt));
    assert!(kinds.contains(&TokenKind::Integer(5)));
    assert!(kinds.contains(&TokenKind::Indent));
    assert!(kinds.contains(&TokenKind::Ret));
}

