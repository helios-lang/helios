#![allow(dead_code)]

mod errors;
mod lexer;
pub mod node;
mod node_;
mod parser;
pub mod source;
pub mod token;

#[cfg(test)]
mod tests;

use crate::lexer::Lexer;
pub use crate::parser::Ast;
use crate::parser::Parser;
use crate::source::Source;

pub fn parse(source: Source) -> Ast {
    let lexer = Lexer::with(source);
    Parser::with(lexer).parse()
}
