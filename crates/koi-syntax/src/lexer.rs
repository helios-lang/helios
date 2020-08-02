use crate::cache::Cache;
use crate::errors::LexerError;
use crate::source::{Cursor, TextSpan};
use crate::tree::token::*;
use std::default::Default;
use std::rc::Rc;
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

#[allow(dead_code)]
/// Checks if the given character is a whitespace character. This includes the
/// space character, the carriage return character, and the tab character. Only
/// the newline character is used to signify a new line.
fn is_whitespace(c: char) -> bool {
    c == ' ' || c == '\r' || c == '\t'
}

#[allow(dead_code)]
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

pub type LexerOut = SyntaxToken;

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
    did_emit_eof_token: bool,
    consumed_chars: Vec<char>,
    mode_stack: Vec<LexerMode>,
    token_cache: Cache<(TokenKind, String), Rc<RawSyntaxToken>>,
}

impl Lexer {
    pub fn with(source: String) -> Self {
        Self {
            cursor: Cursor::with(source),
            did_emit_eof_token: false,
            consumed_chars: Vec::new(),
            mode_stack: vec![LexerMode::Normal],
            token_cache: Cache::new(),
        }
    }

    pub fn next_token(&mut self) -> LexerOut {
        match self.current_mode() {
            LexerMode::Normal => self.tokenize_normal(),
            LexerMode::Grouping => self.tokenize_grouping(),
        }
    }

    #[allow(dead_code)]
    pub(crate) fn push_mode(&mut self, mode: LexerMode) {
        self.mode_stack.push(mode);
    }

    #[allow(dead_code)]
    pub(crate) fn pop_mode(&mut self) -> Option<LexerMode> {
        self.mode_stack.pop()
    }

    fn current_mode(&self) -> LexerMode {
        self.mode_stack.last().cloned().unwrap_or_default()
    }
}

impl Lexer {
    /// Retrieves the next character in the iterator.
    fn next_char(&mut self) -> Option<char> {
        self.cursor.advance().map(|c| {
            self.consumed_chars.push(c);
            c
        })
    }

    /// Peeks the next character without consuming it.
    fn peek(&self) -> char {
        self.peek_at(0)
    }

    /// Peeks the character at the given index without consuming it.
    fn peek_at(&self, n: usize) -> char {
        self.cursor.nth(n)
    }

    /// Checks if the `Lexer` has reached the end of the input.
    pub(crate) fn is_at_end(&self) -> bool {
        self.cursor.source_len() == 0 && self.did_emit_eof_token
    }

