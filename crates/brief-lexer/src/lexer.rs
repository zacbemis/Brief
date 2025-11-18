use crate::token::{Token, TokenKind};
use brief_diagnostic::{FileId, Position, Span};
use std::collections::VecDeque;

/// Lexer for Brief source code
pub struct Lexer {
    source: Vec<char>,
    file_id: FileId,
    pos: usize,
    line: u32,
    column: u32,
    indent_stack: Vec<usize>,
    pending_indents: VecDeque<Token>,
    token_queue: VecDeque<Token>, // For string interpolation parts
    errors: Vec<String>,
    skip_next_line_start: bool, // Flag to skip line start handling after comment+tab
}

impl Lexer {
    pub fn new(source: &str, file_id: FileId) -> Self {
        Self {
            source: source.chars().collect(),
            file_id,
            pos: 0,
            line: 1,
            column: 1,
            indent_stack: vec![0],
            pending_indents: VecDeque::new(),
            token_queue: VecDeque::new(),
            errors: vec![],
            skip_next_line_start: false,
        }
    }

    /// Main entry point: lex the entire source
    pub fn lex(mut self) -> (Vec<Token>, Vec<String>) {
        let mut tokens = Vec::new();
        let mut at_line_start = true;

        while !self.is_at_end() {
            // Handle indentation at start of line
            if at_line_start {
                let indent = self.count_and_consume_leading_tabs();
                if self.is_empty_line() {
                    // Skip empty line (including comments)
                    // Consume everything up to and including the newline
                    // Don't emit a newline token for empty lines - they're just skipped
                    while let Some(ch) = self.peek() {
                        if ch == '\n' || ch == '\r' {
                            // Handle \r\n
                            if ch == '\r' && self.peek_next() == Some('\n') {
                                self.advance(); // Skip \r
                            }
                            self.advance(); // Skip \n
                            // Don't emit newline for empty lines - just skip them
                            break;
                        }
                        self.advance(); // Skip comment or whitespace
                    }
                    continue;
                } else {
                    // Handle indentation for non-empty line
                    self.handle_indentation(indent, &mut tokens);
                    at_line_start = false;
                }
            }

            // Skip whitespace (but not tabs - tabs trigger newline + indentation)
            if self.peek() == Some(' ') {
                self.advance();
                continue;
            }

            // Handle tabs in the middle of a line - they trigger a newline and indentation
            if self.peek() == Some('\t') {
                // Emit newline for the tab (tab will be consumed as indentation on next iteration)
                tokens.push(Token::new(TokenKind::Newline, self.current_span()));
                at_line_start = true;
                continue; // Next iteration will handle indentation (and consume the tab)
            }

            // Skip newlines (but track for next line)
            if self.peek() == Some('\n') || self.peek() == Some('\r') {
                if self.peek() == Some('\r') && self.peek_next() == Some('\n') {
                    self.advance(); // Skip \r
                }
                self.advance(); // Skip \n
                tokens.push(Token::new(TokenKind::Newline, self.current_span()));
                at_line_start = true;
                continue;
            }

            at_line_start = false;

            // Emit any pending indent/dedent tokens
            while let Some(token) = self.pending_indents.pop_front() {
                tokens.push(token);
            }

            // Emit any queued tokens (from string interpolation)
            if let Some(token) = self.token_queue.pop_front() {
                tokens.push(token);
                continue;
            }

            // Tokenize based on current character
            let token = self.next_token();
            // If we got EOF, we're done - don't add it yet, the final EOF will be added at the end
            if token.kind == TokenKind::Eof {
                break;
            }
            // Check if it's a newline before pushing (so we can set at_line_start)
            let is_newline = token.kind == TokenKind::Newline;
            tokens.push(token);
            
            // After pushing a token, check if there are queued tokens (e.g., from string interpolation)
            // These should be emitted immediately
            while let Some(queued_token) = self.token_queue.pop_front() {
                tokens.push(queued_token);
            }
            
            // If we got a newline, mark that we're at the start of the next line
            // (unless we've been told to skip it, e.g., after comment+tab)
            if is_newline {
                if self.skip_next_line_start {
                    // Skip line start handling - indentation already handled
                    self.skip_next_line_start = false;
                    continue; // Continue processing without handling indentation
                }
                at_line_start = true;
                continue; // Next iteration will handle indentation if needed
            }
            
            // After emitting a token, check if the next character is a tab
            // Tabs after tokens trigger a newline and increment indentation by 1
            if self.peek() == Some('\t') {
                // Emit any pending indent/dedent tokens first (should be empty, but just in case)
                while let Some(token) = self.pending_indents.pop_front() {
                    tokens.push(token);
                }
                // Consume the tab
                self.advance();
                // Emit newline
                tokens.push(Token::new(TokenKind::Newline, self.current_span()));
                // Increment indentation by 1 level
                let current_level = *self.indent_stack.last().unwrap();
                let new_level = current_level + 1;
                self.indent_stack.push(new_level);
                // Emit the indent token immediately
                tokens.push(Token::new(
                    TokenKind::Indent,
                    Span::single(self.file_id, Position::new(self.line, 1)),
                ));
                // Don't set at_line_start - we've already handled indentation, just continue processing
                continue; // Continue processing the rest of the line
            }
        }

        // Emit final newline if file doesn't end with one
        if !tokens.last().map_or(false, |t| t.kind == TokenKind::Newline) {
            tokens.push(Token::new(TokenKind::Newline, self.current_span()));
        }

        // Emit dedents for remaining indent levels
        while self.indent_stack.len() > 1 {
            self.indent_stack.pop();
            tokens.push(Token::new(
                TokenKind::Dedent,
                Span::single(self.file_id, Position::new(self.line, self.column)),
            ));
        }

        tokens.push(Token::new(
            TokenKind::Eof,
            Span::single(self.file_id, Position::new(self.line, self.column)),
        ));

        (tokens, self.errors)
    }

