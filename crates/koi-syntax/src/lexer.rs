#![allow(dead_code)]

use crate::source::Cursor;
use std::default::Default;

pub type _LexerOut = ();

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
    mode_stack: Vec<LexerMode>,
}

impl Lexer {
    pub fn with(source: String) -> Self {
        Self {
            cursor: Cursor::with(source),
            mode_stack: vec![LexerMode::Normal],
        }
    }

    pub fn next_token(&mut self) -> _LexerOut {
        match self.current_mode() {
            LexerMode::Normal => self.tokenize_normal(),
            LexerMode::Grouping => self.tokenize_grouping(),
        }
    }

    pub(crate) fn push_mode(&mut self, mode: LexerMode) {
        self.mode_stack.push(mode);
    }

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
        self.cursor.source_len() == 0 //&& self.did_emit_end_token
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
}

impl Lexer {
    fn tokenize_normal(&mut self) -> _LexerOut {
        todo!("Lexer::tokenize_normal")
    }

    fn tokenize_grouping(&mut self) -> _LexerOut {
        todo!("Lexer::tokenize_grouping")
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_lexer() {
//         let source = "let x = 10";
//         let mut lexer = Lexer::with(source.to_string());
//     }
// }
