use brief_ast::{Program, Expr, Stmt, Decl, Block, PostfixOp, BinaryOp};
use brief_diagnostic::Span;
use crate::hir::*;

/// Desugar AST to HIR by removing syntactic sugar
pub fn desugar(program: Program) -> HirProgram {
    let mut desugarer = Desugarer::new();
    desugarer.desugar_program(program)
}

struct Desugarer {
    // Temporary counter for generating unique variable names
    temp_counter: usize,
}

impl Desugarer {
    fn new() -> Self {
        Self {
            temp_counter: 0,
        }
    }

    fn next_temp(&mut self) -> String {
        // Use write! to a pre-allocated String for better performance
        let mut name = String::with_capacity(15); // "__temp_" (7) + up to 8 digits
        use std::fmt::Write;
        let _ = write!(name, "__temp_{}", self.temp_counter);
        self.temp_counter += 1;
        name
    }

    fn desugar_program(&mut self, program: Program) -> HirProgram {
        HirProgram {
            declarations: program.declarations
                .into_iter()
                .map(|d| self.desugar_decl(d))
                .collect(),
            span: program.span,
        }
    }

    fn desugar_decl(&mut self, decl: Decl) -> HirDecl {
        match decl {
            Decl::VarDecl(v) => HirDecl::VarDecl(self.desugar_var_decl(v)),
            Decl::ConstDecl(c) => HirDecl::ConstDecl(self.desugar_const_decl(c)),
            Decl::FuncDecl(f) => HirDecl::FuncDecl(self.desugar_func_decl(f)),
            Decl::ClassDecl(c) => HirDecl::ClassDecl(self.desugar_class_decl(c)),
            Decl::ImportDecl(i) => HirDecl::ImportDecl(HirImportDecl {
                modules: i.modules,
                span: i.span,
            }),
            Decl::Error(span) => HirDecl::Error(span),
        }
    }

    fn desugar_var_decl(&mut self, v: brief_ast::VarDecl) -> HirVarDecl {
        HirVarDecl {
            name: v.name, // Move instead of clone
            symbol: crate::symbol::SymbolRef(0), // Will be set during name resolution
            type_annotation: v.type_annotation,
            initializer: v.initializer.map(|e| self.desugar_expr(e)),
            span: v.span,
        }
    }

    fn desugar_const_decl(&mut self, c: brief_ast::ConstDecl) -> HirConstDecl {
        HirConstDecl {
            name: c.name, // Move instead of clone
            symbol: crate::symbol::SymbolRef(0), // Will be set during name resolution
            initializer: self.desugar_expr(c.initializer),
            span: c.span,
        }
    }

    fn desugar_func_decl(&mut self, f: brief_ast::FuncDecl) -> HirFuncDecl {
        HirFuncDecl {
            name: f.name, // Move instead of clone
            symbol: crate::symbol::SymbolRef(0), // Will be set during name resolution
            params: f.params.into_iter().map(|p| self.desugar_param(p)).collect(),
            return_type: f.return_type,
            body: self.desugar_block(f.body),
            symbol_table: crate::symbol::SymbolTable::new(),
            span: f.span,
        }
    }

    fn desugar_class_decl(&mut self, c: brief_ast::ClassDecl) -> HirClassDecl {
        HirClassDecl {
            name: c.name, // Move instead of clone
            symbol: crate::symbol::SymbolRef(0), // Will be set during name resolution
            constructor: c.constructor.map(|ctor| self.desugar_ctor_decl(ctor)),
            methods: c.methods.into_iter().map(|m| self.desugar_method_decl(m)).collect(),
            span: c.span,
        }
    }

