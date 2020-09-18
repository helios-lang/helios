use crate::lexer::Lexer;

pub type ParserOut = ();

#[allow(dead_code)]
pub struct Parser {
    lexer: Lexer,
    peeked_token: Option<()>,
}

impl Parser {
    pub fn with(lexer: Lexer) -> Self {
        Self { lexer, peeked_token: None }
    }

    pub fn parse(&mut self) -> ParserOut {
        // let mut nodes = Vec::new();

        while !self.lexer.is_at_end() {
            // nodes.push(self.parse_program());
        }

        // SyntaxTree(nodes)
        ()
    }
}
