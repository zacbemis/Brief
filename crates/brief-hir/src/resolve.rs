use brief_diagnostic::Span;
use crate::hir::*;
use crate::symbol::*;
use crate::error::HirError;

const BUILTINS: &[&str] = &[
    "print",
    "len",
    "int",
    "dub",
    "str",
    "rt_concat2",
    "rt_concat3",
    "rt_concat4",
    "rt_concat5",
];

/// Resolve names in HIR and populate symbol tables
pub fn resolve(program: &mut HirProgram) -> Result<(), Vec<HirError>> {
    let mut resolver = Resolver::new();
    resolver.resolve_program(program)
}

struct Resolver {
    errors: Vec<HirError>,
    scopes: Vec<Scope>,
    current_function: Option<usize>, // Index in symbol table for current function
    local_count: usize,
    upvalue_count: usize,
}

impl Resolver {
    fn new() -> Self {
        Self {
            errors: Vec::new(),
            scopes: Vec::new(),
            current_function: None,
            local_count: 0,
            upvalue_count: 0,
        }
    }

    fn resolve_program(&mut self, program: &mut HirProgram) -> Result<(), Vec<HirError>> {
        // Create module-level scope
        self.begin_scope();
        
        // Resolve all top-level declarations
        for decl in &mut program.declarations {
            self.resolve_decl(decl);
        }
        
        self.end_scope();
        
        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(self.errors.clone())
        }
    }

    fn resolve_decl(&mut self, decl: &mut HirDecl) {
        match decl {
            HirDecl::VarDecl(v) => {
                // Add to current scope
                if let Some(symbol) = self.declare_symbol(&v.name, SymbolKind::Local(self.local_count), v.span) {
                    v.symbol = symbol;
                }
                // Resolve initializer
                if let Some(init) = &mut v.initializer {
                    self.resolve_expr(init);
                }
            },
            HirDecl::ConstDecl(c) => {
                // Add to current scope
                if let Some(symbol) = self.declare_symbol(&c.name, SymbolKind::Local(self.local_count), c.span) {
                    c.symbol = symbol;
                }
                // Resolve initializer
                self.resolve_expr(&mut c.initializer);
            },
            HirDecl::FuncDecl(f) => {
                // Add function name to scope (avoid cloning name)
                let func_name = f.name.clone(); // Need clone for Global variant
                if let Some(symbol) = self.declare_symbol(&f.name, SymbolKind::Global(func_name), f.span) {
                    f.symbol = symbol;
                }
                // Resolve function body (with new scope)
                self.resolve_func_decl(f);
            },
            HirDecl::ClassDecl(c) => {
                // Add class name to scope (avoid cloning name)
                let class_name = c.name.clone(); // Need clone for Global variant
                if let Some(symbol) = self.declare_symbol(&c.name, SymbolKind::Global(class_name), c.span) {
                    c.symbol = symbol;
                }
                // Resolve constructor and methods
                if let Some(ctor) = &mut c.constructor {
                    self.resolve_ctor_decl(ctor);
                }
                for method in &mut c.methods {
                    self.resolve_method_decl(method);
                }
            },
            HirDecl::ImportDecl(_) => {
                // Imports are handled separately
            },
            HirDecl::Error(_) => {},
        }
    }

    fn resolve_func_decl(&mut self, func: &mut HirFuncDecl) {
        // Create new scope for function
        self.begin_scope();
        
        // Add parameters to scope
        for (idx, param) in func.params.iter_mut().enumerate() {
            if let Some(symbol) = self.declare_symbol(&param.name, SymbolKind::Param(idx), param.span) {
                param.symbol = symbol;
                // Add to function's symbol table
                func.symbol_table.add_symbol(
                    param.name.clone(),
                    SymbolKind::Param(idx),
                    param.span,
                );
            }
        }
        
        // Resolve function body
        self.resolve_block(&mut func.body);
        
        // Build symbol table for function
        // Add all locals to function's symbol table
        // (This is simplified - in a full implementation, we'd track locals more carefully)
        
        self.end_scope();
    }

    fn resolve_ctor_decl(&mut self, ctor: &mut HirCtorDecl) {
        // Create new scope for constructor
        self.begin_scope();
        
        // Add parameters to scope
        for (idx, param) in ctor.params.iter_mut().enumerate() {
            if let Some(symbol) = self.declare_symbol(&param.name, SymbolKind::Param(idx), param.span) {
                param.symbol = symbol;
                // Add to constructor's symbol table
                ctor.symbol_table.add_symbol(
                    param.name.clone(),
                    SymbolKind::Param(idx),
                    param.span,
                );
            }
        }
        
        // Resolve constructor body
        self.resolve_block(&mut ctor.body);
        
        self.end_scope();
    }

    fn resolve_method_decl(&mut self, method: &mut HirMethodDecl) {
        // Create new scope for method
        self.begin_scope();
        
        // Add parameters to scope
        for (idx, param) in method.params.iter_mut().enumerate() {
            if let Some(symbol) = self.declare_symbol(&param.name, SymbolKind::Param(idx), param.span) {
                param.symbol = symbol;
                // Add to method's symbol table
                method.symbol_table.add_symbol(
                    param.name.clone(),
                    SymbolKind::Param(idx),
                    param.span,
                );
            }
        }
        
        // Resolve method body
        self.resolve_block(&mut method.body);
        
        self.end_scope();
    }

    fn resolve_block(&mut self, block: &mut HirBlock) {
        self.begin_scope();
        
        for stmt in &mut block.statements {
            self.resolve_stmt(stmt);
        }
        
        self.end_scope();
    }

    fn resolve_stmt(&mut self, stmt: &mut HirStmt) {
        match stmt {
            HirStmt::VarDecl(v) => {
                // Add to current scope
                if let Some(symbol) = self.declare_symbol(&v.name, SymbolKind::Local(self.local_count), v.span) {
                    v.symbol = symbol;
                }
                // Resolve initializer
                if let Some(init) = &mut v.initializer {
                    self.resolve_expr(init);
                }
            },
            HirStmt::ConstDecl(c) => {
                // Add to current scope
                if let Some(symbol) = self.declare_symbol(&c.name, SymbolKind::Local(self.local_count), c.span) {
                    c.symbol = symbol;
                }
                // Resolve initializer
                self.resolve_expr(&mut c.initializer);
            },
            HirStmt::If { condition, then_branch, else_branch, .. } => {
                self.resolve_expr(condition);
                self.resolve_block(then_branch);
                if let Some(else_branch) = else_branch {
                    self.resolve_block(else_branch);
                }
            },
            HirStmt::While { condition, body, .. } => {
                self.resolve_expr(condition);
                self.resolve_block(body);
            },
            HirStmt::For { init, condition, increment, body, .. } => {
                if let Some(init) = init {
                    self.resolve_stmt(init);
                }
                if let Some(condition) = condition {
                    self.resolve_expr(condition);
                }
                if let Some(increment) = increment {
                    self.resolve_expr(increment);
                }
                self.resolve_block(body);
            },
            HirStmt::Return { value, .. } => {
                if let Some(value) = value {
                    self.resolve_expr(value);
                }
            },
            HirStmt::Break(_) | HirStmt::Continue(_) => {},
            HirStmt::Expr(expr, _) => {
                self.resolve_expr(expr);
            },
            HirStmt::Error(_) => {},
        }
    }

    fn resolve_expr(&mut self, expr: &mut HirExpr) {
        match expr {
            HirExpr::Variable { name, symbol, span } => {
                // Look up variable in scopes
                if let Some(sym_ref) = self.resolve_variable(name, *span) {
                    *symbol = sym_ref;
                }
            },
            HirExpr::MemberAccess { object, .. } => {
                self.resolve_expr(object);
            },
            HirExpr::Index { object, index, .. } => {
                self.resolve_expr(object);
                self.resolve_expr(index);
            },
            HirExpr::BinaryOp { left, right, .. } => {
                self.resolve_expr(left);
                self.resolve_expr(right);
            },
            HirExpr::UnaryOp { expr, .. } => {
                self.resolve_expr(expr);
            },
            HirExpr::Assign { target, value, .. } => {
                self.resolve_expr(target);
                self.resolve_expr(value);
            },
            HirExpr::Call { callee, args, .. } => {
                self.resolve_expr(callee);
                for arg in args {
                    self.resolve_expr(arg);
                }
            },
            HirExpr::MethodCall { object, args, .. } => {
                self.resolve_expr(object);
                for arg in args {
                    self.resolve_expr(arg);
                }
            },
            HirExpr::Cast { expr, .. } => {
                self.resolve_expr(expr);
            },
            HirExpr::Interpolation { .. } => {
                // Interpolation parts contain AST expressions, not HIR expressions
                // They will be resolved during bytecode generation
            },
            HirExpr::Ternary { condition, then_expr, else_expr, .. } => {
                self.resolve_expr(condition);
                self.resolve_expr(then_expr);
                self.resolve_expr(else_expr);
            },
            HirExpr::Lambda { params, captures, body, .. } => {
                // Create new scope for lambda
                self.begin_scope();
                
                // Add parameters to scope
                for (idx, param) in params.iter_mut().enumerate() {
                    if let Some(symbol) = self.declare_symbol(&param.name, SymbolKind::Param(idx), param.span) {
                        param.symbol = symbol;
                    }
                }
                
                // Resolve body (this will detect captures)
                self.resolve_expr(body);
                
                // TODO: Detect and record upvalues/captures
                // For now, captures remains empty
                
                self.end_scope();
            },
            HirExpr::Integer(_, _) |
            HirExpr::Double(_, _) |
            HirExpr::Character(_, _) |
            HirExpr::String(_, _) |
            HirExpr::Boolean(_, _) |
            HirExpr::Null(_) |
            HirExpr::Error(_) => {},
        }
    }

    fn resolve_variable(&mut self, name: &str, span: Span) -> Option<SymbolRef> {
        // Look up in current scopes (from innermost to outermost)
        for scope in self.scopes.iter().rev() {
            if let Some(symbol) = scope.lookup(name) {
                return Some(symbol);
            }
        }

        if Self::is_builtin(name) {
            return Some(SymbolRef::BUILTIN);
        }

        // Not found - report error
        self.errors.push(HirError::UndefinedVariable {
            name: name.to_string(),
            span,
        });
        None
    }

    fn is_builtin(name: &str) -> bool {
        BUILTINS.contains(&name)
    }

    fn declare_symbol(&mut self, name: &str, kind: SymbolKind, span: Span) -> Option<SymbolRef> {
        // Check if already declared in current scope
        if let Some(scope) = self.scopes.last() {
            if scope.lookup(name).is_some() {
                self.errors.push(HirError::DuplicateSymbol {
                    name: name.to_string(),
                    original_span: span, // TODO: Get actual original span from existing symbol
                    duplicate_span: span,
                });
                return None;
            }
        }
        
        // Add to current scope
        if let Some(scope) = self.scopes.last_mut() {
            // Create a proper symbol reference based on kind
            let symbol_ref = match kind {
                SymbolKind::Local(_) => {
                    let index = self.local_count;
                    self.local_count += 1;
                    SymbolRef(index)
                },
                SymbolKind::Param(idx) => SymbolRef(idx),
                SymbolKind::Upvalue(idx) => SymbolRef(idx),
                SymbolKind::Global(_) => SymbolRef(0), // Globals use a different indexing scheme
            };
            scope.add(name.to_string(), symbol_ref);
            Some(symbol_ref)
        } else {
            None
        }
    }

    fn begin_scope(&mut self) {
        self.scopes.push(Scope::new());
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }
}
