use brief_diagnostic::Span;
use crate::expr::{Expr, Param};
use crate::stmt::Block;
use crate::ty::Type;

/// Declaration node in the AST
#[derive(Debug, Clone, PartialEq)]
pub enum Decl {
    VarDecl(VarDecl),
    ConstDecl(ConstDecl),
    FuncDecl(FuncDecl),
    ClassDecl(ClassDecl),
    ImportDecl(ImportDecl),
    Error(Span),
}

/// Variable declaration
#[derive(Debug, Clone, PartialEq)]
pub struct VarDecl {
    pub name: String,
    pub type_annotation: Option<Type>,
    pub initializer: Option<Expr>,
    pub span: Span,
}

/// Constant declaration
#[derive(Debug, Clone, PartialEq)]
pub struct ConstDecl {
    pub name: String,
    pub initializer: Expr,
    pub span: Span,
}

/// Function declaration
#[derive(Debug, Clone, PartialEq)]
pub struct FuncDecl {
    pub name: String,
    pub params: Vec<Param>,
    pub return_type: Option<Type>,
    pub body: Block,
    pub span: Span,
}

/// Class declaration
#[derive(Debug, Clone, PartialEq)]
pub struct ClassDecl {
    pub name: String,
    pub constructor: Option<CtorDecl>,
    pub methods: Vec<MethodDecl>,
    pub span: Span,
}

/// Constructor declaration
#[derive(Debug, Clone, PartialEq)]
pub struct CtorDecl {
    pub name: String,  // Same as class name
    pub params: Vec<Param>,
    pub body: Block,
    pub span: Span,
    // HIR will desugar implicit obj.name = name assignments
}

/// Method declaration
#[derive(Debug, Clone, PartialEq)]
pub struct MethodDecl {
    pub name: String,
    pub is_instance: bool,  // obj def vs def
    pub params: Vec<Param>,
    pub return_type: Option<Type>,
    pub body: Block,
    pub span: Span,
}

/// Import declaration
#[derive(Debug, Clone, PartialEq)]
pub struct ImportDecl {
    pub modules: Vec<String>,  // import (a, b, c)
    pub span: Span,
}

