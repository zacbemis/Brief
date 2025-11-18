mod common;

use brief_ast::*;
use common::*;
use insta::assert_snapshot;

/// Pretty-print AST with stable ordering (no spans by default)
fn pretty_print_ast(program: &Program) -> String {
    let mut output = String::new();
    pretty_print_program(program, &mut output, 0, false);
    output
}

/// Pretty-print AST with spans (for debug mode)
#[allow(dead_code)]
fn pretty_print_ast_with_spans(program: &Program) -> String {
    let mut output = String::new();
    pretty_print_program(program, &mut output, 0, true);
    output
}

fn pretty_print_program(program: &Program, output: &mut String, indent: usize, include_spans: bool) {
    let indent_str = "  ".repeat(indent);
    output.push_str(&format!("{}Program\n", indent_str));
    if include_spans {
        output.push_str(&format!("{}  span: {:?}\n", indent_str, program.span));
    }
    output.push_str(&format!("{}  declarations:\n", indent_str));
    for decl in &program.declarations {
        pretty_print_decl(decl, output, indent + 2, include_spans);
    }
}

fn pretty_print_decl(decl: &Decl, output: &mut String, indent: usize, include_spans: bool) {
    let indent_str = "  ".repeat(indent);
    match decl {
        Decl::VarDecl(v) => {
            output.push_str(&format!("{}VarDecl\n", indent_str));
            output.push_str(&format!("{}  name: {}\n", indent_str, v.name));
            if let Some(ty) = &v.type_annotation {
                output.push_str(&format!("{}  type: ", indent_str));
                pretty_print_type(ty, output, include_spans);
                output.push('\n');
            }
            if let Some(init) = &v.initializer {
                output.push_str(&format!("{}  initializer: ", indent_str));
                pretty_print_expr(init, output, indent + 2, include_spans);
            }
            if include_spans {
                output.push_str(&format!("{}  span: {:?}\n", indent_str, v.span));
            }
        }
        Decl::ConstDecl(c) => {
            output.push_str(&format!("{}ConstDecl\n", indent_str));
            output.push_str(&format!("{}  name: {}\n", indent_str, c.name));
            output.push_str(&format!("{}  initializer: ", indent_str));
            pretty_print_expr(&c.initializer, output, indent + 2, include_spans);
            if include_spans {
                output.push_str(&format!("{}  span: {:?}\n", indent_str, c.span));
            }
        }
        Decl::FuncDecl(f) => {
            output.push_str(&format!("{}FuncDecl\n", indent_str));
            output.push_str(&format!("{}  name: {}\n", indent_str, f.name));
            output.push_str(&format!("{}  params:\n", indent_str));
            for param in &f.params {
                pretty_print_param(param, output, indent + 2, include_spans);
            }
            if let Some(ty) = &f.return_type {
                output.push_str(&format!("{}  return_type: ", indent_str));
                pretty_print_type(ty, output, include_spans);
                output.push('\n');
            }
            output.push_str(&format!("{}  body:\n", indent_str));
            pretty_print_block(&f.body, output, indent + 2, include_spans);
            if include_spans {
                output.push_str(&format!("{}  span: {:?}\n", indent_str, f.span));
            }
        }
        Decl::ClassDecl(c) => {
            output.push_str(&format!("{}ClassDecl\n", indent_str));
            output.push_str(&format!("{}  name: {}\n", indent_str, c.name));
            if let Some(ctor) = &c.constructor {
                output.push_str(&format!("{}  constructor:\n", indent_str));
                pretty_print_ctor(ctor, output, indent + 2, include_spans);
            }
            output.push_str(&format!("{}  methods:\n", indent_str));
            for method in &c.methods {
                pretty_print_method(method, output, indent + 2, include_spans);
            }
            if include_spans {
                output.push_str(&format!("{}  span: {:?}\n", indent_str, c.span));
            }
        }
        Decl::ImportDecl(_) => {
            output.push_str(&format!("{}ImportDecl\n", indent_str));
            // Import parsing not fully implemented yet
        }
        Decl::Error(span) => {
            output.push_str(&format!("{}Error\n", indent_str));
            if include_spans {
                output.push_str(&format!("{}  span: {:?}\n", indent_str, span));
            }
        }
    }
}

