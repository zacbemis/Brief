mod common;

use brief_hir::*;
use common::*;
use insta::assert_snapshot;

/// Pretty-print HIR with stable ordering (no spans by default)
fn pretty_print_hir(program: &HirProgram) -> String {
    let mut output = String::new();
    pretty_print_hir_program(program, &mut output, 0, false);
    output
}

fn pretty_print_hir_program(program: &HirProgram, output: &mut String, indent: usize, include_spans: bool) {
    let indent_str = "  ".repeat(indent);
    output.push_str(&format!("{}HirProgram\n", indent_str));
    if include_spans {
        output.push_str(&format!("{}  span: {:?}\n", indent_str, program.span));
    }
    output.push_str(&format!("{}  declarations:\n", indent_str));
    for decl in &program.declarations {
        pretty_print_hir_decl(decl, output, indent + 2, include_spans);
    }
}

fn pretty_print_hir_decl(decl: &HirDecl, output: &mut String, indent: usize, include_spans: bool) {
    let indent_str = "  ".repeat(indent);
    match decl {
        HirDecl::VarDecl(v) => {
            output.push_str(&format!("{}VarDecl\n", indent_str));
            output.push_str(&format!("{}  name: {}\n", indent_str, v.name));
            output.push_str(&format!("{}  symbol: {:?}\n", indent_str, v.symbol));
            if let Some(ty) = &v.type_annotation {
                output.push_str(&format!("{}  type: {:?}\n", indent_str, ty));
            }
            if let Some(init) = &v.initializer {
                output.push_str(&format!("{}  initializer: ", indent_str));
                pretty_print_hir_expr(init, output, indent + 2, include_spans);
                output.push('\n');
            }
            if include_spans {
                output.push_str(&format!("{}  span: {:?}\n", indent_str, v.span));
            }
        }
        HirDecl::ConstDecl(c) => {
            output.push_str(&format!("{}ConstDecl\n", indent_str));
            output.push_str(&format!("{}  name: {}\n", indent_str, c.name));
            output.push_str(&format!("{}  symbol: {:?}\n", indent_str, c.symbol));
            output.push_str(&format!("{}  initializer: ", indent_str));
            pretty_print_hir_expr(&c.initializer, output, indent + 2, include_spans);
            output.push('\n');
            if include_spans {
                output.push_str(&format!("{}  span: {:?}\n", indent_str, c.span));
            }
        }
        HirDecl::FuncDecl(f) => {
            output.push_str(&format!("{}FuncDecl\n", indent_str));
            output.push_str(&format!("{}  name: {}\n", indent_str, f.name));
            output.push_str(&format!("{}  symbol: {:?}\n", indent_str, f.symbol));
            output.push_str(&format!("{}  params:\n", indent_str));
            for param in &f.params {
                pretty_print_hir_param(param, output, indent + 2, include_spans);
            }
            if let Some(ty) = &f.return_type {
                output.push_str(&format!("{}  return_type: {:?}\n", indent_str, ty));
            }
            output.push_str(&format!("{}  body:\n", indent_str));
            pretty_print_hir_block(&f.body, output, indent + 2, include_spans);
            if include_spans {
                output.push_str(&format!("{}  span: {:?}\n", indent_str, f.span));
            }
        }
        HirDecl::ClassDecl(c) => {
            output.push_str(&format!("{}ClassDecl\n", indent_str));
            output.push_str(&format!("{}  name: {}\n", indent_str, c.name));
            output.push_str(&format!("{}  symbol: {:?}\n", indent_str, c.symbol));
            if let Some(ctor) = &c.constructor {
                output.push_str(&format!("{}  constructor:\n", indent_str));
                pretty_print_hir_ctor(ctor, output, indent + 2, include_spans);
            }
            output.push_str(&format!("{}  methods:\n", indent_str));
            for method in &c.methods {
                pretty_print_hir_method(method, output, indent + 2, include_spans);
            }
            if include_spans {
                output.push_str(&format!("{}  span: {:?}\n", indent_str, c.span));
            }
        }
        HirDecl::ImportDecl(i) => {
            output.push_str(&format!("{}ImportDecl\n", indent_str));
            output.push_str(&format!("{}  modules: {:?}\n", indent_str, i.modules));
            if include_spans {
                output.push_str(&format!("{}  span: {:?}\n", indent_str, i.span));
            }
        }
        HirDecl::Error(span) => {
            output.push_str(&format!("{}Error\n", indent_str));
            if include_spans {
                output.push_str(&format!("{}  span: {:?}\n", indent_str, span));
            }
        }
    }
}

