use crate::source::{Cursor, Position, Source};
use crate::token::*;
use std::error::Error;
use std::fmt::{self, Display};
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

/// Checks if the given character is a newline or carriage return character
/// (either `\n` or `\r`).
///
/// _TODO: Should we ensure that a carriage return character is preceded by a
/// newline character?_
fn is_newline(c: char) -> bool {
    c == '\n' || c == '\r'
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

enum DidFail<E> {
    Yes(E),
    No,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LexerError {
    OverflowedIntegerLiteral,
    OverflowedFloatLiteral,

    EmptyCharLiteral,
    UnterminatedCharLiteral,
    UnknownEscapeChar(char),
    IllegalTabCharInCharLiteral,
    MultipleCodepointsInCharLiteral,
    MultiLineSpanningChar,

    UnterminatedStrLiteral,
}

impl LexerError {
    pub fn message(&self) -> String {
        self.to_string()
    }

    pub fn code(&self) -> String {
        match self {
            Self::OverflowedIntegerLiteral => "E0010".to_string(),
            Self::OverflowedFloatLiteral => "E0011".to_string(),

            Self::EmptyCharLiteral => "E0012".to_string(),
            Self::UnterminatedCharLiteral => "E0013".to_string(),
            Self::UnknownEscapeChar(_) => "E0014".to_string(),
            Self::IllegalTabCharInCharLiteral => "E0015".to_string(),
            Self::MultipleCodepointsInCharLiteral => "E0016".to_string(),
            Self::MultiLineSpanningChar => "E0017".to_string(),

            Self::UnterminatedStrLiteral => "E0018".to_string(),
        }
    }
}

impl Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::OverflowedIntegerLiteral =>
                "Integer literal overflows when stored as `int32`".to_string(),
            Self::OverflowedFloatLiteral =>
                "Float literal overflows when stored as `float64`".to_string(),

            Self::EmptyCharLiteral =>
                "Character literals must not be empty".to_string(),
            Self::UnterminatedCharLiteral =>
                "Unterminated character literal".to_string(),
            Self::UnknownEscapeChar(c) =>
                format!("Unknown escape character: {:?}", c),
            Self::IllegalTabCharInCharLiteral =>
                "Illegal tab character in character literal".to_string(),
            Self::MultipleCodepointsInCharLiteral =>
                "Character literals should only contain one codepoint".to_string(),
            Self::MultiLineSpanningChar =>
                "Character literal cannot span multiple lines".to_string(),

            Self::UnterminatedStrLiteral =>
                "Unterminated string literal".to_string(),
        };
        write!(f, "{}", message)
    }
}

impl Error for LexerError {}

pub struct Lexer<'a> {
    cursor: Cursor<'a>,
    is_at_start_of_line: bool,
    current_indentation_level: usize,
}

