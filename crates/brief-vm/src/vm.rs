use std::rc::Rc;
use std::collections::HashMap;
use brief_bytecode::{Chunk, Opcode, Constant};
use crate::value::Value;
use crate::frame::Frame;
use crate::heap::Heap;
use crate::error::RuntimeError;

/// Virtual Machine for executing Brief bytecode
pub struct VM {
    frames: Vec<Frame>,
    heap: Heap,
    globals: HashMap<String, Value>,
    // Runtime for builtin functions (optional, stored as trait object to avoid circular dependency)
    runtime: Option<Box<dyn BuiltinRuntime>>,
}

/// Trait for builtin function runtime (to avoid circular dependency)
pub trait BuiltinRuntime: Send + Sync {
    fn call_builtin(&self, name: &str, args: &[Value]) -> Result<Value, RuntimeError>;
    fn is_builtin(&self, name: &str) -> bool;
}

impl VM {
    pub fn new() -> Self {
        Self {
            frames: Vec::new(),
            heap: Heap::new(),
            globals: HashMap::new(),
            runtime: None,
        }
    }
    
    /// Set the runtime
    pub fn set_runtime(&mut self, runtime: Box<dyn BuiltinRuntime>) {
        self.runtime = Some(runtime);
    }

    /// Get current frame (mutable)
    fn current_frame_mut(&mut self) -> Result<&mut Frame, RuntimeError> {
        self.frames.last_mut().ok_or(RuntimeError::StackUnderflow)
    }

    /// Get current frame (immutable)
    fn current_frame(&self) -> Result<&Frame, RuntimeError> {
        self.frames.last().ok_or(RuntimeError::StackUnderflow)
    }

    /// Push a new frame onto the call stack
    pub fn push_frame(&mut self, chunk: Rc<Chunk>, base: usize) {
        self.frames.push(Frame::new(chunk, base));
    }

    /// Pop the current frame from the call stack
    fn pop_frame(&mut self) -> Option<Frame> {
        self.frames.pop()
    }

    /// Run the VM until completion
    pub fn run(&mut self) -> Result<Value, RuntimeError> {
        loop {
            let frame = self.current_frame_mut()?;
            
            let instruction = match frame.current_instruction() {
                Some(inst) => *inst,
                None => {
                    // End of function - return null
                    self.pop_frame();
                    if self.frames.is_empty() {
                        return Ok(Value::Null);
                    }
                    continue;
                }
            };

            frame.advance();

            match instruction.opcode() {
                Opcode::LOADK => {
                    let reg = instruction.a();
                    let const_idx = instruction.b();
                    self.load_constant(reg, const_idx)?;
                },
                Opcode::MOVE => {
                    let dest = instruction.a();
                    let src = instruction.b();
                    self.move_register(dest, src)?;
                },
                Opcode::ADD => {
                    let dest = instruction.a();
                    let left = instruction.b();
                    let right = instruction.c();
                    self.binary_op_impl(dest, left, right, Self::add_value)?;
                },
                Opcode::SUB => {
                    let dest = instruction.a();
                    let left = instruction.b();
                    let right = instruction.c();
                    self.binary_op_impl(dest, left, right, Self::sub_value)?;
                },
                Opcode::MUL => {
                    let dest = instruction.a();
                    let left = instruction.b();
                    let right = instruction.c();
                    self.binary_op_impl(dest, left, right, Self::mul_value)?;
                },
                Opcode::DIVF => {
                    let dest = instruction.a();
                    let left = instruction.b();
                    let right = instruction.c();
                    self.binary_op_impl(dest, left, right, Self::divf_value)?;
                },
                Opcode::DIVI => {
                    let dest = instruction.a();
                    let left = instruction.b();
                    let right = instruction.c();
                    self.binary_op_impl(dest, left, right, Self::divi_value)?;
                },
                Opcode::MOD => {
                    let dest = instruction.a();
                    let left = instruction.b();
                    let right = instruction.c();
                    self.binary_op_impl(dest, left, right, Self::mod_value)?;
                },
                Opcode::POW => {
                    let dest = instruction.a();
                    let left = instruction.b();
                    let right = instruction.c();
                    self.binary_op_impl(dest, left, right, Self::pow_value)?;
                },
                Opcode::CMP_EQ => {
                    let dest = instruction.a();
                    let left = instruction.b();
                    let right = instruction.c();
                    self.binary_op_impl(dest, left, right, |a, b| Ok(Value::Bool(a == b)))?;
                },
                Opcode::CMP_NE => {
                    let dest = instruction.a();
                    let left = instruction.b();
                    let right = instruction.c();
                    self.binary_op_impl(dest, left, right, |a, b| Ok(Value::Bool(a != b)))?;
                },
                Opcode::CMP_LT => {
                    let dest = instruction.a();
                    let left = instruction.b();
                    let right = instruction.c();
                    self.binary_op_impl(dest, left, right, Self::cmp_lt_value)?;
                },
                Opcode::CMP_LE => {
                    let dest = instruction.a();
                    let left = instruction.b();
                    let right = instruction.c();
                    self.binary_op_impl(dest, left, right, Self::cmp_le_value)?;
                },
                Opcode::CMP_GT => {
                    let dest = instruction.a();
                    let left = instruction.b();
                    let right = instruction.c();
                    self.binary_op_impl(dest, left, right, Self::cmp_gt_value)?;
                },
                Opcode::CMP_GE => {
                    let dest = instruction.a();
                    let left = instruction.b();
                    let right = instruction.c();
                    self.binary_op_impl(dest, left, right, Self::cmp_ge_value)?;
                },
                Opcode::NEG => {
                    let dest = instruction.a();
                    let src = instruction.b();
                    self.unary_op_impl(dest, src, Self::neg_value)?;
                },
                Opcode::NOT => {
                    let dest = instruction.a();
                    let src = instruction.b();
                    self.unary_op_impl(dest, src, |v| Ok(Value::Bool(!v.is_truthy())))?;
                },
                Opcode::JIF => {
                    let cond_reg = instruction.a();
                    let offset = instruction.offset();
                    self.jump_if_false(cond_reg, offset)?;
                },
                Opcode::JMP => {
                    let offset = instruction.offset();
                    self.jump(offset)?;
                },
                Opcode::CALL => {
                    let dest = instruction.a();
                    let callee_reg = instruction.b();
                    let arg_count = instruction.c();
                    self.call(dest, callee_reg, arg_count)?;
                },
                Opcode::RET => {
                    let value_reg = instruction.a();
                    return self.return_value(value_reg);
                },
                Opcode::PRINT => {
                    let reg = instruction.a();
                    self.print(reg)?;
                },
                _ => {
                    return Err(RuntimeError::UnknownOpcode);
                }
            }
        }
    }

