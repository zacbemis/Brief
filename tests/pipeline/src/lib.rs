use anyhow::Result;
use brief_diagnostic::FileId;
use brief_lexer::lex;
use brief_parser::parse;
use brief_hir::{lower, emit_bytecode};
use brief_vm::VM;
use brief_runtime::Runtime;
use std::rc::Rc;

pub fn run_source(source: &str) -> Result<()> {
    let file_id = FileId(0);
    let (tokens, lex_errors) = lex(source, file_id);
    if !lex_errors.is_empty() {
        anyhow::bail!("Lex errors: {:?}", lex_errors);
    }

    let (program, parse_errors) = parse(tokens, file_id);
    if !parse_errors.is_empty() {
        anyhow::bail!("Parse errors: {:?}", parse_errors);
    }

    let hir = lower(program).map_err(|errs| anyhow::anyhow!("HIR errors: {:?}", errs))?;
    let chunks = emit_bytecode(&hir);
    if chunks.is_empty() {
        return Ok(());
    }

    let mut vm = VM::new();
    vm.set_runtime(Box::new(Runtime::new()));
    let chunk = Rc::new(chunks[0].clone());
    vm.push_frame(chunk, 0);
    vm.run().map(|_| ())?;
    Ok(())
}