    fn desugar_ctor_decl(&mut self, ctor: brief_ast::CtorDecl) -> HirCtorDecl {
        let mut body = self.desugar_block(ctor.body.clone());
        
        // Desugar implicit assignments: obj.param_name = param_name for each param
        // Only if not explicitly assigned in the body
        let _param_names: std::collections::HashSet<String> = ctor.params
            .iter()
            .map(|p| p.name.clone())
            .collect();
        
        let mut implicit_assigns = Vec::new();
        for param in &ctor.params {
            let param_name = &param.name;
            // Check if this parameter is already assigned in the body
            let already_assigned = body.statements.iter().any(|stmt| {
                matches!(stmt, HirStmt::Expr(expr, _) if {
                    matches!(**expr, HirExpr::Assign { ref target, .. } if {
                        matches!(**target, HirExpr::MemberAccess { ref member, .. } if member == param_name)
                    })
                })
            });
            
            if !already_assigned {
                // Create: obj.param_name = param_name
                let obj_expr = HirExpr::Variable {
                    name: "obj".to_string(),
                    symbol: crate::symbol::SymbolRef(0), // Will be resolved later
                    span: param.span,
                };
                let member_access = HirExpr::MemberAccess {
                    object: Box::new(obj_expr),
                    member: param_name.clone(), // Need to clone here for the member name
                    span: param.span,
                };
                let param_var = HirExpr::Variable {
                    name: param_name.clone(), // Need to clone here for the variable name
                    symbol: crate::symbol::SymbolRef(0), // Will be resolved later
                    span: param.span,
                };
                let assign = HirExpr::Assign {
                    target: Box::new(member_access),
                    value: Box::new(param_var),
                    span: param.span,
                };
                implicit_assigns.push(HirStmt::Expr(Box::new(assign), param.span));
            }
        }
        
        // Prepend implicit assignments to the body
        implicit_assigns.extend(body.statements);
        body.statements = implicit_assigns;
        
        HirCtorDecl {
            name: ctor.name,
            params: ctor.params.into_iter().map(|p| self.desugar_param(p)).collect(),
            body,
            symbol_table: crate::symbol::SymbolTable::new(),
            span: ctor.span,
        }
    }

    fn desugar_method_decl(&mut self, m: brief_ast::MethodDecl) -> HirMethodDecl {
        HirMethodDecl {
            name: m.name.clone(),
            symbol: crate::symbol::SymbolRef(0), // Will be set during name resolution
            is_instance: m.is_instance,
            params: m.params.into_iter().map(|p| self.desugar_param(p)).collect(),
            return_type: m.return_type,
            body: self.desugar_block(m.body),
            symbol_table: crate::symbol::SymbolTable::new(),
            span: m.span,
        }
    }

    fn desugar_param(&mut self, p: brief_ast::Param) -> HirParam {
        HirParam {
            name: p.name, // Already moved, no clone needed
            symbol: crate::symbol::SymbolRef(0), // Will be set during name resolution
            type_annotation: p.type_annotation,
            span: p.span,
        }
    }

    fn desugar_block(&mut self, block: Block) -> HirBlock {
        HirBlock {
            statements: block.statements
                .into_iter()
                .flat_map(|s| self.desugar_stmt(s))
                .collect(),
            span: block.span,
        }
    }

