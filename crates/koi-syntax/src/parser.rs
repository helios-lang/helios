#![allow(dead_code)]

use crate::lexer::Lexer;

pub type ParserOut = ();

pub struct Parser {
    lexer: Lexer,
}

impl Parser {
    pub fn with(lexer: Lexer) -> Self {
        Self { lexer, }
    }

    pub fn parse(&mut self) -> ParserOut {
        todo!("Parser::parse")
    }
}
