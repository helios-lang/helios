use crate::source::{Cursor, Source};
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
}

pub struct Lexer {
    cursor: Cursor,
    mode_stack: Vec<LexerMode>,
}

impl Default for LexerMode {
    fn default() -> Self {
        Self::Normal
    }
}

impl Lexer {
    pub fn with(source: Source) -> Self {
        Self {
            cursor: Cursor::with(source),
            mode_stack: vec![LexerMode::Normal],
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        #[allow(unreachable_patterns)]
        match self.current_mode() {
            LexerMode::Normal => self.tokenize_normal(),
            _ => unimplemented!(),
        }
    }
}

impl Lexer {
    fn current_mode(&self) -> LexerMode {
        self.mode_stack.last().cloned().unwrap_or_default()
    }

    fn tokenize_normal(&mut self) -> Option<Token> {
        todo!("Lexer::tokenize_normal")
    }
}
