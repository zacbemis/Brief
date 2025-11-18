use brief_ast::{InterpPart, BinaryOp};
use brief_bytecode::*;
use crate::hir::*;
use crate::symbol::SymbolRef;

/// Emit bytecode from HIR
pub fn emit(program: &HirProgram) -> Vec<Chunk> {
    let mut emitter = Emitter::new();
    emitter.emit_program(program)
}

struct Emitter {
    chunks: Vec<Chunk>,
    current_chunk: Option<usize>,
    register_counter: u8,
    max_registers: u8,
}

impl Emitter {
    fn new() -> Self {
        Self {
            chunks: Vec::new(),
            current_chunk: None,
            register_counter: 0,
            max_registers: 0,
        }
    }

    fn current_chunk_idx(&self) -> usize {
        self.current_chunk.expect("No current chunk")
    }

    fn allocate_register(&mut self) -> u8 {
        let reg = self.register_counter;
        self.register_counter += 1;
        if self.register_counter > self.max_registers {
            self.max_registers = self.register_counter;
        }
        reg
    }

    fn reserve_register(&mut self, reg: u8) {
        let needed = reg.saturating_add(1);
        if needed > self.register_counter {
            self.register_counter = needed;
        }
        if needed > self.max_registers {
            self.max_registers = needed;
        }
    }

    fn register_for_symbol(&mut self, symbol: SymbolRef) -> u8 {
        let reg = symbol.0 as u8;
        self.reserve_register(reg);
        reg
    }

    fn emit_null_return(&mut self) {
        let null_idx = self.add_constant(Constant::Null);
        let reg = self.allocate_register();
        self.emit_instruction(Instruction::new2(Opcode::LOADK, reg, null_idx));
        self.emit_instruction(Instruction::new1(Opcode::RET, reg));
    }

    fn emit_assign_expr(&mut self, target: &HirExpr, value: &HirExpr, result_reg: u8) {
        if let HirExpr::Variable { name, symbol, .. } = target {
            if *symbol == SymbolRef::BUILTIN {
                panic!("Cannot assign to builtin '{}'", name);
            }
            let dest_reg = self.register_for_symbol(*symbol);
            self.emit_expr(value, dest_reg);
            if dest_reg != result_reg {
                self.emit_instruction(Instruction::new2(Opcode::MOVE, result_reg, dest_reg));
            }
        } else {
            panic!("Complex assignment target not yet supported");
        }
    }

    fn emit_compound_assignment(
        &mut self,
        left: &HirExpr,
        right: &HirExpr,
        result_reg: u8,
        op: BinaryOp,
    ) {
        let (name, symbol) = match left {
            HirExpr::Variable { name, symbol, .. } => (name, symbol),
            _ => panic!("Compound assignment target must be a variable"),
        };

        if *symbol == SymbolRef::BUILTIN {
            panic!("Cannot assign to builtin '{}'", name);
        }

        let dest_reg = self.register_for_symbol(*symbol);
        let right_reg = self.allocate_register();
        self.emit_expr(right, right_reg);

        let opcode = match op {
            BinaryOp::PlusAssign => Opcode::ADD,
            BinaryOp::MinusAssign => Opcode::SUB,
            BinaryOp::StarAssign => Opcode::MUL,
            BinaryOp::SlashAssign => Opcode::DIVF,
            BinaryOp::PercentAssign => Opcode::MOD,
            BinaryOp::PowAssign => Opcode::POW,
            other => panic!("Unsupported compound assignment operator: {:?}", other),
        };

        self.emit_instruction(Instruction::new(opcode, dest_reg, dest_reg, right_reg));
        if dest_reg != result_reg {
            self.emit_instruction(Instruction::new2(Opcode::MOVE, result_reg, dest_reg));
        }
    }

    fn emit_instruction(&mut self, instruction: Instruction) -> usize {
        let idx = self.current_chunk_idx();
        let ip = self.chunks[idx].code.len();
        self.chunks[idx].code.push(instruction);
        ip
    }

    fn add_constant(&mut self, constant: Constant) -> u8 {
        let idx = self.current_chunk_idx();
        self.chunks[idx].add_constant(constant)
    }

    fn get_ip(&self) -> usize {
        let idx = self.current_chunk_idx();
        self.chunks[idx].ip()
    }

    #[allow(dead_code)]
    fn patch_instruction(&mut self, ip: usize, instruction: Instruction) {
        let idx = self.current_chunk_idx();
        self.chunks[idx].patch(ip, instruction);
    }