fn pretty_print_expr(expr: &Expr, output: &mut String, indent: usize, include_spans: bool) {
    let indent_str = "  ".repeat(indent);
    match expr {
        Expr::Integer(n, span) => {
            output.push_str(&format!("Integer({})", n));
            if include_spans {
                output.push_str(&format!(" @ {:?}", span));
            }
        }
        Expr::Double(d, span) => {
            output.push_str(&format!("Double({})", d));
            if include_spans {
                output.push_str(&format!(" @ {:?}", span));
            }
        }
        Expr::Character(c, span) => {
            output.push_str(&format!("Character('{}')", c));
            if include_spans {
                output.push_str(&format!(" @ {:?}", span));
            }
        }
        Expr::String(s, span) => {
            output.push_str(&format!("String(\"{}\")", s));
            if include_spans {
                output.push_str(&format!(" @ {:?}", span));
            }
        }
        Expr::Boolean(b, span) => {
            output.push_str(&format!("Boolean({})", b));
            if include_spans {
                output.push_str(&format!(" @ {:?}", span));
            }
        }
        Expr::Null(span) => {
            output.push_str("Null");
            if include_spans {
                output.push_str(&format!(" @ {:?}", span));
            }
        }
        Expr::Variable(name, span) => {
            output.push_str(&format!("Variable({})", name));
            if include_spans {
                output.push_str(&format!(" @ {:?}", span));
            }
        }
        Expr::BinaryOp { left, op, right, span } => {
            output.push_str(&format!("BinaryOp({:?})\n", op));
            output.push_str(&format!("{}  left: ", indent_str));
            pretty_print_expr(left, output, indent + 2, include_spans);
            output.push('\n');
            output.push_str(&format!("{}  right: ", indent_str));
            pretty_print_expr(right, output, indent + 2, include_spans);
            if include_spans {
                output.push_str(&format!("\n{}  span: {:?}", indent_str, span));
            }
        }
        Expr::UnaryOp { op, expr, span } => {
            output.push_str(&format!("UnaryOp({:?})\n", op));
            output.push_str(&format!("{}  expr: ", indent_str));
            pretty_print_expr(expr, output, indent + 2, include_spans);
            if include_spans {
                output.push_str(&format!("\n{}  span: {:?}", indent_str, span));
            }
        }
        Expr::PostfixOp { expr, op, span } => {
            output.push_str(&format!("PostfixOp({:?})\n", op));
            output.push_str(&format!("{}  expr: ", indent_str));
            pretty_print_expr(expr, output, indent + 2, include_spans);
            if include_spans {
                output.push_str(&format!("\n{}  span: {:?}", indent_str, span));
            }
        }
        Expr::Call { callee, args, span } => {
            output.push_str("Call\n");
            output.push_str(&format!("{}  callee: ", indent_str));
            pretty_print_expr(callee, output, indent + 2, include_spans);
            output.push('\n');
            output.push_str(&format!("{}  args:\n", indent_str));
            for arg in args {
                pretty_print_expr(arg, output, indent + 2, include_spans);
                output.push('\n');
            }
            if include_spans {
                output.push_str(&format!("{}  span: {:?}", indent_str, span));
            }
        }
        Expr::MethodCall { object, method, args, span } => {
            output.push_str("MethodCall\n");
            output.push_str(&format!("{}  object: ", indent_str));
            pretty_print_expr(object, output, indent + 2, include_spans);
            output.push_str(&format!("\n{}  method: {}\n", indent_str, method));
            output.push_str(&format!("{}  args:\n", indent_str));
            for arg in args {
                pretty_print_expr(arg, output, indent + 2, include_spans);
                output.push('\n');
            }
            if include_spans {
                output.push_str(&format!("{}  span: {:?}", indent_str, span));
            }
        }
        Expr::MemberAccess { object, member, span } => {
            output.push_str("MemberAccess\n");
            output.push_str(&format!("{}  object: ", indent_str));
            pretty_print_expr(object, output, indent + 2, include_spans);
            output.push_str(&format!("\n{}  member: {}\n", indent_str, member));
            if include_spans {
                output.push_str(&format!("{}  span: {:?}", indent_str, span));
            }
        }
        Expr::Index { object, index, span } => {
            output.push_str("Index\n");
            output.push_str(&format!("{}  object: ", indent_str));
            pretty_print_expr(object, output, indent + 2, include_spans);
            output.push('\n');
            output.push_str(&format!("{}  index: ", indent_str));
            pretty_print_expr(index, output, indent + 2, include_spans);
            if include_spans {
                output.push_str(&format!("\n{}  span: {:?}", indent_str, span));
            }
        }
        Expr::Cast { expr, target_type, span } => {
            output.push_str("Cast\n");
            output.push_str(&format!("{}  expr: ", indent_str));
            pretty_print_expr(expr, output, indent + 2, include_spans);
            output.push_str(&format!("\n{}  target_type: ", indent_str));
            pretty_print_type(target_type, output, include_spans);
            if include_spans {
                output.push_str(&format!("\n{}  span: {:?}", indent_str, span));
            }
        }
        Expr::Interpolation { parts, span } => {
            output.push_str("Interpolation\n");
            output.push_str(&format!("{}  parts:\n", indent_str));
            for part in parts {
                pretty_print_interp_part(part, output, indent + 2, include_spans);
            }
            if include_spans {
                output.push_str(&format!("{}  span: {:?}", indent_str, span));
            }
        }
        Expr::Ternary { condition, then_expr, else_expr, span } => {
            output.push_str("Ternary\n");
            output.push_str(&format!("{}  condition: ", indent_str));
            pretty_print_expr(condition, output, indent + 2, include_spans);
            output.push('\n');
            output.push_str(&format!("{}  then: ", indent_str));
            pretty_print_expr(then_expr, output, indent + 2, include_spans);
            output.push('\n');
            output.push_str(&format!("{}  else: ", indent_str));
            pretty_print_expr(else_expr, output, indent + 2, include_spans);
            if include_spans {
                output.push_str(&format!("\n{}  span: {:?}", indent_str, span));
            }
        }
        Expr::Lambda { params, body, span } => {
            output.push_str("Lambda\n");
            output.push_str(&format!("{}  params:\n", indent_str));
            for param in params {
                pretty_print_param(param, output, indent + 2, include_spans);
            }
            output.push_str(&format!("{}  body: ", indent_str));
            pretty_print_expr(body, output, indent + 2, include_spans);
            if include_spans {
                output.push_str(&format!("\n{}  span: {:?}", indent_str, span));
            }
        }
        Expr::Error(span) => {
            output.push_str("Error");
            if include_spans {
                output.push_str(&format!(" @ {:?}", span));
            }
        }
    }
}

