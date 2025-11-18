mod common;

use brief_ast::*;
use common::*;

/// Test that lexer and parser work together correctly
#[test]
fn test_lexer_parser_integration() {
    let source = "int x := 42\nx++\ndef add(int a, int b) -> int\n\tret a + b";
    let program = parse_source(source);
    
    // Should have 3 declarations: var, expr stmt, function
    assert!(program.declarations.len() >= 2);
    
    // First should be variable declaration
    match &program.declarations[0] {
        Decl::VarDecl(v) => {
            assert_eq!(v.name, "x");
        }
        _ => panic!("Expected variable declaration"),
    }
}

#[test]
fn test_string_interpolation_integration() {
    let source = "x := \"Hello &name, you are &age years old\"";
    let program = parse_source(source);
    
    match &program.declarations[0] {
        Decl::VarDecl(v) => {
            match &v.initializer {
                Some(Expr::Interpolation { parts, .. }) => {
                    assert!(!parts.is_empty());
                }
                _ => panic!("Expected string interpolation"),
            }
        }
        _ => panic!("Expected variable declaration"),
    }
}

#[test]
fn test_indentation_integration() {
    let source = "if (true)\n\tx := 1\n\tif (false)\n\t\ty := 2";
    let program = parse_source(source);
    // Should parse without errors
    assert!(!program.declarations.is_empty());
}

#[test]
fn test_complex_expression_integration() {
    let source = "x := (1 + 2) * 3 - 4 / 2";
    let program = parse_source(source);
    
    match &program.declarations[0] {
        Decl::VarDecl(v) => {
            assert!(v.initializer.is_some());
        }
        _ => panic!("Expected variable declaration"),
    }
}

#[test]
fn test_class_integration() {
    let source = "cls Dog\n\tobj Dog(name)\n\tdef bark()\n\t\tprint(\"woof\")";
    let program = parse_source(source);
    
    match &program.declarations[0] {
        Decl::ClassDecl(c) => {
            assert_eq!(c.name, "Dog");
        }
        _ => panic!("Expected class declaration"),
    }
}

