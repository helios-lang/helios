use crate::decl::Decl;
use crate::expr::Expr;
use crate::lexer::Lexer;
use crate::token::*;

pub type Ast = Vec<AstNode>;

pub enum AstNode {
    Expr(Expr),
    Decl(Decl),
}

pub struct Parser {
    lexer: Lexer,
    peeked_token: Option<Token>,
}

impl Parser {
    pub fn with(lexer: Lexer) -> Self {
        Self { lexer, peeked_token: None }
    }

    pub fn parse(&mut self) -> Ast {
        let mut nodes = Vec::new();

        while self.lexer.next_token() != None {
            nodes.push(self.program());
        }

        nodes
    }
}

impl Parser {
    fn program(&mut self) -> AstNode {
        todo!()
    }
}