impl<'a> Lexer<'a> {
    pub fn with(source: Source<'a>) -> Self {
        Self {
            cursor: Cursor::with(source),
            is_at_start_of_line: true,
            current_indentation_level: 0,
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        let next_char = self.next_char()?;

        let mut record_new_indent = false;
        let mut new_indent_level = self.current_indentation_level;

        if self.is_at_start_of_line {
            if next_char == ' ' || next_char == '\t' {
                record_new_indent = true;
            } else {
                new_indent_level = 0;
            }
            self.is_at_start_of_line = false;
        }

        // Because we've already advanced to the next character, it's position
        // is one less than our current character's position.
        // TODO: There must be a more elegant way to do this?
        let old_pos = Position::new(
            self.cursor.pos.line,
            self.cursor.pos.character - 1
        );

        let token_kind = match next_char {
            ' ' | '\t' => self.whitespace(next_char),
            '\n'| '\r' => self.newline(),
            '/' => {
                if self.peek() == '/' {
                    self.line_comment()
                } else {
                    self.symbol('/')
                }
            },
            'r' => {
                if self.peek() == '"' {
                    self.raw_string()
                } else {
                    self.identifier('r')
                }
            },
            'f' => {
                if self.peek() == '"' {
                    self.interpolated_string()
                } else {
                    self.identifier('f')
                }
            },
            '"' => self.string(),
            '\''=> self.character(),
            c if is_symbol(c) => self.symbol(c),
            c if is_identifier_start(c) => self.identifier(c),
            c @ '0'..='9' => self.number(c),
            c => TokenKind::Unexpected(c)
        };

        if record_new_indent { new_indent_level = self.cursor.pos.character; }
        self.current_indentation_level = new_indent_level;

        Some(Token::with(token_kind, old_pos..self.cursor.pos))
    }
}

impl<'a> Lexer<'a> {
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
    fn consume_build<F>(&mut self, first_char: char, predicate: F) -> Vec<char>
        where F: Fn(char) -> bool
    {
        let mut vec = Vec::new();
        vec.push(first_char);
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
    fn consume_digits(&mut self, base: NumericBase, first_digit: Option<char>) -> Vec<char> {
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
            NumericBase::Binary => {
                match_digits!('0' => '1');
            },
            NumericBase::Octal => {
                match_digits!('0' => '7');
            },
            NumericBase::Hexadecimal => {
                match_digits!('0' => '9', 'a' => 'f', 'A' => 'F');
            },
            NumericBase::Decimal => {
                match_digits!('0' => '9');
            },
        }

        vec
    }
}

impl<'a> Lexer<'a> {
    fn whitespace(&mut self, whitespace_char: char) -> TokenKind {
        if whitespace_char == ' ' {
            let count = 1 + self.consume_while(|c| c == ' ');
            TokenKind::Whitespace { kind: WhitespaceKind::Space, count }
        } else {
            let count = 1 + self.consume_while(|c| c == '\t');
            TokenKind::Whitespace { kind: WhitespaceKind::Tab, count }
        }
    }

    fn newline(&mut self) -> TokenKind {
        self.consume_while(is_newline);
        self.is_at_start_of_line = true;
        TokenKind::Newline
    }

    /// Matches every character up until the new line character. The characters
    /// consumed will be part of the `TokenKind::LineComment` variant.
    fn line_comment(&mut self) -> TokenKind {
        self.next_char();

        let next_char = self.peek();
        let mut is_doc_comment = false;
        if next_char == '!' || next_char == '/' { is_doc_comment = true }

        self.consume_while(|c| c != '\n');
        TokenKind::LineComment { is_doc_comment }
    }

    /// Matches every character that can be part of an identifier. This includes
    /// upper- and lower-case letters, the underscore, and the hyphen.
    fn identifier(
        &mut self,
        first_char: char
    ) -> TokenKind {
        let vec = self.consume_build(first_char, is_identifier_continue);
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
            "not"   => TokenKind::Keyword(Keyword::Not),
            "or"    => TokenKind::Keyword(Keyword::Or),
            "then"  => TokenKind::Keyword(Keyword::Then),
            "true"  => TokenKind::Keyword(Keyword::True),
            "type"  => TokenKind::Keyword(Keyword::Type),
            "using" => TokenKind::Keyword(Keyword::Using),
            "with"  => TokenKind::Keyword(Keyword::With),
            _       => TokenKind::Identifier(string)
        }
    }

