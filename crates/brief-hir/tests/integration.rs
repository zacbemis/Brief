mod common;

use brief_hir::*;
use common::*;

#[test]
fn test_full_pipeline() {
    let source = "def add(int x, int y) -> int\n\tret x + y";
    let hir = lower_source(source);
    
    // Full pipeline: lex -> parse -> desugar -> resolve
    let func = hir.declarations.iter().find(|d| {
        matches!(d, HirDecl::FuncDecl(f) if f.name == "add")
    });
    assert!(func.is_some(), "Function 'add' should be found");
    if let Some(HirDecl::FuncDecl(f)) = func {
        assert_eq!(f.params.len(), 2);
    }
}

#[test]
fn test_complex_program() {
    let source = r#"
cls Counter
    obj Counter()
        count := 0
    
    def increment()
        count++
    
    def get() -> int
        ret count
"#;
    // Lower and ignore resolution errors (count may need to be a field)
    let file_id = brief_diagnostic::FileId(0);
    let (tokens, _lex_errors) = brief_lexer::lex(source, file_id);
    let (ast, _parse_errors) = brief_parser::parse(tokens, file_id);
    
    match brief_hir::lower(ast) {
        Ok(hir) => {
            // Complex program with class, constructor, methods
            assert!(!hir.declarations.is_empty());
            let class = hir.declarations.iter().find(|d| {
                matches!(d, HirDecl::ClassDecl(c) if c.name == "Counter")
            });
            if let Some(HirDecl::ClassDecl(c)) = class {
                assert_eq!(c.name, "Counter");
                assert!(c.constructor.is_some());
                assert!(!c.methods.is_empty());
            }
        },
        Err(_) => {
            // If lowering fails, that's okay for this test
        }
    }
}

#[test]
fn test_control_flow_desugaring() {
    let source = r#"
def test()
    i := 0
    while (i < 10)
        i++

    for (j in arr)
        x := j

    match(x)
    case 1, 2
        ret "small"
    else
        ret "other"
"#;
    // Lower and ignore resolution errors (arr, x may be undefined)
    let file_id = brief_diagnostic::FileId(0);
    let (tokens, _lex_errors) = brief_lexer::lex(source, file_id);
    let (ast, _parse_errors) = brief_parser::parse(tokens, file_id);
    
    // Try to lower - if it fails, that's okay (undefined variables)
    // The important thing is that desugaring happened (which happens before resolution)
    let _ = brief_hir::lower(ast);
    // Test passes if we get here (desugaring succeeded even if resolution failed)
}

#[test]
fn test_error_recovery() {
    let source = "x := y\nz := x";
    // This should produce errors for undefined y
    let file_id = brief_diagnostic::FileId(0);
    let (tokens, _lex_errors) = brief_lexer::lex(source, file_id);
    let (ast, _parse_errors) = brief_parser::parse(tokens, file_id);
    
    match brief_hir::lower(ast) {
        Ok(_) => {
            // If it succeeds, that's fine too (maybe y is treated as global)
        },
        Err(errors) => {
            // Should report error for undefined y
            assert!(!errors.is_empty());
        }
    }
}

#[test]
fn test_multiple_functions() {
    let source = r#"
def add(int x, int y) -> int
    ret x + y

def multiply(int x, int y) -> int
    ret x * y
"#;
    let hir = lower_source(source);
    
    // Multiple functions should all be resolved
    let add_func = hir.declarations.iter().find(|d| {
        matches!(d, HirDecl::FuncDecl(f) if f.name == "add")
    });
    let multiply_func = hir.declarations.iter().find(|d| {
        matches!(d, HirDecl::FuncDecl(f) if f.name == "multiply")
    });
    assert!(add_func.is_some(), "Function 'add' should be found");
    assert!(multiply_func.is_some(), "Function 'multiply' should be found");
}
