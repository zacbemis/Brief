use brief_diagnostic::Span;
use brief_ast::{BinaryOp, UnaryOp, InterpPart};
use crate::symbol::{SymbolRef, Upvalue};

/// HIR Program
#[derive(Debug, Clone, PartialEq)]
pub struct HirProgram {
    pub declarations: Vec<HirDecl>,
    pub span: Span,
}

/// HIR Declaration
#[derive(Debug, Clone, PartialEq)]
pub enum HirDecl {
    VarDecl(HirVarDecl),
    ConstDecl(HirConstDecl),
    FuncDecl(HirFuncDecl),
    ClassDecl(HirClassDecl),
    ImportDecl(HirImportDecl),
    Error(Span),
}

/// HIR Variable Declaration
#[derive(Debug, Clone, PartialEq)]
pub struct HirVarDecl {
    pub name: String,
    pub symbol: SymbolRef,
    pub type_annotation: Option<brief_ast::Type>,
    pub initializer: Option<HirExpr>,
    pub span: Span,
}

/// HIR Constant Declaration
#[derive(Debug, Clone, PartialEq)]
pub struct HirConstDecl {
    pub name: String,
    pub symbol: SymbolRef,
    pub initializer: HirExpr,
    pub span: Span,
}

/// HIR Function Declaration
#[derive(Debug, Clone)]
pub struct HirFuncDecl {
    pub name: String,
    pub symbol: SymbolRef,
    pub params: Vec<HirParam>,
    pub return_type: Option<brief_ast::Type>,
    pub body: HirBlock,
    pub symbol_table: crate::symbol::SymbolTable,
    pub span: Span,
}

impl PartialEq for HirFuncDecl {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.symbol == other.symbol
            && self.params == other.params
            && self.return_type == other.return_type
            && self.body == other.body
            && self.span == other.span
            // Skip symbol_table comparison
    }
}

/// HIR Parameter
#[derive(Debug, Clone, PartialEq)]
pub struct HirParam {
    pub name: String,
    pub symbol: SymbolRef,
    pub type_annotation: Option<brief_ast::Type>,
    pub span: Span,
}

/// HIR Class Declaration
#[derive(Debug, Clone, PartialEq)]
pub struct HirClassDecl {
    pub name: String,
    pub symbol: SymbolRef,
    pub constructor: Option<HirCtorDecl>,
    pub methods: Vec<HirMethodDecl>,
    pub span: Span,
}

/// HIR Constructor Declaration
#[derive(Debug, Clone)]
pub struct HirCtorDecl {
    pub name: String,
    pub params: Vec<HirParam>,
    pub body: HirBlock,
    pub symbol_table: crate::symbol::SymbolTable,
    pub span: Span,
}

impl PartialEq for HirCtorDecl {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.params == other.params
            && self.body == other.body
            && self.span == other.span
            // Skip symbol_table comparison
    }
}

/// HIR Method Declaration
#[derive(Debug, Clone)]
pub struct HirMethodDecl {
    pub name: String,
    pub symbol: SymbolRef,
    pub is_instance: bool,
    pub params: Vec<HirParam>,
    pub return_type: Option<brief_ast::Type>,
    pub body: HirBlock,
    pub symbol_table: crate::symbol::SymbolTable,
    pub span: Span,
}

impl PartialEq for HirMethodDecl {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.symbol == other.symbol
            && self.is_instance == other.is_instance
            && self.params == other.params
            && self.return_type == other.return_type
            && self.body == other.body
            && self.span == other.span
            // Skip symbol_table comparison
    }
}

/// HIR Import Declaration
#[derive(Debug, Clone, PartialEq)]
pub struct HirImportDecl {
    pub modules: Vec<String>,
    pub span: Span,
}

/// HIR Expression
#[derive(Debug, Clone, PartialEq)]
pub enum HirExpr {
    // Literals (same as AST)
    Integer(i64, Span),
    Double(f64, Span),
    Character(char, Span),
    String(String, Span),
    Boolean(bool, Span),
    Null(Span),
    
