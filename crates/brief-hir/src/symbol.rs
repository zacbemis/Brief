use brief_diagnostic::Span;

/// Symbol reference (index into symbol table)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SymbolRef(pub usize);

impl SymbolRef {
    pub const BUILTIN: Self = Self(usize::MAX);
}

/// Symbol kind indicating where the symbol is stored
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SymbolKind {
    /// Local variable in current function (register index)
    Local(usize),
    /// Parameter (parameter index)
    Param(usize),
    /// Upvalue (captured from outer scope, upvalue index)
    Upvalue(usize),
    /// Global symbol (name)
    Global(String),
}

/// Symbol entry in the symbol table
#[derive(Debug, Clone, PartialEq)]
pub struct Symbol {
    pub name: String,
    pub kind: SymbolKind,
    pub span: Span,
}

/// Symbol table for a function/module
#[derive(Debug, Clone)]
pub struct SymbolTable {
    pub symbols: Vec<Symbol>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            symbols: Vec::new(),
        }
    }

    pub fn add_symbol(&mut self, name: String, kind: SymbolKind, span: Span) -> SymbolRef {
        let index = self.symbols.len();
        self.symbols.push(Symbol { name, kind, span });
        SymbolRef(index)
    }

    pub fn get(&self, index: SymbolRef) -> Option<&Symbol> {
        self.symbols.get(index.0)
    }
}

/// Scope stack for name resolution
/// Uses a Vec for simplicity (scopes are typically small)
/// For larger scopes, consider using HashMap for O(1) lookup
#[derive(Debug, Clone)]
pub struct Scope {
    pub symbols: Vec<(String, SymbolRef)>,
}

impl Scope {
    pub fn new() -> Self {
        Self {
            symbols: Vec::new(),
        }
    }

    pub fn add(&mut self, name: String, symbol: SymbolRef) {
        self.symbols.push((name, symbol));
    }

    /// Lookup a symbol in this scope (searches from most recent to oldest)
    /// Returns the most recent binding if multiple exist (shadowing)
    pub fn lookup(&self, name: &str) -> Option<SymbolRef> {
        // Search backwards to find most recent binding (shadowing)
        self.symbols
            .iter()
            .rev()
            .find(|(n, _)| n == name)
            .map(|(_, sym)| *sym)
    }
}

/// Upvalue information for closures
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Upvalue {
    /// True if the upvalue is a local in the immediately enclosing function,
    /// false if it's an upvalue from a further outer function
    pub is_local: bool,
    /// Index in local registers (if is_local) or upvalue array (if not)
    pub index: usize,
}

