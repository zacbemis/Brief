use std::collections::HashMap;
use brief_vm::{Value, RuntimeError, BuiltinRuntime};
use crate::builtins::*;

/// Runtime for builtin functions
pub struct Runtime {
    builtins: HashMap<String, BuiltinFn>,
}

impl BuiltinRuntime for Runtime {
    fn call_builtin(&self, name: &str, args: &[Value]) -> Result<Value, RuntimeError> {
        if let Some(builtin_fn) = self.get_builtin(name) {
            builtin_fn(args)
        } else {
            Err(RuntimeError::CallError(format!("Unknown builtin: {}", name)))
        }
    }
    
    fn is_builtin(&self, name: &str) -> bool {
        self.builtins.contains_key(name)
    }
}

impl Runtime {
    pub fn new() -> Self {
        let mut builtins = HashMap::new();
        
        // Core builtins
        builtins.insert("print".to_string(), print as BuiltinFn);
        builtins.insert("len".to_string(), len as BuiltinFn);
        
        // Type casting builtins
        builtins.insert("int".to_string(), int_cast as BuiltinFn);
        builtins.insert("dub".to_string(), dub_cast as BuiltinFn);
        builtins.insert("str".to_string(), str_cast as BuiltinFn);
        
        // String concatenation helpers
        builtins.insert("rt_concat2".to_string(), rt_concat2 as BuiltinFn);
        builtins.insert("rt_concat3".to_string(), rt_concat3 as BuiltinFn);
        builtins.insert("rt_concat4".to_string(), rt_concat4 as BuiltinFn);
        builtins.insert("rt_concat5".to_string(), rt_concat5 as BuiltinFn);
        
        Self { builtins }
    }
    
    /// Lookup a builtin function by name
    pub fn get_builtin(&self, name: &str) -> Option<BuiltinFn> {
        self.builtins.get(name).copied()
    }
    
    /// Check if a name is a builtin
    pub fn is_builtin(&self, name: &str) -> bool {
        self.builtins.contains_key(name)
    }
}

impl Default for Runtime {
    fn default() -> Self {
        Self::new()
    }
}

