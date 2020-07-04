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

fn is_grouping_delimiter(c: char) -> bool {
    c == '{' || c == '}' || c == '[' || c == ']' || c == '(' || c == ')'
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

    pub fn next_token(&mut self) -> Token {
        #[allow(unreachable_patterns)]
        match self.current_mode() {
            LexerMode::Normal => self.tokenize_normal(),
            LexerMode::Grouping => self.tokenize_grouping(),
            _ => unimplemented!(),
        }
    }

    pub(crate) fn push_mode(&mut self, mode: LexerMode) {
        self.mode_stack.push(mode);
    }

    pub(crate) fn pop_mode(&mut self) -> Option<LexerMode> {
        self.mode_stack.pop()
    }
}

impl Lexer {
    fn current_mode(&self) -> LexerMode {
        self.mode_stack.last().cloned().unwrap_or_default()
    }

    fn tokenize_normal(&mut self) -> Token {
        let old_pos = self.current_pos();

        if self.did_enter_new_line {
            return Token::with(
                self.indentation(),
                Span::new(old_pos, self.current_pos())
            );
        }

        let next_char = match self.next_char() {
            Some(c) => c,
            None => {
                if !self.indentation_stack.is_empty() {
                    self.indentation_stack.pop();
                    return Token::with(TokenKind::End, Span::new(old_pos, self.current_pos()));
                } else {
                    return Token::with(TokenKind::Eof, Span::new(old_pos, self.current_pos()))
                }
            }
        };

        if is_whitespace(next_char) {
            return self.next_token();
        }

        if next_char == '\n' {
            self.did_enter_new_line = true;
            return self.next_token();
        }

        let kind = match next_char {
            '/' => {
                if self.peek() == '/' {
                    self.line_comment()
                } else {
                    self.symbol(next_char)
                }
            },
            c if is_grouping_delimiter(c) => self.grouping(c),
            c if is_symbol(c) => self.symbol(c),
            c if is_identifier_start(c) => self.identifier(c),
            c @ '0'..='9' => self.number(c),
            c => TokenKind::Unknown(c)
        };

        Token::with(kind, Span::new(old_pos, self.current_pos()))
    }

