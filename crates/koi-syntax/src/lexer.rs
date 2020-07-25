#![allow(dead_code)]

use crate::tree::token::*;
use std::collections::HashMap;
use std::rc::Rc;

struct Lexer {
    token_cache: HashMap<(), Rc<SyntaxToken>>,
    trivia_cache: HashMap<(), Rc<SyntaxTrivia>>,
}

impl Lexer {
    pub fn new() -> Self {
        unimplemented!()
    }

    pub fn next_token(&mut self) -> SyntaxToken {
        unimplemented!()
    }
}
