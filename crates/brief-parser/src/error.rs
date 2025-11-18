use brief_diagnostic::Span;

/// Parse error with rich diagnostic information
#[derive(Debug, Clone, PartialEq)]
pub struct ParseError {
    pub message: String,
    pub span: Span,
    pub secondary_labels: Vec<(Span, String)>,
}

impl ParseError {
    pub fn new(message: String, span: Span) -> Self {
        Self {
            message,
            span,
            secondary_labels: Vec::new(),
        }
    }

    pub fn with_label(mut self, span: Span, label: String) -> Self {
        self.secondary_labels.push((span, label));
        self
    }
}

