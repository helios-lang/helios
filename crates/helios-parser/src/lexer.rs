//! Tokenizing Helios source files.
//!
//! The showrunner of this module is the [`Lexer`] struct. It essentially takes
//! a given input representing a Helios source file and provides an
//! [`Iterator`] of [`Token`]s for it.
//!
//! The lexer aims to be as error-tolerant and UTF-8 friendly as possible (the
//! latter of which is enforced by Rust's `char` type). It is also lossless,
//! meaning that it represents the original text exactly as it is (including
//! whitespace and comments).
//!
//! Refer to [`Lexer`]'s and [`Token`]'s documentation for more information on
//! how tokenization is done.
//!
//! [`parse`]: crate::parse

use helios_diagnostics::Location;
use helios_syntax::{self, SyntaxKind};
use std::ops::Range;
use unicode_xid::UnicodeXID;

use crate::cursor::Cursor;
use crate::message::{LexerMessage, Message};

/// Determines whether or not the given character is a valid beginning of an
/// identifier. A valid start of an identifier is any Unicode code point that
/// satisfies the `XID_Start` property.
fn is_identifier_start(c: char) -> bool {
    // Fast-path for ASCII characters
    ('a' <= c && c <= 'z')
        || ('A' <= c && c <= 'Z')
        || c == '_'
        || c.is_xid_start()
}

/// Determines whether or not the given character is a valid continuation of an
/// identifier. A valid continuation of an identifier is any Unicode code point
/// that satisfies the `XID_Continue` property.
fn is_identifier_continue(c: char) -> bool {
    // Fast-path for ASCII characters
    ('a' <= c && c <= 'z')
        || ('A' <= c && c <= 'Z')
        || ('0' <= c && c <= '9')
        || c == '_'
        || c.is_xid_continue()
}

/// Determines whether or not the given character is a recognised symbol.
#[rustfmt::skip]
fn is_symbol(c: char) -> bool {
    match c {
        '&' | '*' | '@' | '!' | '^' | ':' | ',' | '$' | '.' | '–' | '—' | '=' |
        '-' | '%' | '+' | '#' | '?' | ';' | '£' | '~' | '|' | '/' | '\\'| '<' |
        '>' | '{' | '}' | '[' | ']' | '(' | ')' => true,
        _ => false,
    }
}

/// Determines whether or not the given character is a digit.
fn is_digit(c: char) -> bool {
    matches!(c, '0'..='9')
}

/// Checks whether or not the given character is a whitespace delimiter.
fn is_whitespace(c: char) -> bool {
    matches!(c, ' ' | '\t' | '\r')
}

/// A tuple of a tokenized token and possibly a diagnostic message if there was
/// an issue during the tokenization process.
pub type LexerItem<'source, FileId> = (Token<'source>, Option<Message<FileId>>);

/// Internal type returned by all tokenization methods in the lexer.
type LexerReturn<FileId> = (SyntaxKind, Option<Message<FileId>>);

/// The unit of a tokenized Helios source file.
///
/// This structure holds the [`SyntaxKind`] of a token, the text that formed it,
/// and its range in the source code (using `text_size::TextRange`). It is also
/// the `Item` type of the [`Lexer`] iterator.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Token<'source> {
    pub kind: SyntaxKind,
    pub text: &'source str,
    pub range: Range<usize>,
}

impl<'source> Token<'source> {
    /// Constructs a new [`Token`] with the given kind, text and range.
    pub fn new(
        kind: SyntaxKind,
        text: &'source str,
        range: Range<usize>,
    ) -> Self {
        Self { kind, text, range }
    }
}

/// A lazy, lossless lexer for the Helios programming language.
///
/// This lexer works with `char`s to seamlessly work with Unicode characters. It
/// also implements the [`Iterator`] trait, meaning that tokenization happens
/// lazily (i.e. the whole source text is not tokenized at once).
///
/// This structure shouldn't need to be manipulated manually. It is instead
/// strongly recommended to call the [`parse`] function which abstracts over the
/// tokenization and parsing processes of a Helios source.
///
/// [`parse`]: crate::parse
pub struct Lexer<'source, FileId> {
    file_id: FileId,
    cursor: Cursor<'source>,
}

