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
fn test_simple_character() {
    let kinds = lex_kinds("'a'");
    
    assert_eq!(
        kinds,
        vec![
            TokenKind::Character('a'),
            TokenKind::Newline,
            TokenKind::Eof
        ]
    );
}

#[test]
fn test_character_escape_sequences() {
    let tokens = lex_tokens("'\\n' '\\t' '\\r' '\\\\' '\\'' '\\\"' '\\0'");
    
    // Filter to only character tokens (skip spaces/newlines)
    let chars: Vec<char> = tokens
        .iter()
        .filter_map(|t| {
            if let TokenKind::Character(c) = t.kind {
                Some(c)
            } else {
                None
            }
        })
        .collect();
    
    assert_eq!(chars, vec!['\n', '\t', '\r', '\\', '\'', '"', '\0']);
}

#[test]
fn test_character_unicode() {
    let tokens = lex_tokens("'\\u{1F600}'");  // 😀 emoji
    
    if let TokenKind::Character(ch) = tokens[0].kind {
        assert_eq!(ch, '😀');
    } else {
        panic!("Expected Character token");
    }
}

#[test]
fn test_character_unicode_hex() {
    let tokens = lex_tokens("'\\u{0041}'");  // 'A'
    
    assert_eq!(tokens[0].kind, TokenKind::Character('A'));
}

#[test]
fn test_multiple_characters() {
    let kinds = lex_kinds("'a' 'b' 'c'");
    
    assert_eq!(
        kinds,
        vec![
            TokenKind::Character('a'),
            TokenKind::Character('b'),
            TokenKind::Character('c'),
            TokenKind::Newline,
            TokenKind::Eof
        ]
    );
}