    fn handle_indentation(&mut self, indent: usize, tokens: &mut Vec<Token>) {
        let current_level = *self.indent_stack.last().unwrap();

        if indent > current_level {
            // Increase indentation - emit one Indent token for each level
            let mut level = current_level + 1;
            while level <= indent {
                self.indent_stack.push(level);
                self.pending_indents.push_back(Token::new(
                    TokenKind::Indent,
                    Span::single(self.file_id, Position::new(self.line, 1)),
                ));
                level += 1;
            }
        } else if indent < current_level {
            // Decrease indentation
            while self.indent_stack.len() > 1 {
                let top_level = *self.indent_stack.last().unwrap();
                if top_level <= indent {
                    break;
                }
                self.indent_stack.pop();
                tokens.push(Token::new(
                    TokenKind::Dedent,
                    Span::single(self.file_id, Position::new(self.line, 1)),
                ));
            }

            // Error if indent doesn't match any level (stack should have at least base level 0)
            let final_level = *self.indent_stack.last().unwrap();
            if final_level != indent {
                self.errors.push(format!(
                    "inconsistent indentation at line {}",
                    self.line
                ));
            }
        }
        // If indent == current_level, do nothing (same level, no change needed)
    }

    fn count_and_consume_leading_tabs(&mut self) -> usize {
        let mut count = 0;

        while self.pos < self.source.len() {
            match self.source[self.pos] {
                '\t' => {
                    count += 1;
                    self.pos += 1;
                    self.column += 1;
                }
                ' ' => {
                    // Error: spaces used for indentation
                    self.errors.push(format!(
                        "spaces cannot be used for indentation (use tabs) at line {}",
                        self.line
                    ));
                    break;
                }
                _ => break,
            }
        }

        count
    }

