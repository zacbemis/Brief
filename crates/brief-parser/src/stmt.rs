use crate::parser::Parser;
use brief_ast::*;
use brief_diagnostic::Span;
use brief_lexer::TokenKind;

impl Parser {
    /// Parse a statement
    pub fn parse_statement(&mut self) -> Stmt {
        if self.check(&TokenKind::If) {
            self.parse_if_statement()
        } else if self.check(&TokenKind::While) {
            self.parse_while_statement()
        } else if self.check(&TokenKind::For) {
            self.parse_for_statement()
        } else if self.check(&TokenKind::Match) {
            self.parse_match_statement()
        } else if self.check(&TokenKind::Ret) {
            self.parse_return_statement()
        } else if self.check(&TokenKind::Break) {
            self.parse_break_statement()
        } else if self.check(&TokenKind::Continue) {
            self.parse_continue_statement()
        } else if self.is_declaration_start() {
            // Variable or constant declaration
            if self.check(&TokenKind::Const) {
                Stmt::ConstDecl(self.parse_const_declaration())
            } else {
                Stmt::VarDecl(self.parse_var_declaration())
            }
        } else {
            // Expression statement
            let expr = self.parse_expression();
            let span = expr.span();
            Stmt::Expr(expr, span)
        }
    }

    /// Check if we're at the start of a declaration
    fn is_declaration_start(&self) -> bool {
        self.check(&TokenKind::Const) || self.is_type_keyword() || self.is_identifier()
    }

    /// Parse a block (indentation-based)
    pub fn parse_block(&mut self) -> Block {
        let start_span = self.current_span();
        let mut statements = Vec::new();

        // Check if we have an Indent token (multi-line block)
        if self.check(&TokenKind::Indent) {
            self.advance(); // Consume Indent

            // Parse statements until Dedent
            while !self.check(&TokenKind::Dedent) && !self.is_at_end() {
                statements.push(self.parse_statement());

                // Consume newline between statements
                if self.check(&TokenKind::Newline) {
                    self.advance();
                }
            }

            // Consume Dedent
            if self.check(&TokenKind::Dedent) {
                self.advance();
            }
        } else {
            // Single-line statement - no block, just one statement
            statements.push(self.parse_statement());
        }

        let end_span = self.current_span();
        Block {
            statements,
            span: Span::new(self.file_id(), start_span.start, end_span.end),
        }
    }

    /// Parse if statement
    fn parse_if_statement(&mut self) -> Stmt {
        let start_span = self.current_span();
        self.advance(); // Consume 'if'

        self.expect(TokenKind::LeftParen, "Expected '(' after 'if'");
        let condition = self.parse_expression();
        self.expect(TokenKind::RightParen, "Expected ')' after if condition");

        let then_branch = self.parse_block();
        let else_branch = if self.check(&TokenKind::Else) {
            self.advance();
            Some(self.parse_block())
        } else {
            None
        };

        let end_span = self.current_span();
        Stmt::If {
            condition,
            then_branch,
            else_branch,
            span: Span::new(self.file_id(), start_span.start, end_span.end),
        }
    }

    /// Parse while statement
    fn parse_while_statement(&mut self) -> Stmt {
        let start_span = self.current_span();
        self.advance(); // Consume 'while'

        self.expect(TokenKind::LeftParen, "Expected '(' after 'while'");
        let condition = self.parse_expression();
        self.expect(TokenKind::RightParen, "Expected ')' after while condition");

        let body = self.parse_block();

        let end_span = self.current_span();
        Stmt::While {
            condition,
            body,
            span: Span::new(self.file_id(), start_span.start, end_span.end),
        }
    }

