/// Unique identifier for a source file
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FileId(pub u32);

/// Source position (line and column, 1-indexed)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub line: u32,
    pub column: u32,
}

impl Position {
    pub fn new(line: u32, column: u32) -> Self {
        Self { line, column }
    }
}

/// Source span (start and end positions)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub file_id: FileId,
    pub start: Position,
    pub end: Position,
}

impl Span {
    pub fn new(file_id: FileId, start: Position, end: Position) -> Self {
        Self { file_id, start, end }
    }

    pub fn single(file_id: FileId, pos: Position) -> Self {
        Self {
            file_id,
            start: pos,
            end: pos,
        }
    }
}