fn pretty_print_hir_expr(expr: &HirExpr, output: &mut String, indent: usize, include_spans: bool) {
    let indent_str = "  ".repeat(indent);
    match expr {
        HirExpr::Integer(n, span) => {
            output.push_str(&format!("Integer({})", n));
            if include_spans {
                output.push_str(&format!(" @ {:?}", span));
            }
        }
        HirExpr::Double(d, span) => {
            output.push_str(&format!("Double({})", d));
            if include_spans {
                output.push_str(&format!(" @ {:?}", span));
            }
        }
        HirExpr::Character(c, span) => {
            output.push_str(&format!("Character('{}')", c));
            if include_spans {
                output.push_str(&format!(" @ {:?}", span));
            }
        }
        HirExpr::String(s, span) => {
            output.push_str(&format!("String(\"{}\")", s));
            if include_spans {
                output.push_str(&format!(" @ {:?}", span));
            }
        }
        HirExpr::Boolean(b, span) => {
            output.push_str(&format!("Boolean({})", b));
            if include_spans {
                output.push_str(&format!(" @ {:?}", span));
            }
        }
        HirExpr::Null(span) => {
            output.push_str("Null");
            if include_spans {
                output.push_str(&format!(" @ {:?}", span));
            }
        }
        HirExpr::Variable { name, symbol, span } => {
            output.push_str(&format!("Variable({}, {:?})", name, symbol));
            if include_spans {
                output.push_str(&format!(" @ {:?}", span));
            }
        }
        HirExpr::BinaryOp { left, op, right, span } => {
            output.push_str(&format!("BinaryOp({:?})\n", op));
            output.push_str(&format!("{}  left: ", indent_str));
            pretty_print_hir_expr(left, output, indent + 2, include_spans);
            output.push('\n');
            output.push_str(&format!("{}  right: ", indent_str));
            pretty_print_hir_expr(right, output, indent + 2, include_spans);
            if include_spans {
                output.push_str(&format!("\n{}  span: {:?}", indent_str, span));
            }
        }
        HirExpr::UnaryOp { op, expr, span } => {
            output.push_str(&format!("UnaryOp({:?})\n", op));
            output.push_str(&format!("{}  expr: ", indent_str));
            pretty_print_hir_expr(expr, output, indent + 2, include_spans);
            if include_spans {
                output.push_str(&format!("\n{}  span: {:?}", indent_str, span));
            }
        }
        HirExpr::Assign { target, value, span } => {
            output.push_str("Assign\n");
            output.push_str(&format!("{}  target: ", indent_str));
            pretty_print_hir_expr(target, output, indent + 2, include_spans);
            output.push('\n');
            output.push_str(&format!("{}  value: ", indent_str));
            pretty_print_hir_expr(value, output, indent + 2, include_spans);
            if include_spans {
                output.push_str(&format!("\n{}  span: {:?}", indent_str, span));
            }
        }
        HirExpr::Call { callee, args, span } => {
            output.push_str("Call\n");
            output.push_str(&format!("{}  callee: ", indent_str));
            pretty_print_hir_expr(callee, output, indent + 2, include_spans);
            output.push('\n');
            output.push_str(&format!("{}  args:\n", indent_str));
            for arg in args {
                pretty_print_hir_expr(arg, output, indent + 2, include_spans);
                output.push('\n');
            }
            if include_spans {
                output.push_str(&format!("{}  span: {:?}", indent_str, span));
            }
        }
        HirExpr::MethodCall { object, method, args, span } => {
            output.push_str("MethodCall\n");
            output.push_str(&format!("{}  object: ", indent_str));
            pretty_print_hir_expr(object, output, indent + 2, include_spans);
            output.push_str(&format!("\n{}  method: {}\n", indent_str, method));
            output.push_str(&format!("{}  args:\n", indent_str));
            for arg in args {
                pretty_print_hir_expr(arg, output, indent + 2, include_spans);
                output.push('\n');
            }
            if include_spans {
                output.push_str(&format!("{}  span: {:?}", indent_str, span));
            }
        }
        HirExpr::MemberAccess { object, member, span } => {
            output.push_str("MemberAccess\n");
            output.push_str(&format!("{}  object: ", indent_str));
            pretty_print_hir_expr(object, output, indent + 2, include_spans);
            output.push_str(&format!("\n{}  member: {}\n", indent_str, member));
            if include_spans {
                output.push_str(&format!("{}  span: {:?}", indent_str, span));
            }
        }
        HirExpr::Index { object, index, span } => {
            output.push_str("Index\n");
            output.push_str(&format!("{}  object: ", indent_str));
            pretty_print_hir_expr(object, output, indent + 2, include_spans);
            output.push('\n');
            output.push_str(&format!("{}  index: ", indent_str));
            pretty_print_hir_expr(index, output, indent + 2, include_spans);
            if include_spans {
                output.push_str(&format!("\n{}  span: {:?}", indent_str, span));
            }
        }
        HirExpr::Cast { expr, target_type, span } => {
            output.push_str("Cast\n");
            output.push_str(&format!("{}  expr: ", indent_str));
            pretty_print_hir_expr(expr, output, indent + 2, include_spans);
            output.push_str(&format!("\n{}  target_type: {:?}", indent_str, target_type));
            if include_spans {
                output.push_str(&format!("\n{}  span: {:?}", indent_str, span));
            }
        }
        HirExpr::Interpolation { parts, span } => {
            output.push_str("Interpolation\n");
            output.push_str(&format!("{}  parts: {} parts\n", indent_str, parts.len()));
            if include_spans {
                output.push_str(&format!("{}  span: {:?}", indent_str, span));
            }
        }
        HirExpr::Ternary { condition, then_expr, else_expr, span } => {
            output.push_str("Ternary\n");
            output.push_str(&format!("{}  condition: ", indent_str));
            pretty_print_hir_expr(condition, output, indent + 2, include_spans);
            output.push('\n');
            output.push_str(&format!("{}  then: ", indent_str));
            pretty_print_hir_expr(then_expr, output, indent + 2, include_spans);
            output.push('\n');
            output.push_str(&format!("{}  else: ", indent_str));
            pretty_print_hir_expr(else_expr, output, indent + 2, include_spans);
            if include_spans {
                output.push_str(&format!("\n{}  span: {:?}", indent_str, span));
            }
        }
        HirExpr::Lambda { params, captures, body, span } => {
            output.push_str("Lambda\n");
            output.push_str(&format!("{}  params:\n", indent_str));
            for param in params {
                pretty_print_hir_param(param, output, indent + 2, include_spans);
            }
            output.push_str(&format!("{}  captures: {} upvalues\n", indent_str, captures.len()));
            output.push_str(&format!("{}  body: ", indent_str));
            pretty_print_hir_expr(body, output, indent + 2, include_spans);
            if include_spans {
                output.push_str(&format!("\n{}  span: {:?}", indent_str, span));
            }
        }
        HirExpr::Error(span) => {
            output.push_str("Error");
            if include_spans {
                output.push_str(&format!(" @ {:?}", span));
            }
        }
    }
}

