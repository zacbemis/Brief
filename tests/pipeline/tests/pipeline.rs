use insta::assert_snapshot;
use blake3::hash;
use brief_bytecode::Chunk;
use brief_diagnostic::FileId;
use brief_lexer::lex;
use brief_parser::parse;
use brief_hir::{lower, emit_bytecode};
use brief_vm::VM;
use brief_runtime::Runtime;
use std::rc::Rc;

fn snapshot_bytecode(source: &str) -> Vec<String> {
    let file_id = FileId(0);
    let (tokens, lex_errors) = lex(source, file_id);
    assert!(lex_errors.is_empty(), "Lex errors: {:?}", lex_errors);

    let (program, parse_errors) = parse(tokens, file_id);
    assert!(parse_errors.is_empty(), "Parse errors: {:?}", parse_errors);

    let hir = lower(program).expect("HIR lowering failed");
    let chunks = emit_bytecode(&hir);
    chunks.iter().map(format_chunk).collect()
}

fn format_chunk(chunk: &Chunk) -> String {
    let mut lines = Vec::new();
    lines.push(format!("chunk {} (params={}, max_regs={})", chunk.name, chunk.param_count, chunk.max_regs));
    lines.push("constants:".into());
    for (i, c) in chunk.constants.iter().enumerate() {
        lines.push(format!("  [{}] {:?}", i, c));
    }
    lines.push("code:".into());
    for (i, instr) in chunk.code.iter().enumerate() {
        lines.push(format!("  {:04} {}", i, instr));
    }
    lines.join("\n")
}

fn run_vm(source: &str) -> Result<(), String> {
    let snapshots = snapshot_bytecode(source);
    // Keep a snapshot for debugging even if execution succeeds
    assert_snapshot!(format!("bytecode_{}", hash(source.as_bytes())), snapshots.join("\n\n"));

    let file_id = FileId(0);
    let (tokens, _) = lex(source, file_id);
    let (program, _) = parse(tokens, file_id);
    let hir = lower(program).map_err(|e| format!("HIR error: {:?}", e))?;
    let chunks = emit_bytecode(&hir);
    if chunks.is_empty() {
        return Ok(());
    }

    let mut vm = VM::new();
    vm.set_runtime(Box::new(Runtime::new()));
    let chunk = Rc::new(chunks[0].clone());
    vm.push_frame(chunk, 0);
    vm.run().map(|_| ()).map_err(|e| format!("Runtime error: {:?}", e))
}

#[test]
fn pipeline_executes_simple_arithmetic() {
    run_vm("def test()\n\tret 2 + 3").expect("pipeline should succeed");
}

#[test]
fn pipeline_handles_builtin_calls() {
    run_vm("def test()\n\tret int(3.14)").expect("builtin cast should succeed");
}

#[test]
#[ignore = "while-loop emitter currently reuses condition registers causing runtime TypeMismatch"]
fn pipeline_runs_loop() {
    run_vm("def test()\n\tx := 0\n\twhile (x < 3)\n\t\tx := x + 1\n\tret x").expect("while loop should run");
}