    fn patch_offset(&mut self, ip: usize, offset: i16) {
        let idx = self.current_chunk_idx();
        let mut inst = self.chunks[idx].code[ip];
        inst.set_offset(offset);
        self.chunks[idx].code[ip] = inst;
    }

    fn patch_jump_target(&mut self, ip: usize, target_ip: usize) {
        let offset = (target_ip as isize - (ip as isize + 1)) as i16;
        self.patch_offset(ip, offset);
    }

    fn emit_program(&mut self, program: &HirProgram) -> Vec<Chunk> {
        // Emit all function declarations as chunks
        for decl in &program.declarations {
            match decl {
                HirDecl::FuncDecl(f) => {
                    self.emit_function(f);
                },
                HirDecl::ClassDecl(c) => {
                    // Emit class methods
                    for method in &c.methods {
                        self.emit_method(method);
                    }
                    // Emit constructor if present
                    if let Some(ctor) = &c.constructor {
                        self.emit_constructor(ctor, &c.name);
                    }
                },
                _ => {
                    // Top-level variables/constants are handled differently
                    // For now, skip them (they'll be in a main function or module init)
                }
            }
        }
        self.chunks.clone()
    }

    fn emit_function(&mut self, func: &HirFuncDecl) {
        let mut chunk = Chunk::new(func.name.clone());
        chunk.param_count = func.params.len() as u8;
        
        self.chunks.push(chunk);
        self.current_chunk = Some(self.chunks.len() - 1);
        self.register_counter = func.params.len() as u8; // Parameters use first registers
        
        // Emit function body (tail expression returns)
        self.emit_block(&func.body, true);
        self.emit_null_return();
        
        // Update chunk metadata
        let idx = self.current_chunk_idx();
        self.chunks[idx].max_regs = self.max_registers;
        self.chunks[idx].upvalue_count = 0; // TODO: Calculate upvalues
        
        self.register_counter = 0;
        self.max_registers = 0;
    }

    fn emit_method(&mut self, method: &HirMethodDecl) {
        let mut chunk = Chunk::new(method.name.clone());
        chunk.param_count = method.params.len() as u8;
        
        self.chunks.push(chunk);
        self.current_chunk = Some(self.chunks.len() - 1);
        self.register_counter = method.params.len() as u8;
        
        // Emit method body
        self.emit_block(&method.body, true);
        self.emit_null_return();
        
        // Update chunk metadata
        let idx = self.current_chunk_idx();
        self.chunks[idx].max_regs = self.max_registers;
        
        self.register_counter = 0;
        self.max_registers = 0;
    }

    fn emit_constructor(&mut self, ctor: &HirCtorDecl, class_name: &str) {
        let name = format!("{}::new", class_name);
        let mut chunk = Chunk::new(name);
        chunk.param_count = ctor.params.len() as u8;
        
        self.chunks.push(chunk);
        self.current_chunk = Some(self.chunks.len() - 1);
        self.register_counter = ctor.params.len() as u8;
        
        // Emit constructor body
        self.emit_block(&ctor.body, true);
        self.emit_null_return();
        
        // Update chunk metadata
        let idx = self.current_chunk_idx();
        self.chunks[idx].max_regs = self.max_registers;
        
        self.register_counter = 0;
        self.max_registers = 0;
    }

    fn emit_block(&mut self, block: &HirBlock, tail_return: bool) {
        let stmt_count = block.statements.len();
        for (idx, stmt) in block.statements.iter().enumerate() {
            let is_tail = tail_return && idx == stmt_count.saturating_sub(1);
            if is_tail {
                match stmt {
                    HirStmt::Expr(expr, _) => {
                        let reg = self.allocate_register();
                        self.emit_expr(expr, reg);
                        self.emit_instruction(Instruction::new1(Opcode::RET, reg));
                        continue;
                    }
                    HirStmt::If { condition, then_branch, else_branch, .. } => {
                        let reg = self.allocate_register();
                        self.emit_if_with_result(condition, then_branch, else_branch, reg);
                        self.emit_instruction(Instruction::new1(Opcode::RET, reg));
                        continue;
                    }
                    _ => {}
                }
            }
            self.emit_stmt(stmt);
        }
    }

