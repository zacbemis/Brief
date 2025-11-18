use brief_diagnostic::Span;
use crate::expr::Expr;
use crate::decl::{VarDecl, ConstDecl};

/// Statement node in the AST
#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    // Declarations (can appear in statement context)
    VarDecl(VarDecl),
    ConstDecl(ConstDecl),
    
    // Control flow
    If {
        condition: Expr,
        then_branch: Block,
        else_branch: Option<Block>,
        span: Span,
    },
    While {
        condition: Expr,
        body: Block,
        span: Span,
    },
    For {
        init: Option<Box<Stmt>>,  // Variable decl or expression
        condition: Option<Expr>,
        increment: Option<Expr>,
        body: Block,
        span: Span,
    },
    ForIn {
        var: String,
        iterable: Expr,
        body: Block,
        span: Span,
    },
    Match {
        expr: Expr,
        cases: Vec<MatchCase>,
        else_branch: Option<Block>,
        span: Span,
    },
    
    // Control
    Return {
        value: Option<Expr>,
        span: Span,
    },
    Break(Span),
    Continue(Span),
    
    // Expression statement
    Expr(Expr, Span),
    
    // Error placeholder
    Error(Span),
}

/// Block of statements (indentation-based)
#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    pub statements: Vec<Stmt>,
    pub span: Span,
}

/// Match case with potentially multiple patterns
#[derive(Debug, Clone, PartialEq)]
pub struct MatchCase {
    pub patterns: Vec<Expr>,  // Multiple patterns allowed: case 'A', 'B'
    pub body: Block,
    pub span: Span,
}

