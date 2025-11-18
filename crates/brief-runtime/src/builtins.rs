use brief_vm::{Value, RuntimeError};

/// Builtin function type
/// Note: VM is passed separately to avoid circular dependency
pub type BuiltinFn = fn(&[Value]) -> Result<Value, RuntimeError>;

/// Print builtin: print(value)
pub fn print(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.is_empty() {
        return Err(RuntimeError::CallError("print requires at least 1 argument".to_string()));
    }
    println!("{}", args[0]);
    Ok(Value::Null)
}

/// Length builtin: len(value)
/// Stub for now - returns 0 until arrays/strings are fully implemented
pub fn len(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.is_empty() {
        return Err(RuntimeError::CallError("len requires 1 argument".to_string()));
    }
    match &args[0] {
        Value::Str(s) => Ok(Value::Int(s.len() as i64)),
        // TODO: Implement for arrays when they're added
        _ => Err(RuntimeError::TypeMismatch {
            expected: "string or array".to_string(),
            got: format!("{:?}", args[0]),
        }),
    }
}

/// Integer cast builtin: int(value)
pub fn int_cast(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.is_empty() {
        return Err(RuntimeError::CallError("int requires 1 argument".to_string()));
    }
    match &args[0] {
        Value::Int(i) => Ok(Value::Int(*i)),
        Value::Double(d) => Ok(Value::Int(*d as i64)),
        Value::Bool(b) => Ok(Value::Int(if *b { 1 } else { 0 })),
        Value::Str(s) => {
            s.parse::<i64>()
                .map(Value::Int)
                .map_err(|_| RuntimeError::CallError(format!("Cannot convert string '{}' to integer", s)))
        },
        Value::Null => Err(RuntimeError::CallError("Cannot convert null to integer".to_string())),
    }
}

/// Double cast builtin: dub(value)
pub fn dub_cast(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.is_empty() {
        return Err(RuntimeError::CallError("dub requires 1 argument".to_string()));
    }
    match &args[0] {
        Value::Int(i) => Ok(Value::Double(*i as f64)),
        Value::Double(d) => Ok(Value::Double(*d)),
        Value::Bool(b) => Ok(Value::Double(if *b { 1.0 } else { 0.0 })),
        Value::Str(s) => {
            s.parse::<f64>()
                .map(Value::Double)
                .map_err(|_| RuntimeError::CallError(format!("Cannot convert string '{}' to double", s)))
        },
        Value::Null => Err(RuntimeError::CallError("Cannot convert null to double".to_string())),
    }
}

/// String cast builtin: str(value)
pub fn str_cast(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.is_empty() {
        return Err(RuntimeError::CallError("str requires 1 argument".to_string()));
    }
    // Optimize: if already a string, return it directly
    match &args[0] {
        Value::Str(s) => Ok(Value::Str(s.clone())), // Clone needed for ownership
        other => Ok(Value::Str(other.to_string())),
    }
}

/// String concatenation helper: rt_concatN(args...)
/// Concatenates N string arguments efficiently

pub fn rt_concat2(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() < 2 {
        return Err(RuntimeError::CallError("rt_concat2 requires 2 arguments".to_string()));
    }
    // Optimize: handle different cases to avoid unnecessary conversions
    match (&args[0], &args[1]) {
        (Value::Str(a), Value::Str(b)) => {
            // Both strings - most efficient path
            let mut result = String::with_capacity(a.len() + b.len());
            result.push_str(a);
            result.push_str(b);
            return Ok(Value::Str(result));
        },
        (Value::Str(a), b) => {
            let b_str = b.to_string();
            let mut result = String::with_capacity(a.len() + b_str.len());
            result.push_str(a);
            result.push_str(&b_str);
            return Ok(Value::Str(result));
        },
        (a, Value::Str(b)) => {
            let a_str = a.to_string();
            let mut result = String::with_capacity(a_str.len() + b.len());
            result.push_str(&a_str);
            result.push_str(b);
            return Ok(Value::Str(result));
        },
        (a, b) => {
            // Both non-strings - need to convert both
            let a_str = a.to_string();
            let b_str = b.to_string();
            let mut result = String::with_capacity(a_str.len() + b_str.len());
            result.push_str(&a_str);
            result.push_str(&b_str);
            return Ok(Value::Str(result));
        },
    };
}

pub fn rt_concat3(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() < 3 {
        return Err(RuntimeError::CallError("rt_concat3 requires 3 arguments".to_string()));
    }
    // Calculate capacity first, then concatenate
    let total_len: usize = args.iter().take(3).map(|v| match v {
        Value::Str(s) => s.len(),
        _ => v.to_string().len(), // Only for capacity estimation
    }).sum();
    let mut result = String::with_capacity(total_len);
    for arg in args.iter().take(3) {
        match arg {
            Value::Str(s) => result.push_str(s), // No clone needed
            v => result.push_str(&v.to_string()),
        }
    }
    Ok(Value::Str(result))
}

pub fn rt_concat4(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() < 4 {
        return Err(RuntimeError::CallError("rt_concat4 requires 4 arguments".to_string()));
    }
    // Calculate capacity first, then concatenate
    let total_len: usize = args.iter().take(4).map(|v| match v {
        Value::Str(s) => s.len(),
        _ => v.to_string().len(), // Only for capacity estimation
    }).sum();
    let mut result = String::with_capacity(total_len);
    for arg in args.iter().take(4) {
        match arg {
            Value::Str(s) => result.push_str(s), // No clone needed
            v => result.push_str(&v.to_string()),
        }
    }
    Ok(Value::Str(result))
}

pub fn rt_concat5(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() < 5 {
        return Err(RuntimeError::CallError("rt_concat5 requires 5 arguments".to_string()));
    }
    // Calculate capacity first, then concatenate
    let total_len: usize = args.iter().take(5).map(|v| match v {
        Value::Str(s) => s.len(),
        _ => v.to_string().len(), // Only for capacity estimation
    }).sum();
    let mut result = String::with_capacity(total_len);
    for arg in args.iter().take(5) {
        match arg {
            Value::Str(s) => result.push_str(s), // No clone needed
            v => result.push_str(&v.to_string()),
        }
    }
    Ok(Value::Str(result))
}

