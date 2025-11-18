use crate::parser::Parser;
use brief_ast::*;
use brief_diagnostic::Span;
use brief_lexer::TokenKind;

impl Parser {
    /// Parse an expression (entry point)
    pub fn parse_expression(&mut self) -> Expr {
        self.parse_assignment()
    }

    /// Assignment expressions (right-associative)
    fn parse_assignment(&mut self) -> Expr {
        let expr = self.parse_ternary();

        if self.match_token(&[
            TokenKind::Assign,
            TokenKind::InitAssign,
            TokenKind::PlusAssign,
            TokenKind::MinusAssign,
            TokenKind::StarAssign,
            TokenKind::SlashAssign,
            TokenKind::PercentAssign,
            TokenKind::PowAssign,
        ]) {
            let op_token = self.previous().unwrap();
            let op = match op_token.kind {
                TokenKind::Assign => BinaryOp::Assign,
                TokenKind::InitAssign => BinaryOp::InitAssign,
                TokenKind::PlusAssign => BinaryOp::PlusAssign,
                TokenKind::MinusAssign => BinaryOp::MinusAssign,
                TokenKind::StarAssign => BinaryOp::StarAssign,
                TokenKind::SlashAssign => BinaryOp::SlashAssign,
                TokenKind::PercentAssign => BinaryOp::PercentAssign,
                TokenKind::PowAssign => BinaryOp::PowAssign,
                _ => unreachable!(),
            };
            let value = self.parse_assignment(); // Right-associative
            let span = Span::new(self.file_id(), expr.span().start, value.span().end);
            return Expr::BinaryOp {
                left: Box::new(expr),
                op,
                right: Box::new(value),
                span,
            };
        }

        expr
    }

    /// Ternary operator (right-associative)
    fn parse_ternary(&mut self) -> Expr {
        let expr = self.parse_logical_or();

        if self.check(&TokenKind::Question) {
            let start_span = expr.span();
            self.advance();
            let then_expr = self.parse_expression();
            self.expect(TokenKind::Colon, "Expected ':' after ternary condition");
            let else_expr = self.parse_ternary(); // Right-associative
            let end_span = else_expr.span();
            return Expr::Ternary {
                condition: Box::new(expr),
                then_expr: Box::new(then_expr),
                else_expr: Box::new(else_expr),
                span: Span::new(self.file_id(), start_span.start, end_span.end),
            };
        }

        expr
    }

    /// Logical OR (left-associative)
    fn parse_logical_or(&mut self) -> Expr {
        let mut expr = self.parse_logical_and();

        while self.match_token(&[TokenKind::Or]) {
            let op = BinaryOp::Or;
            let right = self.parse_logical_and();
            let span = Span::new(self.file_id(), expr.span().start, right.span().end);
            expr = Expr::BinaryOp {
                left: Box::new(expr),
                op,
                right: Box::new(right),
                span,
            };
        }

        expr
    }

    /// Logical AND (left-associative)
    fn parse_logical_and(&mut self) -> Expr {
        let mut expr = self.parse_bitwise_or();

        while self.match_token(&[TokenKind::And]) {
            let op = BinaryOp::And;
            let right = self.parse_bitwise_or();
            let span = Span::new(self.file_id(), expr.span().start, right.span().end);
            expr = Expr::BinaryOp {
                left: Box::new(expr),
                op,
                right: Box::new(right),
                span,
            };
        }

        expr
    }

    /// Bitwise OR (left-associative)
    fn parse_bitwise_or(&mut self) -> Expr {
        let mut expr = self.parse_bitwise_xor();

        while self.match_token(&[TokenKind::BitOr]) {
            let op = BinaryOp::BitOr;
            let right = self.parse_bitwise_xor();
            let span = Span::new(self.file_id(), expr.span().start, right.span().end);
            expr = Expr::BinaryOp {
                left: Box::new(expr),
                op,
                right: Box::new(right),
                span,
            };
        }

        expr
    }

    /// Bitwise XOR (left-associative)
    fn parse_bitwise_xor(&mut self) -> Expr {
        let mut expr = self.parse_bitwise_and();

        while self.match_token(&[TokenKind::BitXor]) {
            let op = BinaryOp::BitXor;
            let right = self.parse_bitwise_and();
            let span = Span::new(self.file_id(), expr.span().start, right.span().end);
            expr = Expr::BinaryOp {
                left: Box::new(expr),
                op,
                right: Box::new(right),
                span,
            };
        }

        expr
    }

