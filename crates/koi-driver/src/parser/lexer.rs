#![allow(dead_code)]

use crate::source::{Cursor, Position, Source};
use crate::parser::token::*;
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

/// Checks if the given character is a whitespace character.
fn is_whitespace(c: char) -> bool {
    c == ' ' ||  c == '\t'
}

pub enum LexerMode {
    Normal,
}

impl Default for LexerMode {
    fn default() -> Self {
        Self::Normal
    }
}

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
            ' ' | '\t' => self.whitespace(),
            '\n'| '\r' => self.newline(),
            '/' => {
                if self.peek() == '/' {
                    self.line_comment()
                } else {
                    self.symbol('/')
                }
            },
            // '\''=> unimplemented!("character literal"),
            '"' => self.string(),
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
    fn whitespace(&mut self) -> TokenKind {
        self.consume_while(is_whitespace);
        TokenKind::Whitespace
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
            let parsed = i32::from_str_radix(&*string, radix)
                             .expect("Failed to parse integer");
            TokenKind::Literal(Literal::Int { base, value: parsed })
        } else {
            let all = [&integer_part[..], &fractional_part[..]].concat();
            let string: String = all[..].into_iter().collect();
            let parsed: f64 = string.parse().expect("Failed to parse float");
            // TODO: We should only allow floats to be in decimal base
            TokenKind::Literal(Literal::Float { base, value: parsed })
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

    /// Consumes all the characters found within quotation marks (`"`).
    ///
    /// Simple escape sequences are recognised and are dealt accordingly. These
    /// sequences include `\\n`, `\\`, `\"`, `\t`, `\n`, and `\r`. Any other
    /// escape sequence is invalid, thus this method will panic.
    ///
    /// String interpolation is currently unsupported.
    fn string(&mut self) -> TokenKind {
        let mut ignore_whitespace = false;
        let mut string_content = Vec::new();

        while let Some(c) = self.next_char() {
            match c {
                // We reached the end of the string.
                '"' => {
                    let content = string_content.iter().collect();
                    return TokenKind::Literal(
                        Literal::Str { content, terminated: true }
                    );
                },
                // We are at the start of an escape sequence.
                '\\' => match self.peek() {
                    // Keep the next character if it is a backslash or a double
                    // quote character.
                    '\\' | '"' => {
                        string_content.push(self.next_char().unwrap())
                    },
                    // If it is a valid escape code character, we'll insert an
                    // actual unicode character as if it was present on the
                    // string we're building.
                    e @ 't' | e @ 'n' | e @ 'r' | e @ '\n' => {
                        if e == 't' {
                            string_content.push('\u{0009}');
                        } else if e == 'n' {
                            string_content.push('\u{000A}');
                        } else if e == 'r' {
                            string_content.push('\u{000D}')
                        } else if e == '\n' {
                            ignore_whitespace = true;
                        }
                        self.next_char();
                    },
                    // Otherwise we found an invalid escape sequence. We'll
                    // panic here.
                    c => {
                        eprintln! {
                            "Invalid escape sequence `\\{character}`. The \
                            character `{character}` cannot be escaped.",
                            character=c
                        };
                        std::process::exit(1);
                    }
                }
                // Whitespace characters are ignored if followed by a `\` and a
                // newline character.
                ' ' | '\n' | '\t' | '\r' if ignore_whitespace => (),
                // We push any other character into the vector to be part of the
                // string.
                c => {
                    ignore_whitespace = false;
                    string_content.push(c)
                }
            }
        }

        let content = string_content.iter().collect();
        TokenKind::Literal(Literal::Str { content, terminated: false })
    }
}
