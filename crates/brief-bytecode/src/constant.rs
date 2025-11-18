/// Constant pool entry
#[derive(Debug, Clone, PartialEq)]
pub enum Constant {
    Int(i64),
    Double(f64),
    Bool(bool),
    Str(String),  // Interned string
    Null,
}

impl Constant {
    /// Get the type name of this constant
    pub fn type_name(&self) -> &'static str {
        match self {
            Constant::Int(_) => "Int",
            Constant::Double(_) => "Double",
            Constant::Bool(_) => "Bool",
            Constant::Str(_) => "Str",
            Constant::Null => "Null",
        }
    }
}

impl std::fmt::Display for Constant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Constant::Int(n) => write!(f, "{}", n),
            Constant::Double(d) => write!(f, "{}", d),
            Constant::Bool(b) => write!(f, "{}", b),
            Constant::Str(s) => write!(f, "\"{}\"", s),
            Constant::Null => write!(f, "null"),
        }
    }
}

