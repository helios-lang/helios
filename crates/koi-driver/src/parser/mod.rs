pub mod lexer;
pub mod token;

#[cfg(test)]
mod tests;

use crate::source::Source;
use lexer::Lexer;
use token::Token;

pub type Ast = Vec<Token>;

pub struct Parser<'a> {
    lexer: Lexer<'a>,
}

impl<'a> Parser<'a> {
    pub fn with(lexer: Lexer<'a>) -> Self {
        Self { lexer }
    }

    pub fn parse(&mut self) -> Ast {
        let mut tokens = Ast::new();

        while let Some(token) = self.lexer.next_token() {
            tokens.push(token)
        }

        tokens
    }
}

pub fn parse<'a>(source: Source<'a>) -> Ast {
    let lexer = Lexer::with(source);
    Parser::with(lexer).parse()
}
