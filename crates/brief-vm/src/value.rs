/// Runtime value representation
#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Int(i64),
    Double(f64),
    Bool(bool),
    Str(String),  // Heap-allocated (GC'd)
    Null,
    // Obj(ObjPtr),  // For future objects
}

impl Value {
    /// Check truthiness: only false and null are falsey
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Bool(false) => false,
            Value::Null => false,
            _ => true,
        }
    }

    /// Convert to string representation
    pub fn to_string(&self) -> String {
        match self {
            Value::Int(i) => i.to_string(),
            Value::Double(d) => d.to_string(),
            Value::Bool(b) => b.to_string(),
            Value::Str(s) => s.clone(),
            Value::Null => "null".to_string(),
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Int(i) => write!(f, "{}", i),
            Value::Double(d) => write!(f, "{}", d),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Str(s) => write!(f, "{}", s),
            Value::Null => write!(f, "null"),
        }
    }
}