    fn emit_block_value(&mut self, block: &HirBlock, target_reg: u8) {
        if block.statements.is_empty() {
            let null_idx = self.add_constant(Constant::Null);
            self.emit_instruction(Instruction::new2(Opcode::LOADK, target_reg, null_idx));
            return;
        }

        let last_idx = block.statements.len() - 1;
        for (idx, stmt) in block.statements.iter().enumerate() {
            if idx == last_idx {
                match stmt {
                    HirStmt::Expr(expr, _) => {
                        self.emit_expr(expr, target_reg);
                    }
                    HirStmt::If { condition, then_branch, else_branch, .. } => {
                        self.emit_if_with_result(condition, then_branch, else_branch, target_reg);
                    }
                    HirStmt::Return { value, .. } => {
                        if let Some(expr) = value {
                            self.emit_expr(expr, target_reg);
                        } else {
                            let null_idx = self.add_constant(Constant::Null);
                            self.emit_instruction(Instruction::new2(Opcode::LOADK, target_reg, null_idx));
                        }
                    }
                    _ => {
                        self.emit_stmt(stmt);
                        let null_idx = self.add_constant(Constant::Null);
                        self.emit_instruction(Instruction::new2(Opcode::LOADK, target_reg, null_idx));
                    }
                }
            } else {
                self.emit_stmt(stmt);
            }
        }
    }

    fn emit_if_with_result(&mut self, condition: &HirExpr, then_branch: &HirBlock, else_branch: &Option<HirBlock>, result_reg: u8) {
        let cond_reg = self.allocate_register();
        self.emit_expr(condition, cond_reg);

        let jmp_if_false_ip = self.get_ip();
        self.emit_instruction(Instruction::new2(Opcode::JIF, cond_reg, 0));

        self.emit_block_value(then_branch, result_reg);
        let jump_over_else_ip = self.get_ip();
        self.emit_instruction(Instruction::new1(Opcode::JMP, 0));

        let else_start_ip = self.get_ip();
        self.patch_jump_target(jmp_if_false_ip, else_start_ip);

        if let Some(else_branch) = else_branch {
            self.emit_block_value(else_branch, result_reg);
        } else {
            let null_idx = self.add_constant(Constant::Null);
            self.emit_instruction(Instruction::new2(Opcode::LOADK, result_reg, null_idx));
        }

        let else_end_ip = self.get_ip();
        self.patch_jump_target(jump_over_else_ip, else_end_ip);
    }

    fn emit_stmt(&mut self, stmt: &HirStmt) {
        match stmt {
            HirStmt::VarDecl(v) => {
                let target_reg = self.register_for_symbol(v.symbol);
                if let Some(init) = &v.initializer {
                    self.emit_expr(init, target_reg);
                } else {
                    let null_idx = self.add_constant(Constant::Null);
                    self.emit_instruction(Instruction::new2(Opcode::LOADK, target_reg, null_idx));
                }
            },
            HirStmt::ConstDecl(c) => {
                let target_reg = self.register_for_symbol(c.symbol);
                self.emit_expr(&c.initializer, target_reg);
            },
            HirStmt::If { condition, then_branch, else_branch, .. } => {
                self.emit_if(condition, then_branch, else_branch);
            },
            HirStmt::While { condition, body, .. } => {
                self.emit_while(condition, body);
            },
            HirStmt::For { init, condition, increment, body, .. } => {
                self.emit_for(init, condition, increment, body);
            },
            HirStmt::Return { value, .. } => {
                if let Some(value) = value {
                    let reg = self.allocate_register();
                    self.emit_expr(value, reg);
                    self.emit_instruction(Instruction::new1(Opcode::RET, reg));
                } else {
                    // Return null
                    let null_idx = self.add_constant(Constant::Null);
                    let reg = self.allocate_register();
                    self.emit_instruction(Instruction::new2(Opcode::LOADK, reg, null_idx));
                    self.emit_instruction(Instruction::new1(Opcode::RET, reg));
                }
            },
            HirStmt::Break(_) | HirStmt::Continue(_) => {
                // TODO: Implement break/continue (needs loop context)
            },
            HirStmt::Expr(expr, _) => {
                let reg = self.allocate_register();
                self.emit_expr(expr, reg);
            },
            HirStmt::Error(_) => {
                // Skip error nodes
            },
        }
    }