    /// Parse for statement (C-style or for-in)
    fn parse_for_statement(&mut self) -> Stmt {
        let start_span = self.current_span();
        self.advance(); // Consume 'for'

        self.expect(TokenKind::LeftParen, "Expected '(' after 'for'");

        // Check if it's a for-in loop: for (var in expr)
        if self.is_identifier()
            && self
                .peek_nth(1)
                .map(|t| t.kind == TokenKind::In)
                .unwrap_or(false)
        {
            let var = self.expect_identifier("Expected variable name in for-in loop");
            self.expect(TokenKind::In, "Expected 'in' in for-in loop");
            let iterable = self.parse_expression();
            self.expect(
                TokenKind::RightParen,
                "Expected ')' after for-in expression",
            );

            let body = self.parse_block();

            let end_span = self.current_span();
            Stmt::ForIn {
                var,
                iterable,
                body,
                span: Span::new(self.file_id(), start_span.start, end_span.end),
            }
        } else {
            // C-style for loop: for (init; condition; increment)
            let init = if self.check(&TokenKind::Semicolon) {
                None
            } else {
                Some(Box::new(self.parse_for_init()))
            };

            self.expect(TokenKind::Semicolon, "Expected ';' after for init");

            let condition = if self.check(&TokenKind::Semicolon) {
                None
            } else {
                Some(self.parse_expression())
            };

            self.expect(TokenKind::Semicolon, "Expected ';' after for condition");

            let increment = if self.check(&TokenKind::RightParen) {
                None
            } else {
                Some(self.parse_expression())
            };

            self.expect(TokenKind::RightParen, "Expected ')' after for increment");

            let body = self.parse_block();

            let end_span = self.current_span();
            Stmt::For {
                init,
                condition,
                increment,
                body,
                span: Span::new(self.file_id(), start_span.start, end_span.end),
            }
        }
    }

    /// Parse for loop initialization (variable declaration or expression)
    fn parse_for_init(&mut self) -> Stmt {
        if self.is_type_keyword()
            || (self.is_identifier()
                && self
                    .peek_nth(1)
                    .map(|t| t.kind == TokenKind::InitAssign)
                    .unwrap_or(false))
        {
            // Variable declaration
            if self.check(&TokenKind::Const) {
                Stmt::ConstDecl(self.parse_const_declaration())
            } else {
                Stmt::VarDecl(self.parse_var_declaration())
            }
        } else {
            // Expression
            let expr = self.parse_expression();
            Stmt::Expr(expr, self.current_span())
        }
    }

    /// Parse match statement
    fn parse_match_statement(&mut self) -> Stmt {
        let start_span = self.current_span();
        self.advance(); // Consume 'match'

        self.expect(TokenKind::LeftParen, "Expected '(' after 'match'");
        let expr = self.parse_expression();
        self.expect(TokenKind::RightParen, "Expected ')' after match expression");

        let mut cases = Vec::new();

        while self.check(&TokenKind::Case) {
            cases.push(self.parse_match_case());
        }

        let else_branch = if self.check(&TokenKind::Else) {
            self.advance();
            Some(self.parse_block())
        } else {
            None
        };

        let end_span = self.current_span();
        Stmt::Match {
            expr,
            cases,
            else_branch,
            span: Span::new(self.file_id(), start_span.start, end_span.end),
        }
    }

    /// Parse a match case
    fn parse_match_case(&mut self) -> MatchCase {
        let start_span = self.current_span();
        self.advance(); // Consume 'case'

        let mut patterns = Vec::new();

        // Parse first pattern
        patterns.push(self.parse_expression());

        // Parse comma-separated patterns: case 'A', 'B', 'C'
        while self.check(&TokenKind::Comma) {
            self.advance();
            patterns.push(self.parse_expression());
        }

        let body = self.parse_block();

        MatchCase {
            patterns,
            body,
            span: start_span,
        }
    }

    /// Parse return statement
    fn parse_return_statement(&mut self) -> Stmt {
        let start_span = self.current_span();
        self.advance(); // Consume 'ret'

        // Check if there's a value expression (not newline, dedent, or indent)
        let value = if !self.check(&TokenKind::Newline)
            && !self.check(&TokenKind::Dedent)
            && !self.check(&TokenKind::Indent)
            && !self.is_at_end()
        {
            Some(self.parse_expression())
        } else {
            None
        };

        Stmt::Return {
            value,
            span: start_span,
        }
    }

    /// Parse break statement
    fn parse_break_statement(&mut self) -> Stmt {
        let span = self.current_span();
        self.advance(); // Consume 'break'
        Stmt::Break(span)
    }

    /// Parse continue statement
    fn parse_continue_statement(&mut self) -> Stmt {
        let span = self.current_span();
        self.advance(); // Consume 'continue'
        Stmt::Continue(span)
    }
}