fn pretty_print_interp_part(part: &InterpPart, output: &mut String, indent: usize, include_spans: bool) {
    let indent_str = "  ".repeat(indent);
    match part {
        InterpPart::Text(text) => {
            output.push_str(&format!("{}Text(\"{}\")\n", indent_str, text));
        }
        InterpPart::Ident(name, span) => {
            output.push_str(&format!("{}Ident({})", indent_str, name));
            if include_spans {
                output.push_str(&format!(" @ {:?}", span));
            }
            output.push('\n');
        }
        InterpPart::Path(expr, span) => {
            output.push_str(&format!("{}Path:\n", indent_str));
            pretty_print_expr(expr, output, indent + 1, include_spans);
            if include_spans {
                output.push_str(&format!("\n{}  span: {:?}", indent_str, span));
            }
            output.push('\n');
        }
    }
}

fn pretty_print_stmt(stmt: &Stmt, output: &mut String, indent: usize, include_spans: bool) {
    let indent_str = "  ".repeat(indent);
    match stmt {
        Stmt::Expr(expr, span) => {
            output.push_str(&format!("{}Expr:\n", indent_str));
            pretty_print_expr(expr, output, indent + 1, include_spans);
            if include_spans {
                output.push_str(&format!("\n{}  span: {:?}", indent_str, span));
            }
        }
        Stmt::If { condition, then_branch, else_branch, span } => {
            output.push_str(&format!("{}If\n", indent_str));
            output.push_str(&format!("{}  condition: ", indent_str));
            pretty_print_expr(condition, output, indent + 2, include_spans);
            output.push_str(&format!("\n{}  then:\n", indent_str));
            pretty_print_block(then_branch, output, indent + 2, include_spans);
            if let Some(else_branch) = else_branch {
                output.push_str(&format!("{}  else:\n", indent_str));
                pretty_print_block(else_branch, output, indent + 2, include_spans);
            }
            if include_spans {
                output.push_str(&format!("{}  span: {:?}", indent_str, span));
            }
        }
        Stmt::While { condition, body, span } => {
            output.push_str(&format!("{}While\n", indent_str));
            output.push_str(&format!("{}  condition: ", indent_str));
            pretty_print_expr(condition, output, indent + 2, include_spans);
            output.push_str(&format!("\n{}  body:\n", indent_str));
            pretty_print_block(body, output, indent + 2, include_spans);
            if include_spans {
                output.push_str(&format!("{}  span: {:?}", indent_str, span));
            }
        }
        Stmt::For { init, condition, increment, body, span } => {
            output.push_str(&format!("{}For\n", indent_str));
            if let Some(init) = init {
                output.push_str(&format!("{}  init:\n", indent_str));
                pretty_print_stmt(init, output, indent + 2, include_spans);
            }
            if let Some(condition) = condition {
                output.push_str(&format!("{}  condition: ", indent_str));
                pretty_print_expr(condition, output, indent + 2, include_spans);
                output.push('\n');
            }
            if let Some(increment) = increment {
                output.push_str(&format!("{}  increment: ", indent_str));
                pretty_print_expr(increment, output, indent + 2, include_spans);
                output.push('\n');
            }
            output.push_str(&format!("{}  body:\n", indent_str));
            pretty_print_block(body, output, indent + 2, include_spans);
            if include_spans {
                output.push_str(&format!("{}  span: {:?}", indent_str, span));
            }
        }
        Stmt::ForIn { var, iterable, body, span } => {
            output.push_str(&format!("{}ForIn\n", indent_str));
            output.push_str(&format!("{}  var: {}\n", indent_str, var));
            output.push_str(&format!("{}  iterable: ", indent_str));
            pretty_print_expr(iterable, output, indent + 2, include_spans);
            output.push_str(&format!("\n{}  body:\n", indent_str));
            pretty_print_block(body, output, indent + 2, include_spans);
            if include_spans {
                output.push_str(&format!("{}  span: {:?}", indent_str, span));
            }
        }
        Stmt::Match { expr, cases, else_branch, span } => {
            output.push_str(&format!("{}Match\n", indent_str));
            output.push_str(&format!("{}  expr: ", indent_str));
            pretty_print_expr(expr, output, indent + 2, include_spans);
            output.push_str(&format!("\n{}  cases:\n", indent_str));
            for case in cases {
                pretty_print_match_case(case, output, indent + 2, include_spans);
            }
            if let Some(else_branch) = else_branch {
                output.push_str(&format!("{}  else:\n", indent_str));
                pretty_print_block(else_branch, output, indent + 2, include_spans);
            }
            if include_spans {
                output.push_str(&format!("{}  span: {:?}", indent_str, span));
            }
        }
        Stmt::Return { value, span } => {
            output.push_str(&format!("{}Return\n", indent_str));
            if let Some(value) = value {
                output.push_str(&format!("{}  value: ", indent_str));
                pretty_print_expr(value, output, indent + 2, include_spans);
            }
            if include_spans {
                output.push_str(&format!("\n{}  span: {:?}", indent_str, span));
            }
        }
        Stmt::Break(span) => {
            output.push_str(&format!("{}Break", indent_str));
            if include_spans {
                output.push_str(&format!(" @ {:?}", span));
            }
        }
        Stmt::Continue(span) => {
            output.push_str(&format!("{}Continue", indent_str));
            if include_spans {
                output.push_str(&format!(" @ {:?}", span));
            }
        }
        Stmt::VarDecl(v) => {
            output.push_str(&format!("{}VarDecl\n", indent_str));
            output.push_str(&format!("{}  name: {}\n", indent_str, v.name));
            if let Some(ty) = &v.type_annotation {
                output.push_str(&format!("{}  type: ", indent_str));
                pretty_print_type(ty, output, include_spans);
                output.push('\n');
            }
            if let Some(init) = &v.initializer {
                output.push_str(&format!("{}  initializer: ", indent_str));
                pretty_print_expr(init, output, indent + 2, include_spans);
            }
            if include_spans {
                output.push_str(&format!("\n{}  span: {:?}", indent_str, v.span));
            }
        }
        Stmt::ConstDecl(c) => {
            output.push_str(&format!("{}ConstDecl\n", indent_str));
            output.push_str(&format!("{}  name: {}\n", indent_str, c.name));
            output.push_str(&format!("{}  initializer: ", indent_str));
            pretty_print_expr(&c.initializer, output, indent + 2, include_spans);
            if include_spans {
                output.push_str(&format!("\n{}  span: {:?}", indent_str, c.span));
            }
        }
        Stmt::Error(span) => {
            output.push_str(&format!("{}Error", indent_str));
            if include_spans {
                output.push_str(&format!(" @ {:?}", span));
            }
        }
    }
}

