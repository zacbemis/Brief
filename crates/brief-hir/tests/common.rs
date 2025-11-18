use brief_lexer::lex;
use brief_parser::parse;
use brief_hir::lower;
use brief_diagnostic::FileId;

/// Helper function to parse source and lower to HIR
pub fn lower_source(source: &str) -> brief_hir::HirProgram {
    let file_id = FileId(0);
    let (tokens, _lex_errors) = lex(source, file_id);
    let (ast, _parse_errors) = parse(tokens, file_id);
    lower(ast).unwrap_or_else(|errors| {
        panic!("HIR lowering failed: {:?}", errors);
    })
}

/// Helper function to parse source and return HIR errors
#[allow(dead_code)]
pub fn lower_errors(source: &str) -> Vec<brief_hir::HirError> {
    let file_id = FileId(0);
    let (tokens, _lex_errors) = lex(source, file_id);
    let (ast, _parse_errors) = parse(tokens, file_id);
    lower(ast).unwrap_err()
}
