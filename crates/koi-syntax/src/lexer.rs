#![allow(dead_code)]

use crate::source::{Cursor, TextSpan};
use crate::tree::token::*;
use std::collections::HashMap;
use std::fmt::Debug;
use unicode_xid::UnicodeXID;

struct Cache<T>(HashMap<String, T>);

impl<T> Cache<T> {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    /// _Implementation plan:_
    /// Lookup item and, if found, return a ref-counted value of it, otherwise
    /// add it to the cache before returning it.
    pub fn lookup<F, S>(&mut self, key: S, create_value: F) -> &T
    where
        F: Fn(String) -> T,
        S: Into<String>,
        T: Debug,
    {
        let key = key.into();
        let value = self.0.entry(key.clone()).or_insert_with(|| create_value(key));
        value
    }
}

struct Lexer {
    cursor: Cursor,
    pub(crate) token_cache: Cache<RawSyntaxToken>,
    pub(crate) trivia_cache: Cache<SyntaxTrivia>,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Self {
            cursor: Cursor::new(source),
            token_cache: Cache::new(),
            trivia_cache: Cache::new(),
        }
    }

    pub fn next_token(&mut self) -> SyntaxToken {
        let start = self.current_pos();
        let _leading_trivia = self.consume_trivia(true);

        let next_char = match self.next_char() {
            Some(next_char) => next_char,
            None => {
                return SyntaxToken::new(
                    self.token_cache.lookup("\0", |_| RawSyntaxToken::new(TokenKind::Eof)),
                    TextSpan::new(start, 0),
                )
            }
        };

        let raw = match next_char {
            c if is_identifier_start(c) => self.lex_identifier(c),
            _ => unimplemented!()
        };

        // Error: Cannot borrow `*self` as mutable more than once at a time
        // let _trailing_trivia = self.consume_trivia(false);

        SyntaxToken::new(raw.0, TextSpan::new(start, raw.1))
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
    fn is_at_end(&self) -> bool {
        self.cursor.source_len() == 0
    }

    fn current_pos(&self) -> usize {
        self.cursor.pos
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

    fn consume_trivia(&mut self, is_leading: bool) -> Vec<SyntaxTrivia> {
        let mut trivia = Vec::new();

        if is_leading {
            let count = self.consume_while(|c| c == '\n');
            if count > 0 { trivia.push(SyntaxTrivia::LineFeed(count)) }
        }

        let count = self.consume_while(|c| c == ' ');
        if count > 0 { trivia.push(SyntaxTrivia::Space(count)) }

        let count = self.consume_while(|c| c == '\t');
        if count > 0 { trivia.push(SyntaxTrivia::Tab(count)) }

        trivia
    }
}

impl Lexer {
    fn lex_identifier(&mut self, first_char: char) -> (&RawSyntaxToken, usize) {
        let rest = self.consume_build(is_identifier_continue);
        let vec = [&vec![first_char], &rest[..]].concat();
        let count = vec.len();
        let string: String = vec.into_iter().collect();
        (self.token_cache.lookup(string, RawSyntaxTokenBuilder::keyword_or_identifier), count)
    }
}

struct RawSyntaxTokenBuilder;

impl RawSyntaxTokenBuilder {
    /// Attempts to match the provided `string` to a keyword, returning a
    /// `TokenKind::Keyword` if a match is found, otherwise a
    /// `TokenKind::Identifier`.
    fn keyword_or_identifier(string: String) -> RawSyntaxToken {
        let kind = match &*string {
            "and"       => TokenKind::Keyword(Keyword::And),
            "case"      => TokenKind::Keyword(Keyword::Case),
            "def"       => TokenKind::Keyword(Keyword::Def),
            "else"      => TokenKind::Keyword(Keyword::Else),
            "enum"      => TokenKind::Keyword(Keyword::Enum),
            "if"        => TokenKind::Keyword(Keyword::If),
            "interal"   => TokenKind::Keyword(Keyword::Internal),
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
            "type"      => TokenKind::Keyword(Keyword::Type),
            "using"     => TokenKind::Keyword(Keyword::Using),
            "val"       => TokenKind::Keyword(Keyword::Val),
            "with"      => TokenKind::Keyword(Keyword::With),
            _           => TokenKind::Identifier
        };

        RawSyntaxToken::new(kind)
    }

    fn number(_string: String) -> RawSyntaxToken {
        todo!("Lexer::lex_number")
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer() {
        let mut lexer = Lexer::new("struct let let let struct if".to_string());

        loop {
            let now = std::time::Instant::now();
            match lexer.next_token() {
                token if token.kind() == TokenKind::Eof => break,
                token => {
                    println!("{}µs\t| {:?}", now.elapsed().as_micros(), token)
                }
            }
        }

        println!("Cached tokens: {}", lexer.token_cache.0.len());
        println!("Cache: {:#?}", lexer.token_cache.0);
        println!("Done.");
    }
}
