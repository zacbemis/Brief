use brief_lexer::lex;
use brief_parser::parse;
use brief_diagnostic::FileId;

/// Helper function to parse source and return the program (ignoring errors for now)
#[allow(dead_code)]
pub fn parse_source(source: &str) -> brief_ast::Program {
    let file_id = FileId(0);
    let (tokens, _lex_errors) = lex(source, file_id);
    let (program, _parse_errors) = parse(tokens, file_id);
    program
}

/// Helper function to parse source and return parse errors
#[allow(dead_code)]
pub fn parse_errors(source: &str) -> Vec<brief_parser::ParseError> {
    let file_id = FileId(0);
    let (tokens, _lex_errors) = lex(source, file_id);
    let (_program, parse_errors) = parse(tokens, file_id);
    parse_errors
}

/// Helper function to parse source and return both program and errors
#[allow(dead_code)]
pub fn parse_with_errors(source: &str) -> (brief_ast::Program, Vec<brief_parser::ParseError>) {
    let file_id = FileId(0);
    let (tokens, _lex_errors) = lex(source, file_id);
    parse(tokens, file_id)
}

