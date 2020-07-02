use crate::lexer::Lexer;
use crate::token::*;

pub struct Parser {
    lexer: Lexer,
    peeked_token: Option<Token>,
}
