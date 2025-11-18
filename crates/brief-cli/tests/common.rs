use brief_lexer::lex;
use brief_parser::parse;
use brief_hir::{lower, emit_bytecode};
use brief_vm::VM;
use brief_runtime::Runtime;
use brief_diagnostic::FileId;
use std::rc::Rc;

pub fn run_code(source: &str) -> Result<brief_vm::Value, String> {
    let file_id = FileId(0);

    let (tokens, lex_errors) = lex(source, file_id);
    if !lex_errors.is_empty() {
        return Err(format!("Lex errors: {:?}", lex_errors));
    }

    let (program, parse_errors) = parse(tokens, file_id);
    if !parse_errors.is_empty() {
        return Err(format!("Parse errors: {:?}", parse_errors));
    }

    let hir_program = match lower(program) {
        Ok(hir) => hir,
        Err(errors) => {
            return Err(format!("HIR errors: {:?}", errors));
        }
    };

    let chunks = emit_bytecode(&hir_program);
    if std::env::var("BRIEF_DEBUG_CHUNK").is_ok() {
        for (idx, chunk) in chunks.iter().enumerate() {
            eprintln!("Emitted chunk #{} - {} (max_regs={})", idx, chunk.name, chunk.max_regs);
            eprintln!("  constants:");
            for (cidx, constant) in chunk.constants.iter().enumerate() {
                eprintln!("    [{}] {:?}", cidx, constant);
            }
            for (ip, instr) in chunk.code.iter().enumerate() {
                eprintln!("  {:04}: {}", ip, instr);
            }
        }
    }
    if chunks.is_empty() {
        return Ok(brief_vm::Value::Null);
    }

    let mut vm = VM::new();
    let runtime = Runtime::new();
    vm.set_runtime(Box::new(runtime));

    let main_chunk = Rc::new(chunks[0].clone());
    vm.push_frame(main_chunk, 0);

    match vm.run() {
        Ok(value) => {
            if std::env::var("BRIEF_TRACE_RESULT").is_ok() {
                eprintln!("VM result: {:?}", value);
            }
            Ok(value)
        },
        Err(e) => {
            eprintln!("Runtime error: {:?}", e);
            Err(format!("Runtime error: {:?} | chunks: {:?}", e, chunks))
        }
    }
}