fn pretty_print_block(block: &Block, output: &mut String, indent: usize, include_spans: bool) {
    let indent_str = "  ".repeat(indent);
    output.push_str(&format!("{}Block\n", indent_str));
    if include_spans {
        output.push_str(&format!("{}  span: {:?}\n", indent_str, block.span));
    }
    output.push_str(&format!("{}  statements:\n", indent_str));
    for stmt in &block.statements {
        pretty_print_stmt(stmt, output, indent + 2, include_spans);
        output.push('\n');
    }
}

fn pretty_print_match_case(case: &MatchCase, output: &mut String, indent: usize, include_spans: bool) {
    let indent_str = "  ".repeat(indent);
    output.push_str(&format!("{}MatchCase\n", indent_str));
    output.push_str(&format!("{}  patterns:\n", indent_str));
    for pattern in &case.patterns {
        pretty_print_expr(pattern, output, indent + 2, include_spans);
        output.push('\n');
    }
    output.push_str(&format!("{}  body:\n", indent_str));
    pretty_print_block(&case.body, output, indent + 2, include_spans);
    if include_spans {
        output.push_str(&format!("{}  span: {:?}", indent_str, case.span));
    }
}

fn pretty_print_param(param: &Param, output: &mut String, indent: usize, include_spans: bool) {
    let indent_str = "  ".repeat(indent);
    output.push_str(&format!("{}Param\n", indent_str));
    output.push_str(&format!("{}  name: {}\n", indent_str, param.name));
    if let Some(ty) = &param.type_annotation {
        output.push_str(&format!("{}  type: ", indent_str));
        pretty_print_type(ty, output, include_spans);
        output.push('\n');
    }
    if include_spans {
        output.push_str(&format!("{}  span: {:?}\n", indent_str, param.span));
    }
}

