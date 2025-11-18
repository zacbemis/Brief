use brief_diagnostic::Span;

/// Type node in the AST
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int,
    Char,
    Str,
    Dub,
    Bool,
    Array {
        base: Box<Type>,
        dims: Vec<ArrayDim>,
        span: Span,
    },
    Map {
        key_type: Box<Type>,
        value_type: Box<Type>,
        span: Span,
    },
    Function {
        params: Vec<Type>,
        return_type: Box<Type>,
        span: Span,
    },
}

/// Array dimension specification
#[derive(Debug, Clone, PartialEq)]
pub enum ArrayDim {
    Fixed(usize),      // int[10]
    Dynamic,           // int{}
    Stack,             // int{stk}
    Queue,             // int{que}
}

// Example: int[10][10] → Array { base: Int, dims: [Fixed(10), Fixed(10)] }
// Example: int[][] → Array { base: Int, dims: [Dynamic, Dynamic] }

