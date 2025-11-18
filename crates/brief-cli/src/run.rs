use std::path::Path;
use std::rc::Rc;
use brief_lexer::lex;
use brief_parser::parse;
use brief_hir::{lower, emit_bytecode};
use brief_vm::VM;
use brief_runtime::Runtime;
use brief_diagnostic::FileId;
use crate::error::{CliError, ExitCode};

/// Run a Brief source file
pub fn run_file(path: &Path) -> Result<ExitCode, CliError> {
    // 1. Read file
    let source = std::fs::read_to_string(path)?;
    let file_id = FileId(0); // For now, use a single file ID
    
    // 2. Lex
    let (tokens, lex_errors) = lex(&source, file_id);
    if !lex_errors.is_empty() {
        eprintln!("Lexical errors:");
        for err in &lex_errors {
            eprintln!("  {:?}", err);
        }
        return Ok(ExitCode::CompileError);
    }
    
    // 3. Parse
    let (program, parse_errors) = parse(tokens, file_id);
    if !parse_errors.is_empty() {
        eprintln!("Parse errors:");
        for err in &parse_errors {
            eprintln!("  {:?}", err);
        }
        return Ok(ExitCode::CompileError);
    }
    
    // 4. Lower to HIR
    let hir_program = match lower(program) {
        Ok(hir) => hir,
        Err(errors) => {
            eprintln!("HIR errors:");
            for err in &errors {
                eprintln!("  {:?}", err);
            }
            return Ok(ExitCode::CompileError);
        }
    };
    
    // 5. Emit bytecode
    let chunks = emit_bytecode(&hir_program);
    
    if chunks.is_empty() {
        // No functions to execute - this is OK for empty programs
        return Ok(ExitCode::Success);
    }
    
    // 6. Create VM with runtime
    let mut vm = VM::new();
    let runtime = Runtime::new();
    vm.set_runtime(Box::new(runtime));
    
    // 7. Execute chunks
    // For now, execute the first chunk (main function)
    // TODO: Find and execute main function properly
    let main_chunk = Rc::new(chunks[0].clone());
    vm.push_frame(main_chunk, 0);
    
    // 8. Run VM
    match vm.run() {
        Ok(_) => Ok(ExitCode::Success),
        Err(e) => {
            eprintln!("Runtime error: {}", e);
            Ok(ExitCode::RuntimeError)
        }
    }
}