    fn tokenize_grouping(&mut self) -> Token {
        // todo!("Lexer::tokenize_grouping")
        self.tokenize_normal()
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
    pub(crate) fn is_at_end(&self) -> bool {
        self.cursor.source_len() == 0
    }

    pub(crate) fn current_pos(&self) -> Position {
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

    /// Matches every character up until the new line character. The characters
    /// consumed will be part of the `TokenKind::LineComment` variant.
    fn line_comment(&mut self) -> TokenKind {
        self.next_char();

        let next_char = self.peek();
        let mut is_doc_comment = false;
        if next_char == '!' || next_char == '/' {
            self.next_char();
            is_doc_comment = true;
        }

        // Remove the first whitespace character
        if is_whitespace(self.peek()) {
            self.next_char();
        }

        let content =
            if /* self.should_consume_doc_comments && */ is_doc_comment {
                Some(self.consume_build(|c| c != '\n').into_iter().collect())
            } else {
                self.consume_while(|c| c != '\n');
                None
            };

        TokenKind::LineComment { is_doc_comment, content }
    }

    fn grouping(&mut self, delimiter: char) -> TokenKind {
        match delimiter {
            '{' => TokenKind::GroupingStart(GroupingDelimiter::Bracket),
            '[' => TokenKind::GroupingStart(GroupingDelimiter::Bracket),
            '(' => TokenKind::GroupingStart(GroupingDelimiter::Paren),
            '}' => TokenKind::GroupingEnd(GroupingDelimiter::Bracket),
            ']' => TokenKind::GroupingEnd(GroupingDelimiter::Bracket),
            ')' => TokenKind::GroupingEnd(GroupingDelimiter::Paren),
            _ => panic!("Invalid grouping delimiter: {:?}", delimiter)
        }
    }

    /// Matches any character that is a valid symbol.
    ///
    /// _TODO:_ Perhaps we should handle cases with confused symbols, such as
    /// U+037E, the Greek question mark, which looks like a semicolon (compare
    /// ';' with ';').
    fn symbol(&mut self, symbol: char) -> TokenKind {
        match symbol {
            '?' => {
                if (self.peek(), self.peek_at(1)) == ('?', '?') {
                    // Consume the next two question marks
                    self.next_char();
                    self.next_char();
                    TokenKind::Keyword(Keyword::Unimplemented)
                } else {
                    TokenKind::Symbol(Symbol::Question)
                }
            },
            _ => {
                match Symbol::compose(symbol, self.peek()) {
                    Some(symbol) => {
                        self.next_char();
                        TokenKind::Symbol(symbol)
                    },
                    None => TokenKind::Symbol(Symbol::from_char(symbol))
                }
            }
        }
    }

    /// Matches every character that can be part of an identifier. This includes
    /// upper and lower-case letters, the underscore, and the hyphen.
    fn identifier(&mut self, first_char: char) -> TokenKind {
        let rest = self.consume_build(is_identifier_continue);
        let vec = [&vec![first_char], &rest[..]].concat();
        let string: String = vec.into_iter().collect();
        self.keyword_or_identifier(string)
    }

    /// Attempts to match the given string to a keyword, returning a
    /// `TokenKind::Keyword` if a match is found, otherwise a
    /// `TokenKind::Identifier`.
    fn keyword_or_identifier(&mut self, string: String) -> TokenKind {
        match &*string {
            "and"   => TokenKind::Keyword(Keyword::And),
            "def"   => TokenKind::Keyword(Keyword::Def),
            "do"    => TokenKind::Keyword(Keyword::Do),
            "else"  => TokenKind::Keyword(Keyword::Else),
            "false" => TokenKind::Keyword(Keyword::False),
            "if"    => TokenKind::Keyword(Keyword::If),
            "let"   => TokenKind::Keyword(Keyword::Let),
            "match" => TokenKind::Keyword(Keyword::Match),
            "module"=> TokenKind::Keyword(Keyword::Module),
            "not"   => TokenKind::Keyword(Keyword::Not),
            "or"    => TokenKind::Keyword(Keyword::Or),
            "public"=> TokenKind::Keyword(Keyword::Public),
            "then"  => TokenKind::Keyword(Keyword::Then),
            "true"  => TokenKind::Keyword(Keyword::True),
            "type"  => TokenKind::Keyword(Keyword::Type),
            "using" => TokenKind::Keyword(Keyword::Using),
            "val"   => TokenKind::Keyword(Keyword::Val),
            "with"  => TokenKind::Keyword(Keyword::With),
            _       => TokenKind::Identifier(string)
        }
    }

    /// Matches any valid sequence of digits that can form an integer or float
    /// literal. Both literal forms support the binary, octal, and hexadecimal
    /// bases in addition to the default decimal system.
    fn number(&mut self, first_digit: char) -> TokenKind {
        let mut base = Base::Decimal;
        let mut radix = 10;

        let integer_part = {
            if first_digit == '0' {
                match self.peek() {
                    // Binary literal.
                    'b' => {
                        base = Base::Binary;
                        radix = 2;
                        self.next_char();
                        self.consume_digits(Base::Binary, None)
                    },
                    // Octal literal.
                    'o' => {
                        base = Base::Octal;
                        radix = 8;
                        self.next_char();
                        self.consume_digits(Base::Octal, None)
                    },
                    // Hexadecimal literal.
                    'x' => {
                        base = Base::Hexadecimal;
                        radix = 16;
                        self.next_char();
                        self.consume_digits(Base::Hexadecimal, None)
                    },
                    // Decimal literal. We ignore the decimal point to avoid it
                    // from being pushed into the `integer_part` vector (it'll
                    // be the first element of the `fractional_part` vector
                    // later on instead).
                    '0'..='9' | '_' => {
                        self.consume_digits(Base::Decimal, None)
                    }
                    // Just 0.
                    _ => vec!['0']
                }
            } else {
                self.consume_digits(Base::Decimal, Some(first_digit))
            }
        };

        let mut fractional_part: Vec<char> = Vec::new();

        if self.peek() == '.' && self.peek_at(1) != '.' {
            fractional_part.push(self.next_char().unwrap());
            match self.peek() {
                '0'..='9' | '_' => {
                    let mut rest =
                        self.consume_digits(Base::Decimal, None);
                    fractional_part.append(&mut rest);
                },
                _ => fractional_part.push('0')
            }
        }

        if fractional_part.is_empty() {
            let string: String = integer_part[..].into_iter().collect();
            match i32::from_str_radix(&*string, radix) {
                Ok(value) => TokenKind::Literal(Literal::Int { base, value }),
                _ => TokenKind::Error(LexerError::OverflowedIntLiteral)
            }
        } else {
            let all = [&integer_part[..], &fractional_part[..]].concat();
            let string: String = all[..].into_iter().collect();
            match string.parse() {
                Ok(value) => TokenKind::Literal(Literal::Float { base, value }),
                _ => TokenKind::Error(LexerError::OverflowedFloatLiteral)
            }
        }
    }
}