    /// Bitwise AND (left-associative)
    fn parse_bitwise_and(&mut self) -> Expr {
        let mut expr = self.parse_equality();

        while self.match_token(&[TokenKind::BitAnd]) {
            let op = BinaryOp::BitAnd;
            let right = self.parse_equality();
            let span = Span::new(self.file_id(), expr.span().start, right.span().end);
            expr = Expr::BinaryOp {
                left: Box::new(expr),
                op,
                right: Box::new(right),
                span,
            };
        }

        expr
    }

    /// Equality operators (left-associative)
    fn parse_equality(&mut self) -> Expr {
        let mut expr = self.parse_comparison();

        while self.match_token(&[TokenKind::Eq, TokenKind::Ne]) {
            let op = match self.previous().unwrap().kind {
                TokenKind::Eq => BinaryOp::Eq,
                TokenKind::Ne => BinaryOp::Ne,
                _ => unreachable!(),
            };
            let right = self.parse_comparison();
            let span = Span::new(self.file_id(), expr.span().start, right.span().end);
            expr = Expr::BinaryOp {
                left: Box::new(expr),
                op,
                right: Box::new(right),
                span,
            };
        }

        expr
    }

    /// Comparison operators (left-associative)
    fn parse_comparison(&mut self) -> Expr {
        let mut expr = self.parse_shift();

        while self.match_token(&[TokenKind::Lt, TokenKind::Le, TokenKind::Gt, TokenKind::Ge]) {
            let op = match self.previous().unwrap().kind {
                TokenKind::Lt => BinaryOp::Lt,
                TokenKind::Le => BinaryOp::Le,
                TokenKind::Gt => BinaryOp::Gt,
                TokenKind::Ge => BinaryOp::Ge,
                _ => unreachable!(),
            };
            let right = self.parse_shift();
            let span = Span::new(self.file_id(), expr.span().start, right.span().end);
            expr = Expr::BinaryOp {
                left: Box::new(expr),
                op,
                right: Box::new(right),
                span,
            };
        }

        expr
    }

    /// Shift operators (left-associative)
    fn parse_shift(&mut self) -> Expr {
        let mut expr = self.parse_addition();

        while self.match_token(&[TokenKind::Shl, TokenKind::Shr]) {
            let op = match self.previous().unwrap().kind {
                TokenKind::Shl => BinaryOp::Shl,
                TokenKind::Shr => BinaryOp::Shr,
                _ => unreachable!(),
            };
            let right = self.parse_addition();
            let span = Span::new(self.file_id(), expr.span().start, right.span().end);
            expr = Expr::BinaryOp {
                left: Box::new(expr),
                op,
                right: Box::new(right),
                span,
            };
        }

        expr
    }

    /// Addition and subtraction (left-associative)
    fn parse_addition(&mut self) -> Expr {
        let mut expr = self.parse_multiplication();

        while self.match_token(&[TokenKind::Plus, TokenKind::Minus]) {
            let op = match self.previous().unwrap().kind {
                TokenKind::Plus => BinaryOp::Add,
                TokenKind::Minus => BinaryOp::Sub,
                _ => unreachable!(),
            };
            let right = self.parse_multiplication();
            let span = Span::new(self.file_id(), expr.span().start, right.span().end);
            expr = Expr::BinaryOp {
                left: Box::new(expr),
                op,
                right: Box::new(right),
                span,
            };
        }

        expr
    }

    /// Multiplication, division, and modulo (left-associative)
    fn parse_multiplication(&mut self) -> Expr {
        let mut expr = self.parse_power();

        while self.match_token(&[TokenKind::Star, TokenKind::Slash, TokenKind::Percent]) {
            let op = match self.previous().unwrap().kind {
                TokenKind::Star => BinaryOp::Mul,
                TokenKind::Slash => BinaryOp::Div,
                TokenKind::Percent => BinaryOp::Mod,
                _ => unreachable!(),
            };
            let right = self.parse_power();
            let span = Span::new(self.file_id(), expr.span().start, right.span().end);
            expr = Expr::BinaryOp {
                left: Box::new(expr),
                op,
                right: Box::new(right),
                span,
            };
        }

        expr
    }

    /// Power operator (right-associative)
    fn parse_power(&mut self) -> Expr {
        let mut expr = self.parse_unary();

        while self.match_token(&[TokenKind::Pow]) {
            let op = BinaryOp::Pow;
            let right = self.parse_power(); // Right-associative
            let span = Span::new(self.file_id(), expr.span().start, right.span().end);
            expr = Expr::BinaryOp {
                left: Box::new(expr),
                op,
                right: Box::new(right),
                span,
            };
        }

        expr
    }