fn pretty_print_type(ty: &Type, output: &mut String, include_spans: bool) {
    match ty {
        Type::Int => output.push_str("Int"),
        Type::Char => output.push_str("Char"),
        Type::Str => output.push_str("Str"),
        Type::Dub => output.push_str("Dub"),
        Type::Bool => output.push_str("Bool"),
        Type::Array { base, dims, span } => {
            output.push_str("Array(");
            pretty_print_type(base, output, include_spans);
            output.push_str(", dims: [");
            for (i, dim) in dims.iter().enumerate() {
                if i > 0 {
                    output.push_str(", ");
                }
                match dim {
                    brief_ast::ty::ArrayDim::Fixed(n) => output.push_str(&format!("Fixed({})", n)),
                    brief_ast::ty::ArrayDim::Dynamic => output.push_str("Dynamic"),
                    brief_ast::ty::ArrayDim::Stack => output.push_str("Stack"),
                    brief_ast::ty::ArrayDim::Queue => output.push_str("Queue"),
                }
            }
            output.push(']');
            if include_spans {
                output.push_str(&format!(" @ {:?}", span));
            }
            output.push(')');
        }
        Type::Map { key_type, value_type, span } => {
            output.push_str("Map(");
            pretty_print_type(key_type, output, include_spans);
            output.push_str(": ");
            pretty_print_type(value_type, output, include_spans);
            if include_spans {
                output.push_str(&format!(" @ {:?}", span));
            }
            output.push(')');
        }
        Type::Function { params, return_type, span } => {
            output.push_str("Function(");
            output.push_str("params: [");
            for (i, param) in params.iter().enumerate() {
                if i > 0 {
                    output.push_str(", ");
                }
                pretty_print_type(param, output, include_spans);
            }
            output.push_str("], return: ");
            pretty_print_type(return_type, output, include_spans);
            if include_spans {
                output.push_str(&format!(" @ {:?}", span));
            }
            output.push(')');
        }
    }
}

