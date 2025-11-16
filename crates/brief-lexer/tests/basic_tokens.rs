use brief_lexer::{lex, TokenKind};
use brief_diagnostic::FileId;

fn lex_kinds(source: &str) -> Vec<TokenKind> {
    let (tokens, _errors) = lex(source, FileId(0));
    tokens.into_iter().map(|t| t.kind).collect()
}

#[test]
fn test_keywords() {
    let kinds = lex_kinds("int char str dub bool if else while for in break continue match case def ret cls obj const null true false");
    
    assert_eq!(
        kinds,
        vec![
            TokenKind::Int, TokenKind::Char, TokenKind::Str, TokenKind::Dub, TokenKind::Bool,
            TokenKind::If, TokenKind::Else, TokenKind::While, TokenKind::For, TokenKind::In,
            TokenKind::Break, TokenKind::Continue, TokenKind::Match, TokenKind::Case,
            TokenKind::Def, TokenKind::Ret, TokenKind::Cls, TokenKind::Obj, TokenKind::Const,
            TokenKind::Null, TokenKind::True, TokenKind::False,
            TokenKind::Newline, TokenKind::Eof
        ]
    );
}

#[test]
fn test_punctuation() {
    let kinds = lex_kinds("()[]{},;.->");
    
    assert_eq!(
        kinds,
        vec![
            TokenKind::LeftParen, TokenKind::RightParen,
            TokenKind::LeftBracket, TokenKind::RightBracket,
            TokenKind::LeftBrace, TokenKind::RightBrace,
            TokenKind::Comma, TokenKind::Semicolon,
            TokenKind::Dot, TokenKind::Arrow,
            TokenKind::Newline, TokenKind::Eof
        ]
    );
}

#[test]
fn test_special_tokens() {
    let kinds = lex_kinds("x\ny");
    
    // Should have newline between x and y
    let expected: Vec<TokenKind> = vec![
        TokenKind::Identifier("x".to_string()),
        TokenKind::Newline,
        TokenKind::Identifier("y".to_string()),
        TokenKind::Newline,
        TokenKind::Eof
    ];
    
    assert_eq!(kinds, expected);
}

