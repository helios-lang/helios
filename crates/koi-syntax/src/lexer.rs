use crate::errors::LexerError;
use crate::source::{Cursor, Position, Source, Span};
use crate::token::*;
use std::default::Default;
use unicode_xid::UnicodeXID;

/// Checks if the given character is a valid start of an identifier. A valid
/// start of an identifier is any Unicode code point that satisfies the
/// `XID_Start` property.
fn is_identifier_start(c: char) -> bool {
    c.is_xid_start() || c == '_'
}

/// Checks if the given character is a valid continuation of an identifier.
/// A valid continuation of an identifier is any Unicode code point that
/// satisfies the `XID_Continue` property.
fn is_identifier_continue(c: char) -> bool {
    c.is_xid_continue()
}

/// Checks if the given character is a whitespace character. This includes the
/// space character, the tab character, and the carriage return character. Only
/// the newline character is used to signify a new line.
fn is_whitespace(c: char) -> bool {
    c == ' ' || c == '\r' || c == '\t'
}

/// Checks if the given character is a recognised symbol.
fn is_symbol(c: char) -> bool {
    match c {
        '&' | '*' | '@' | '!' | '^' | ':' | ',' | '$' | '.' | '–' | '—' | '=' |
        '-' | '%' | '+' | '#' | '?' | ';' | '£' | '~' | '|' | '/' | '\\'| '<' |
        '>' | '{' | '}' | '[' | ']' | '(' | ')' => true,
        _ => false
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum LexerMode {
    Normal,
    Grouping,
}

impl Default for LexerMode {
    fn default() -> Self {
        Self::Normal
    }
}

pub struct Lexer {
    cursor: Cursor,
    did_enter_new_line: bool,
    current_indentation: usize,
    should_emit_end_token: bool,
    indentation_stack: Vec<usize>,
    mode_stack: Vec<LexerMode>,
}

impl Lexer {
    pub fn with(source: Source) -> Self {
        Self {
            cursor: Cursor::with(source),
            did_enter_new_line: false,
            current_indentation: 0,
            should_emit_end_token: false,
            indentation_stack: vec![],
            mode_stack: vec![LexerMode::Normal],
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        #[allow(unreachable_patterns)]
        match self.current_mode() {
            LexerMode::Normal => self.tokenize_normal(),
            LexerMode::Grouping => self.tokenize_grouping(),
            _ => unimplemented!(),
        }
    }

    pub fn push_mode(&mut self, mode: LexerMode) {
        self.mode_stack.push(mode);
    }

    pub fn pop_mode(&mut self) -> Option<LexerMode> {
        self.mode_stack.pop()
    }
}

impl Lexer {
    fn current_mode(&self) -> LexerMode {
        self.mode_stack.last().cloned().unwrap_or_default()
    }

    fn tokenize_normal(&mut self) -> Option<Token> {
        let old_pos = self.current_position();

        if self.did_enter_new_line {
            return Some(
                Token::with(
                    self.indentation(),
                    Span::new(old_pos, self.current_position())
                )
            );
        }

        let next_char = self.next_char()?;

        if is_whitespace(next_char) {
            return self.next_token();
        }

        if next_char == '\n' {
            self.did_enter_new_line = true;
            return self.next_token();
        }

        todo!("Lexer::tokenize_normal")
    }

    fn tokenize_grouping(&mut self) -> Option<Token> {
        todo!("Lexer::tokenize_grouping")
    }
}

impl Lexer {
    /// Moves to the next character in the iterator.
    fn next_char(&mut self) -> Option<char> {
        match self.cursor.advance() {
            Some(c) => Some(c),
            None => None
        }
    }

    /// Peeks the next character without consuming it.
    fn peek(&self) -> char {
        self.peek_at(0)
    }

    /// Peeks the character at the given index without consuming it.
    fn peek_at(&self, n: usize) -> char {
        self.cursor.nth(n)
    }

    /// Checks if the `Cursor` has reached the end of the input.
    fn is_at_end(&self) -> bool {
        self.cursor.source_len() == 0
    }

    fn current_position(&self) -> Position {
        self.cursor.pos
    }

    /// Consumes the input while the given `predicate` holds true. Returns the
    /// total consumed count traversed.
    fn consume_while<F>(&mut self, predicate: F) -> usize
    where F: Fn(char) -> bool
    {
        let mut consumed = 0;
        while predicate(self.peek()) && !self.is_at_end() {
            self.next_char();
            consumed += 1;
        }
        consumed
    }

    /// Consumes the input while the given `predicate` holds true, building a
    /// `Vec<char>` for all the characters eaten.
    fn consume_build<F>(&mut self, predicate: F) -> Vec<char>
    where F: Fn(char) -> bool
    {
        let mut vec = Vec::new();
        while predicate(self.peek()) && !self.is_at_end() {
            if let Some(c) = self.next_char() {
                vec.push(c);
            }
        }
        vec
    }

    /// Consumes the `input` for all the valid digits of the given `base` up
    /// until a non-digit character is reached, building a `Vec<char>` for all
    /// the digit characters eaten. Underscores (`_`) are also consumed, being
    /// ignored when found.
    fn consume_digits(&mut self, base: Base, first_digit: Option<char>) -> Vec<char> {
        let mut vec = Vec::new();
        if let Some(d) = first_digit { vec.push(d); }

        /// Matches the digits with the pattern(s) provided, including the
        /// underscore separator (which is ignored). Any other character will
        /// break the match expression.
        ///
        /// # Example
        ///
        /// ```
        /// // This will match all the digits from `0` through `9` (inclusive)
        /// // and the decimal point `.` (i.e., this will match all the digits
        /// // of a decimal number).
        /// match_digits!('0' => '9', '.');
        /// ```
        macro_rules! match_digits {
            ( $($start:expr $(=> $end:expr)*),+ ) => {
                loop {
                    match self.peek() {
                        '_' => {
                            self.next_char();
                        },
                        $( | $start $(..=$end)* )+ => {
                            vec.push(
                                self.next_char()
                                    .expect("Failed to get next char")
                            );
                        },
                        _ => break
                    }
                }
            }
        };

        match base {
            Base::Binary => {
                match_digits!('0' => '1');
            },
            Base::Octal => {
                match_digits!('0' => '7');
            },
            Base::Hexadecimal => {
                match_digits!('0' => '9', 'a' => 'f', 'A' => 'F');
            },
            Base::Decimal => {
                match_digits!('0' => '9');
            },
        }

        vec
    }
}

impl Lexer {
    /// Consumes indentation and returns the appropriate indentation token kind.
    ///
    /// The following outlines which token kind is determined:
    /// * If we've increased our indentation, we'll return `TokenKind::Begin`.
    /// * If we did not change the level of indentation, we'll return
    ///   `TokenKind::Newline`.
    /// * Otherwise, we've decreased our indentation level and so we'll emit as
    ///   many `TokenKind::End` tokens as required to go back to the new
    ///   indentation level.
    fn indentation(&mut self) -> TokenKind {
        if self.should_emit_end_token {
            let last_block = *self.indentation_stack.last().unwrap_or(&0);

            // We have encountered bad indentation (it is larger than expected)
            if self.current_indentation > last_block {
                self.did_enter_new_line = false;
                self.should_emit_end_token = false;
                TokenKind::Error(
                    LexerError::BadIndent {
                        expected: last_block,
                        found: self.current_indentation
                    }
                )

            // We still have to decrease our indentation
            } else if self.current_indentation < last_block {
                self.indentation_stack.pop();
                TokenKind::End

            // We have decreased by a sufficient amount
            } else {
                self.did_enter_new_line = false;
                self.should_emit_end_token = false;
                TokenKind::Newline
            }
        } else {
            self.current_indentation = self.consume_while(is_whitespace);
            let last_block = *self.indentation_stack.last().unwrap_or(&0);

            // Skip line if it's empty
            if self.peek() == '\n' {
                self.next_char();
                self.did_enter_new_line = true;
                return self.indentation();
            }

            // We have increased our indentation
            if self.current_indentation > last_block {
                self.indentation_stack.push(self.current_indentation);
                self.did_enter_new_line = false;
                TokenKind::Begin

            // We have decreased our indentation
            } else if self.current_indentation < last_block {
                self.should_emit_end_token = true;
                self.indentation()

            // We have the same level of indentation
            } else {
                self.did_enter_new_line = false;
                TokenKind::Newline
            }
        }
    }
}