impl<'source, FileId> Lexer<'source, FileId>
where
    FileId: Clone + Default,
{
    /// Construct a new [`Lexer`] with a reference to the source text.
    ///
    /// The lexer will initialise with the default [`LexerMode`] and set the
    /// cursor position to the start.
    pub fn new(file_id: FileId, source: &'source str) -> Self {
        Self {
            file_id,
            cursor: Cursor::new(source),
        }
    }

    /// Returns a [`SyntaxKind::UnknownChar`] with an error message detailing
    /// the provided unknown character and its location in the file.
    fn unknown(&self, character: char, start: usize) -> LexerReturn<FileId> {
        let message = Message::new(
            LexerMessage::UnknownCharacter(character),
            Location::new(self.file_id.clone(), start..(start + 1)),
        );

        (SyntaxKind::UnknownChar, Some(message))
    }
}

impl<'source, FileId> Lexer<'source, FileId> {
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

    /// Checks if the lexer has reached the end of the input.
    pub(crate) fn is_at_end(&self) -> bool {
        self.cursor.is_at_end()
    }

    /// Returns the current position of the lexer.
    pub(crate) fn current_pos(&self) -> usize {
        self.cursor.pos()
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

    /// Consumes the input while the given `predicate` holds true, returning a
    /// slice over the characters consumed.
    fn consume_build<F>(&mut self, predicate: F) -> &str
    where
        F: Fn(char) -> bool,
    {
        self.cursor.checkpoint();
        self.consume_while(predicate);
        self.cursor.slice()
    }
}

impl<'source, FileId> Lexer<'source, FileId> {
    fn lex_newline(&mut self, _: char) -> LexerReturn<FileId> {
        // We only count spaces as indentation sigils.
        // TODO: Emit an error if we find a TAB character here.
        self.consume_while(|c| c == ' ');
        (SyntaxKind::Newline, None)
    }

    /// Tokenizes a line comment.
    ///
    /// A line comment starts with a pound/hashtag (`#`) and ends at the next
    /// line feed or the end of file, whichever comes first. This function also
    /// handles documentation comments, which start with two pounds (`##`) or
    /// the familiar shebang sequence (`#!`).
    fn lex_comment(&mut self, _: char) -> LexerReturn<FileId> {
        // Check if it is a doc-comment
        if self.peek() == '#' || self.peek() == '!' {
            self.consume_while(|c| c != '\n');
            (SyntaxKind::DocComment, None)
        } else {
            self.consume_while(|c| c != '\n');
            (SyntaxKind::Comment, None)
        }
    }

    /// Tokenizes a contiguous series of whitespace delimiters.
    fn lex_whitespace(&mut self, _: char) -> LexerReturn<FileId> {
        self.consume_while(is_whitespace);
        (SyntaxKind::Whitespace, None)
    }

    /// Tokenizes a valid symbol.
    ///
    /// _TODO:_ Perhaps we could handle cases with confused symbols, such as
    /// U+037E, the Greek question mark, which looks like a semicolon (compare
    /// ';' with ';').
    fn lex_symbol(&mut self, symbol: char) -> LexerReturn<FileId> {
        match symbol {
            '?' => {
                if (self.peek(), self.peek_at(1)) == ('?', '?') {
                    // Consume the next two question marks
                    self.next_char();
                    self.next_char();
                    (SyntaxKind::Placeholder, None)
                } else {
                    (SyntaxKind::Sym_Question, None)
                }
            }
            _ => {
                if let Some(symbol) =
                    helios_syntax::symbol_from_chars(&[symbol, self.peek()])
                {
                    self.next_char();
                    (symbol, None)
                } else {
                    (helios_syntax::symbol_from_char(symbol), None)
                }
            }
        }
    }

