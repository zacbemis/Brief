pub mod hir;
pub mod symbol;
pub mod desugar;
pub mod resolve;
pub mod error;
pub mod emit;

pub use hir::*;
pub use symbol::*;
pub use error::*;

use brief_ast::Program;

/// Convert AST to HIR by desugaring and resolving names
pub fn lower(program: Program) -> Result<HirProgram, Vec<HirError>> {
    // First desugar
    let mut hir_program = desugar::desugar(program);
    
    // Then resolve names
    resolve::resolve(&mut hir_program)?;
    
    Ok(hir_program)
}

/// Convert HIR to bytecode chunks
pub fn emit_bytecode(program: &HirProgram) -> Vec<brief_bytecode::Chunk> {
    emit::emit(program)
}

