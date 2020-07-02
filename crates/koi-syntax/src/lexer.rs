use crate::source::Cursor;
use crate::token::Token;

pub struct Lexer {
    cursor: Cursor,
}

impl Lexer {
    pub fn with(cursor: Cursor) -> Self {
        Self { cursor }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        todo!()
    }
}
