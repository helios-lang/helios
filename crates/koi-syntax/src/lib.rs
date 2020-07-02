#![allow(dead_code)]

mod decl;
mod expr;
mod lexer;
mod parser;
mod source;
mod token;

use crate::lexer::Lexer;
use crate::parser::{Ast, Parser};
use crate::source::Source;

pub fn parse(source: Source) -> Ast {
    let lexer = Lexer::with(source);
    Parser::with(lexer).parse()
}