    // Helper methods for opcode execution

    fn load_constant(&mut self, reg: u8, const_idx: u8) -> Result<(), RuntimeError> {
        let frame = self.current_frame_mut()?;
        let constant = frame.chunk.constants.get(const_idx as usize)
            .ok_or(RuntimeError::InvalidConstantIndex(const_idx))?;
        
        let value = match constant {
            Constant::Int(n) => Value::Int(*n),
            Constant::Double(d) => Value::Double(*d),
            Constant::Bool(b) => Value::Bool(*b),
            Constant::Str(s) => Value::Str(s.clone()),
            Constant::Null => Value::Null,
        };

        if reg as usize >= frame.registers.len() {
            return Err(RuntimeError::InvalidRegister(reg));
        }
        frame.registers[reg as usize] = value;
        Ok(())
    }

    fn move_register(&mut self, dest: u8, src: u8) -> Result<(), RuntimeError> {
        let frame = self.current_frame_mut()?;
        if src as usize >= frame.registers.len() || dest as usize >= frame.registers.len() {
            return Err(RuntimeError::InvalidRegister(if src as usize >= frame.registers.len() { src } else { dest }));
        }
        // Use clone for now (Value is Clone, and we may need the source later)
        // TODO: Consider move optimization if source register is dead
        frame.registers[dest as usize] = frame.registers[src as usize].clone();
        Ok(())
    }

    fn binary_op_impl<F>(&mut self, dest: u8, left_reg: u8, right_reg: u8, op: F) -> Result<(), RuntimeError>
    where
        F: FnOnce(&Value, &Value) -> Result<Value, RuntimeError>,
    {
        let frame = self.current_frame_mut()?;
        if left_reg as usize >= frame.registers.len() || 
           right_reg as usize >= frame.registers.len() || 
           dest as usize >= frame.registers.len() {
            return Err(RuntimeError::InvalidRegister(dest));
        }
        let left = frame.registers[left_reg as usize].clone();
        let right = frame.registers[right_reg as usize].clone();
        let result = op(&left, &right)?;
        frame.registers[dest as usize] = result;
        Ok(())
    }

