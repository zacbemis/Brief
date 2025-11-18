use brief_diagnostic::Span;
use crate::decl::Decl;

/// Root program node
#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub declarations: Vec<Decl>,
    pub span: Span,
}