    fn is_empty_line(&self) -> bool {
        let mut pos = self.pos;
        while pos < self.source.len() {
            match self.source[pos] {
                ' ' | '\t' => pos += 1,
                '\n' | '\r' => return true,
                '/' if pos + 1 < self.source.len() && self.source[pos + 1] == '/' => {
                    // Comment line - skip to newline or tab
                    while pos < self.source.len() {
                        if self.source[pos] == '\n' || self.source[pos] == '\r' {
                            return true; // Empty line (comment only)
                        }
                        if self.source[pos] == '\t' {
                            return false; // Not empty - tab indicates more content
                        }
                        pos += 1;
                    }
                    return true; // EOF after comment
                }
                _ => return false,
            }
        }
        true
    }

    fn next_token(&mut self) -> Token {
        // Check if we're at EOF
        if self.is_at_end() {
            return Token::new(
                TokenKind::Eof,
                Span::single(self.file_id, self.current_pos()),
            );
        }

        let start = self.current_pos();

        let ch = self.advance().unwrap();

        let kind = match ch {
            // Single character operators
            '+' => {
                if self.match_char('=') {
                    TokenKind::PlusAssign
                } else if self.match_char('+') {
                    TokenKind::Inc
                } else {
                    TokenKind::Plus
                }
            }
            '-' => {
                if self.match_char('=') {
                    TokenKind::MinusAssign
                } else if self.match_char('>') {
                    TokenKind::Arrow
                } else if self.match_char('-') {
                    TokenKind::Dec
                } else {
                    TokenKind::Minus
                }
            }
            '*' => {
                if self.match_char('*') {
                    if self.match_char('=') {
                        TokenKind::PowAssign
                    } else {
                        TokenKind::Pow
                    }
                } else if self.match_char('=') {
                    TokenKind::StarAssign
                } else {
                    TokenKind::Star
                }
            }
            '/' => {
                if self.match_char('/') {
                    self.skip_line_comment();
                    // After skipping a line comment, if there's a tab, it's just whitespace
                    // (not a line break) - skip it and continue to the next token
                    // The comment line's indentation is already handled, so y will be
                    // at the same indent level as the comment
                    if self.peek() == Some('\t') {
                        self.advance(); // Skip the tab (it's just whitespace)
                    }
                    // Continue to next token (recursive call is safe - comments are not deeply nested)
                    return self.next_token();
                } else if self.match_char('*') {
                    self.skip_block_comment();
                    // Continue to next token (recursive call is safe - block comments handle nesting)
                    return self.next_token();
                } else if self.match_char('=') {
                    TokenKind::SlashAssign
                } else {
                    TokenKind::Slash
                }
            }
            '%' => {
                if self.match_char('=') {
                    TokenKind::PercentAssign
                } else {
                    TokenKind::Percent
                }
            }
            '=' => {
                if self.match_char('=') {
                    TokenKind::Eq
                } else {
                    TokenKind::Assign
                }
            }
            '!' => {
                if self.match_char('=') {
                    TokenKind::Ne
                } else {
                    TokenKind::Not
                }
            }
            '<' => {
                if self.match_char('=') {
                    TokenKind::Le
                } else if self.match_char('<') {
                    TokenKind::Shl
                } else {
                    TokenKind::Lt
                }
            }
            '>' => {
                if self.match_char('=') {
                    TokenKind::Ge
                } else if self.match_char('>') {
                    TokenKind::Shr
                } else {
                    TokenKind::Gt
                }
            }
            '&' => {
                if self.match_char('&') {
                    TokenKind::And
                } else {
                    TokenKind::BitAnd
                }
            }
            '|' => {
                if self.match_char('|') {
                    TokenKind::Or
                } else {
                    TokenKind::BitOr
                }
            }
            '^' => TokenKind::BitXor,
            '~' => TokenKind::BitNot,
            '?' => TokenKind::Question,
            ':' => {
                if self.match_char('=') {
                    TokenKind::InitAssign
                } else {
                    TokenKind::Colon
                }
            }

            // Punctuation
            '(' => TokenKind::LeftParen,
            ')' => TokenKind::RightParen,
            '[' => TokenKind::LeftBracket,
            ']' => TokenKind::RightBracket,
            '{' => TokenKind::LeftBrace,
            '}' => TokenKind::RightBrace,
            ',' => TokenKind::Comma,
            ';' => TokenKind::Semicolon,
            '.' => {
                // Check if this is the start of a number (e.g., .5)
                if let Some(next_ch) = self.peek() {
                    if next_ch.is_ascii_digit() {
                        // This is a number starting with a decimal point
                        self.pos -= 1; // Back up to include the dot
                        self.column -= 1;
                        return self.lex_number();
                    }
                }
                TokenKind::Dot
            }

            // Literals
            '"' => return self.lex_string(),
            '\'' => return self.lex_char(),

            // Numbers
            '0'..='9' => {
                self.pos -= 1; // Back up to include the digit
                self.column -= 1;
                return self.lex_number();
            }

            // Identifiers and keywords
            'a'..='z' | 'A'..='Z' | '_' => {
                self.pos -= 1; // Back up to include the first char
                self.column -= 1;
                return self.lex_identifier();
            }

            // Whitespace (should be handled above, but just in case)
            ' ' => return self.next_token(), // Skip spaces
            // Tabs should be handled by the main loop, but if we see one here
            // (e.g., after a comment), skip it and continue
            '\t' => return self.next_token(), // Skip tab and continue

            _ => {
                self.errors.push(format!(
                    "unexpected character '{}' at line {} column {}",
                    ch, self.line, self.column
                ));
                return self.next_token(); // Skip and continue
            }
        };

        Token::new(kind, self.span_from(start))
    }

