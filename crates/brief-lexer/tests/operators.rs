use brief_lexer::{lex, TokenKind};
use brief_diagnostic::FileId;

fn lex_kinds(source: &str) -> Vec<TokenKind> {
    let (tokens, _errors) = lex(source, FileId(0));
    tokens.into_iter().map(|t| t.kind).collect()
}

#[test]
fn test_arithmetic_operators() {
    let kinds = lex_kinds("+ - * / %");
    
    assert_eq!(
        kinds,
        vec![
            TokenKind::Plus,
            TokenKind::Minus,
            TokenKind::Star,
            TokenKind::Slash,
            TokenKind::Percent,
            TokenKind::Newline,
            TokenKind::Eof
        ]
    );
}

#[test]
fn test_power_operator() {
    let kinds = lex_kinds("**");
    
    assert_eq!(
        kinds,
        vec![
            TokenKind::Pow,
            TokenKind::Newline,
            TokenKind::Eof
        ]
    );
}

#[test]
fn test_assignment_operators() {
    let kinds = lex_kinds("= := += -= *= /= %= **=");
    
    assert_eq!(
        kinds,
        vec![
            TokenKind::Assign,
            TokenKind::InitAssign,
            TokenKind::PlusAssign,
            TokenKind::MinusAssign,
            TokenKind::StarAssign,
            TokenKind::SlashAssign,
            TokenKind::PercentAssign,
            TokenKind::PowAssign,
            TokenKind::Newline,
            TokenKind::Eof
        ]
    );
}

#[test]
fn test_increment_decrement() {
    let kinds = lex_kinds("++ --");
    
    assert_eq!(
        kinds,
        vec![
            TokenKind::Inc,
            TokenKind::Dec,
            TokenKind::Newline,
            TokenKind::Eof
        ]
    );
}

#[test]
fn test_comparison_operators() {
    let kinds = lex_kinds("== != < <= > >=");
    
    assert_eq!(
        kinds,
        vec![
            TokenKind::Eq,
            TokenKind::Ne,
            TokenKind::Lt,
            TokenKind::Le,
            TokenKind::Gt,
            TokenKind::Ge,
            TokenKind::Newline,
            TokenKind::Eof
        ]
    );
}

#[test]
fn test_boolean_operators() {
    let kinds = lex_kinds("! && ||");
    
    assert_eq!(
        kinds,
        vec![
            TokenKind::Not,
            TokenKind::And,
            TokenKind::Or,
            TokenKind::Newline,
            TokenKind::Eof
        ]
    );
}

#[test]
fn test_bitwise_operators() {
    let kinds = lex_kinds(">> << & | ^ ~");
    
    assert_eq!(
        kinds,
        vec![
            TokenKind::Shr,
            TokenKind::Shl,
            TokenKind::BitAnd,
            TokenKind::BitOr,
            TokenKind::BitXor,
            TokenKind::BitNot,
            TokenKind::Newline,
            TokenKind::Eof
        ]
    );
}

#[test]
fn test_ternary_operator() {
    let kinds = lex_kinds("? :");
    
    assert_eq!(
        kinds,
        vec![
            TokenKind::Question,
            TokenKind::Colon,
            TokenKind::Newline,
            TokenKind::Eof
        ]
    );
}

#[test]
fn test_arrow_operator() {
    let kinds = lex_kinds("->");
    
    assert_eq!(
        kinds,
        vec![
            TokenKind::Arrow,
            TokenKind::Newline,
            TokenKind::Eof
        ]
    );
}

#[test]
fn test_operator_precedence_ambiguity() {
    // Test that operators are tokenized correctly even when ambiguous
    let kinds = lex_kinds("x++ y--");
    
    assert_eq!(
        kinds,
        vec![
            TokenKind::Identifier("x".to_string()),
            TokenKind::Inc,
            TokenKind::Identifier("y".to_string()),
            TokenKind::Dec,
            TokenKind::Newline,
            TokenKind::Eof
        ]
    );
}

#[test]
fn test_star_vs_pow() {
    // * should not be confused with **
    let kinds = lex_kinds("* **");
    
    assert_eq!(
        kinds,
        vec![
            TokenKind::Star,
            TokenKind::Pow,
            TokenKind::Newline,
            TokenKind::Eof
        ]
    );
}