    fn unary_op_impl<F>(&mut self, dest: u8, src_reg: u8, op: F) -> Result<(), RuntimeError>
    where
        F: FnOnce(&Value) -> Result<Value, RuntimeError>,
    {
        let frame = self.current_frame_mut()?;
        if src_reg as usize >= frame.registers.len() || dest as usize >= frame.registers.len() {
            return Err(RuntimeError::InvalidRegister(if src_reg as usize >= frame.registers.len() { src_reg } else { dest }));
        }
        let value = frame.registers[src_reg as usize].clone();
        let result = op(&value)?;
        frame.registers[dest as usize] = result;
        Ok(())
    }

    fn jump_if_false(&mut self, cond_reg: u8, offset: i16) -> Result<(), RuntimeError> {
        let frame = self.current_frame_mut()?;
        if cond_reg as usize >= frame.registers.len() {
            return Err(RuntimeError::InvalidRegister(cond_reg));
        }
        let cond = &frame.registers[cond_reg as usize];
        if !cond.is_truthy() {
            // Jump: offset is relative to current IP
            let new_ip = (frame.ip as i32 + offset as i32) as usize;
            if new_ip > frame.chunk.code.len() {
                return Err(RuntimeError::CallError("Jump out of bounds".to_string()));
            }
            frame.ip = new_ip;
        }
        Ok(())
    }

    fn jump(&mut self, offset: i16) -> Result<(), RuntimeError> {
        let frame = self.current_frame_mut()?;
        let new_ip = (frame.ip as i32 + offset as i32) as usize;
        if new_ip > frame.chunk.code.len() {
            return Err(RuntimeError::CallError("Jump out of bounds".to_string()));
        }
        frame.ip = new_ip;
        Ok(())
    }

    fn call(&mut self, dest: u8, callee_reg: u8, arg_count: u8) -> Result<(), RuntimeError> {
        // Extract all needed data first (function name and args)
        let (function_name, args) = {
            let frame = self.current_frame_mut()?;
            if callee_reg as usize >= frame.registers.len() {
                return Err(RuntimeError::InvalidRegister(callee_reg));
            }
            
            // Extract function name if it's a string
            let function_name = match &frame.registers[callee_reg as usize] {
                Value::Str(name) => Some(name.clone()),
                _ => None,
            };
            
            // Collect arguments (starting at callee_reg + 1)
            let mut args = Vec::new();
            for i in 0..arg_count {
                let arg_reg = callee_reg + 1 + i;
                if arg_reg as usize >= frame.registers.len() {
                    return Err(RuntimeError::InvalidRegister(arg_reg));
                }
                args.push(frame.registers[arg_reg as usize].clone());
            }
            
            (function_name, args)
        };
        
        // For now, assume callee is a string (function name) for builtin calls
        // TODO: Support actual function objects when they're implemented
        if let Some(function_name) = function_name {
            // Try to call as builtin
            let result = if let Some(runtime) = &self.runtime {
                runtime.call_builtin(&function_name, &args)?
            } else {
                return Err(RuntimeError::CallError("Runtime not available for builtin calls".to_string()));
            };
            
            // Store result in destination register
            let frame = self.current_frame_mut()?;
            if dest as usize >= frame.registers.len() {
                return Err(RuntimeError::InvalidRegister(dest));
            }
            frame.registers[dest as usize] = result;
            Ok(())
        } else {
            // TODO: Support function objects
            Err(RuntimeError::CallError("Function calls not yet fully implemented".to_string()))
        }
    }

    fn return_value(&mut self, value_reg: u8) -> Result<Value, RuntimeError> {
        let frame = self.current_frame_mut()?;
        if value_reg as usize >= frame.registers.len() {
            return Err(RuntimeError::InvalidRegister(value_reg));
        }
        let value = frame.registers[value_reg as usize].clone();
        self.pop_frame();
        
        if self.frames.is_empty() {
            Ok(value)
        } else {
            // TODO: Store return value in calling frame
            Ok(value)
        }
    }

    fn print(&mut self, reg: u8) -> Result<(), RuntimeError> {
        let frame = self.current_frame()?;
        if reg as usize >= frame.registers.len() {
            return Err(RuntimeError::InvalidRegister(reg));
        }
        let value = &frame.registers[reg as usize];
        println!("{}", value);
        Ok(())
    }

    // Arithmetic operations (static methods to avoid borrow issues)