    fn emit_if(&mut self, condition: &HirExpr, then_branch: &HirBlock, else_branch: &Option<HirBlock>) {
        let cond_reg = self.allocate_register();
        self.emit_expr(condition, cond_reg);
        
        let jmp_if_false_ip = self.get_ip();
        self.emit_instruction(Instruction::new2(Opcode::JIF, cond_reg, 0)); // Offset patched later
        
        // Emit then branch
        self.emit_block(then_branch, false);
        
        let then_end_ip = self.get_ip();
        let else_start_ip = if else_branch.is_some() {
            // Emit jump over else branch
            let jmp_over_else_ip = self.get_ip();
            self.emit_instruction(Instruction::new1(Opcode::JMP, 0)); // Offset patched later
            jmp_over_else_ip
        } else {
            then_end_ip
        };
        
        // Patch JIF offset
        self.patch_jump_target(jmp_if_false_ip, else_start_ip);
        
        // Emit else branch if present
        if let Some(else_branch) = else_branch {
            self.emit_block(else_branch, false);
            let else_end_ip = self.get_ip();
            self.patch_jump_target(else_start_ip, else_end_ip);
        }
    }

    fn emit_while(&mut self, condition: &HirExpr, body: &HirBlock) {
        let loop_start_ip = self.get_ip();
        
        // Emit condition
        let cond_reg = self.allocate_register();
        self.emit_expr(condition, cond_reg);
        
        // Jump if false (to end)
        let jmp_if_false_ip = self.get_ip();
        self.emit_instruction(Instruction::new2(Opcode::JIF, cond_reg, 0)); // Offset patched later
        
        // Emit body
        self.emit_block(body, false);
        
        // Jump back to start
        let loop_end_ip = self.get_ip();
        let back_jmp_offset = (loop_start_ip as i16) - (loop_end_ip as i16) - 1;
        self.emit_instruction(Instruction::new1(Opcode::JMP, 0));
        self.patch_offset(loop_end_ip, back_jmp_offset);
        
        // Patch JIF to jump to end
        self.patch_jump_target(jmp_if_false_ip, loop_end_ip + 1);
    }

    fn emit_for(&mut self, init: &Option<Box<HirStmt>>, condition: &Option<Box<HirExpr>>, increment: &Option<Box<HirExpr>>, body: &HirBlock) {
        // Emit init
        if let Some(init) = init {
            self.emit_stmt(init);
        }
        
        let loop_start_ip = self.get_ip();
        
        // Emit condition (or use true if no condition)
        let cond_reg = if let Some(condition) = condition {
            let reg = self.allocate_register();
            self.emit_expr(condition, reg);
            reg
        } else {
            // Infinite loop - load true
            let true_idx = self.add_constant(Constant::Bool(true));
            let reg = self.allocate_register();
            self.emit_instruction(Instruction::new2(Opcode::LOADK, reg, true_idx));
            reg
        };
        
        // Jump if false (to end)
        let jmp_if_false_ip = self.get_ip();
        self.emit_instruction(Instruction::new2(Opcode::JIF, cond_reg, 0)); // Offset patched later
        
        // Emit body
        self.emit_block(body, false);
        
        // Emit increment
        if let Some(increment) = increment {
            let inc_reg = self.allocate_register();
            self.emit_expr(increment, inc_reg);
        }
        
        // Jump back to start
        let loop_end_ip = self.get_ip();
        let back_jmp_offset = (loop_start_ip as i16) - (loop_end_ip as i16) - 1;
        self.emit_instruction(Instruction::new1(Opcode::JMP, 0));
        self.patch_offset(loop_end_ip, back_jmp_offset);
        
        // Patch JIF to jump to end
        self.patch_jump_target(jmp_if_false_ip, loop_end_ip + 1);
    }

