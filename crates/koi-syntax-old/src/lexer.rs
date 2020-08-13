use crate::errors::LexerError;
use crate::source::{Cursor, Position, Source, Span};
use crate::token::*;
use std::default::Default;
use unicode_xid::UnicodeXID;

/// Checks if the given character is a valid start of an identifier. A valid
/// start of an identifier is any Unicode code point that satisfies the
/// `XID_Start` property.
fn is_identifier_start(c: char) -> bool {
    // Fast-path for ASCII identifiers
    ('a' <= c && c <= 'z')
        || ('A' <= c && c <= 'Z')
        || c == '_'
        || c.is_xid_start()
}

/// Checks if the given character is a valid continuation of an identifier.
/// A valid continuation of an identifier is any Unicode code point that
/// satisfies the `XID_Continue` property.
fn is_identifier_continue(c: char) -> bool {
    // Fast-path for ASCII identifiers
    ('a' <= c && c <= 'z')
        || ('A' <= c && c <= 'Z')
        || ('0' <= c && c <= '9')
        || c == '_'
        || c.is_xid_continue()
}

/// Checks if the given character is a whitespace character. This includes the
/// space character, the carriage return character, and the tab character. Only
/// the newline character is used to signify a new line.
fn is_whitespace(c: char) -> bool {
    c == ' ' || c == '\r' || c == '\t'
}

/// Checks if the given character is a grouping delimiter.
fn is_grouping_delimiter(c: char) -> bool {
    match c {
        '{' | '}' | '[' | ']' | '(' | ')' => true,
        _ => false,
    }
}