fn pretty_print_ctor(ctor: &CtorDecl, output: &mut String, indent: usize, include_spans: bool) {
    let indent_str = "  ".repeat(indent);
    output.push_str(&format!("{}CtorDecl\n", indent_str));
    output.push_str(&format!("{}  name: {}\n", indent_str, ctor.name));
    output.push_str(&format!("{}  params:\n", indent_str));
    for param in &ctor.params {
        pretty_print_param(param, output, indent + 2, include_spans);
    }
    output.push_str(&format!("{}  body:\n", indent_str));
    pretty_print_block(&ctor.body, output, indent + 2, include_spans);
    if include_spans {
        output.push_str(&format!("{}  span: {:?}", indent_str, ctor.span));
    }
}

fn pretty_print_method(method: &MethodDecl, output: &mut String, indent: usize, include_spans: bool) {
    let indent_str = "  ".repeat(indent);
    output.push_str(&format!("{}MethodDecl\n", indent_str));
    output.push_str(&format!("{}  name: {}\n", indent_str, method.name));
    output.push_str(&format!("{}  is_instance: {}\n", indent_str, method.is_instance));
    output.push_str(&format!("{}  params:\n", indent_str));
    for param in &method.params {
        pretty_print_param(param, output, indent + 2, include_spans);
    }
    if let Some(ty) = &method.return_type {
        output.push_str(&format!("{}  return_type: ", indent_str));
        pretty_print_type(ty, output, include_spans);
        output.push('\n');
    }
    output.push_str(&format!("{}  body:\n", indent_str));
    pretty_print_block(&method.body, output, indent + 2, include_spans);
    if include_spans {
        output.push_str(&format!("{}  span: {:?}", indent_str, method.span));
    }
}

// Snapshot tests

#[test]
fn snapshot_simple_expressions() {
    let source = "x := 1 + 2 * 3";
    let program = parse_source(source);
    assert_snapshot!("simple_expressions", pretty_print_ast(&program));
}

#[test]
fn snapshot_arithmetic_operators() {
    let source = "x := 1 + 2 - 3 * 4 / 5 % 6";
    let program = parse_source(source);
    assert_snapshot!("arithmetic_operators", pretty_print_ast(&program));
}

#[test]
fn snapshot_logical_operators() {
    let source = "x := true && false || true";
    let program = parse_source(source);
    assert_snapshot!("logical_operators", pretty_print_ast(&program));
}

