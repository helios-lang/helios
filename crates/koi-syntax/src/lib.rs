pub mod cache;
pub mod errors;
pub mod lexer;
pub mod parser;
pub mod source;
pub mod tree;

use lexer::Lexer;
use parser::{Parser, ParserOut};

pub fn parse(source: String) -> ParserOut {
    let lexer = Lexer::with(source);
    Parser::with(lexer).parse()
}