    fn add_value(left: &Value, right: &Value) -> Result<Value, RuntimeError> {
        match (left, right) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a + b)),
            (Value::Double(a), Value::Double(b)) => Ok(Value::Double(a + b)),
            (Value::Int(a), Value::Double(b)) => Ok(Value::Double(*a as f64 + b)),
            (Value::Double(a), Value::Int(b)) => Ok(Value::Double(a + *b as f64)),
            (Value::Str(a), Value::Str(b)) => {
                // Optimize string concatenation with pre-allocated capacity
                let mut result = String::with_capacity(a.len() + b.len());
                result.push_str(a);
                result.push_str(b);
                Ok(Value::Str(result))
            },
            (Value::Str(a), b) => {
                let b_str = b.to_string();
                let mut result = String::with_capacity(a.len() + b_str.len());
                result.push_str(a);
                result.push_str(&b_str);
                Ok(Value::Str(result))
            },
            (a, Value::Str(b)) => {
                let a_str = a.to_string();
                let mut result = String::with_capacity(a_str.len() + b.len());
                result.push_str(&a_str);
                result.push_str(b);
                Ok(Value::Str(result))
            },
            _ => Err(RuntimeError::TypeMismatch {
                expected: "numeric or string".to_string(),
                got: format!("{:?} + {:?}", left, right),
            }),
        }
    }

    fn sub_value(left: &Value, right: &Value) -> Result<Value, RuntimeError> {
        match (left, right) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a - b)),
            (Value::Double(a), Value::Double(b)) => Ok(Value::Double(a - b)),
            (Value::Int(a), Value::Double(b)) => Ok(Value::Double(*a as f64 - b)),
            (Value::Double(a), Value::Int(b)) => Ok(Value::Double(a - *b as f64)),
            _ => Err(RuntimeError::TypeMismatch {
                expected: "numeric".to_string(),
                got: format!("{:?} - {:?}", left, right),
            }),
        }
    }

    fn mul_value(left: &Value, right: &Value) -> Result<Value, RuntimeError> {
        match (left, right) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a * b)),
            (Value::Double(a), Value::Double(b)) => Ok(Value::Double(a * b)),
            (Value::Int(a), Value::Double(b)) => Ok(Value::Double(*a as f64 * b)),
            (Value::Double(a), Value::Int(b)) => Ok(Value::Double(a * *b as f64)),
            _ => Err(RuntimeError::TypeMismatch {
                expected: "numeric".to_string(),
                got: format!("{:?} * {:?}", left, right),
            }),
        }
    }

    fn divf_value(left: &Value, right: &Value) -> Result<Value, RuntimeError> {
        match (left, right) {
            (Value::Int(a), Value::Int(b)) => {
                if *b == 0 {
                    Err(RuntimeError::DivisionByZero)
                } else {
                    Ok(Value::Double(*a as f64 / *b as f64))
                }
            },
            (Value::Double(a), Value::Double(b)) => {
                if *b == 0.0 {
                    Err(RuntimeError::DivisionByZero)
                } else {
                    Ok(Value::Double(a / b))
                }
            },
            (Value::Int(a), Value::Double(b)) => {
                if *b == 0.0 {
                    Err(RuntimeError::DivisionByZero)
                } else {
                    Ok(Value::Double(*a as f64 / b))
                }
            },
            (Value::Double(a), Value::Int(b)) => {
                if *b == 0 {
                    Err(RuntimeError::DivisionByZero)
                } else {
                    Ok(Value::Double(a / *b as f64))
                }
            },
            _ => Err(RuntimeError::TypeMismatch {
                expected: "numeric".to_string(),
                got: format!("{:?} / {:?}", left, right),
            }),
        }
    }

    fn divi_value(left: &Value, right: &Value) -> Result<Value, RuntimeError> {
        match (left, right) {
            (Value::Int(a), Value::Int(b)) => {
                if *b == 0 {
                    Err(RuntimeError::DivisionByZero)
                } else {
                    Ok(Value::Int(a / b))
                }
            },
            (Value::Double(a), Value::Double(b)) => {
                if *b == 0.0 {
                    Err(RuntimeError::DivisionByZero)
                } else {
                    Ok(Value::Int((a / b) as i64))
                }
            },
            (Value::Int(a), Value::Double(b)) => {
                if *b == 0.0 {
                    Err(RuntimeError::DivisionByZero)
                } else {
                    Ok(Value::Int((*a as f64 / b) as i64))
                }
            },
            (Value::Double(a), Value::Int(b)) => {
                if *b == 0 {
                    Err(RuntimeError::DivisionByZero)
                } else {
                    Ok(Value::Int((a / *b as f64) as i64))
                }
            },
            _ => Err(RuntimeError::TypeMismatch {
                expected: "numeric".to_string(),
                got: format!("{:?} / {:?}", left, right),
            }),
        }
    }

    fn mod_value(left: &Value, right: &Value) -> Result<Value, RuntimeError> {
        match (left, right) {
            (Value::Int(a), Value::Int(b)) => {
                if *b == 0 {
                    Err(RuntimeError::DivisionByZero)
                } else {
                    Ok(Value::Int(a % b))
                }
            },
            (Value::Double(a), Value::Double(b)) => {
                if *b == 0.0 {
                    Err(RuntimeError::DivisionByZero)
                } else {
                    Ok(Value::Double(a % b))
                }
            },
            (Value::Int(a), Value::Double(b)) => {
                if *b == 0.0 {
                    Err(RuntimeError::DivisionByZero)
                } else {
                    Ok(Value::Double(*a as f64 % b))
                }
            },
            (Value::Double(a), Value::Int(b)) => {
                if *b == 0 {
                    Err(RuntimeError::DivisionByZero)
                } else {
                    Ok(Value::Double(a % *b as f64))
                }
            },
            _ => Err(RuntimeError::TypeMismatch {
                expected: "numeric".to_string(),
                got: format!("{:?} % {:?}", left, right),
            }),
        }
    }

    fn pow_value(left: &Value, right: &Value) -> Result<Value, RuntimeError> {
        match (left, right) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Double((*a as f64).powf(*b as f64))),
            (Value::Double(a), Value::Double(b)) => Ok(Value::Double(a.powf(*b))),
            (Value::Int(a), Value::Double(b)) => Ok(Value::Double((*a as f64).powf(*b))),
            (Value::Double(a), Value::Int(b)) => Ok(Value::Double(a.powf(*b as f64))),
            _ => Err(RuntimeError::TypeMismatch {
                expected: "numeric".to_string(),
                got: format!("{:?} ** {:?}", left, right),
            }),
        }
    }

    fn cmp_lt_value(left: &Value, right: &Value) -> Result<Value, RuntimeError> {
        match (left, right) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a < b)),
            (Value::Double(a), Value::Double(b)) => Ok(Value::Bool(a < b)),
            (Value::Int(a), Value::Double(b)) => Ok(Value::Bool((*a as f64) < *b)),
            (Value::Double(a), Value::Int(b)) => Ok(Value::Bool(*a < (*b as f64))),
            _ => Err(RuntimeError::TypeMismatch {
                expected: "numeric".to_string(),
                got: format!("{:?} < {:?}", left, right),
            }),
        }
    }

    fn cmp_le_value(left: &Value, right: &Value) -> Result<Value, RuntimeError> {
        match (left, right) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a <= b)),
            (Value::Double(a), Value::Double(b)) => Ok(Value::Bool(a <= b)),
            (Value::Int(a), Value::Double(b)) => Ok(Value::Bool((*a as f64) <= *b)),
            (Value::Double(a), Value::Int(b)) => Ok(Value::Bool(*a <= (*b as f64))),
            _ => Err(RuntimeError::TypeMismatch {
                expected: "numeric".to_string(),
                got: format!("{:?} <= {:?}", left, right),
            }),
        }
    }

    fn cmp_gt_value(left: &Value, right: &Value) -> Result<Value, RuntimeError> {
        match (left, right) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a > b)),
            (Value::Double(a), Value::Double(b)) => Ok(Value::Bool(a > b)),
            (Value::Int(a), Value::Double(b)) => Ok(Value::Bool((*a as f64) > *b)),
            (Value::Double(a), Value::Int(b)) => Ok(Value::Bool(*a > (*b as f64))),
            _ => Err(RuntimeError::TypeMismatch {
                expected: "numeric".to_string(),
                got: format!("{:?} > {:?}", left, right),
            }),
        }
    }

    fn cmp_ge_value(left: &Value, right: &Value) -> Result<Value, RuntimeError> {
        match (left, right) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a >= b)),
            (Value::Double(a), Value::Double(b)) => Ok(Value::Bool(a >= b)),
            (Value::Int(a), Value::Double(b)) => Ok(Value::Bool((*a as f64) >= *b)),
            (Value::Double(a), Value::Int(b)) => Ok(Value::Bool(*a >= (*b as f64))),
            _ => Err(RuntimeError::TypeMismatch {
                expected: "numeric".to_string(),
                got: format!("{:?} >= {:?}", left, right),
            }),
        }
    }

    fn neg_value(value: &Value) -> Result<Value, RuntimeError> {
        match value {
            Value::Int(n) => Ok(Value::Int(-n)),
            Value::Double(d) => Ok(Value::Double(-d)),
            _ => Err(RuntimeError::TypeMismatch {
                expected: "numeric".to_string(),
                got: format!("{:?}", value),
            }),
        }
    }
}

impl Default for VM {
    fn default() -> Self {
        Self::new()
    }
}

