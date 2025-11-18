use brief_lexer::lex;
use brief_parser::parse;
use brief_hir::{lower, emit_bytecode};
use brief_diagnostic::FileId;

fn emit_source(source: &str) -> Vec<brief_bytecode::Chunk> {
    let file_id = FileId(0);
    let (tokens, _lex_errors) = lex(source, file_id);
    let (ast, _parse_errors) = parse(tokens, file_id);
    let hir = lower(ast).unwrap_or_else(|errors| {
        eprintln!("HIR lowering errors: {:?}", errors);
        panic!("HIR lowering failed");
    });
    emit_bytecode(&hir)
}

#[test]
fn test_emit_simple_function() {
    let source = r#"
def test():
    return 42
"#;
    let chunks = emit_source(source);
    assert_eq!(chunks.len(), 1);
    let chunk = &chunks[0];
    assert_eq!(chunk.name, "test");
    assert_eq!(chunk.param_count, 0);
    assert!(!chunk.code.is_empty());
}

#[test]
fn test_emit_literals() {
    let source = r#"
def test():
    x := 42
    y := 3.14
    z := true
    s := "hello"
"#;
    let chunks = emit_source(source);
    assert_eq!(chunks.len(), 1);
    let chunk = &chunks[0];
    // Should have constants for literals
    assert!(!chunk.constants.is_empty());
}

#[test]
fn test_emit_arithmetic() {
    let source = r#"
def test():
    x := 1 + 2
    y := 3 * 4
    z := 10 - 5
"#;
    let chunks = emit_source(source);
    assert_eq!(chunks.len(), 1);
    let chunk = &chunks[0];
    // Should have ADD, MUL, SUB instructions
    assert!(!chunk.code.is_empty());
}

#[test]
fn test_emit_if_statement() {
    let source = r#"
def test():
    if true:
        x := 1
    else:
        y := 2
    return 0
"#;
    let chunks = emit_source(source);
    assert_eq!(chunks.len(), 1);
    let chunk = &chunks[0];
    // Should have JIF and JMP instructions
    assert!(!chunk.code.is_empty());
}

#[test]
fn test_emit_while_loop() {
    let source = r#"
def test():
    while true:
        x := 1
"#;
    let chunks = emit_source(source);
    assert_eq!(chunks.len(), 1);
    let chunk = &chunks[0];
    // Should have JIF and JMP for loop
    assert!(!chunk.code.is_empty());
}

#[test]
fn test_emit_function_with_params() {
    let source = r#"
def add(a, b):
    return a + b
"#;
    let chunks = emit_source(source);
    assert_eq!(chunks.len(), 1);
    let chunk = &chunks[0];
    assert_eq!(chunk.name, "add");
    assert_eq!(chunk.param_count, 2);
}

#[test]
fn test_emit_multiple_functions() {
    let source = r#"
def func1():
    x := 1

def func2():
    y := 2
"#;
    let chunks = emit_source(source);
    assert_eq!(chunks.len(), 2);
    assert_eq!(chunks[0].name, "func1");
    assert_eq!(chunks[1].name, "func2");
}

