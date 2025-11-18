use crate::parser::Parser;
use brief_ast::ty::ArrayDim;
use brief_ast::*;
use brief_diagnostic::Span;
use brief_lexer::TokenKind;

impl Parser {
    /// Parse a type
    pub fn parse_type(&mut self) -> Type {
        let start_span = self.current_span();
        let base_type = match self.peek_kind() {
            Some(TokenKind::Int) => {
                self.advance();
                Type::Int
            }
            Some(TokenKind::Char) => {
                self.advance();
                Type::Char
            }
            Some(TokenKind::Str) => {
                self.advance();
                Type::Str
            }
            Some(TokenKind::Dub) => {
                self.advance();
                Type::Dub
            }
            Some(TokenKind::Bool) => {
                self.advance();
                Type::Bool
            }
            _ => {
                self.error_at_current("Expected type");
                return Type::Int; // Fallback
            }
        };

        self.parse_array_or_map_type(base_type, start_span)
    }

    /// Parse array or map type after base type
    fn parse_array_or_map_type(&mut self, base: Type, span: Span) -> Type {
        // Check for map first: int:str{}
        if self.check(&TokenKind::Colon) {
            self.advance();
            let value_type = self.parse_type();
            self.expect(TokenKind::LeftBrace, "Expected '{' in map type");
            self.expect(TokenKind::RightBrace, "Expected '}' in map type");

            Type::Map {
                key_type: Box::new(base),
                value_type: Box::new(value_type),
                span,
            }
        }
        // Check for array: int[] or int{}
        else if self.check(&TokenKind::LeftBracket) || self.check(&TokenKind::LeftBrace) {
            let mut dims = Vec::new();
            let mut is_bracket = self.check(&TokenKind::LeftBracket);
            self.advance();

            // Parse dimensions
            loop {
                if is_bracket && self.check(&TokenKind::RightBracket) {
                    // Empty bracket: int[]
                    dims.push(ArrayDim::Dynamic);
                    self.advance();
                } else if is_bracket && matches!(self.peek_kind(), Some(TokenKind::Integer(_))) {
                    // Fixed-size array: int[10]
                    if let Some(TokenKind::Integer(n)) = self.peek_kind() {
                        let size = *n as usize;
                        self.advance(); // Consume integer
                        self.expect(TokenKind::RightBracket, "Expected ']' after array size");
                        dims.push(ArrayDim::Fixed(size));
                    }
                } else if !is_bracket {
                    // Dynamic array or special: int{} or int{stk} or int{que}
                    // Check for special types before the closing brace
                    if let Some(TokenKind::Identifier(s)) = self.peek_kind() {
                        match s.as_str() {
                            "stk" => {
                                dims.push(ArrayDim::Stack);
                                self.advance(); // Consume 'stk'
                                self.expect(TokenKind::RightBrace, "Expected '}' after 'stk'");
                            }
                            "que" => {
                                dims.push(ArrayDim::Queue);
                                self.advance(); // Consume 'que'
                                self.expect(TokenKind::RightBrace, "Expected '}' after 'que'");
                            }
                            _ => {
                                // Regular dynamic array: int{}
                                self.expect(TokenKind::RightBrace, "Expected '}' in array type");
                                dims.push(ArrayDim::Dynamic);
                            }
                        }
                    } else {
                        // Empty braces: int{}
                        self.expect(TokenKind::RightBrace, "Expected '}' in array type");
                        dims.push(ArrayDim::Dynamic);
                    }
                } else {
                    break;
                }

                // Check for more dimensions
                if !self.check(&TokenKind::LeftBracket) && !self.check(&TokenKind::LeftBrace) {
                    break;
                }
                is_bracket = self.check(&TokenKind::LeftBracket);
                self.advance();
            }

            Type::Array {
                base: Box::new(base),
                dims,
                span,
            }
        } else if self.check(&TokenKind::Colon) {
            // Map type: int:str{}
            self.advance();
            let value_type = self.parse_type();
            self.expect(TokenKind::LeftBrace, "Expected '{' in map type");
            self.expect(TokenKind::RightBrace, "Expected '}' in map type");

            Type::Map {
                key_type: Box::new(base),
                value_type: Box::new(value_type),
                span,
            }
        } else {
            base
        }
    }
}
