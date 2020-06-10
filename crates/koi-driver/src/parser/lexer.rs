use crate::source::{Cursor, Position, Source};
use crate::parser::token::*;
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

#[derive(Clone, Debug, PartialEq)]
pub enum LexerError {
    InvalidStringLiteralEscapeChar(char),
    EmptyCharacterLiteral,
}

impl Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::InvalidStringLiteralEscapeChar(c) =>
                format!("invalid escape character: {:?}", c),
            Self::EmptyCharacterLiteral =>
                format!("empty character literal")
        };
        write!(f, "{}", message)
    }
}

impl Error for LexerError {}

pub struct Lexer<'a> {
    cursor: Cursor<'a>,
}

impl<'a> Lexer<'a> {
    pub fn with(source: Source<'a>) -> Self {
        Self { cursor: Cursor::with(source) }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        let next_char = self.next_char()?;

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
            // '\''=> unimplemented!("character literal"),
            c if is_symbol(c) => self.symbol(c),
            c if is_identifier_start(c) => self.identifier(c),
            c @ '0'..='9' => self.number(c),
            c => TokenKind::Unknown(c)
        };

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

    /// Peaks the next character at the given index without consuming it.
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
    fn consume_build<F>(
        &mut self,
        first_char: char,
        predicate: F
    ) -> Vec<char>
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
    fn consume_digits(
        &mut self,
        base: NumericBase,
        first_digit: Option<char>
    ) -> Vec<char> {
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

    /// Checks if the given character reoccurs for a given time.
    fn is_reoccuring(&mut self, c: char, occurence: usize) -> bool {
        let mut found = 0;
        while self.peek() == c {
            self.next_char();
            found += 1;
        }
        found == occurence
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
            let parsed = i32::from_str_radix(&*string, radix);
            let value = match parsed {
                Ok(value) => IntValue::Value(value),
                // It should be fine to assume we overflowed here
                _ => IntValue::Overflowed
            };

            TokenKind::Literal(Literal::Int { base, value })
        } else {
            let all = [&integer_part[..], &fractional_part[..]].concat();
            let string: String = all[..].into_iter().collect();
            let parsed = string.parse();
            let value = match parsed {
                Ok(value) => FloatValue::Value(value),
                // It should be fine to assume we overflowed here
                _ => FloatValue::Overflowed
            };

            // TODO: We should only allow floats to be in decimal base
            TokenKind::Literal(Literal::Float { base, value })
        }
    }

    /// Matches any character that is a valid symbol.
    fn symbol(&mut self, symbol: char) -> TokenKind {
        match self.peek() {
            '=' => {
                self.next_char();
                TokenKind::Symbol(Symbol::from_char_with_equal(symbol))
            },
            '?' => {
                if self.is_reoccuring('?', 2) {
                    TokenKind::Keyword(Keyword::Unimplemented)
                } else {
                    TokenKind::Symbol(Symbol::Question)
                }
            },
            _ => TokenKind::Symbol(Symbol::from_char(symbol))
        }
    }

    /// Consumes a string literal.
    ///
    /// The lexer will consume all the characters found between (and including)
    /// the quotation marks (`"`). Simple escape sequences are recognised and
    /// dealt accordingly. These sequences include `\↵`, `\\`, `\"`, `\t`, `\n`,
    /// and `\r`. Any other escape sequence is invalid and therefore this
    /// method will return `TokenKind::Error`.
    fn string(&mut self) -> TokenKind {
        let mut ignore_whitespace = false;
        let mut error = None::<LexerError>;
        let mut string_content = Vec::new();

        while let Some(c) = self.next_char() {
            match c {
                // We reached the end of the string
                '"' => {
                    if let Some(error) = error {
                        return TokenKind::Error(error);
                    } else {
                        let content = string_content.iter().collect();
                        return TokenKind::Literal(
                            Literal::Str { content, terminated: true }
                        );
                    }
                },
                // We are at the start of an escape sequence
                '\\' => match self.peek() {
                    // Keep the next character if it is a backslash or a double
                    // quote character
                    '\\' | '"' => {
                        string_content.push(self.next_char().unwrap())
                    },
                    // The next character is a line feed, and thus we need to
                    // ignore all whitespace characters that follow
                    '\n' => {
                        ignore_whitespace = true;
                    },
                    // If it is a valid escape code character, we'll insert an
                    // actual unicode character as if it was present on the
                    // string we're building
                    e @ 't' | e @ 'n' | e @ 'r' => {
                        if e == 't' {
                            string_content.push('\u{0009}');
                        } else if e == 'n' {
                            string_content.push('\u{000A}');
                        } else if e == 'r' {
                            string_content.push('\u{000D}');
                        }
                        self.next_char();
                    },
                    // Otherwise we found an invalid escape sequence – we'll
                    // panic here
                    c => {
                        // We'll only keep the first error
                        if error == None {
                            error = Some(LexerError::InvalidStringLiteralEscapeChar(c));
                        }
                        string_content.push(self.next_char().unwrap());
                    }
                }
                // Whitespace characters are ignored if followed by a `\` and a
                // line feed character.
                ' ' | '\n' | '\t' | '\r' if ignore_whitespace => (),
                // We push any other character into the vector to be part of the
                // string.
                c => {
                    ignore_whitespace = false;
                    string_content.push(c)
                }
            }
        }

        // We are here if the string literal is unterminated
        if let Some(error) = error {
            TokenKind::Error(error)
        } else {
            let content = string_content.iter().collect();
            TokenKind::Literal(Literal::Str { content, terminated: false })
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
                    return TokenKind::Literal(
                        Literal::Str { content, terminated: true }
                    )
                },
                c => content.push(c)
            }
        }

        // We are here if the string literal is unterminated
        let content = content.iter().collect();
        TokenKind::Literal(Literal::Str { content, terminated: false })
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
            TokenKind::Literal(Literal::Str { content, terminated }) => {
                TokenKind::Literal(Literal::FStr { content, terminated })
            },
            token_kind => token_kind
        }
    }
}
