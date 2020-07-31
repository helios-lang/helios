#![allow(dead_code)]

use crate::lexer::Lexer;

pub type _ParserOut = ();

pub struct Parser {
    lexer: Lexer,
}

impl Parser {
    pub fn with(lexer: Lexer) -> Self {
        Self { lexer, }
    }

    pub fn parse(&mut self) -> _ParserOut {
        todo!("Parser::parse")
    }
}
