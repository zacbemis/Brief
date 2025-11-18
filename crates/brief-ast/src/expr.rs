use brief_diagnostic::Span;

/// Expression node in the AST
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    // Literals
    Integer(i64, Span),
    Double(f64, Span),
    Character(char, Span),
    String(String, Span),  // Complete string (with interpolation parts)
    Boolean(bool, Span),
    Null(Span),
    
    // Variables and access
    Variable(String, Span),
    MemberAccess {
        object: Box<Expr>,
        member: String,
        span: Span,
    },
    Index {
        object: Box<Expr>,
        index: Box<Expr>,
        span: Span,
    },
    
    // Operations
    BinaryOp {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
        span: Span,
    },
    UnaryOp {
        op: UnaryOp,
        expr: Box<Expr>,
        span: Span,
    },
    PostfixOp {
        expr: Box<Expr>,
        op: PostfixOp,
        span: Span,
    },
    
    // Calls
    Call {
        callee: Box<Expr>,
        args: Vec<Expr>,
        span: Span,
    },
    MethodCall {
        object: Box<Expr>,
        method: String,
        args: Vec<Expr>,
        span: Span,
    },
    
    // Type casting
    Cast {
        expr: Box<Expr>,
        target_type: crate::ty::Type,
        span: Span,
    },
    
    // String interpolation
    Interpolation {
        parts: Vec<InterpPart>,
        span: Span,
    },
    
    // Ternary
    Ternary {
        condition: Box<Expr>,
        then_expr: Box<Expr>,
        else_expr: Box<Expr>,
        span: Span,
    },
    
    // Lambda
    Lambda {
        params: Vec<Param>,
        body: Box<Expr>,  // Single expression or block
        span: Span,
    },
    
    // Error placeholder
    Error(Span),
}

impl Expr {
    /// Get the span of an expression
    pub fn span(&self) -> Span {
        match self {
            Expr::Integer(_, span) |
            Expr::Double(_, span) |
            Expr::Character(_, span) |
            Expr::String(_, span) |
            Expr::Boolean(_, span) |
            Expr::Null(span) |
            Expr::Variable(_, span) |
            Expr::Error(span) => *span,
            Expr::MemberAccess { span, .. } |
            Expr::Index { span, .. } |
            Expr::BinaryOp { span, .. } |
            Expr::UnaryOp { span, .. } |
            Expr::PostfixOp { span, .. } |
            Expr::Call { span, .. } |
            Expr::MethodCall { span, .. } |
            Expr::Cast { span, .. } |
            Expr::Interpolation { span, .. } |
            Expr::Ternary { span, .. } |
            Expr::Lambda { span, .. } => *span,
        }
    }
}

/// Part of a string interpolation
#[derive(Debug, Clone, PartialEq)]
pub enum InterpPart {
    Text(String),
    Ident(String, Span),
    Path(Box<Expr>, Span),  // obj.field expression
}

/// Binary operators
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOp {
    // Arithmetic
    Add, Sub, Mul, Div, Mod, Pow,
    // Comparison
    Eq, Ne, Lt, Le, Gt, Ge,
    // Logical
    And, Or,
    // Bitwise
    BitAnd, BitOr, BitXor,
    // Shift
    Shl, Shr,
    // Assignment (desugared in HIR)
    Assign, InitAssign, PlusAssign, MinusAssign, StarAssign, SlashAssign, PercentAssign, PowAssign,
}

/// Unary operators
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOp {
    Not,      // !
    BitNot,   // ~
    Neg,      // -
    Pos,      // +
}

/// Postfix operators
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PostfixOp {
    Inc,  // ++
    Dec,  // --
}

/// Function parameter (used in lambdas and function declarations)
#[derive(Debug, Clone, PartialEq)]
pub struct Param {
    pub name: String,
    pub type_annotation: Option<crate::ty::Type>,
    pub span: Span,
}

