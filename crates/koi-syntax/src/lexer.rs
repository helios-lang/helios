#![allow(dead_code)]

use crate::cache::Cache;
use crate::source::{Cursor, TextSpan};
use crate::tree::token::{self, RawSyntaxToken, SyntaxToken, TokenKind};
use std::default::Default;
use std::rc::Rc;

pub type _LexerOut = SyntaxToken;

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
    pub(crate) token_cache: Cache<(TokenKind, String), Rc<RawSyntaxToken>>,
}

impl Lexer {
    pub fn with(source: String) -> Self {
        Self {
            cursor: Cursor::with(source),
            mode_stack: vec![LexerMode::Normal],
            token_cache: Cache::new(),
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
        use token::*;

        let start = self.current_pos();
        let _next_char = match self.next_char() {
            Some(c) => c,
            None => return SyntaxToken::with(
                Rc::new(RawSyntaxToken::with(TokenKind::Eof, "\0".to_string())),
                TextSpan::new(start, 0)
            )
        };

        let key = (TokenKind::Keyword(Keyword::Let), "let".to_string());
        let raw = self.token_cache.lookup_with(key.clone(), || {
            Rc::new(RawSyntaxToken::with(key.0, key.1))
        }).clone();

        SyntaxToken::with(raw, TextSpan::new(start, self.current_pos()))
    }

    fn tokenize_grouping(&mut self) -> _LexerOut {
        todo!("Lexer::tokenize_grouping")
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_lexer() {
//         let source = "let x = 10";
//         let mut lexer = Lexer::with(source.to_string());
//         loop {
//             let token = lexer.next_token();
//             match token.kind() {
//                 TokenKind::Eof => break,
//                 kind => {
//                     println!("- {:?}@{}", kind, token.span());
//                     println!(": {}", lexer.token_cache.len());
//                 },
//             }
//         }
//     }
// }