    fn lex_string(&mut self) -> Token {
        let start = self.current_pos();
        let mut current_text = String::new();
        let mut text_start = start;

        loop {
            if self.is_at_end() {
                self.errors.push(format!(
                    "unterminated string starting at line {} column {}",
                    start.line, start.column
                ));
                break;
            }

            match self.peek() {
                Some('"') => {
                    self.advance(); // Consume closing quote
                    // Emit final text part (even if empty) - but only if we have queued tokens
                    // (meaning there was an interpolation, so we need to maintain the sequence)
                    if !self.token_queue.is_empty() {
                        let span = Span::new(
                            self.file_id,
                            text_start,
                            Position::new(self.line, self.column - 1),
                        );
                        // Queue the final text part (even if empty)
                        self.token_queue.push_back(Token::new(TokenKind::StrPart(current_text), span));
                    } else if !current_text.is_empty() {
                        // No interpolation, just return the text part
                        let span = Span::new(
                            self.file_id,
                            text_start,
                            Position::new(self.line, self.column - 1),
                        );
                        return Token::new(TokenKind::StrPart(current_text), span);
                    } else {
                        // Empty string with no interpolation
                        return Token::new(
                            TokenKind::StrPart(String::new()),
                            Span::new(self.file_id, start, Position::new(self.line, self.column - 1)),
                        );
                    }
                    // String ended with interpolation - return first queued token
                    if let Some(first_token) = self.token_queue.pop_front() {
                        return first_token;
                    }
                    return Token::new(
                        TokenKind::StrPart(String::new()),
                        Span::new(self.file_id, start, Position::new(self.line, self.column - 1)),
                    );
                }
                Some('\\') => {
                    // Escape sequence
                    self.advance(); // Skip backslash
                    if let Some(escaped) = self.lex_escape_sequence() {
                        current_text.push(escaped);
                    }
                }
                Some('&') => {
                    // Check for interpolation or escaped &
                    if self.peek_next() == Some('&') {
                        // Escaped &
                        self.advance(); // Skip first &
                        self.advance(); // Skip second &
                        current_text.push('&');
                    } else {
                        // Interpolation - emit current text part (even if empty)
                        let text_end = self.current_pos();
                        let span = Span::new(self.file_id, text_start, text_end);
                        // Move current_text instead of cloning (we clear it anyway)
                        let text_token = Token::new(TokenKind::StrPart(current_text), span);
                        // Queue the text token
                        self.token_queue.push_back(text_token);
                        current_text = String::new(); // Reset for next part

                        // Lex interpolation
                        let interp_start = self.current_pos();
                        self.advance(); // Skip &
                        // Check if next character is valid for interpolation
                        let is_valid_interp_start = self.peek().map_or(false, |c| {
                            c.is_ascii_alphanumeric() || c == '_' || c == '.' || c == '(' || c == ')'
                        });
                        if is_valid_interp_start {
                            let ident = self.lex_interpolation_ident();
                            let interp_end = self.current_pos();
                            let interp_span = Span::new(self.file_id, interp_start, interp_end);
                            
                            // Check for path (contains '.' or '(') only once
                            let has_dot = ident.contains('.');
                            let interp_kind = if has_dot || ident.contains('(') {
                                TokenKind::InterpPath(ident)
                            } else {
                                TokenKind::InterpIdent(ident)
                            };
                            
                            // Queue interpolation token
                            self.token_queue.push_back(Token::new(interp_kind, interp_span));
                            
                            // Update text_start for next text part
                            text_start = self.current_pos();
                        } else {
                            self.errors.push(format!(
                                "invalid interpolation at line {} column {}",
                                self.line, self.column
                            ));
                            // Continue as if it was just a regular character
                            current_text.push('&');
                        }
                    }
                }
                Some(ch) => {
                    current_text.push(ch);
                    self.advance();
                }
                None => break,
            }
        }

        // Unterminated string - return what we have
        Token::new(TokenKind::StrPart(current_text), self.span_from(start))
    }