    pub(crate) fn current_pos(&self) -> usize {
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
    where
        F: Fn(char) -> bool,
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
    where
        F: Fn(char) -> bool,
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
    fn tokenize_normal(&mut self) -> LexerOut {
        let leading_trivia = self.lex_trivia(true);
        let start = self.current_pos();

        // Reset consumed chars
        self.consumed_chars.drain(..);
        let next_char = match self.next_char() {
            Some(c) => c,
            None => {
                self.did_emit_eof_token = true;

                let kind = TokenKind::Eof;
                let text = "\0".to_string();

                let eof_raw = self.token_cache.lookup(
                    (kind.clone(), text.clone()),
                    Rc::new(RawSyntaxToken::with(kind, text))
                );

                return SyntaxToken::with_trivia(
                    Rc::clone(eof_raw),
                    TextSpan::new(self.current_pos(), 0),
                    leading_trivia,
                    Vec::new()
                );
            }
        };

        let kind = match next_char {
            c if is_symbol(c) => self.lex_symbol(c),
            c if is_identifier_start(c) => self.lex_identifier(c),
            c @ '0'..='9' => self.lex_number(c),
            c => TokenKind::Unknown(c),
        };

        let end = self.current_pos();
        let text = self.consumed_chars.drain(..).collect::<String>();
        let trailing_trivia = self.lex_trivia(false);
        let raw = self.token_cache.lookup(
            (kind.clone(), text.clone()),
            Rc::new(RawSyntaxToken::with(kind, text))
        );

        SyntaxToken::with_trivia(
            Rc::clone(raw),
            TextSpan::from_bounds(start, end),
            leading_trivia,
            trailing_trivia
        )
    }

    fn tokenize_grouping(&mut self) -> LexerOut {
        todo!("Lexer::tokenize_grouping")
    }
}

impl Lexer {
    /// Builds a collection of `SyntaxTrivia`.
    ///
    /// SyntaxTrivia are pieces of syntax that are not essential to the
    /// semantics of the program, such as whitespace and line comments. This
    /// information is tacked on to most tokens, establishing any trivia that
    /// appears before or after it.
    fn lex_trivia(&mut self, is_leading: bool) -> Vec<SyntaxTrivia> {
        let mut trivia = Vec::new();
        let start = self.current_pos();

        loop {
            match (self.peek(), self.peek_at(1)) {
                ('\n', _) if is_leading => {
                    let count = self.consume_while(|c| c == '\n');
                    trivia.push(SyntaxTrivia::LineFeed(count))
                },
                ('\r', '\n') if is_leading => {
                    // Consume peeked tokens
                    self.next_char();
                    self.next_char();

                    // We already have 1 CRLF sequence
                    let mut count = 1;
                    while ('\r', '\n') == (self.peek(), self.peek_at(1)) {
                        // Consume peeked tokens
                        self.next_char();
                        self.next_char();
                        count += 1;
                    }

                    trivia.push(SyntaxTrivia::CarriageReturnLineFeed(count))
                },
                ('\r', c) if c != '\n' => {
                    let count = self.consume_while(|c| c == '\r');
                    trivia.push(SyntaxTrivia::CarriageReturn(count))
                },
                (' ', _) => {
                    let count = self.consume_while(|c| c == ' ');
                    trivia.push(SyntaxTrivia::Space(count))
                },
                ('\t', _) => {
                    let count = self.consume_while(|c| c == '\t');
                    trivia.push(SyntaxTrivia::Tab(count))
                },
                ('/', '/') => {
                    // Consume peeked tokens
                    self.next_char();
                    self.next_char();

                    let is_doc_comment = self.consume('/') || self.consume('!');

                    // Consume until we're before a LF, CRLF or EOF character
                    loop {
                        match (self.peek(), self.peek_at(1)) {
                            ('\n', _) | ('\0', _) | ('\r', '\n') => break,
                            _ => { self.next_char(); }
                        }
                    }

                    trivia.push(SyntaxTrivia::LineComment {
                        is_doc_comment,
                        len: TextSpan::from_bounds(start, self.current_pos()).length()
                    })
                },
                _ => break,
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

    /// Matches every character that can be part of an identifier. This includes
    /// upper and lower-case letters, decimal digits and the underscore.
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
                // Binary literal
                'b' => {
                    base = Base::Binary;
                    self.next_char();
                    self.consume_digits(Base::Binary, None);
                },
                // Octal literal
                'o' => {
                    base = Base::Octal;
                    self.next_char();
                    self.consume_digits(Base::Octal, None);
                },
                // Hexadecimal literal
                'x' => {
                    base = Base::Hexadecimal;
                    self.next_char();
                    self.consume_digits(Base::Hexadecimal, None);
                },
                // Decimal literal
                '0'..='9' | '_' => {
                    self.consume_digits(Base::Decimal, None);
                }
                // Just 0
                _ => ()
            }
        } else {
            self.consume_digits(Base::Decimal, Some(first_digit));
        }

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer() {
        let source = "// Adding two variables\nlet a = 10\n";
        let mut lexer = Lexer::with(source.to_string());
        let mut tokens = Vec::new();

        while !lexer.is_at_end() {
            let token = lexer.next_token();
            tokens.push(token);
        }

        assert_eq!(tokens, vec! {
            SyntaxToken::with_trivia(
                Rc::new(RawSyntaxToken::with(
                    TokenKind::Keyword(Keyword::Let),
                    "let".to_string()
                )),
                TextSpan::new(24, 3),
                vec![
                    SyntaxTrivia::LineComment { is_doc_comment: false, len: 23 },
                    SyntaxTrivia::LineFeed(1),
                ],
                vec![SyntaxTrivia::Space(1)],
            ),
            SyntaxToken::with_trivia(
                Rc::new(RawSyntaxToken::with(
                    TokenKind::Identifier,
                    "a".to_string()
                )),
                TextSpan::new(28, 1),
                vec![],
                vec![SyntaxTrivia::Space(1)],
            ),
            SyntaxToken::with_trivia(
                Rc::new(RawSyntaxToken::with(
                    TokenKind::Symbol(Symbol::Eq),
                    "=".to_string()
                )),
                TextSpan::new(30, 1),
                vec![],
                vec![SyntaxTrivia::Space(1)],
            ),
            SyntaxToken::with_trivia(
                Rc::new(RawSyntaxToken::with(
                    TokenKind::Literal(Literal::Integer(Base::Decimal)),
                    "10".to_string()
                )),
                TextSpan::new(32, 2),
                vec![],
                vec![],
            ),
            SyntaxToken::with_trivia(
                Rc::new(RawSyntaxToken::with(
                    TokenKind::Eof,
                    "\0".to_string()
                )),
                TextSpan::new(35, 0),
                vec![SyntaxTrivia::LineFeed(1)],
                vec![],
            ),
        });
    }
}
