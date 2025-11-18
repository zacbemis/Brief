use brief_ast::{Expr, BinaryOp, Param};
use brief_diagnostic::{FileId, Position, Span};

fn span(line: u32, column: u32) -> Span {
    let pos = Position::new(line, column);
    Span::single(FileId(0), pos)
}

#[test]
fn literal_expr_span_is_preserved() {
    let literal_span = span(1, 1);
    let expr = Expr::Integer(42, literal_span);
    assert_eq!(expr.span(), literal_span);
}

#[test]
fn nested_expr_span_reports_outer_span() {
    let outer_span = Span::new(
        FileId(0),
        Position::new(1, 1),
        Position::new(1, 10),
    );
    let left = Expr::Integer(1, span(1, 1));
    let right = Expr::Integer(2, span(1, 5));
    let expr = Expr::BinaryOp {
        left: Box::new(left),
        op: BinaryOp::Add,
        right: Box::new(right),
        span: outer_span,
    };
    assert_eq!(expr.span(), outer_span, "BinaryOp::span should return the enclosing span");
}

#[test]
fn lambda_param_metadata_is_retained() {
    let param_span = span(2, 3);
    let param = Param {
        name: "value".into(),
        type_annotation: None,
        span: param_span,
    };
    let body_span = span(2, 10);
    let lambda_span = Span::new(FileId(0), Position::new(2, 1), Position::new(2, 12));
    let lambda = Expr::Lambda {
        params: vec![param.clone()],
        body: Box::new(Expr::Variable("value".into(), body_span)),
        span: lambda_span,
    };
    assert_eq!(lambda.span(), lambda_span);
    if let Expr::Lambda { params, .. } = lambda {
        assert_eq!(params, vec![param]);
    } else {
        panic!("Expected lambda expression");
    }
}

