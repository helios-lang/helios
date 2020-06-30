#![allow(dead_code)]
#![allow(unused_imports)]

pub mod decl;
pub mod expr;
pub mod lexer;
pub mod token;
pub mod source;

#[cfg(test)]
mod tests;

use crate::source::Source;
use expr::Expr;
use decl::Decl;
use lexer::{Lexer, LexerMode};
use token::{Token, TokenKind};

pub type Ast = Vec<AstNode>;

pub type Result<T> = std::result::Result<T, ParserError>;

#[derive(Debug)]
pub enum AstNode {
    Expr(Expr),
    Decl(Decl),
}

pub type ParserError = ();

pub struct Parser {
    lexer: Lexer,
    peeked_token: Option<Token>,
}

impl Parser {
    pub fn with(lexer: Lexer) -> Self {
        Self { lexer, peeked_token: None }
    }

    pub fn parse(&mut self) -> Ast {
        let mut nodes = Ast::new();

        while let Some(token) = self.lexer.next_token() {
            match self.program(token) {
                Ok(node) => nodes.push(node),
                Err(err) => eprintln!("Parser error: {:?}", err)
            }
        }

        nodes
    }

    fn peek(&mut self) -> Option<Token> {
        if self.peeked_token.is_none() {
            self.peeked_token = self.lexer.next_token();
        }

        self.peeked_token.clone()
    }

    fn next_token(&mut self) -> Option<Token> {
        match self.peeked_token.take() {
            Some(token) => Some(token),
            None => Some(self.lexer.next_token()?)
        }
    }
}

impl Parser {
    fn program(&mut self, token: Token) -> Result<AstNode> {
        self.top_level(token)
    }

    fn top_level(&mut self, token: Token) -> Result<AstNode> {
        Ok(AstNode::Expr(self.expression(token)?))
    }

    // fn declaration(&mut self, token: Token) -> Result<Decl> {
    //     unimplemented!()
    // }

    fn expression(&mut self, token: Token) -> Result<Expr> {
        use token::Keyword;
        match token.kind {
            TokenKind::Keyword(Keyword::If) => unimplemented!("if-expression"),
            TokenKind::Keyword(Keyword::Let) => unimplemented!("let-expression"),
            TokenKind::Keyword(Keyword::Match) => unimplemented!("match-expression"),
            _ => self.equality(token)
        }
    }

    fn equality(&mut self, _token: Token) -> Result<Expr> {
        unimplemented!("equality-expression")
    }
}

pub fn parse(source: Source, should_consume_doc_comments: bool) -> Ast {
    let lexer = Lexer::with(source, should_consume_doc_comments);
    Parser::with(lexer).parse()
}