    fn desugar_stmt(&mut self, stmt: Stmt) -> Vec<HirStmt> {
        match stmt {
            Stmt::VarDecl(v) => vec![HirStmt::VarDecl(self.desugar_var_decl(v))],
            Stmt::ConstDecl(c) => vec![HirStmt::ConstDecl(self.desugar_const_decl(c))],
            Stmt::If { condition, then_branch, else_branch, span } => {
                vec![HirStmt::If {
                    condition: Box::new(self.desugar_expr(condition)),
                    then_branch: self.desugar_block(then_branch),
                    else_branch: else_branch.map(|b| self.desugar_block(b)),
                    span,
                }]
            },
            Stmt::While { condition, body, span } => {
                vec![HirStmt::While {
                    condition: Box::new(self.desugar_expr(condition)),
                    body: self.desugar_block(body),
                    span,
                }]
            },
            Stmt::For { init, condition, increment, body, span } => {
                let mut stmts = Vec::new();
                
                // Desugar init
                if let Some(init_stmt) = init {
                    stmts.extend(self.desugar_stmt(*init_stmt));
                }
                
                // Create while loop
                let condition_expr = condition.map(|e| self.desugar_expr(e));
                let body_block = self.desugar_block(body);
                let increment_expr = increment.map(|e| self.desugar_expr(e));
                
                // Build while loop with increment at the end
                let mut while_body_stmts = body_block.statements;
                if let Some(inc) = increment_expr {
                    while_body_stmts.push(HirStmt::Expr(Box::new(inc), span));
                }
                let while_body = HirBlock {
                    statements: while_body_stmts,
                    span: body_block.span,
                };
                
                let while_condition = condition_expr.unwrap_or_else(|| {
                    HirExpr::Boolean(true, span) // Infinite loop if no condition
                });
                
                stmts.push(HirStmt::While {
                    condition: Box::new(while_condition),
                    body: while_body,
                    span,
                });
                
                stmts
            },
            Stmt::ForIn { var, iterable, body, span } => {
                // Desugar: for (v in arr) { body }
                // to:
                //   i := 0
                //   while (i < len(arr))
                //     v := arr[i]
                //     <body>
                //     i++
                
                let index_var = self.next_temp();
                let iterable_expr = self.desugar_expr(iterable);
                let body_block = self.desugar_block(body);
                
                // Create index variable: i := 0
                let index_init = HirStmt::VarDecl(HirVarDecl {
                    name: index_var.clone(),
                    symbol: crate::symbol::SymbolRef(0),
                    type_annotation: None,
                    initializer: Some(HirExpr::Integer(0, span)),
                    span,
                });
                
                // Create loop variable: v := arr[i]
                let index_expr = HirExpr::Variable {
                    name: index_var.clone(),
                    symbol: crate::symbol::SymbolRef(0),
                    span,
                };
                let array_access = HirExpr::Index {
                    object: Box::new(iterable_expr.clone()),
                    index: Box::new(index_expr.clone()),
                    span,
                };
                let loop_var_init = HirStmt::VarDecl(HirVarDecl {
                    name: var.clone(),
                    symbol: crate::symbol::SymbolRef(0),
                    type_annotation: None,
                    initializer: Some(array_access),
                    span,
                });
                
                // Create condition: i < len(arr)
                // For now, use a placeholder - len() will be handled in bytecode
                let len_call = HirExpr::Call {
                    callee: Box::new(HirExpr::Variable {
                        name: "len".to_string(),
                        symbol: crate::symbol::SymbolRef(0),
                        span,
                    }),
                    args: vec![iterable_expr],
                    span,
                };
                let condition = HirExpr::BinaryOp {
                    left: Box::new(index_expr.clone()),
                    op: BinaryOp::Lt,
                    right: Box::new(len_call),
                    span,
                };
                
                // Create increment: i++
                let increment = HirExpr::Assign {
                    target: Box::new(index_expr),
                    value: Box::new(HirExpr::BinaryOp {
                        left: Box::new(HirExpr::Variable {
                            name: index_var.clone(),
                            symbol: crate::symbol::SymbolRef(0),
                            span,
                        }),
                        op: BinaryOp::Add,
                        right: Box::new(HirExpr::Integer(1, span)),
                        span,
                    }),
                    span,
                };
                
                // Build while body: v := arr[i]; <body>; i++
                let mut while_body_stmts = vec![loop_var_init];
                while_body_stmts.extend(body_block.statements);
                while_body_stmts.push(HirStmt::Expr(Box::new(increment), span));
                
                vec![
                    index_init,
                    HirStmt::While {
                        condition: Box::new(condition),
                        body: HirBlock {
                            statements: while_body_stmts,
                            span: body_block.span,
                        },
                        span,
                    },
                ]
            },
            Stmt::Match { expr, cases, else_branch, span } => {
                // Desugar match to if/else chain
                // match(expr) case A, B: ... case C: ... else: ...
                // becomes:
                //   temp := expr
                //   if (temp == A || temp == B) { ... }
                //   else if (temp == C) { ... }
                //   else { ... }
                
                let temp_var = self.next_temp();
                let expr_hir = self.desugar_expr(expr);
                
                // Create temp variable
                let temp_init = HirStmt::VarDecl(HirVarDecl {
                    name: temp_var.clone(),
                    symbol: crate::symbol::SymbolRef(0),
                    type_annotation: None,
                    initializer: Some(expr_hir),
                    span,
                });
                
                // Build if/else chain from cases
                let else_branch_hir = else_branch.map(|b| self.desugar_block(b));
                let mut if_chain = self.build_match_if_chain(
                    &temp_var,
                    cases.into_iter().rev().collect(), // Reverse to build from last to first
                    else_branch_hir,
                    span,
                );
                
                let mut result = vec![temp_init];
                result.append(&mut if_chain);
                result
            },
            Stmt::Return { value, span } => {
                vec![HirStmt::Return {
                    value: value.map(|e| self.desugar_expr(e)),
                    span,
                }]
            },
            Stmt::Break(span) => vec![HirStmt::Break(span)],
            Stmt::Continue(span) => vec![HirStmt::Continue(span)],
            Stmt::Expr(expr, span) => {
                vec![HirStmt::Expr(Box::new(self.desugar_expr(expr)), span)]
            },
            Stmt::Error(span) => vec![HirStmt::Error(span)],
        }
    }

