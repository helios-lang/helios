use crate::{Ast, AstNode};
use crate::expr::Expr;
use crate::lexer::Lexer;
use crate::token::*;

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

        while self.peek() != None {
            nodes.push(AstNode::Expr(self.expression()))
        }

        nodes
    }
}

impl Parser {
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

    fn check(&mut self, kind: TokenKind) -> bool {
        match self.peek() {
            Some(token) => token.kind == kind,
            None => false
        }
    }

    fn expect(&mut self, kind: TokenKind) -> bool {
        if self.check(kind) {
            self.next_token();
            true
        } else {
            false
        }
    }
}

impl Parser {
    fn expression(&mut self) -> Expr {
        if self.expect(TokenKind::Keyword(Keyword::Let)) {
            return self.let_expression();
        }

        self.equality_expression()
    }

    fn equality_expression(&mut self) -> Expr {
        todo!()
    }

    fn let_expression(&mut self) -> Expr {
        let identifier = match self.next_token() {
            Some(token) => match token.kind {
                TokenKind::Identifier(i) => Expr::Identifier(i),
                t => panic!("Unexpected {:?}", t)
            },
            None => panic!("Unexpected EOF")
        };

        println!(">>> {:?}", identifier);
        todo!()
    }
}
