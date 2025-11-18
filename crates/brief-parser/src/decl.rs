use brief_ast::*;
use brief_lexer::TokenKind;
use brief_diagnostic::Span;
use crate::parser::Parser;

impl Parser {
    /// Parse function declaration
    pub(crate) fn parse_function_declaration(&mut self) -> FuncDecl {
        let start_span = self.current_span();
        self.advance(); // Consume 'def'

        let name = self.expect_identifier("Expected function name");
        self.expect(TokenKind::LeftParen, "Expected '(' after function name");

        let params = self.parse_parameter_list();

        self.expect(TokenKind::RightParen, "Expected ')' after parameters");

        // Optional return type
        let return_type = if self.check(&TokenKind::Arrow) {
            self.advance();
            Some(self.parse_type())
        } else {
            None
        };

        let body = self.parse_block();

        let end_span = self.current_span();
        FuncDecl {
            name,
            params,
            return_type,
            body,
            span: Span::new(self.file_id(), start_span.start, end_span.end),
        }
    }

    /// Parse parameter list
    pub(crate) fn parse_parameter_list(&mut self) -> Vec<Param> {
        let mut params = Vec::new();

        if !self.check(&TokenKind::RightParen) {
            loop {
                // Optional type annotation
                let type_annotation = if self.is_type_keyword() {
                    Some(self.parse_type())
                } else {
                    None
                };

                let name = self.expect_identifier("Expected parameter name");
                let span = self.previous().unwrap().span;

                params.push(Param {
                    name,
                    type_annotation,
                    span,
                });

                if !self.match_token(&[TokenKind::Comma]) {
                    break;
                }
            }
        }

        params
    }

    /// Parse class declaration
    pub(crate) fn parse_class_declaration(&mut self) -> ClassDecl {
        let start_span = self.current_span();
        self.advance(); // Consume 'cls'

        let name = self.expect_identifier("Expected class name");

        // Expect Indent for class body
        self.expect(TokenKind::Indent, "Expected indented class body");
        self.advance();

        let mut constructor = None;
        let mut methods = Vec::new();

        while !self.check(&TokenKind::Dedent) && !self.is_at_end() {
            if self.check(&TokenKind::Obj) {
                // Check if next token is the class name (constructor) or 'def' (instance method)
                // Cache the peek to avoid multiple lookups
                let next_token = self.peek_nth(1);
                if let Some(token) = next_token {
                    match &token.kind {
                        TokenKind::Identifier(next_name) if next_name == &name => {
                            // Constructor: obj ClassName(...)
                            constructor = Some(self.parse_constructor(&name));
                        }
                        TokenKind::Def => {
                            // Instance method: obj def method(...)
                            methods.push(self.parse_method(true));
                        }
                        TokenKind::Identifier(_) => {
                            // Instance method: obj def method(...) - but identifier doesn't match class name
                            methods.push(self.parse_method(true));
                        }
                        _ => {
                            self.error_at_current("Expected constructor or method after 'obj'");
                            self.synchronize();
                        }
                    }
                } else {
                    self.error_at_current("Expected constructor or method after 'obj'");
                    self.synchronize();
                }
            } else if self.check(&TokenKind::Def) {
                // Static method: def method(...)
                methods.push(self.parse_method(false));
            } else {
                self.error_at_current("Expected 'obj' or 'def' in class body");
                self.synchronize();
            }

            if self.check(&TokenKind::Newline) {
                self.advance();
            }
        }

        if self.check(&TokenKind::Dedent) {
            self.advance();
        }

        let end_span = self.current_span();
        ClassDecl {
            name,
            constructor,
            methods,
            span: Span::new(self.file_id(), start_span.start, end_span.end),
        }
    }

    /// Parse constructor declaration
    pub(crate) fn parse_constructor(&mut self, class_name: &str) -> CtorDecl {
        let start_span = self.current_span();
        self.advance(); // Consume 'obj'

        // Verify class name matches
        let name = self.expect_identifier("Expected constructor name");
        if name != class_name {
            self.error_at_current(&format!("Constructor name must match class name '{}'", class_name));
        }

        self.expect(TokenKind::LeftParen, "Expected '(' after constructor name");
        let params = self.parse_parameter_list();
        self.expect(TokenKind::RightParen, "Expected ')' after constructor parameters");

        let body = self.parse_block();

        let end_span = self.current_span();
        CtorDecl {
            name,
            params,
            body,
            span: Span::new(self.file_id(), start_span.start, end_span.end),
        }
    }

    /// Parse method declaration
    pub(crate) fn parse_method(&mut self, is_instance: bool) -> MethodDecl {
        let start_span = self.current_span();

        if is_instance {
            self.advance(); // Consume 'obj'
        }

        self.advance(); // Consume 'def'

        let name = self.expect_identifier("Expected method name");
        self.expect(TokenKind::LeftParen, "Expected '(' after method name");

        let params = self.parse_parameter_list();

        self.expect(TokenKind::RightParen, "Expected ')' after parameters");

        // Optional return type
        let return_type = if self.check(&TokenKind::Arrow) {
            self.advance();
            Some(self.parse_type())
        } else {
            None
        };

        let body = self.parse_block();

        let end_span = self.current_span();
        MethodDecl {
            name,
            is_instance,
            params,
            return_type,
            body,
            span: Span::new(self.file_id(), start_span.start, end_span.end),
        }
    }

    /// Parse constant declaration
    pub(crate) fn parse_const_declaration(&mut self) -> ConstDecl {
        let start_span = self.current_span();
        self.advance(); // Consume 'const'

        let name = self.expect_identifier("Expected constant name");
        self.expect(TokenKind::InitAssign, "Expected ':=' after constant name");
        let initializer = self.parse_expression();

        let end_span = self.current_span();
        ConstDecl {
            name,
            initializer,
            span: Span::new(self.file_id(), start_span.start, end_span.end),
        }
    }

    /// Parse variable declaration
    pub(crate) fn parse_var_declaration(&mut self) -> VarDecl {
        let start_span = self.current_span();

        // Optional type annotation
        let type_annotation = if self.is_type_keyword() {
            Some(self.parse_type())
        } else {
            None
        };

        // Variable name
        let name = self.expect_identifier("Expected variable name");

        // Optional initializer
        let initializer = if self.check(&TokenKind::InitAssign) {
            self.advance();
            Some(self.parse_expression())
        } else {
            None
        };

        let end_span = self.current_span();
        VarDecl {
            name,
            type_annotation,
            initializer,
            span: Span::new(self.file_id(), start_span.start, end_span.end),
        }
    }
}