    fn build_match_if_chain(
        &mut self,
        temp_var: &str,
        mut cases: Vec<brief_ast::MatchCase>,
        else_branch: Option<HirBlock>,
        span: Span,
    ) -> Vec<HirStmt> {
        if cases.is_empty() {
            if let Some(else_block) = else_branch {
                return else_block.statements;
            }
            return Vec::new();
        }
        
        let case = cases.pop().unwrap();
        let case_body = self.desugar_block(case.body);
        
        // Build condition: temp == pattern1 || temp == pattern2 || ...
        let temp_expr = HirExpr::Variable {
            name: temp_var.to_string(),
            symbol: crate::symbol::SymbolRef(0),
            span,
        };
        
        let mut condition = None;
        for pattern in case.patterns {
            let pattern_hir = self.desugar_expr(pattern);
            let eq = HirExpr::BinaryOp {
                left: Box::new(temp_expr.clone()),
                op: BinaryOp::Eq,
                right: Box::new(pattern_hir),
                span,
            };
            
            condition = Some(match condition {
                None => eq,
                Some(prev) => HirExpr::BinaryOp {
                    left: Box::new(prev),
                    op: BinaryOp::Or,
                    right: Box::new(eq),
                    span,
                },
            });
        }
        
        let condition = condition.unwrap_or_else(|| {
            HirExpr::Boolean(true, span) // If no patterns, always match
        });
        
        // Build else branch from remaining cases
        let else_branch = if cases.is_empty() {
            else_branch
        } else {
            let else_stmts = self.build_match_if_chain(temp_var, cases, else_branch, span);
            Some(HirBlock {
                statements: else_stmts,
                span,
            })
        };
        
        vec![HirStmt::If {
            condition: Box::new(condition),
            then_branch: case_body,
            else_branch,
            span,
        }]
    }

    fn desugar_expr(&mut self, expr: Expr) -> HirExpr {
        match expr {
            Expr::Integer(n, span) => HirExpr::Integer(n, span),
            Expr::Double(d, span) => HirExpr::Double(d, span),
            Expr::Character(c, span) => HirExpr::Character(c, span),
            Expr::String(s, span) => HirExpr::String(s, span),
            Expr::Boolean(b, span) => HirExpr::Boolean(b, span),
            Expr::Null(span) => HirExpr::Null(span),
            Expr::Variable(name, span) => HirExpr::Variable {
                name,
                symbol: crate::symbol::SymbolRef(0), // Will be set during name resolution
                span,
            },
            Expr::MemberAccess { object, member, span } => {
                HirExpr::MemberAccess {
                    object: Box::new(self.desugar_expr(*object)),
                    member,
                    span,
                }
            },
            Expr::Index { object, index, span } => {
                HirExpr::Index {
                    object: Box::new(self.desugar_expr(*object)),
                    index: Box::new(self.desugar_expr(*index)),
                    span,
                }
            },
            Expr::BinaryOp { left, op, right, span } => {
                HirExpr::BinaryOp {
                    left: Box::new(self.desugar_expr(*left)),
                    op,
                    right: Box::new(self.desugar_expr(*right)),
                    span,
                }
            },
            Expr::UnaryOp { op, expr, span } => {
                HirExpr::UnaryOp {
                    op,
                    expr: Box::new(self.desugar_expr(*expr)),
                    span,
                }
            },
            Expr::PostfixOp { expr, op, span } => {
                // Desugar x++ to x = x + 1
                // Desugar x-- to x = x - 1
                let expr_hir = self.desugar_expr(*expr);
                let one = HirExpr::Integer(1, span);
                let op = match op {
                    PostfixOp::Inc => BinaryOp::Add,
                    PostfixOp::Dec => BinaryOp::Sub,
                };
                HirExpr::Assign {
                    target: Box::new(expr_hir.clone()),
                    value: Box::new(HirExpr::BinaryOp {
                        left: Box::new(expr_hir),
                        op,
                        right: Box::new(one),
                        span,
                    }),
                    span,
                }
            },
            Expr::Call { callee, args, span } => {
                HirExpr::Call {
                    callee: Box::new(self.desugar_expr(*callee)),
                    args: args.into_iter().map(|a| self.desugar_expr(a)).collect(),
                    span,
                }
            },
            Expr::MethodCall { object, method, args, span } => {
                HirExpr::MethodCall {
                    object: Box::new(self.desugar_expr(*object)),
                    method,
                    args: args.into_iter().map(|a| self.desugar_expr(a)).collect(),
                    span,
                }
            },
            Expr::Cast { expr, target_type, span } => {
                HirExpr::Cast {
                    expr: Box::new(self.desugar_expr(*expr)),
                    target_type,
                    span,
                }
            },
            Expr::Interpolation { parts, span } => {
                HirExpr::Interpolation { parts, span }
            },
            Expr::Ternary { condition, then_expr, else_expr, span } => {
                HirExpr::Ternary {
                    condition: Box::new(self.desugar_expr(*condition)),
                    then_expr: Box::new(self.desugar_expr(*then_expr)),
                    else_expr: Box::new(self.desugar_expr(*else_expr)),
                    span,
                }
            },
            Expr::Lambda { params, body, span } => {
                HirExpr::Lambda {
                    params: params.into_iter().map(|p| self.desugar_param(p)).collect(),
                    captures: Vec::new(), // Will be filled during name resolution
                    body: Box::new(self.desugar_expr(*body)),
                    span,
                }
            },
            Expr::Error(span) => HirExpr::Error(span),
        }
    }
}