    /// Matches any valid sequence of digits that can form an integer or float
    /// literal. Both literal forms support the binary, octal, and hexadecimal
    /// bases in addition to the default decimal system.
    fn number(&mut self, first_digit: char) -> TokenKind {
        let mut base = NumericBase::Decimal;
        let mut radix = 10;

        let integer_part = {
            if first_digit == '0' {
                match self.peek() {
                    // Binary literal.
                    'b' => {
                        base = NumericBase::Binary;
                        radix = 2;
                        self.next_char();
                        self.consume_digits(NumericBase::Binary, None)
                    },
                    // Octal literal.
                    'o' => {
                        base = NumericBase::Octal;
                        radix = 8;
                        self.next_char();
                        self.consume_digits(NumericBase::Octal, None)
                    },
                    // Hexadecimal literal.
                    'x' => {
                        base = NumericBase::Hexadecimal;
                        radix = 16;
                        self.next_char();
                        self.consume_digits(NumericBase::Hexadecimal, None)
                    },
                    // Decimal literal. We ignore the decimal point to avoid it
                    // from being pushed into the `integer_part` vector (it'll
                    // be the first element of the `fractional_part` vector
                    // later on instead).
                    '0'..='9' | '_' => {
                        self.consume_digits(NumericBase::Decimal, None)
                    }
                    // Just 0.
                    _ => vec!['0']
                }
            } else {
                self.consume_digits(NumericBase::Decimal, Some(first_digit))
            }
        };

        let mut fractional_part: Vec<char> = Vec::new();

        if self.peek() == '.' && self.peek_at(2) != '.' {
            fractional_part.push(self.next_char().unwrap());
            match self.peek() {
                '0'..='9' | '_' => {
                    let mut rest =
                        self.consume_digits(NumericBase::Decimal, None);
                    fractional_part.append(&mut rest);
                },
                _ => fractional_part.push('0')
            }
        }

        if fractional_part.is_empty() {
            let string: String = integer_part[..].into_iter().collect();
            match i32::from_str_radix(&*string, radix) {
                Ok(value) => TokenKind::Literal(Literal::Int { base, value }),
                _ => TokenKind::Error(LexerError::OverflowedIntegerLiteral)
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

    /// Matches any character that is a valid symbol.
    ///
    /// _TODO:_ Perhaps we should handle cases with misleading symbols, such as
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

    /// Abstraction for consuming an escape sequence.
    ///
    /// For now, simple escape sequences are recognised. These include: `\\`,
    /// `\'`, `\"`, `\n`, `\r`, `\t` and `\0`. Implementation is required for
    /// hexadecimal and unicode escape sequences.
    fn escape_sequence(&mut self, contents: &mut Vec<char>) -> DidFail<char> {
        match self.peek() {
            '\'' => { contents.push('\''); DidFail::No },
            '\"' => { contents.push('\"'); DidFail::No },
            'n'  => { contents.push('\u{000A}'); DidFail::No },
            'r'  => { contents.push('\u{000D}'); DidFail::No },
            't'  => { contents.push('\u{0009}'); DidFail::No },
            '0'  => { contents.push('\u{0000}'); DidFail::No },
            c => DidFail::Yes(c)
        }
    }

    /// Consumes a character literal.
    ///
    /// The lexer will attempt to consume every character between single quotes
    /// (`'`). This is so we can capture unicode and hexadecimal escapes, which
    /// can span multiple characters in the source code. For now, however, only
    /// the simple escapes are recognised: `\\`, `\'`, `\"`, `\n`, `\r`, `\t`
    /// and `\0`. `\↵` is an illegal escape sequence since characters cannot
    /// span multiple lines.
    fn character(&mut self) -> TokenKind {
        let mut error = None::<LexerError>;
        let mut contents = Vec::new();

        while let Some(c) = self.next_char() {
            match c {
                // We reached the end of the character
                '\'' => {
                    if let Some(error) = error {
                        return TokenKind::Error(error);
                    } else if contents.len() > 1 {
                        return TokenKind::Error(
                            LexerError::MultipleCodepointsInCharLiteral);
                    } else if let Some(character) = contents.pop() {
                        return TokenKind::Literal(Literal::Char(character));
                    } else {
                        return TokenKind::Error(LexerError::EmptyCharLiteral);
                    }
                },
                '\\' => {
                    match self.escape_sequence(&mut contents) {
                        DidFail::No => {
                            self.next_char();
                        },
                        DidFail::Yes(c) => {
                            if error == None {
                                error = Some(LexerError::UnknownEscapeChar(c));
                            }
                            contents.push(self.next_char().unwrap());
                        }
                    }
                }
                '\n' | '\r' => {
                    if error == None {
                        error = Some(LexerError::MultiLineSpanningChar);
                    }
                },
                '\t' => {
                    if error == None {
                        error = Some(LexerError::IllegalTabCharInCharLiteral)
                    }
                },
                c => contents.push(c)
            }
        }

        // We are here if the character literal is unterminated
        if let Some(error) = error {
            TokenKind::Error(error)
        } else if contents.len() > 1 {
            TokenKind::Error(LexerError::MultipleCodepointsInCharLiteral)
        } else if let Some(_) = contents.pop() {
            // TokenKind::Literal(Literal::Char(character))
            TokenKind::Error(LexerError::UnterminatedCharLiteral)
        } else {
            TokenKind::Error(LexerError::EmptyCharLiteral)
        }
    }

    /// Consumes a string literal.
    ///
    /// The lexer will consume all the characters found between (and including)
    /// the quotation marks (`"`). Simple escape sequences are recognised and
    /// dealt accordingly. These sequences include `\↵`, `\\`, `\"`, `\n`, `\r`,
    /// `\t` and `\0`. Any other escape sequence is invalid and therefore this
    /// method will return `TokenKind::Error`.
    fn string(&mut self) -> TokenKind {
        let mut ignore_whitespace = false;
        let mut error = None::<LexerError>;
        let mut contents = Vec::new();

        while let Some(c) = self.next_char() {
            match c {
                // We reached the end of the string
                '"' => {
                    if let Some(error) = error {
                        return TokenKind::Error(error);
                    } else {
                        let content = contents.iter().collect();
                        return TokenKind::Literal(Literal::Str(content));
                    }
                },
                // We are at the start of an escape sequence
                '\\' => match self.peek() {
                    // Keep the next character if it is a backslash or a double
                    // quote character
                    '\\' | '"' => {
                        contents.push(self.next_char().unwrap())
                    },
                    // The next character is a line feed, and thus we need to
                    // ignore all whitespace characters that follow
                    '\n' => {
                        ignore_whitespace = true;
                    },
                    // If it is a valid escape code character, we'll insert an
                    // actual unicode character as if it was present on the
                    // string we're building
                    // ... TODO: Unicode escapes ...
                    e @ 'n' | e @ 'r' | e @ 't' | e @ '0' => {
                        if e == 'n' {
                            contents.push('\u{000A}');
                        } else if e == 'r' {
                            contents.push('\u{000D}');
                        } else if e == 't' {
                            contents.push('\u{0009}');
                        } else {
                            contents.push('\u{0000}');
                        }
                        self.next_char();
                    },
                    // Otherwise we found an invalid escape sequence – we'll
                    // panic here
                    c => {
                        // We'll only keep the first error
                        if error == None {
                            error = Some(LexerError::UnknownEscapeChar(c));
                        }
                        contents.push(self.next_char().unwrap());
                    }
                }
                // Whitespace characters are ignored if followed by a `\` and a
                // line feed character.
                ' ' | '\r' | '\n' | '\t' if ignore_whitespace => (),
                // We push any other character into the vector to be part of the
                // string.
                c => {
                    ignore_whitespace = false;
                    contents.push(c)
                }
            }
        }

        // We are here if the string literal is unterminated
        if let Some(error) = error {
            TokenKind::Error(error)
        } else {
            // let content = contents.iter().collect();
            // TokenKind::Literal(Literal::Str { content, terminated: false })
            TokenKind::Error(LexerError::UnterminatedStrLiteral)
        }
    }

    /// Consumes a raw string literal.
    ///
    /// Unlike a string literal, escape sequences are not specially treated.
    /// This method will consume the tag `r` before consuming every character
    /// between (and including) the quotation marks (`"`), as presented in the
    /// source code.
    ///
    /// As such, backslashes (`\`) appear the same number of times as the source
    /// code. This makes it is easier to input strings that typically contain
    /// a lot of backslashes, such as Regex strings or Windows directory paths.
    fn raw_string(&mut self) -> TokenKind {
        self.next_char();
        let mut content = Vec::new();

        while let Some(c) = self.next_char() {
            match c {
                // We reached the end of the string.
                '"' => {
                    let content = content.iter().collect();
                    return TokenKind::Literal(Literal::Str(content));
                },
                c => content.push(c)
            }
        }

        // We are here if the string literal is unterminated
        // let content = content.iter().collect();
        // TokenKind::Literal(Literal::Str { content, terminated: false })
        TokenKind::Error(LexerError::UnterminatedStrLiteral)
    }

    /// Consumes an interpolated string literal (f-string).
    ///
    /// For now, it behaves exactly the same as a normal string literal. The
    /// only difference is that it also consumes the tag `f` before consuming
    /// all the characters between (and including) the quotation marks (`"`).
    ///
    /// The actual process of interpolation is handled by the parser. This
    /// method returns the token kind (`FStr`), which lets the parser know that
    /// it must handle this string literal differently.
    fn interpolated_string(&mut self) -> TokenKind {
        self.next_char();
        match self.string() {
            // We'll just map `Literal::Str` to `Literal::FStr` for now
            TokenKind::Literal(Literal::Str(content)) => {
                TokenKind::Literal(Literal::FStr(content))
            },
            token_kind => token_kind
        }
    }
}
