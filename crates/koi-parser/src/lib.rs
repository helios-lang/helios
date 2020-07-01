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
use expr::{Expr, ExprLiteral};
use decl::Decl;
use lexer::{Lexer, LexerMode};
use token::*;

pub type Ast = Vec<AstNode>;

pub type Result<T> = std::result::Result<T, ParserError>;

#[derive(Debug)]
pub enum AstNode {
    Expr(Expr),
    Decl(Decl),
    Eof,
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

        while self.peek() != None {
            match self.program() {
                Ok(node) => nodes.push(node),
                Err(err) => eprintln!("Parser error: {:?}", err)
            }
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

    fn check_all(&mut self, kinds: &[TokenKind]) -> bool {
        for kind in kinds {
            if self.check(kind.clone()) {
                return true;
            }
        }

        false
    }
}

impl Parser {
    fn program(&mut self) -> Result<AstNode> {
        if self.check(TokenKind::Newline) {
            self.next_token();
            if let None = self.peek() {
                return Ok(AstNode::Eof);
            }
        }

        self.top_level()
    }

    fn top_level(&mut self) -> Result<AstNode> {
        // if token.kind == TokenKind::Keyword(Keyword::Def) {
        //     return Ok(AstNode::Decl(self.function_declaration()?));
        // }

        // if token.kind == TokenKind::Keyword(Keyword::Module) {
        //     return Ok(AstNode::Decl(self.module_declaration()?));
        // }

        // if token.kind == TokenKind::Keyword(Keyword::Type) {
        //     return Ok(AstNode::Decl(self.type_declaration()?));
        // }

        // if token.kind == TokenKind::Keyword(Keyword::Using) {
        //     return Ok(AstNode::Decl(self.using_declaration()?));
        // }

        Ok(AstNode::Expr(self.expression()?))
    }

    // fn function_declaration(&mut self) -> Result<Decl> {
    //     unimplemented!("function-declaration")
    // }

    // fn module_declaration(&mut self) -> Result<Decl> {
    //     unimplemented!("module-declaration")
    // }

    // fn type_declaration(&mut self) -> Result<Decl> {
    //     unimplemented!("type-declaration")
    // }

    // fn using_declaration(&mut self) -> Result<Decl> {
    //     unimplemented!("using-declaration")
    // }

    fn expression(&mut self) -> Result<Expr> {
        // if token.kind == TokenKind::Keyword(Keyword::If) {
        //     return Ok(self.if_expression()?);
        // }

        Ok(self.equality_expression()?)
    }

    fn equality_expression(&mut self) -> Result<Expr> {
        let mut lhs = self.comparison_expression()?;

        while self.check_all(&[
            TokenKind::Symbol(Symbol::BangEq),
            TokenKind::Symbol(Symbol::Eq),
        ]) {
            let operator = self.next_token().expect("Unwrap equality expression");
            let rhs = self.comparison_expression()?;
            lhs = Expr::Binary(operator, Box::new(lhs), Box::new(rhs))
        }

        Ok(lhs)
    }

    fn comparison_expression(&mut self) -> Result<Expr> {
        let mut lhs = self.additive_expression()?;

        while self.check_all(&[
            TokenKind::Symbol(Symbol::Lt),
            TokenKind::Symbol(Symbol::LtEq),
            TokenKind::Symbol(Symbol::Gt),
            TokenKind::Symbol(Symbol::GtEq),
        ]) {
            let operator = self.next_token().expect("Unwrap comparison expression");
            let rhs = self.additive_expression()?;
            lhs = Expr::Binary(operator, Box::new(lhs), Box::new(rhs))
        }

        Ok(lhs)
    }

    fn additive_expression(&mut self) -> Result<Expr> {
        let mut lhs = self.multiplicative_expression()?;

        while self.check_all(&[
            TokenKind::Symbol(Symbol::Plus),
            TokenKind::Symbol(Symbol::Minus),
        ]) {
            let operator = self.next_token().expect("Unwrap comparison expression");
            let rhs = self.multiplicative_expression()?;
            lhs = Expr::Binary(operator, Box::new(lhs), Box::new(rhs))
        }

        Ok(lhs)
    }

    fn multiplicative_expression(&mut self) -> Result<Expr> {
        let mut lhs = self.unary_expression()?;

        while self.check_all(&[
            TokenKind::Symbol(Symbol::Asterisk),
            TokenKind::Symbol(Symbol::ForwardSlash),
        ]) {
            let operator = self.next_token().expect("Unwrap comparison expression");
            let rhs = self.unary_expression()?;
            lhs = Expr::Binary(operator, Box::new(lhs), Box::new(rhs))
        }

        Ok(lhs)
    }

    fn unary_expression(&mut self) -> Result<Expr> {
        while self.check_all(&[
            TokenKind::Symbol(Symbol::Bang),
            TokenKind::Symbol(Symbol::Minus),
        ]) {
            let operator = self.next_token().expect("Unwrap comparison expression");
            let rhs = self.additive_expression()?;
            return Ok(Expr::Unary(operator, Box::new(rhs)))
        }

        Ok(self.primary()?)
    }

    fn primary(&mut self) -> Result<Expr> {
        match self.next_token() {
            None => Ok(Expr::Missing),
            Some(token) => match token.kind.clone() {
                TokenKind::Keyword(Keyword::False) => {
                    Ok(Expr::Literal(ExprLiteral::Bool(false)))
                },
                TokenKind::Keyword(Keyword::True) => {
                    Ok(Expr::Literal(ExprLiteral::Bool(true)))
                },
                TokenKind::Literal(literal) => match literal {
                    Literal::Int { value, .. } => {
                        Ok(Expr::Literal(ExprLiteral::Int(value)))
                    },
                    Literal::Float { value, .. } => {
                        Ok(Expr::Literal(ExprLiteral::Float(value)))
                    },
                    l => unimplemented!("Literal {:?}", l)
                },
                k => unimplemented!("Kind {:?}", k)
            }
        }
    }
}

pub fn parse(source: Source, should_consume_doc_comments: bool) -> Ast {
    let lexer = Lexer::with(source, should_consume_doc_comments);
    Parser::with(lexer).parse()
}
