#![allow(dead_code)]

use crate::tree::token::*;
use std::collections::HashMap;
use std::rc::Rc;

struct Cache<T>(HashMap<(), Rc<T>>);

impl<T> Cache<T> {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    /// _Implementation plan:_
    /// Lookup item and, if found, return a ref-counted value of it, otherwise
    /// add it to the cache before returning it.
    pub fn lookup(&mut self, _value: T) -> T {
        unimplemented!("Cache::lookup")
    }
}

struct Lexer {
    token_cache: Cache<RawSyntaxToken>,
    trivia_cache: Cache<SyntaxTrivia>,
}

impl Lexer {
    pub fn new() -> Self {
        Self { token_cache: Cache::new(), trivia_cache: Cache::new() }
    }

    pub fn next_token(&mut self) -> SyntaxToken {
        unimplemented!()
    }
}
