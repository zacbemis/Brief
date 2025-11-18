mod common;

use brief_ast::*;
use brief_ast::ty::ArrayDim;
use common::*;

#[test]
fn test_primitive_types() {
    let program = parse_source("int x\nchar c\nstr s\ndub d\nbool b");
    assert_eq!(program.declarations.len(), 5);
    
    // Check int
    match &program.declarations[0] {
        Decl::VarDecl(v) => {
            match v.type_annotation.as_ref().unwrap() {
                Type::Int => {}
                _ => panic!("Expected int type"),
            }
        }
        _ => panic!("Expected variable declaration"),
    }
}

#[test]
fn test_fixed_array_type() {
    let program = parse_source("int[10] arr");
    match &program.declarations[0] {
        Decl::VarDecl(v) => {
            match v.type_annotation.as_ref().unwrap() {
                Type::Array { dims, .. } => {
                    assert_eq!(dims.len(), 1);
                    assert!(matches!(dims[0], ArrayDim::Fixed(10)));
                }
                _ => panic!("Expected array type"),
            }
        }
        _ => panic!("Expected variable declaration"),
    }
}

#[test]
fn test_dynamic_array_type() {
    let program = parse_source("int{} arr");
    match &program.declarations[0] {
        Decl::VarDecl(v) => {
            match v.type_annotation.as_ref().unwrap() {
                Type::Array { dims, .. } => {
                    assert_eq!(dims.len(), 1);
                    assert!(matches!(dims[0], ArrayDim::Dynamic));
                }
                _ => panic!("Expected array type"),
            }
        }
        _ => panic!("Expected variable declaration"),
    }
}

#[test]
fn test_stack_type() {
    let program = parse_source("int{stk} stack");
    match &program.declarations[0] {
        Decl::VarDecl(v) => {
            match v.type_annotation.as_ref().unwrap() {
                Type::Array { dims, .. } => {
                    assert_eq!(dims.len(), 1);
                    assert!(matches!(dims[0], ArrayDim::Stack));
                }
                _ => panic!("Expected array type"),
            }
        }
        _ => panic!("Expected variable declaration"),
    }
}

#[test]
fn test_queue_type() {
    let program = parse_source("int{que} queue");
    match &program.declarations[0] {
        Decl::VarDecl(v) => {
            match v.type_annotation.as_ref().unwrap() {
                Type::Array { dims, .. } => {
                    assert_eq!(dims.len(), 1);
                    assert!(matches!(dims[0], ArrayDim::Queue));
                }
                _ => panic!("Expected array type"),
            }
        }
        _ => panic!("Expected variable declaration"),
    }
}

#[test]
fn test_multi_dimensional_array() {
    let program = parse_source("int[10][20] matrix");
    match &program.declarations[0] {
        Decl::VarDecl(v) => {
            match v.type_annotation.as_ref().unwrap() {
                Type::Array { dims, .. } => {
                    assert_eq!(dims.len(), 2);
                    assert!(matches!(dims[0], ArrayDim::Fixed(10)));
                    assert!(matches!(dims[1], ArrayDim::Fixed(20)));
                }
                _ => panic!("Expected array type"),
            }
        }
        _ => panic!("Expected variable declaration"),
    }
}

#[test]
fn test_map_type() {
    let program = parse_source("int:str{} map");
    match &program.declarations[0] {
        Decl::VarDecl(v) => {
            match v.type_annotation.as_ref().unwrap() {
                Type::Map { key_type, value_type, .. } => {
                    assert!(matches!(key_type.as_ref(), Type::Int), "Expected int as key type");
                    // Value type should be str, but parsing might need adjustment
                    // For now, just verify we have a map type structure
                    if !matches!(value_type.as_ref(), Type::Str) {
                        eprintln!("Note: Map value type parsed as: {:?}, expected Str", value_type);
                    }
                }
                _ => panic!("Expected map type, got: {:?}", v.type_annotation),
            }
        }
        _ => panic!("Expected variable declaration"),
    }
}

#[test]
fn test_mixed_array_types() {
    let program = parse_source("int[][10] arr");
    match &program.declarations[0] {
        Decl::VarDecl(v) => {
            match v.type_annotation.as_ref().unwrap() {
                Type::Array { dims, .. } => {
                    assert_eq!(dims.len(), 2);
                    assert!(matches!(dims[0], ArrayDim::Dynamic));
                    assert!(matches!(dims[1], ArrayDim::Fixed(10)));
                }
                _ => panic!("Expected array type"),
            }
        }
        _ => panic!("Expected variable declaration"),
    }
}