    fn lex_interpolation_ident(&mut self) -> String {
        let mut ident = String::new();
        while let Some(ch) = self.peek() {
            if ch.is_ascii_alphanumeric() || ch == '_' || ch == '.' || ch == '(' || ch == ')' {
                ident.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        ident
    }

    fn lex_char(&mut self) -> Token {
        let start = self.current_pos();
        let ch = if let Some(c) = self.advance() {
            if c == '\\' {
                self.lex_escape_sequence().unwrap_or('\0')
            } else {
                c
            }
        } else {
            self.errors.push(format!(
                "unterminated character literal at line {} column {}",
                self.line, self.column
            ));
            return Token::new(TokenKind::Character('\0'), self.span_from(start));
        };

        if self.peek() != Some('\'') {
            self.errors.push(format!(
                "character literal must be single character at line {} column {}",
                self.line, self.column
            ));
        } else {
            self.advance(); // Consume closing quote
        }

        Token::new(TokenKind::Character(ch), self.span_from(start))
    }

    fn lex_escape_sequence(&mut self) -> Option<char> {
        match self.advance()? {
            'n' => Some('\n'),
            't' => Some('\t'),
            'r' => Some('\r'),
            '\\' => Some('\\'),
            '\'' => Some('\''),
            '"' => Some('"'),
            '0' => Some('\0'),
            'u' => {
                // Unicode escape \u{...}
                if self.peek() == Some('{') {
                    self.advance(); // Skip {
                    let mut code = String::new();
                    while let Some(ch) = self.peek() {
                        if ch == '}' {
                            self.advance(); // Skip }
                            break;
                        } else if ch.is_ascii_hexdigit() {
                            code.push(ch);
                            self.advance();
                        } else {
                            break;
                        }
                    }
                    if let Ok(code_point) = u32::from_str_radix(&code, 16) {
                        char::from_u32(code_point)
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn lex_number(&mut self) -> Token {
        let start = self.current_pos();
        let mut num_str = String::new();
        // Note: Negative numbers are handled as separate tokens (Minus, Number)
        // So we never see '-' here - it's already consumed as an operator

        // Check if we're starting with a decimal point (e.g., .5)
        let starts_with_dot = self.peek() == Some('.');
        if starts_with_dot {
            num_str.push('0'); // Add leading zero for numbers like .5
            num_str.push('.');
            self.advance();
        } else {
            // Integer part
            while let Some(ch) = self.peek() {
                if ch.is_ascii_digit() {
                    num_str.push(ch);
                    self.advance();
                } else {
                    break;
                }
            }
        }

        // Check for decimal point (if we haven't already seen it)
        let mut has_decimal = starts_with_dot;
        if !starts_with_dot && self.peek() == Some('.') {
            num_str.push('.');
            self.advance();
            has_decimal = true;
        }

        // Fractional part (required if we have a decimal point)
        if has_decimal {
            while let Some(ch) = self.peek() {
                if ch.is_ascii_digit() {
                    num_str.push(ch);
                    self.advance();
                } else {
                    break;
                }
            }
            // Parse as double
            if let Ok(value) = num_str.parse::<f64>() {
                Token::new(TokenKind::Double(value), self.span_from(start))
            } else {
                self.errors.push(format!(
                    "invalid double literal at line {} column {}",
                    self.line, self.column
                ));
                Token::new(TokenKind::Double(0.0), self.span_from(start))
            }
        } else {
            // Parse as integer
            if let Ok(value) = num_str.parse::<i64>() {
                Token::new(TokenKind::Integer(value), self.span_from(start))
            } else {
                self.errors.push(format!(
                    "invalid integer literal at line {} column {}",
                    self.line, self.column
                ));
                Token::new(TokenKind::Integer(0), self.span_from(start))
            }
        }
    }

    fn lex_identifier(&mut self) -> Token {
        let start = self.current_pos();
        let mut ident = String::new();

        while let Some(ch) = self.peek() {
            if ch.is_ascii_alphanumeric() || ch == '_' {
                ident.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        let kind = if TokenKind::is_keyword(&ident) {
            TokenKind::from_keyword(&ident).unwrap()
        } else {
            TokenKind::Identifier(ident)
        };

        Token::new(kind, self.span_from(start))
    }

    fn skip_line_comment(&mut self) {
        while let Some(ch) = self.peek() {
            if ch == '\n' || ch == '\r' {
                // Consume the newline - the main loop will handle it
                if ch == '\r' && self.peek_next() == Some('\n') {
                    self.advance(); // Skip \r
                }
                self.advance(); // Skip \n
                break;
            }
            // Tabs also indicate line breaks in Brief
            if ch == '\t' {
                break; // Don't consume the tab - let the main loop handle it
            }
            self.advance();
        }
    }

    fn skip_block_comment(&mut self) {
        let mut depth = 1; // Start at depth 1 since we already saw the opening /*
        
        while !self.is_at_end() && depth > 0 {
            let ch = self.peek();
            let next_ch = self.peek_next();
            
            if ch == Some('/') && next_ch == Some('*') {
                // Nested comment start
                depth += 1;
                self.advance(); // Skip /
                self.advance(); // Skip *
            } else if ch == Some('*') && next_ch == Some('/') {
                // Comment end
                depth -= 1;
                self.advance(); // Skip *
                self.advance(); // Skip /
            } else {
                self.advance();
            }
        }
    }

    // Helper methods
    fn is_at_end(&self) -> bool {
        self.pos >= self.source.len()
    }

    fn peek(&self) -> Option<char> {
        if self.pos < self.source.len() {
            Some(self.source[self.pos])
        } else {
            None
        }
    }

    fn peek_next(&self) -> Option<char> {
        if self.pos + 1 < self.source.len() {
            Some(self.source[self.pos + 1])
        } else {
            None
        }
    }

    fn advance(&mut self) -> Option<char> {
        if self.pos < self.source.len() {
            let ch = self.source[self.pos];
            self.pos += 1;
            if ch == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
            Some(ch)
        } else {
            None
        }
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.peek() == Some(expected) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn current_pos(&self) -> Position {
        Position::new(self.line, self.column)
    }

    fn current_span(&self) -> Span {
        Span::single(self.file_id, self.current_pos())
    }

    fn span_from(&self, start: Position) -> Span {
        Span::new(self.file_id, start, self.current_pos())
    }
}