    /// Unary operators (right-associative)
    fn parse_unary(&mut self) -> Expr {
        if self.match_token(&[
            TokenKind::Not,
            TokenKind::BitNot,
            TokenKind::Minus,
            TokenKind::Plus,
        ]) {
            let op_token = self.previous().unwrap();
            let op_token_span = op_token.span;
            let op = match op_token.kind {
                TokenKind::Not => UnaryOp::Not,
                TokenKind::BitNot => UnaryOp::BitNot,
                TokenKind::Minus => UnaryOp::Neg,
                TokenKind::Plus => UnaryOp::Pos,
                _ => unreachable!(),
            };
            let expr = self.parse_unary(); // Right-associative
            let expr_span = expr.span();
            let span = Span::new(self.file_id(), op_token_span.start, expr_span.end);
            return Expr::UnaryOp {
                op,
                expr: Box::new(expr),
                span,
            };
        }

        self.parse_postfix()
    }

    /// Postfix operators and primary expressions
    fn parse_postfix(&mut self) -> Expr {
        let mut expr = self.parse_primary();

        loop {
            // Postfix increment/decrement
            if self.match_token(&[TokenKind::Inc, TokenKind::Dec]) {
                let op = match self.previous().unwrap().kind {
                    TokenKind::Inc => PostfixOp::Inc,
                    TokenKind::Dec => PostfixOp::Dec,
                    _ => unreachable!(),
                };
                let span = Span::new(
                    self.file_id(),
                    expr.span().start,
                    self.previous().unwrap().span.end,
                );
                expr = Expr::PostfixOp {
                    expr: Box::new(expr),
                    op,
                    span,
                };
            }
            // Function call
            else if self.check(&TokenKind::LeftParen) {
                expr = self.finish_call(expr);
            }
            // Member access
            else if self.match_token(&[TokenKind::Dot]) {
                let name = self.expect_identifier("Expected property name after '.'");
                let span = Span::new(
                    self.file_id(),
                    expr.span().start,
                    self.previous().unwrap().span.end,
                );
                expr = Expr::MemberAccess {
                    object: Box::new(expr),
                    member: name,
                    span,
                };
            }
            // Index access
            else if self.check(&TokenKind::LeftBracket) {
                expr = self.finish_index(expr);
            }
            // Type cast
            else if self.check_type_keyword() {
                expr = self.finish_cast(expr);
            } else {
                break;
            }
        }

        expr
    }

    /// Parse a primary expression
    fn parse_primary(&mut self) -> Expr {
        // Extract the kind first to avoid borrow conflicts
        let kind = self.peek_kind().cloned();
        match kind {
            Some(TokenKind::True) => {
                let token = self.advance().unwrap();
                Expr::Boolean(true, token.span)
            }
            Some(TokenKind::False) => {
                let token = self.advance().unwrap();
                Expr::Boolean(false, token.span)
            }
            Some(TokenKind::Null) => {
                let token = self.advance().unwrap();
                Expr::Null(token.span)
            }
            Some(TokenKind::Integer(n)) => {
                let token = self.advance().unwrap();
                Expr::Integer(n, token.span)
            }
            Some(TokenKind::Double(d)) => {
                let token = self.advance().unwrap();
                Expr::Double(d, token.span)
            }
            Some(TokenKind::Character(c)) => {
                let token = self.advance().unwrap();
                Expr::Character(c, token.span)
            }
            Some(TokenKind::StrPart(_)) => self.parse_string_interpolation(),
            Some(TokenKind::Identifier(_)) => {
                let name = self.expect_identifier("Expected identifier");
                let span = self.previous().unwrap().span;
                Expr::Variable(name, span)
            }
            Some(TokenKind::Int)
            | Some(TokenKind::Char)
            | Some(TokenKind::Str)
            | Some(TokenKind::Dub)
            | Some(TokenKind::Bool) => {
                let token = self.advance().unwrap();
                let name = match &token.kind {
                    TokenKind::Int => "int",
                    TokenKind::Char => "char",
                    TokenKind::Str => "str",
                    TokenKind::Dub => "dub",
                    TokenKind::Bool => "bool",
                    _ => unreachable!(),
                };
                Expr::Variable(name.to_string(), token.span)
            }
            Some(TokenKind::LeftParen) => self.parse_grouping(),
            _ => {
                self.error_at_current("Expected expression");
                Expr::Error(self.current_span())
            }
        }
    }

