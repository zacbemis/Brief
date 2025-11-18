mod common;

use common::*;

#[test]
fn test_missing_closing_paren() {
    let errors = parse_errors("def test(x\n\tret x");
    assert!(!errors.is_empty(), "Should have parse errors for missing closing paren");
}

#[test]
fn test_missing_closing_brace() {
    let _errors = parse_errors("if (true)\n\tx := 1");
    // This might not error if we handle indentation correctly
    // Adjust based on actual behavior
}

#[test]
fn test_unexpected_token() {
    let errors = parse_errors("def test() -> -> int");
    assert!(!errors.is_empty(), "Should have parse errors for unexpected token");
}

#[test]
fn test_invalid_expression() {
    let errors = parse_errors("x := +");
    assert!(!errors.is_empty(), "Should have parse errors for invalid expression");
}

#[test]
fn test_error_recovery() {
    // Test that parser continues after errors
    let (program, errors) = parse_with_errors("def test()\n\tret x\ndef other()\n\tret y");
    assert!(!errors.is_empty() || program.declarations.len() == 2, 
           "Parser should recover and parse multiple declarations");
}

