use crate::source::{Cursor, Source};
use crate::token::Token;

pub struct Lexer {
    cursor: Cursor,
}

impl Lexer {
    pub fn with(source: Source) -> Self {
        Self { cursor: Cursor::with(source) }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        todo!()
    }
}