#[test]
fn snapshot_comparison_operators() {
    let source = "x := 1 < 2 && 3 >= 4";
    let program = parse_source(source);
    assert_snapshot!("comparison_operators", pretty_print_ast(&program));
}

#[test]
fn snapshot_unary_operators() {
    let source = "x := -5\ny := !true";
    let program = parse_source(source);
    assert_snapshot!("unary_operators", pretty_print_ast(&program));
}

#[test]
fn snapshot_if_else() {
    let source = "if (x == 1)\n\tret \"one\"\nelse\n\tret \"other\"";
    let program = parse_source(source);
    assert_snapshot!("if_else", pretty_print_ast(&program));
}

#[test]
fn snapshot_while_loop() {
    let source = "while (i < 10)\n\ti++";
    let program = parse_source(source);
    assert_snapshot!("while_loop", pretty_print_ast(&program));
}

#[test]
fn snapshot_for_loop() {
    let source = "for (i := 0; i < 10; i++)\n\tprint(i)";
    let program = parse_source(source);
    assert_snapshot!("for_loop", pretty_print_ast(&program));
}

#[test]
fn snapshot_for_in_loop() {
    let source = "for (num in arr)\n\tprint(num)";
    let program = parse_source(source);
    assert_snapshot!("for_in_loop", pretty_print_ast(&program));
}

#[test]
fn snapshot_match_statement() {
    let source = "match(grade)\ncase 'A'\n\tprint(\"Excellent\")\nelse\n\tprint(\"Other\")";
    let program = parse_source(source);
    assert_snapshot!("match_statement", pretty_print_ast(&program));
}

#[test]
fn snapshot_match_multiple_patterns() {
    let source = "match(x)\ncase 1, 2, 3\n\tprint(\"small\")\nelse\n\tprint(\"other\")";
    let program = parse_source(source);
    assert_snapshot!("match_multiple_patterns", pretty_print_ast(&program));
}

#[test]
fn snapshot_function_declaration() {
    let source = "def add(int x, int y) -> int\n\tret x + y";
    let program = parse_source(source);
    assert_snapshot!("function_declaration", pretty_print_ast(&program));
}

#[test]
fn snapshot_class_declaration() {
    let source = "cls Dog\n\tobj Dog(name)\n\tdef bark()\n\t\tprint(\"woof\")";
    let program = parse_source(source);
    assert_snapshot!("class_declaration", pretty_print_ast(&program));
}

#[test]
fn snapshot_string_interpolation() {
    let source = "x := \"Hello &name, you are &age years old\"";
    let program = parse_source(source);
    assert_snapshot!("string_interpolation", pretty_print_ast(&program));
}

#[test]
fn snapshot_type_annotations() {
    let source = "int x\nint[10] arr\nint:str{} map";
    let program = parse_source(source);
    assert_snapshot!("type_annotations", pretty_print_ast(&program));
}

#[test]
fn snapshot_complex_nested() {
    let source = "if (x)\n\tif (y)\n\t\tif (z)\n\t\t\tret 1";
    let program = parse_source(source);
    assert_snapshot!("complex_nested", pretty_print_ast(&program));
}

// Negative tests (error recovery)

#[test]
fn snapshot_error_missing_paren() {
    let source = "def test(x\n\tret x";
    let program = parse_source(source);
    assert_snapshot!("error_missing_paren", pretty_print_ast(&program));
}

#[test]
fn snapshot_error_unexpected_token() {
    let source = "def test() -> -> int";
    let program = parse_source(source);
    assert_snapshot!("error_unexpected_token", pretty_print_ast(&program));
}

#[test]
fn snapshot_error_invalid_expression() {
    let source = "x := +";
    let program = parse_source(source);
    assert_snapshot!("error_invalid_expression", pretty_print_ast(&program));
}

#[test]
fn snapshot_error_recovery_multiple() {
    let source = "def test()\n\tret x\ndef other()\n\tret y";
    let program = parse_source(source);
    assert_snapshot!("error_recovery_multiple", pretty_print_ast(&program));
}

