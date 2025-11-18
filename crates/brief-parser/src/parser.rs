use crate::error::ParseError;
use brief_ast::*;
use brief_diagnostic::{FileId, Position, Span};
use brief_lexer::{Token, TokenKind};

/// Recursive-descent parser for Brief language
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    errors: Vec<ParseError>,
    file_id: FileId,
    error_count: usize,
    max_errors: usize,
}

impl Parser {
    /// Create a new parser
    pub fn new(tokens: Vec<Token>, file_id: FileId) -> Self {
        Self {
            tokens,
            current: 0,
            errors: Vec::new(),
            file_id,
            error_count: 0,
            max_errors: 50,
        }
    }

    /// Get all parse errors
    pub fn get_errors(&self) -> &[ParseError] {
        &self.errors
    }

    /// Main entry point: parse the entire program
    pub fn parse(&mut self) -> Program {
        let start_span = self.current_span();
        let mut declarations = Vec::new();

        while !self.is_at_end() {
            declarations.push(self.parse_declaration());

            // Consume newlines between declarations
            while self.check(&TokenKind::Newline) {
                self.advance();
            }
        }

        let end_span = self.current_span();
        Program {
            declarations,
            span: Span::new(self.file_id, start_span.start, end_span.end),
        }
    }

    // ============================================================================
    // Token Stream Navigation
    // ============================================================================

    pub(crate) fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    pub(crate) fn peek_kind(&self) -> Option<&TokenKind> {
        self.peek().map(|t| &t.kind)
    }

    pub(crate) fn peek_nth(&self, n: usize) -> Option<&Token> {
        self.tokens.get(self.current + n)
    }

    pub(crate) fn advance(&mut self) -> Option<&Token> {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    pub(crate) fn previous(&self) -> Option<&Token> {
        if self.current > 0 {
            self.tokens.get(self.current - 1)
        } else {
            None
        }
    }

    pub(crate) fn is_at_end(&self) -> bool {
        matches!(self.peek_kind(), Some(TokenKind::Eof) | None)
    }

    pub(crate) fn check(&self, kind: &TokenKind) -> bool {
        self.peek_kind().map(|k| k == kind).unwrap_or(false)
    }

    pub(crate) fn match_token(&mut self, kinds: &[TokenKind]) -> bool {
        for kind in kinds {
            if self.check(kind) {
                self.advance();
                return true;
            }
        }
        false
    }

    pub(crate) fn consume(&mut self, kind: TokenKind, message: &str) -> Result<&Token, ()> {
        if self.check(&kind) {
            Ok(self.advance().unwrap())
        } else {
            self.error_at_current(message);
            Err(())
        }
    }

    pub(crate) fn expect(&mut self, kind: TokenKind, message: &str) {
        let _ = self.consume(kind, message);
    }

    pub(crate) fn current_span(&self) -> Span {
        if let Some(token) = self.peek() {
            token.span
        } else if let Some(token) = self.previous() {
            token.span
        } else {
            Span::single(self.file_id, Position::new(1, 1))
        }
    }

    pub(crate) fn file_id(&self) -> FileId {
        self.file_id
    }

    // ============================================================================
    // Error Handling
    // ============================================================================

    pub(crate) fn error(&mut self, token: &Token, message: &str) {
        if self.error_count >= self.max_errors {
            return;
        }

        self.error_count += 1;

        let mut error = ParseError::new(message.to_string(), token.span);

        // Add secondary labels for context
        if let Some(prev) = self.previous() {
            error = error.with_label(prev.span, "Previous token here".to_string());
        }

        self.errors.push(error);
    }

    pub(crate) fn error_at_current(&mut self, message: &str) {
        if let Some(token) = self.peek().cloned() {
            self.error(&token, message);
        }
    }

    /// Panic-mode error recovery: synchronize to next safe token
    pub(crate) fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self
                .previous()
                .map(|t| t.kind == TokenKind::Semicolon)
                .unwrap_or(false)
            {
                return;
            }

            match self.peek_kind() {
                Some(TokenKind::Newline)
                | Some(TokenKind::Dedent)
                | Some(TokenKind::RightParen)
                | Some(TokenKind::RightBracket)
                | Some(TokenKind::RightBrace)
                | Some(TokenKind::Else)
                | Some(TokenKind::Case)
                | Some(TokenKind::Match)
                | Some(TokenKind::Def)
                | Some(TokenKind::Cls) => return,
                _ => {
                    self.advance();
                }
            }
        }
    }

    // ============================================================================
    // Declaration Parsing
    // ============================================================================

    fn parse_declaration(&mut self) -> Decl {
        let start_span = self.current_span();

        // Note: Import syntax will be handled later - for now, treat as identifier
        if self.check(&TokenKind::Def) {
            Decl::FuncDecl(self.parse_function_declaration())
        } else if self.check(&TokenKind::Cls) {
            Decl::ClassDecl(self.parse_class_declaration())
        } else if self.check(&TokenKind::Const) {
            Decl::ConstDecl(self.parse_const_declaration())
        } else if self.is_type_keyword() || self.is_identifier() {
            // Variable declaration or expression statement
            Decl::VarDecl(self.parse_var_declaration())
        } else {
            self.error_at_current("Expected declaration");
            self.synchronize();
            Decl::Error(start_span)
        }
    }

    // ============================================================================
    // Helper Methods
    // ============================================================================

    pub(crate) fn is_type_keyword(&self) -> bool {
        matches!(
            self.peek_kind(),
            Some(TokenKind::Int)
                | Some(TokenKind::Char)
                | Some(TokenKind::Str)
                | Some(TokenKind::Dub)
                | Some(TokenKind::Bool)
        )
    }

    pub(crate) fn is_identifier(&self) -> bool {
        matches!(self.peek_kind(), Some(TokenKind::Identifier(_)))
    }

    pub(crate) fn expect_identifier(&mut self, message: &str) -> String {
        match self.peek_kind() {
            Some(TokenKind::Identifier(name)) => {
                let name = name.clone();
                self.advance();
                name
            }
            _ => {
                self.error_at_current(message);
                "".to_string()
            }
        }
    }
}