    // Variables (with resolved symbol)
    Variable {
        name: String,
        symbol: SymbolRef,
        span: Span,
    },
    
    // Member access
    MemberAccess {
        object: Box<HirExpr>,
        member: String,
        span: Span,
    },
    
    // Index
    Index {
        object: Box<HirExpr>,
        index: Box<HirExpr>,
        span: Span,
    },
    
    // Operations (no PostfixOp - desugared to Assign)
    BinaryOp {
        left: Box<HirExpr>,
        op: BinaryOp,
        right: Box<HirExpr>,
        span: Span,
    },
    UnaryOp {
        op: UnaryOp,
        expr: Box<HirExpr>,
        span: Span,
    },
    
    // Assignment (desugared from PostfixOp and assignment operators)
    Assign {
        target: Box<HirExpr>,
        value: Box<HirExpr>,
        span: Span,
    },
    
    // Calls
    Call {
        callee: Box<HirExpr>,
        args: Vec<HirExpr>,
        span: Span,
    },
    MethodCall {
        object: Box<HirExpr>,
        method: String,
        args: Vec<HirExpr>,
        span: Span,
    },
    
    // Type casting
    Cast {
        expr: Box<HirExpr>,
        target_type: brief_ast::Type,
        span: Span,
    },
    
    // String interpolation
    Interpolation {
        parts: Vec<InterpPart>,
        span: Span,
    },
    
    // Ternary
    Ternary {
        condition: Box<HirExpr>,
        then_expr: Box<HirExpr>,
        else_expr: Box<HirExpr>,
        span: Span,
    },
    
    // Lambda (desugared from y(x) := expr)
    Lambda {
        params: Vec<HirParam>,
        captures: Vec<Upvalue>,
        body: Box<HirExpr>,
        span: Span,
    },
    
    // Error placeholder
    Error(Span),
}

/// HIR Statement
#[derive(Debug, Clone, PartialEq)]
pub enum HirStmt {
    // Declarations
    VarDecl(HirVarDecl),
    ConstDecl(HirConstDecl),
    
    // Control flow (no ForIn, no Match - desugared)
    If {
        condition: Box<HirExpr>,
        then_branch: HirBlock,
        else_branch: Option<HirBlock>,
        span: Span,
    },
    While {
        condition: Box<HirExpr>,
        body: HirBlock,
        span: Span,
    },
    For {
        init: Option<Box<HirStmt>>,
        condition: Option<Box<HirExpr>>,
        increment: Option<Box<HirExpr>>,
        body: HirBlock,
        span: Span,
    },
    
    // Control
    Return {
        value: Option<HirExpr>,
        span: Span,
    },
    Break(Span),
    Continue(Span),
    
    // Expression statement
    Expr(Box<HirExpr>, Span),
    
    // Error placeholder
    Error(Span),
}

/// HIR Block
#[derive(Debug, Clone, PartialEq)]
pub struct HirBlock {
    pub statements: Vec<HirStmt>,
    pub span: Span,
}

impl HirExpr {
    pub fn span(&self) -> Span {
        match self {
            HirExpr::Integer(_, span) |
            HirExpr::Double(_, span) |
            HirExpr::Character(_, span) |
            HirExpr::String(_, span) |
            HirExpr::Boolean(_, span) |
            HirExpr::Null(span) |
            HirExpr::Error(span) => *span,
            HirExpr::Variable { span, .. } |
            HirExpr::MemberAccess { span, .. } |
            HirExpr::Index { span, .. } |
            HirExpr::BinaryOp { span, .. } |
            HirExpr::UnaryOp { span, .. } |
            HirExpr::Assign { span, .. } |
            HirExpr::Call { span, .. } |
            HirExpr::MethodCall { span, .. } |
            HirExpr::Cast { span, .. } |
            HirExpr::Interpolation { span, .. } |
            HirExpr::Ternary { span, .. } |
            HirExpr::Lambda { span, .. } => *span,
        }
    }
}