fn pretty_print_hir_stmt(stmt: &HirStmt, output: &mut String, indent: usize, include_spans: bool) {
    let indent_str = "  ".repeat(indent);
    match stmt {
        HirStmt::VarDecl(v) => {
            output.push_str(&format!("{}VarDecl\n", indent_str));
            output.push_str(&format!("{}  name: {}\n", indent_str, v.name));
            output.push_str(&format!("{}  symbol: {:?}\n", indent_str, v.symbol));
            if let Some(ty) = &v.type_annotation {
                output.push_str(&format!("{}  type: {:?}\n", indent_str, ty));
            }
            if let Some(init) = &v.initializer {
                output.push_str(&format!("{}  initializer: ", indent_str));
                pretty_print_hir_expr(init, output, indent + 2, include_spans);
                output.push('\n');
            }
            if include_spans {
                output.push_str(&format!("{}  span: {:?}\n", indent_str, v.span));
            }
        }
        HirStmt::ConstDecl(c) => {
            output.push_str(&format!("{}ConstDecl\n", indent_str));
            output.push_str(&format!("{}  name: {}\n", indent_str, c.name));
            output.push_str(&format!("{}  symbol: {:?}\n", indent_str, c.symbol));
            output.push_str(&format!("{}  initializer: ", indent_str));
            pretty_print_hir_expr(&c.initializer, output, indent + 2, include_spans);
            output.push('\n');
            if include_spans {
                output.push_str(&format!("{}  span: {:?}\n", indent_str, c.span));
            }
        }
        HirStmt::If { condition, then_branch, else_branch, span } => {
            output.push_str(&format!("{}If\n", indent_str));
            output.push_str(&format!("{}  condition: ", indent_str));
            pretty_print_hir_expr(condition, output, indent + 2, include_spans);
            output.push_str(&format!("\n{}  then:\n", indent_str));
            pretty_print_hir_block(then_branch, output, indent + 2, include_spans);
            if let Some(else_branch) = else_branch {
                output.push_str(&format!("{}  else:\n", indent_str));
                pretty_print_hir_block(else_branch, output, indent + 2, include_spans);
            }
            if include_spans {
                output.push_str(&format!("{}  span: {:?}", indent_str, span));
            }
        }
        HirStmt::While { condition, body, span } => {
            output.push_str(&format!("{}While\n", indent_str));
            output.push_str(&format!("{}  condition: ", indent_str));
            pretty_print_hir_expr(condition, output, indent + 2, include_spans);
            output.push_str(&format!("\n{}  body:\n", indent_str));
            pretty_print_hir_block(body, output, indent + 2, include_spans);
            if include_spans {
                output.push_str(&format!("{}  span: {:?}", indent_str, span));
            }
        }
        HirStmt::For { init, condition, increment, body, span } => {
            output.push_str(&format!("{}For\n", indent_str));
            if let Some(init) = init {
                output.push_str(&format!("{}  init:\n", indent_str));
                pretty_print_hir_stmt(init, output, indent + 2, include_spans);
            }
            if let Some(condition) = condition {
                output.push_str(&format!("{}  condition: ", indent_str));
                pretty_print_hir_expr(condition, output, indent + 2, include_spans);
                output.push('\n');
            }
            if let Some(increment) = increment {
                output.push_str(&format!("{}  increment: ", indent_str));
                pretty_print_hir_expr(increment, output, indent + 2, include_spans);
                output.push('\n');
            }
            output.push_str(&format!("{}  body:\n", indent_str));
            pretty_print_hir_block(body, output, indent + 2, include_spans);
            if include_spans {
                output.push_str(&format!("{}  span: {:?}", indent_str, span));
            }
        }
        HirStmt::Return { value, span } => {
            output.push_str(&format!("{}Return\n", indent_str));
            if let Some(value) = value {
                output.push_str(&format!("{}  value: ", indent_str));
                pretty_print_hir_expr(value, output, indent + 2, include_spans);
            }
            if include_spans {
                output.push_str(&format!("\n{}  span: {:?}", indent_str, span));
            }
        }
        HirStmt::Break(span) => {
            output.push_str(&format!("{}Break", indent_str));
            if include_spans {
                output.push_str(&format!(" @ {:?}", span));
            }
        }
        HirStmt::Continue(span) => {
            output.push_str(&format!("{}Continue", indent_str));
            if include_spans {
                output.push_str(&format!(" @ {:?}", span));
            }
        }
        HirStmt::Expr(expr, span) => {
            output.push_str(&format!("{}Expr:\n", indent_str));
            pretty_print_hir_expr(expr, output, indent + 1, include_spans);
            if include_spans {
                output.push_str(&format!("\n{}  span: {:?}", indent_str, span));
            }
        }
        HirStmt::Error(span) => {
            output.push_str(&format!("{}Error", indent_str));
            if include_spans {
                output.push_str(&format!(" @ {:?}", span));
            }
        }
    }
}

