pub mod cache;
pub mod errors;
pub mod lexer;
pub mod parser;
pub mod source;
pub mod tree;

use lexer::Lexer;
use parser::{Parser, _ParserOut};

pub fn parse(source: String) -> _ParserOut {
    let lexer = Lexer::with(source);
    Parser::with(lexer).parse()
}