    /// Tokenizes a contiguous series of characters that may be part of an
    /// identifier.
    ///
    /// This includes upper- and lower-case letters, decimal digits and the
    /// underscore.
    fn lex_identifier(&mut self, first_char: char) -> LexerReturn<FileId> {
        let mut string = String::new();
        string.push(first_char);
        string.push_str(self.consume_build(is_identifier_continue));
        (self.lex_keyword_or_identifier(string.as_str()), None)
    }

    /// Attempts to tokenize the provided string into a keyword or identifier.
    #[rustfmt::skip]
    fn lex_keyword_or_identifier(&mut self, slice: &str) -> SyntaxKind {
        match slice {
            "and"       => SyntaxKind::Kwd_And,
            "as"        => SyntaxKind::Kwd_As,
            "case"      => SyntaxKind::Kwd_Case,
            "else"      => SyntaxKind::Kwd_Else,
            "enum"      => SyntaxKind::Kwd_Enum,
            "for"       => SyntaxKind::Kwd_For,
            "forall"    => SyntaxKind::Kwd_Forall,
            "func"      => SyntaxKind::Kwd_Func,
            "if"        => SyntaxKind::Kwd_If,
            "impl"      => SyntaxKind::Kwd_Impl,
            "import"    => SyntaxKind::Kwd_Import,
            "in"        => SyntaxKind::Kwd_In,
            "iter"      => SyntaxKind::Kwd_Iter,
            "let"       => SyntaxKind::Kwd_Let,
            "module"    => SyntaxKind::Kwd_Module,
            "not"       => SyntaxKind::Kwd_Not,
            "of"        => SyntaxKind::Kwd_Of,
            "or"        => SyntaxKind::Kwd_Or,
            "range"     => SyntaxKind::Kwd_Range,
            "record"    => SyntaxKind::Kwd_Record,
            "type"      => SyntaxKind::Kwd_Type,
            "var"       => SyntaxKind::Kwd_Var,
            "while"     => SyntaxKind::Kwd_While,
            "with"      => SyntaxKind::Kwd_With,
            "yield"     => SyntaxKind::Kwd_Yield,
            "_"         => SyntaxKind::ReservedIdentifier,
            _           => SyntaxKind::Identifier,
        }
    }