    /// Parse a grouped expression: (expr)
    fn parse_grouping(&mut self) -> Expr {
        let start_span = self.advance().unwrap().span;
        let expr = self.parse_expression();
        self.expect(TokenKind::RightParen, "Expected ')' after expression");
        let end_span = self.previous().unwrap().span;
        let span = Span::new(self.file_id(), start_span.start, end_span.end);
        // Return the expression with updated span
        match expr {
            Expr::Error(_) => Expr::Error(span),
            _ => expr, // Keep the expression as-is (span already set)
        }
    }

    /// Parse string interpolation
    fn parse_string_interpolation(&mut self) -> Expr {
        let start_span = self.current_span();
        let mut parts = Vec::new();

        // Parse StrPart
        if let Some(TokenKind::StrPart(text)) = self.peek_kind() {
            let text = text.clone();
            self.advance();
            if !text.is_empty() {
                parts.push(InterpPart::Text(text));
            }
        }

        // Parse interpolation parts
        loop {
            // Extract token kind first to avoid borrow conflicts
            let token_kind = self.peek_kind().cloned();
            if let Some(kind) = token_kind {
                match kind {
                    TokenKind::InterpIdent(name) => {
                        let token = self.advance().unwrap();
                        parts.push(InterpPart::Ident(name, token.span));
                    }
                    TokenKind::InterpPath(path) => {
                        // Get span before advancing to avoid borrow conflict
                        let span = self
                            .peek()
                            .map(|t| t.span)
                            .unwrap_or_else(|| self.current_span());
                        self.advance();
                        // Parse path expression (e.g., obj.field)
                        let expr = self.parse_interpolation_path(&path, span);
                        parts.push(InterpPart::Path(expr, span));
                    }
                    TokenKind::StrPart(text) => {
                        self.advance();
                        if !text.is_empty() {
                            parts.push(InterpPart::Text(text));
                        }
                    }
                    _ => break,
                }
            } else {
                break;
            }
        }

        let end_span = self.current_span();
        Expr::Interpolation {
            parts,
            span: Span::new(self.file_id(), start_span.start, end_span.end),
        }
    }

    /// Parse an interpolation path like obj.field
    fn parse_interpolation_path(&mut self, path: &str, span: Span) -> Box<Expr> {
        // Simple implementation: parse as member access chain
        // For now, just create a variable reference
        // TODO: Parse actual path expressions
        Box::new(Expr::Variable(path.to_string(), span))
    }

    /// Finish a function call: expr(args)
    fn finish_call(&mut self, callee: Expr) -> Expr {
        let start_span = callee.span();
        self.advance(); // Consume '('
        let mut args = Vec::new();

        if !self.check(&TokenKind::RightParen) {
            loop {
                args.push(self.parse_expression());
                if !self.match_token(&[TokenKind::Comma]) {
                    break;
                }
            }
        }

        self.expect(TokenKind::RightParen, "Expected ')' after arguments");
        let end_span = self.previous().unwrap().span;
        Expr::Call {
            callee: Box::new(callee),
            args,
            span: Span::new(self.file_id(), start_span.start, end_span.end),
        }
    }

    /// Finish an index access: expr[index]
    fn finish_index(&mut self, object: Expr) -> Expr {
        let start_span = object.span();
        self.advance(); // Consume '['
        let index = self.parse_expression();
        self.expect(TokenKind::RightBracket, "Expected ']' after index");
        let end_span = self.previous().unwrap().span;
        Expr::Index {
            object: Box::new(object),
            index: Box::new(index),
            span: Span::new(self.file_id(), start_span.start, end_span.end),
        }
    }

    /// Finish a type cast: expr type
    fn finish_cast(&mut self, expr: Expr) -> Expr {
        let start_span = expr.span();
        let target_type = self.parse_type();
        let end_span = self.current_span();
        Expr::Cast {
            expr: Box::new(expr),
            target_type,
            span: Span::new(self.file_id(), start_span.start, end_span.end),
        }
    }

    fn check_type_keyword(&self) -> bool {
        matches!(
            self.peek_kind(),
            Some(TokenKind::Int)
                | Some(TokenKind::Char)
                | Some(TokenKind::Str)
                | Some(TokenKind::Dub)
                | Some(TokenKind::Bool)
        )
    }
}
