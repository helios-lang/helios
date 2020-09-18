use crate::lexer::Lexer;

pub type ParserOut = ();

#[allow(dead_code)]
pub struct Parser {
    lexer: Lexer,
    peeked_token: Option<()>,
}

impl Parser {
    pub fn with(lexer: Lexer) -> Self {
        Self {
            lexer,
            peeked_token: None,
        }
    }

    pub fn parse(&mut self) -> ParserOut {
        todo!("Parser::parse")
    }
}