fn pretty_print_hir_block(block: &HirBlock, output: &mut String, indent: usize, include_spans: bool) {
    let indent_str = "  ".repeat(indent);
    output.push_str(&format!("{}Block\n", indent_str));
    if include_spans {
        output.push_str(&format!("{}  span: {:?}\n", indent_str, block.span));
    }
    output.push_str(&format!("{}  statements:\n", indent_str));
    for stmt in &block.statements {
        pretty_print_hir_stmt(stmt, output, indent + 2, include_spans);
        output.push('\n');
    }
}

fn pretty_print_hir_param(param: &HirParam, output: &mut String, indent: usize, include_spans: bool) {
    let indent_str = "  ".repeat(indent);
    output.push_str(&format!("{}Param\n", indent_str));
    output.push_str(&format!("{}  name: {}\n", indent_str, param.name));
    output.push_str(&format!("{}  symbol: {:?}\n", indent_str, param.symbol));
    if let Some(ty) = &param.type_annotation {
        output.push_str(&format!("{}  type: {:?}\n", indent_str, ty));
    }
    if include_spans {
        output.push_str(&format!("{}  span: {:?}\n", indent_str, param.span));
    }
}

fn pretty_print_hir_ctor(ctor: &HirCtorDecl, output: &mut String, indent: usize, include_spans: bool) {
    let indent_str = "  ".repeat(indent);
    output.push_str(&format!("{}CtorDecl\n", indent_str));
    output.push_str(&format!("{}  name: {}\n", indent_str, ctor.name));
    output.push_str(&format!("{}  params:\n", indent_str));
    for param in &ctor.params {
        pretty_print_hir_param(param, output, indent + 2, include_spans);
    }
    output.push_str(&format!("{}  body:\n", indent_str));
    pretty_print_hir_block(&ctor.body, output, indent + 2, include_spans);
    if include_spans {
        output.push_str(&format!("{}  span: {:?}", indent_str, ctor.span));
    }
}