    fn emit_expr(&mut self, expr: &HirExpr, target_reg: u8) {
        match expr {
            HirExpr::Integer(n, _) => {
                let idx = self.add_constant(Constant::Int(*n));
                self.emit_instruction(Instruction::new2(Opcode::LOADK, target_reg, idx));
            },
            HirExpr::Double(d, _) => {
                let idx = self.add_constant(Constant::Double(*d));
                self.emit_instruction(Instruction::new2(Opcode::LOADK, target_reg, idx));
            },
            HirExpr::Boolean(b, _) => {
                let idx = self.add_constant(Constant::Bool(*b));
                self.emit_instruction(Instruction::new2(Opcode::LOADK, target_reg, idx));
            },
            HirExpr::String(s, _) => {
                let idx = self.add_constant(Constant::Str(s.clone()));
                self.emit_instruction(Instruction::new2(Opcode::LOADK, target_reg, idx));
            },
            HirExpr::Null(_) => {
                let idx = self.add_constant(Constant::Null);
                self.emit_instruction(Instruction::new2(Opcode::LOADK, target_reg, idx));
            },
            HirExpr::Character(c, _) => {
                // Characters are represented as integers in bytecode
                let idx = self.add_constant(Constant::Int(*c as i64));
                self.emit_instruction(Instruction::new2(Opcode::LOADK, target_reg, idx));
            },
            HirExpr::Variable { name, symbol, .. } => {
                if *symbol == SymbolRef::BUILTIN {
                    let idx = self.add_constant(Constant::Str(name.clone()));
                    self.emit_instruction(Instruction::new2(Opcode::LOADK, target_reg, idx));
                } else {
                    let src_reg = self.register_for_symbol(*symbol);
                    if src_reg != target_reg {
                        self.emit_instruction(Instruction::new2(Opcode::MOVE, target_reg, src_reg));
                    }
                }
            },
            HirExpr::BinaryOp { left, op, right, .. } => {
                if matches!(op, brief_ast::BinaryOp::Assign | brief_ast::BinaryOp::InitAssign) {
                    self.emit_assign_expr(left, right, target_reg);
                    return;
                }
                match op {
                    brief_ast::BinaryOp::And => {
                        self.emit_expr(left, target_reg);
                        let jif_ip = self.get_ip();
                        self.emit_instruction(Instruction::new2(Opcode::JIF, target_reg, 0));
                        self.emit_expr(right, target_reg);
                        let end_ip = self.get_ip();
                        self.patch_jump_target(jif_ip, end_ip);
                    },
                    brief_ast::BinaryOp::Or => {
                        self.emit_expr(left, target_reg);
                        let jif_ip = self.get_ip();
                        self.emit_instruction(Instruction::new2(Opcode::JIF, target_reg, 0));
                        let skip_ip = self.get_ip();
                        self.emit_instruction(Instruction::new1(Opcode::JMP, 0));
                        let right_start = self.get_ip();
                        self.patch_jump_target(jif_ip, right_start);
                        self.emit_expr(right, target_reg);
                        let end_ip = self.get_ip();
                        self.patch_jump_target(skip_ip, end_ip);
                    },
                    brief_ast::BinaryOp::PlusAssign
                    | brief_ast::BinaryOp::MinusAssign
                    | brief_ast::BinaryOp::StarAssign
                    | brief_ast::BinaryOp::SlashAssign
                    | brief_ast::BinaryOp::PercentAssign
                    | brief_ast::BinaryOp::PowAssign => {
                        self.emit_compound_assignment(left, right, target_reg, *op);
                    },
                    _ => {
                        let left_reg = self.allocate_register();
                        let right_reg = self.allocate_register();
                        self.emit_expr(left, left_reg);
                        self.emit_expr(right, right_reg);
                        
                        let opcode = match op {
                            brief_ast::BinaryOp::Add => Opcode::ADD,
                            brief_ast::BinaryOp::Sub => Opcode::SUB,
                            brief_ast::BinaryOp::Mul => Opcode::MUL,
                            brief_ast::BinaryOp::Div => Opcode::DIVF, // Default to float division
                            brief_ast::BinaryOp::Mod => Opcode::MOD,
                            brief_ast::BinaryOp::Pow => Opcode::POW,
                            brief_ast::BinaryOp::Eq => Opcode::CMP_EQ,
                            brief_ast::BinaryOp::Ne => Opcode::CMP_NE,
                            brief_ast::BinaryOp::Lt => Opcode::CMP_LT,
                            brief_ast::BinaryOp::Le => Opcode::CMP_LE,
                            brief_ast::BinaryOp::Gt => Opcode::CMP_GT,
                            brief_ast::BinaryOp::Ge => Opcode::CMP_GE,
                            _ => panic!("Unexpected binary operator in HIR: {:?}", op),
                        };
                        
                        self.emit_instruction(Instruction::new(opcode, target_reg, left_reg, right_reg));
                    }
                }
            },
            HirExpr::UnaryOp { op, expr, .. } => {
                let expr_reg = self.allocate_register();
                self.emit_expr(expr, expr_reg);
                
                let opcode = match op {
                    brief_ast::UnaryOp::Neg => Opcode::NEG,
                    brief_ast::UnaryOp::Not => Opcode::NOT,
                    _ => panic!("Unsupported unary operator"),
                };
                
                self.emit_instruction(Instruction::new2(opcode, target_reg, expr_reg));
            },
            HirExpr::Assign { target, value, .. } => {
                // Emit value
                let value_reg = self.allocate_register();
                self.emit_expr(value, value_reg);
                
                // Emit target (get register)
                // For now, assume target is a variable
                if let HirExpr::Variable { name, symbol, .. } = target.as_ref() {
                    if *symbol == SymbolRef::BUILTIN {
                        panic!("Cannot assign to builtin '{}'", name);
                    }
                    let target_reg = self.register_for_symbol(*symbol);
                    self.emit_instruction(Instruction::new2(Opcode::MOVE, target_reg, value_reg));
                } else {
                    // TODO: Handle member access, index, etc.
                    panic!("Complex assignment target not yet supported");
                }
            },
            HirExpr::Call { callee, args, .. } => {
                // Emit callee
                let callee_reg = self.allocate_register();
                self.emit_expr(callee, callee_reg);
                
                // Emit arguments
                let arg_regs: Vec<u8> = args.iter().map(|arg| {
                    let reg = self.allocate_register();
                    self.emit_expr(arg, reg);
                    reg
                }).collect();
                
                // For now, assume first arg is in callee_reg+1
                // TODO: Proper argument passing
                if !arg_regs.is_empty() {
                    // Move args to consecutive registers
                    for (i, arg_reg) in arg_regs.iter().enumerate() {
                        let dest_reg = callee_reg + 1 + i as u8;
                        if *arg_reg != dest_reg {
                            self.emit_instruction(Instruction::new2(Opcode::MOVE, dest_reg, *arg_reg));
                        }
                    }
                }
                
                self.emit_instruction(Instruction::new(Opcode::CALL, target_reg, callee_reg, args.len() as u8));
            },
            HirExpr::MethodCall { object, .. } => {
                // TODO: Implement method calls
                // For now, treat as regular call
                let obj_reg = self.allocate_register();
                self.emit_expr(object, obj_reg);
                
                // Emit method call (simplified)
                // TODO: Proper method dispatch
                panic!("Method calls not yet implemented");
            },
            HirExpr::MemberAccess { .. } => {
                // TODO: Implement member access
                panic!("Member access not yet implemented");
            },
            HirExpr::Index { .. } => {
                // TODO: Implement index access
                panic!("Index access not yet implemented");
            },
            HirExpr::Cast { .. } => {
                // TODO: Implement type casting
                panic!("Type casting not yet implemented");
            },
            HirExpr::Interpolation { parts, .. } => {
                // Support plain strings (no embedded expressions) for now
                if parts.iter().all(|part| matches!(part, InterpPart::Text(_))) {
                    let mut text = String::new();
                    for part in parts {
                        if let InterpPart::Text(chunk) = part {
                            text.push_str(chunk);
                        }
                    }
                    let idx = self.add_constant(Constant::Str(text));
                    self.emit_instruction(Instruction::new2(Opcode::LOADK, target_reg, idx));
                } else {
                    // TODO: Implement string interpolation lowering
                    panic!("String interpolation with expressions not yet implemented");
                }
            },
            HirExpr::Ternary { condition, then_expr, else_expr, .. } => {
                // Emit as if/else
                let cond_reg = self.allocate_register();
                self.emit_expr(condition, cond_reg);
                
                let jmp_if_false_ip = self.get_ip();
                self.emit_instruction(Instruction::new2(Opcode::JIF, cond_reg, 0));
                
                // Emit then
                self.emit_expr(then_expr, target_reg);
                
                let then_end_ip = self.get_ip();
                let jmp_over_else_ip = self.get_ip();
                self.emit_instruction(Instruction::new1(Opcode::JMP, 0));
                
                // Patch JIF
                let else_offset = (then_end_ip - jmp_if_false_ip) as i16;
                self.patch_offset(jmp_if_false_ip, else_offset);
                
                // Emit else
                self.emit_expr(else_expr, target_reg);
                
                // Patch jump over else
                let else_end_ip = self.get_ip();
                let jmp_offset = (else_end_ip - jmp_over_else_ip) as i16;
                self.patch_offset(jmp_over_else_ip, jmp_offset);
            },
            HirExpr::Lambda { .. } => {
                // TODO: Implement lambda compilation
                panic!("Lambda compilation not yet implemented");
            },
            HirExpr::Error(_) => {
                // Emit null for error nodes
                let idx = self.add_constant(Constant::Null);
                self.emit_instruction(Instruction::new2(Opcode::LOADK, target_reg, idx));
            },
        }
    }
}