    /// Tokenizes a contiguous series of characters that may be part of an
    /// integer or float literal.
    ///
    /// _NOTE:_ The lexer does not verify if the the number literal is correctly
    /// formatted in its base.
    fn lex_number(&mut self, c: char) -> LexerReturn<FileId> {
        fn is_digit_continue(c: char) -> bool {
            matches!(c, '_' | '0'..='9' | 'a'..='z' | 'A'..='Z')
        }

        // First, we'll check if the number literal is in a non-decimal base.
        if matches!((c, self.peek()), ('0', 'b') | ('0', 'o') | ('0', 'x')) {
            // Number literals of non-decimal base can only be integers, so
            // we'll consume any digit that may be part of a number (including
            // invalid letters like 'z').
            self.consume_while(is_digit_continue);
            (SyntaxKind::Lit_Integer, None)
        } else {
            // This number literal is in decimal base, so we'll consume the
            // integer part first.
            self.consume_while(is_digit_continue);

            // If there is a dot after the integer part, and the next character
            // after it does NOT start an identifier, then this must be a float
            // literal. Otherwise, it may be a field access (e.g. `10.foo`),
            // which isn't valid anyway, but we don't need to worry about it
            // here in the lexer.
            if self.peek() == '.' && !is_identifier_start(self.peek_at(1)) {
                self.next_char();
                self.consume_while(is_digit_continue);
                (SyntaxKind::Lit_Float, None)
            } else {
                (SyntaxKind::Lit_Integer, None)
            }
        }
    }
}

impl<'source, FileId> Iterator for Lexer<'source, FileId>
where
    FileId: Clone + Default,
{
    type Item = LexerItem<'source, FileId>;

    fn next(&mut self) -> Option<Self::Item> {
        self.cursor.checkpoint();
        let start = self.current_pos();

        let (kind, message) = match self.cursor.advance()? {
            c if c == '\n' => self.lex_newline(c),
            c if c == '#' => self.lex_comment(c),
            c if is_whitespace(c) => self.lex_whitespace(c),
            c if is_symbol(c) => self.lex_symbol(c),
            c if is_identifier_start(c) => self.lex_identifier(c),
            c if is_digit(c) => self.lex_number(c),
            c => self.unknown(c, start),
        };

        let end = self.current_pos();
        let text = self.cursor.slice();

        Some((Token::new(kind, text, start..end), message))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(input: &str, kind: SyntaxKind) {
        let mut lexer = Lexer::new(0u8, input);
        let (token, _) = lexer.next().unwrap();
        assert_eq!(token.kind, kind);
        assert_eq!(token.text, input);
    }

    #[test]
    fn test_lex_line_comment() {
        // Normal line comments
        check("#", SyntaxKind::Comment);
        check("#abc", SyntaxKind::Comment);
        check("# abc 123", SyntaxKind::Comment);
        check("# This is a random line comment", SyntaxKind::Comment);

        // Documentation comments
        check("##", SyntaxKind::DocComment);
        check("##abc", SyntaxKind::DocComment);
        check("## abc 123", SyntaxKind::DocComment);
        check("## This is a random line comment", SyntaxKind::DocComment);

        // Module comments
        check("#!", SyntaxKind::DocComment);
        check("#!abc", SyntaxKind::DocComment);
        check("#! abc 123", SyntaxKind::DocComment);
        check("#! This is a random line comment", SyntaxKind::DocComment);
    }

    #[test]
    fn test_lex_keywords() {
        check("and", SyntaxKind::Kwd_And);
        check("as", SyntaxKind::Kwd_As);
        check("case", SyntaxKind::Kwd_Case);
        check("else", SyntaxKind::Kwd_Else);
        check("enum", SyntaxKind::Kwd_Enum);
        check("for", SyntaxKind::Kwd_For);
        check("forall", SyntaxKind::Kwd_Forall);
        check("func", SyntaxKind::Kwd_Func);
        check("if", SyntaxKind::Kwd_If);
        check("impl", SyntaxKind::Kwd_Impl);
        check("import", SyntaxKind::Kwd_Import);
        check("in", SyntaxKind::Kwd_In);
        check("iter", SyntaxKind::Kwd_Iter);
        check("let", SyntaxKind::Kwd_Let);
        check("module", SyntaxKind::Kwd_Module);
        check("not", SyntaxKind::Kwd_Not);
        check("of", SyntaxKind::Kwd_Of);
        check("or", SyntaxKind::Kwd_Or);
        check("range", SyntaxKind::Kwd_Range);
        check("record", SyntaxKind::Kwd_Record);
        check("type", SyntaxKind::Kwd_Type);
        check("var", SyntaxKind::Kwd_Var);
        check("while", SyntaxKind::Kwd_While);
        check("with", SyntaxKind::Kwd_With);
        check("yield", SyntaxKind::Kwd_Yield);
    }

    #[test]
    fn test_lex_symbols() {
        check("&", SyntaxKind::Sym_Ampersand);
        check("*", SyntaxKind::Sym_Asterisk);
        check("@", SyntaxKind::Sym_At);
        check("\\", SyntaxKind::Sym_BackSlash);
        check("!", SyntaxKind::Sym_Bang);
        check("^", SyntaxKind::Sym_Caret);
        check(":", SyntaxKind::Sym_Colon);
        check(",", SyntaxKind::Sym_Comma);
        check("$", SyntaxKind::Sym_Dollar);
        check(".", SyntaxKind::Sym_Dot);
        check("—", SyntaxKind::Sym_EmDash);
        check("–", SyntaxKind::Sym_EnDash);
        check("=", SyntaxKind::Sym_Eq);
        check("/", SyntaxKind::Sym_ForwardSlash);
        check("-", SyntaxKind::Sym_Minus);
        check("%", SyntaxKind::Sym_Percent);
        check("|", SyntaxKind::Sym_Pipe);
        check("+", SyntaxKind::Sym_Plus);
        check("?", SyntaxKind::Sym_Question);
        check(";", SyntaxKind::Sym_Semicolon);
        check("£", SyntaxKind::Sym_Sterling);
        check("~", SyntaxKind::Sym_Tilde);

        check("<", SyntaxKind::Sym_Lt);
        check(">", SyntaxKind::Sym_Gt);
        check("<=", SyntaxKind::Sym_LtEq);
        check(">=", SyntaxKind::Sym_GtEq);
        check("<-", SyntaxKind::Sym_LThinArrow);
        check("->", SyntaxKind::Sym_RThinArrow);
        check("=>", SyntaxKind::Sym_ThickArrow);
        check(":=", SyntaxKind::Sym_Walrus);

        check("{", SyntaxKind::Sym_LBrace);
        check("}", SyntaxKind::Sym_RBrace);
        check("[", SyntaxKind::Sym_LBracket);
        check("]", SyntaxKind::Sym_RBracket);
        check("(", SyntaxKind::Sym_LParen);
        check(")", SyntaxKind::Sym_RParen);

        check("???", SyntaxKind::Placeholder);
    }

    #[test]
    fn test_lex_semantically_valid_literal_integers() {
        // Decimal integers
        check("0", SyntaxKind::Lit_Integer);
        check("0_", SyntaxKind::Lit_Integer);
        check("123", SyntaxKind::Lit_Integer);
        check("123_456", SyntaxKind::Lit_Integer);

        // Binary integers
        check("0b000", SyntaxKind::Lit_Integer);
        check("0b111", SyntaxKind::Lit_Integer);
        check("0b101", SyntaxKind::Lit_Integer);
        check("0b000_000", SyntaxKind::Lit_Integer);
        check("0b111_111", SyntaxKind::Lit_Integer);
        check("0b101_101", SyntaxKind::Lit_Integer);

        // Octal integers
        check("0o000", SyntaxKind::Lit_Integer);
        check("0o777", SyntaxKind::Lit_Integer);
        check("0o767", SyntaxKind::Lit_Integer);
        check("0o000_000", SyntaxKind::Lit_Integer);
        check("0o777_777", SyntaxKind::Lit_Integer);
        check("0o767_767", SyntaxKind::Lit_Integer);

        // Hexadecimal integers
        check("0x000", SyntaxKind::Lit_Integer);
        check("0xfff", SyntaxKind::Lit_Integer);
        check("0xfef", SyntaxKind::Lit_Integer);
        check("0x000_000", SyntaxKind::Lit_Integer);
        check("0xfff_fff", SyntaxKind::Lit_Integer);
        check("0xfef_fef", SyntaxKind::Lit_Integer);
    }

    #[test]
    fn test_lex_semantically_invalid_literal_integers() {
        // Decimal integers
        check("0z", SyntaxKind::Lit_Integer);
        check("1z2y3x", SyntaxKind::Lit_Integer);
        check("1z2y3x_4w5v6u", SyntaxKind::Lit_Integer);

        // Binary integers
        check("0b0z0y0x", SyntaxKind::Lit_Integer);
        check("0b1z1y1x", SyntaxKind::Lit_Integer);
        check("0b1z0y1x", SyntaxKind::Lit_Integer);
        check("0b0z0y0x_0w0v0u", SyntaxKind::Lit_Integer);
        check("0b1z1y1x_1w1v1u", SyntaxKind::Lit_Integer);
        check("0b1z0y1x_1w0v1u", SyntaxKind::Lit_Integer);

        // Octal integers
        check("0o0z0y0x", SyntaxKind::Lit_Integer);
        check("0o7z7y7x", SyntaxKind::Lit_Integer);
        check("0o7z6y7x", SyntaxKind::Lit_Integer);
        check("0o0z0y0x_0w0v0u", SyntaxKind::Lit_Integer);
        check("0o7z7y7x_7w7v7u", SyntaxKind::Lit_Integer);
        check("0o7z6y7x_7w6v7u", SyntaxKind::Lit_Integer);

        // Hexadecimal integers
        check("0x0z0y0x", SyntaxKind::Lit_Integer);
        check("0xfzfyfx", SyntaxKind::Lit_Integer);
        check("0xfzeyfx", SyntaxKind::Lit_Integer);
        check("0x0z0y0x_0w0v0u", SyntaxKind::Lit_Integer);
        check("0xfzfyfx_fwfvfu", SyntaxKind::Lit_Integer);
        check("0xfzeyfx_fwevfu", SyntaxKind::Lit_Integer);
    }

    #[test]
    fn test_lex_semantically_valid_literal_floats() {
        check("0.", SyntaxKind::Lit_Float);
        check("0.0", SyntaxKind::Lit_Float);
        check("0_.", SyntaxKind::Lit_Float);
        check("0_.0_", SyntaxKind::Lit_Float);
        check("000.000", SyntaxKind::Lit_Float);
        check("1.23456", SyntaxKind::Lit_Float);
        check("12345.6", SyntaxKind::Lit_Float);
        check("123.456", SyntaxKind::Lit_Float);
    }

    #[test]
    fn test_lex_semantically_invalid_literal_floats() {
        check("0a0b0c.0d0e", SyntaxKind::Lit_Float);
        check("1a.2b3c4d5e6", SyntaxKind::Lit_Float);
        check("1a2b.3c4d5e6", SyntaxKind::Lit_Float);
        check("1a2b3c.4d5e6", SyntaxKind::Lit_Float);
        check("1a2b3c4d.5e6", SyntaxKind::Lit_Float);
        check("1a2b3c4d5e.6", SyntaxKind::Lit_Float);
    }

    #[test]
    fn test_lex_identifiers() {
        check("_", SyntaxKind::ReservedIdentifier);
        check("_a", SyntaxKind::Identifier);

        check("a", SyntaxKind::Identifier);
        check("abc", SyntaxKind::Identifier);
        check("abc123", SyntaxKind::Identifier);
        check("abc123_abc", SyntaxKind::Identifier);
        check("abc123_abc123", SyntaxKind::Identifier);
    }

    #[test]
    fn test_lex_identifiers_unicode() {
        // Latin-extended
        check("åçéîñøœßü", SyntaxKind::Identifier);
        check("njerëzore", SyntaxKind::Identifier);
        check("čovjek", SyntaxKind::Identifier);
        check("člověk", SyntaxKind::Identifier);

        // Other scripts
        check("بشري", SyntaxKind::Identifier); // Arabic
        check("ሰው", SyntaxKind::Identifier); // Amharic
        check("մարդ", SyntaxKind::Identifier); // Armenian
        check("মানব", SyntaxKind::Identifier); // Bengali
        check("人的", SyntaxKind::Identifier); // Chinese
        check("человек", SyntaxKind::Identifier); // Cyrillic
        check("मानव", SyntaxKind::Identifier); // Devanagari
        check("ადამიანური", SyntaxKind::Identifier); // Gregorian
        check("άνθρωπος", SyntaxKind::Identifier); // Greek
        check("માનવ", SyntaxKind::Identifier); // Gujarati
        check("אנוש", SyntaxKind::Identifier); // Hebrew
        check("ヒューマン", SyntaxKind::Identifier); // Japanese (Katakana)
        check("ಮಾನವ", SyntaxKind::Identifier); // Kannada
        check("មនុស្ស", SyntaxKind::Identifier); // Khmer
        check("인간", SyntaxKind::Identifier); // Korean
        check("ມະນຸດ", SyntaxKind::Identifier); // Lao
        check("മനുഷ്യൻ", SyntaxKind::Identifier); // Malayalam
        check("လူ့", SyntaxKind::Identifier); // Myanmar
        check("ମାନବ", SyntaxKind::Identifier); // Odia
        check("มนุษย์", SyntaxKind::Identifier); // Thai
    }
}