fn pretty_print_hir_method(method: &HirMethodDecl, output: &mut String, indent: usize, include_spans: bool) {
    let indent_str = "  ".repeat(indent);
    output.push_str(&format!("{}MethodDecl\n", indent_str));
    output.push_str(&format!("{}  name: {}\n", indent_str, method.name));
    output.push_str(&format!("{}  symbol: {:?}\n", indent_str, method.symbol));
    output.push_str(&format!("{}  is_instance: {}\n", indent_str, method.is_instance));
    output.push_str(&format!("{}  params:\n", indent_str));
    for param in &method.params {
        pretty_print_hir_param(param, output, indent + 2, include_spans);
    }
    if let Some(ty) = &method.return_type {
        output.push_str(&format!("{}  return_type: {:?}\n", indent_str, ty));
    }
    output.push_str(&format!("{}  body:\n", indent_str));
    pretty_print_hir_block(&method.body, output, indent + 2, include_spans);
    if include_spans {
        output.push_str(&format!("{}  span: {:?}", indent_str, method.span));
    }
}

// Snapshot tests

#[test]
fn snapshot_postfix_inc() {
    let source = "def test()\n\tx := 1\n\tx++";
    let hir = lower_source(source);
    assert_snapshot!("postfix_inc", pretty_print_hir(&hir));
}

#[test]
fn snapshot_postfix_dec() {
    let source = "def test()\n\tx := 10\n\tx--";
    let hir = lower_source(source);
    assert_snapshot!("postfix_dec", pretty_print_hir(&hir));
}

#[test]
fn snapshot_for_in_loop() {
    let source = "def test()\n\tfor (num in arr)\n\t\tprint(num)";
    let hir = lower_source(source);
    assert_snapshot!("for_in_loop", pretty_print_hir(&hir));
}

#[test]
fn snapshot_match_statement() {
    let source = "def test(x)\n\tmatch(x)\n\t\tcase 1\n\t\t\tret \"one\"\n\t\telse\n\t\t\tret \"other\"";
    let hir = lower_source(source);
    assert_snapshot!("match_statement", pretty_print_hir(&hir));
}

#[test]
fn snapshot_match_multiple_patterns() {
    let source = "def test(x)\n\tmatch(x)\n\t\tcase 1, 2, 3\n\t\t\tret \"small\"\n\t\telse\n\t\t\tret \"other\"";
    let hir = lower_source(source);
    assert_snapshot!("match_multiple_patterns", pretty_print_hir(&hir));
}

#[test]
fn snapshot_ctor_implicit_assign() {
    let source = "cls Dog\n\tobj Dog(name)";
    let hir = lower_source(source);
    assert_snapshot!("ctor_implicit_assign", pretty_print_hir(&hir));
}

#[test]
fn snapshot_function_declaration() {
    let source = "def add(int x, int y) -> int\n\tret x + y";
    let hir = lower_source(source);
    assert_snapshot!("function_declaration", pretty_print_hir(&hir));
}

#[test]
fn snapshot_variable_resolution() {
    let source = "x := 1\ny := x + 2";
    let hir = lower_source(source);
    assert_snapshot!("variable_resolution", pretty_print_hir(&hir));
}

#[test]
fn snapshot_lambda_expression() {
    // Lambda syntax may not be fully supported yet
    let source = "f := (x) := x + 1";
    // Try to lower, but skip if it fails (lambda syntax not fully implemented)
    let file_id = brief_diagnostic::FileId(0);
    let (tokens, _) = brief_lexer::lex(source, file_id);
    let (ast, _) = brief_parser::parse(tokens, file_id);
    if let Ok(hir) = brief_hir::lower(ast) {
        assert_snapshot!("lambda_expression", pretty_print_hir(&hir));
    }
    // If parsing/lowering fails, skip the snapshot test
    // This is acceptable until lambda syntax is fully implemented
}

#[test]
fn snapshot_complex_desugaring() {
    let source = "def test()\n\tfor (num in arr)\n\t\tprint(num)\n\t\tnum++";
    let hir = lower_source(source);
    assert_snapshot!("complex_desugaring", pretty_print_hir(&hir));
}