/// Checks if the given character is a recognised symbol.
fn is_symbol(c: char) -> bool {
    match c {
        '&' | '*' | '@' | '!' | '^' | ':' | ',' | '$' | '.' | '–' | '—' | '=' |
        '-' | '%' | '+' | '#' | '?' | ';' | '£' | '~' | '|' | '/' | '\\'| '<' |
        '>' | '{' | '}' | '[' | ']' | '(' | ')' => true,
        _ => false,
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum LexerMode {
    Normal,
    Grouping,
    IndentedBlock,
}

impl Default for LexerMode {
    fn default() -> Self {
        Self::Normal
    }
}

pub struct Lexer {
    cursor: Cursor,
    current_indentation: usize,
    indentation_stack: Vec<usize>,
    mode_stack: Vec<LexerMode>,
    did_emit_end_token: bool,
}

impl Lexer {
    pub fn with(source: Source) -> Self {
        Self {
            cursor: Cursor::with(source),
            current_indentation: 0,
            indentation_stack: Vec::new(),
            mode_stack: vec![LexerMode::Normal],
            did_emit_end_token: false,
        }
    }

    pub fn next_token(&mut self) -> Token {
        #[allow(unreachable_patterns)]
        match self.current_mode() {
            LexerMode::Normal => self.tokenize_normal(),
            LexerMode::Grouping => self.tokenize_grouping(),
            LexerMode::IndentedBlock => self.tokenize_indented_block(),
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

        let next_char = match self.next_char() {
            Some(next_char) => {
                if next_char == '\n' {
                    return self.tokenize_indented_block();
                } else if is_whitespace(next_char) {
                    return self.tokenize_normal();
                } else {
                    next_char
                }
            },
            None => {
                let kind =
                    if self.indentation_stack.len() > 1 {
                        TokenKind::End
                    } else {
                        self.did_emit_end_token = true;
                        TokenKind::Eof
                    };

                self.indentation_stack.pop();
                return Token::with(kind, Span::new(old_pos, self.current_pos()));
            }
        };

        let kind = match next_char {
            '/' if self.peek() == '/' => self.lex_symbol(next_char),
            c if is_grouping_delimiter(c) => self.lex_grouping(c),
            c if is_symbol(c) => self.lex_symbol(c),
            c if is_identifier_start(c) => self.lex_identifier(c),
            c @ '0'..='9' => self.lex_number(c),
            c => TokenKind::Unknown(c),
        };

        let trailing_trivia = self.lex_trivia();

        Token::with_trivia(
            kind,
            Span::new(old_pos, self.current_pos()),
            Vec::new(),
            trailing_trivia
        )
    }

    fn tokenize_grouping(&mut self) -> Token {
        let old_pos = self.current_pos();

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

        // Skip whitespace
        if is_whitespace(next_char) || next_char == '\n' {
            return self.next_token();
        }

        let kind = match next_char {
            '/' if self.peek() != '/' => self.lex_symbol(next_char),
            c if is_grouping_delimiter(c) => self.lex_grouping(c),
            c if is_symbol(c) => self.lex_symbol(c),
            c if is_identifier_start(c) => self.lex_identifier(c),
            c @ '0'..='9' => self.lex_number(c),
            c => TokenKind::Unknown(c),
        };

        Token::with(kind, Span::new(old_pos, self.current_pos()))
    }

    /// _FIXME_: Return a `LexerError::BadIndent` if the new indentation is
    /// not equal to the expected indentation.
    fn tokenize_indented_block(&mut self) -> Token {
        // Consume any trailing whitespace
        self.consume_while(is_whitespace);

        // Consume indentation at the start of the next line
        if self.consume('\n') { self.consume_while(is_whitespace); }

        self.current_indentation = self.current_pos().column;
        let previous_indentation = *self.indentation_stack.last().unwrap_or(&0);

        let kind =
            if self.current_indentation > previous_indentation {
                self.indentation_stack.push(self.current_indentation);
                TokenKind::Begin
            } else if self.current_indentation < previous_indentation {
                self.indentation_stack.pop();
                TokenKind::End
            } else {
                TokenKind::Newline
            };

        Token::with(kind, Span::zero_width(self.current_pos()))
    }
}

impl Lexer {
    /// Retrieves the next character in the iterator.
    fn next_char(&mut self) -> Option<char> {
        self.cursor.advance()
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
        self.cursor.source_len() == 0 && self.did_emit_end_token
    }

    pub(crate) fn current_pos(&self) -> Position {
        self.cursor.pos
    }

    /// Attempts to consume the next character if it matches the provided
    /// character `c`. Returns a `bool` indicating if it was successful or not.
    fn consume(&mut self, c: char) -> bool {
        if self.peek() == c {
            self.next_char();
            true
        } else {
            false
        }
    }

    /// Consumes the input while the given `predicate` holds true. Returns the
    /// count of characters traversed.
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
    /// `Vec<char>` for all the characters consumed.
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

    /// Consumes all the valid digits of the given `base` up until a non-digit
    /// character is reached, building a `Vec<char>` for all the characters
    /// consumed. Underscores (`_`) are also consumed, but are ignored when
    /// encountered.
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
            Base::Binary => match_digits!('0' => '1'),
            Base::Octal => match_digits!('0' => '7'),
            Base::Hexadecimal => match_digits!('0' => '9', 'a' => 'f', 'A' => 'F'),
            Base::Decimal => match_digits!('0' => '9'),
        }

        vec
    }
}

impl Lexer {
    /// Builds a collection of `Trivia`.
    ///
    /// Trivia are pieces of syntax that are not essential to the semantics of
    /// the program, such as whitespace and line comments. This information is
    /// tacked on to most tokens, establishing any trivia that appears before
    /// or after it.
    fn lex_trivia(&mut self) -> Vec<Trivia> {
        let mut trivia = Vec::new();

        while is_whitespace(self.peek()) || (self.consume('/') && self.consume('/')) {
            let count = self.consume_while(|c| c == ' ');
            if count > 0 { trivia.push(Trivia::Spaces(count)) }

            let count = self.consume_while(|c| c == '\t');
            if count > 0 { trivia.push(Trivia::Tabs(count)) }

            let count = self.consume_while(|c| c == '\r');
            if count > 0 { trivia.push(Trivia::CarriageReturn(count)) }

            let start_pos = self.current_pos();
            if self.consume('/') && self.consume('/') {
                let is_doc_comment = self.consume('/') || self.consume('!');
                self.consume_while(|c| c != '\n');

                trivia.push(Trivia::LineComment {
                    is_doc_comment,
                    span: Span::new(start_pos, self.current_pos())
                })
            }
        }

        trivia
    }

    /// Matches any character that is a valid symbol.
    ///
    /// _TODO:_ Perhaps we could handle cases with confused symbols, such as
    /// U+037E, the Greek question mark, which looks like a semicolon (compare
    /// ';' with ';').
    fn lex_symbol(&mut self, symbol: char) -> TokenKind {
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
                if let Some(symbol) = Symbol::compose(symbol, self.peek()) {
                    self.next_char();
                    TokenKind::Symbol(symbol)
                } else {
                    TokenKind::Symbol(Symbol::from_char(symbol))
                }
            }
        }
    }

    /// Returns the appropriate grouping delimiter for the given character.
    fn lex_grouping(&mut self, delimiter: char) -> TokenKind {
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

    /// Matches every character that can be part of an identifier. This includes
    /// upper and lower-case letters, the underscore, and the hyphen.
    fn lex_identifier(&mut self, first_char: char) -> TokenKind {
        let rest = self.consume_build(is_identifier_continue);
        let vec = [&vec![first_char], &rest[..]].concat();
        let string: String = vec.into_iter().collect();
        self.lex_keyword_or_identifier(string)
    }

    /// Attempts to match the provided `string` to a keyword, returning a
    /// `TokenKind::Keyword` if a match is found, otherwise a
    /// `TokenKind::Identifier`.
    fn lex_keyword_or_identifier(&mut self, string: String) -> TokenKind {
        match &*string {
            "and"       => TokenKind::Keyword(Keyword::And),
            "case"      => TokenKind::Keyword(Keyword::Case),
            "const"     => TokenKind::Keyword(Keyword::Const),
            "def"       => TokenKind::Keyword(Keyword::Def),
            "else"      => TokenKind::Keyword(Keyword::Else),
            "enum"      => TokenKind::Keyword(Keyword::Enum),
            "if"        => TokenKind::Keyword(Keyword::If),
            "internal"  => TokenKind::Keyword(Keyword::Internal),
            "let"       => TokenKind::Keyword(Keyword::Let),
            "match"     => TokenKind::Keyword(Keyword::Match),
            "module"    => TokenKind::Keyword(Keyword::Module),
            "mut"       => TokenKind::Keyword(Keyword::Mut),
            "not"       => TokenKind::Keyword(Keyword::Not),
            "of"        => TokenKind::Keyword(Keyword::Of),
            "or"        => TokenKind::Keyword(Keyword::Or),
            "public"    => TokenKind::Keyword(Keyword::Public),
            "ref"       => TokenKind::Keyword(Keyword::Ref),
            "return"    => TokenKind::Keyword(Keyword::Return),
            "struct"    => TokenKind::Keyword(Keyword::Struct),
            "trait"     => TokenKind::Keyword(Keyword::Trait),
            "type"      => TokenKind::Keyword(Keyword::Type),
            "using"     => TokenKind::Keyword(Keyword::Using),
            "var"       => TokenKind::Keyword(Keyword::Var),
            "with"      => TokenKind::Keyword(Keyword::With),
            _           => TokenKind::Identifier
        }
    }

    /// Matches any valid sequence of digits that can form an integer or float
    /// literal. Only integer literals support the binary, octal, and
    /// hexadecimal bases, in addition to the default decimal base.
    fn lex_number(&mut self, first_digit: char) -> TokenKind {
        let mut base = Base::Decimal;

        if first_digit == '0' {
            match self.peek() {
                // Binary literal.
                'b' => {
                    base = Base::Binary;
                    self.next_char();
                    self.consume_digits(Base::Binary, None);
                },
                // Octal literal.
                'o' => {
                    base = Base::Octal;
                    self.next_char();
                    self.consume_digits(Base::Octal, None);
                },
                // Hexadecimal literal.
                'x' => {
                    base = Base::Hexadecimal;
                    self.next_char();
                    self.consume_digits(Base::Hexadecimal, None);
                },
                // Decimal literal. We ignore the decimal point to avoid it
                // from being pushed into the `integer_part` vector (it'll
                // be the first element of the `fractional_part` vector
                // later on instead).
                '0'..='9' | '_' => {
                    self.consume_digits(Base::Decimal, None);
                }
                // Just 0.
                _ => ()
            }
        } else {
            self.consume_digits(Base::Decimal, Some(first_digit));
        };

        let mut has_fractional_part = false;

        if self.peek() == '.' && self.peek_at(1) != '.' {
            self.next_char();
            has_fractional_part = true;
            match self.peek() {
                '0'..='9' | '_' => {
                    self.consume_digits(Base::Decimal, None);
                },
                _ => ()
            }
        }

        if !has_fractional_part {
            TokenKind::Literal(Literal::Integer(base))
        } else {
            if base == Base::Decimal {
                TokenKind::Literal(Literal::Float)
            } else {
                TokenKind::Error(LexerError::UnsupportedFloatLiteralBase(base))
            }
        }
    }
}
