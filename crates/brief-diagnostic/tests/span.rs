use brief_diagnostic::{FileId, Position, Span};

#[test]
fn span_new_uses_distinct_start_and_end() {
    let file = FileId(7);
    let start = Position::new(1, 1);
    let end = Position::new(2, 5);
    let span = Span::new(file, start, end);
    assert_eq!(span.file_id, file);
    assert_eq!(span.start, start);
    assert_eq!(span.end, end);
}

#[test]
fn span_single_sets_identical_bounds() {
    let file = FileId(3);
    let pos = Position::new(10, 20);
    let span = Span::single(file, pos);
    assert_eq!(span.file_id, file);
    assert_eq!(span.start, pos);
    assert_eq!(span.end, pos);
}

