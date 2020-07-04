use crate::decl::Decl;
use crate::expr::{self, Expr, ExprLiteral};
use crate::lexer::Lexer;
use crate::source::Span;
use crate::token::*;

pub type Ast = Vec<AstNode>;

#[derive(Debug)]
pub enum AstNode {
    Expr(Expr),
    Decl(Decl),
    Eof,
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

        while !self.lexer.is_at_end() {
            nodes.push(self.parse_program());
        }

        nodes
    }
}

impl Parser {
    fn peek(&mut self) -> Option<Token> {
        if self.peeked_token.is_none() {
            self.peeked_token = Some(self.lexer.next_token());
        }

        self.peeked_token.clone()
    }

    fn next_token(&mut self) -> Token {
        match self.peeked_token.take() {
            Some(token) => token,
            None => self.lexer.next_token()
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

    fn consume(&mut self, kind: TokenKind) -> Token {
        if let Some(token) = self.peek() {
            if token.kind == kind {
                return self.next_token();
            } else {
                return Token::with(
                    TokenKind::Missing(Box::new(kind)),
                    Span::new(token.span.start, token.span.start)
                );
            }
        }

        panic!("Unhandled case: consuming when peeking has None value");
    }

    fn try_consume(&mut self, kind: TokenKind) -> bool {
        if self.check(kind) {
            self.next_token();
            true
        } else {
            false
        }
    }
}

impl Parser {
    fn parse_program(&mut self) -> AstNode {
        self.try_consume(TokenKind::Newline);

        // TODO: This may never be called
        if self.try_consume(TokenKind::Eof) {
            return AstNode::Eof;
        }

        AstNode::Expr(self.parse_expression())
    }

    fn parse_expression(&mut self) -> Expr {
        if self.try_consume(TokenKind::Newline) {
            return Expr::Unexpected(TokenKind::Newline);
        }

        if self.try_consume(TokenKind::Eof) {
            return Expr::Missing;
        }

        if self.try_consume(TokenKind::Keyword(Keyword::Let)) {
            return self.parse_let_expression();
        }

        if self.try_consume(TokenKind::Keyword(Keyword::If)) {
            return self.parse_if_expression();
        }

        self.parse_equality_expression()
    }

    fn parse_let_expression(&mut self) -> Expr {
        let mut local_binding = expr::LocalBinding::new();

        local_binding
            .identifier(self.consume(TokenKind::Identifier))
            .equal_symbol(self.consume(TokenKind::Symbol(Symbol::Eq)))
            .expression(self.parse_expression_block());

        Expr::LocalBindingExpr(local_binding)
    }

    fn parse_if_expression(&mut self) -> Expr {
        let mut if_expression = expr::IfExpr::new();

        if_expression.pattern = Some(Box::new(self.parse_expression()));
        if_expression.then_keyword = Some(self.consume(TokenKind::Keyword(Keyword::Then)));
        if_expression.expression = Some(Box::new(self.parse_expression_block()));

        self.try_consume(TokenKind::Newline);
        if self.try_consume(TokenKind::Keyword(Keyword::Else)) {
            if_expression.else_clause = Some(Box::new(self.parse_else_clause()));
        }

        Expr::IfExpr(if_expression)
    }

    fn parse_else_clause(&mut self) -> Expr {
        if self.try_consume(TokenKind::Keyword(Keyword::If)) {
            self.parse_if_expression()
        } else {
            self.parse_expression_block()
        }
    }

    fn parse_expression_block(&mut self) -> Expr {
        if self.try_consume(TokenKind::Begin) {
            return self.parse_expression_block_list();
        }

        self.parse_expression()
    }

    fn parse_expression_block_list(&mut self) -> Expr {
        let mut expressions = Vec::new();
        expressions.push(Box::new(self.parse_expression()));

        while self.check_all(&[
            TokenKind::Newline,
            TokenKind::Symbol(Symbol::Semicolon),
        ]) {
            self.next_token();
            expressions.push(Box::new(self.parse_expression()));
        }

        self.consume(TokenKind::End);

        Expr::ExprBlock(expressions)
    }

    fn parse_equality_expression(&mut self) -> Expr {
        let mut lhs = self.parse_comparison_expression();

        while self.check_all(&[
            TokenKind::Symbol(Symbol::BangEq),
            TokenKind::Symbol(Symbol::Eq),
        ]) {
            let operator = self.next_token();
            let rhs = self.parse_comparison_expression();
            lhs = Expr::Binary(operator, Box::new(lhs), Box::new(rhs))
        }

        lhs
    }

    fn parse_comparison_expression(&mut self) -> Expr {
        let mut lhs = self.parse_additive_expression();

        while self.check_all(&[
            TokenKind::Symbol(Symbol::Lt),
            TokenKind::Symbol(Symbol::LtEq),
            TokenKind::Symbol(Symbol::Gt),
            TokenKind::Symbol(Symbol::GtEq),
        ]) {
            let operator = self.next_token();
            let rhs = self.parse_additive_expression();
            lhs = Expr::Binary(operator, Box::new(lhs), Box::new(rhs))
        }

        lhs
    }

    fn parse_additive_expression(&mut self) -> Expr {
        let mut lhs = self.parse_multiplicative_expression();

        while self.check_all(&[
            TokenKind::Symbol(Symbol::Plus),
            TokenKind::Symbol(Symbol::Minus),
        ]) {
            let operator = self.next_token();
            let rhs = self.parse_multiplicative_expression();
            lhs = Expr::Binary(operator, Box::new(lhs), Box::new(rhs))
        }

        lhs
    }

    fn parse_multiplicative_expression(&mut self) -> Expr {
        let mut lhs = self.parse_unary_expression();

        while self.check_all(&[
            TokenKind::Symbol(Symbol::Asterisk),
            TokenKind::Symbol(Symbol::ForwardSlash),
        ]) {
            let operator = self.next_token();
            let rhs = self.parse_unary_expression();
            lhs = Expr::Binary(operator, Box::new(lhs), Box::new(rhs))
        }

        lhs
    }

    fn parse_unary_expression(&mut self) -> Expr {
        while self.check_all(&[
            TokenKind::Symbol(Symbol::Bang),
            TokenKind::Symbol(Symbol::Minus),
        ]) {
            let operator = self.next_token();
            let rhs = self.parse_additive_expression();
            return Expr::Unary(operator, Box::new(rhs))
        }

        self.parse_primary()
    }

    fn parse_primary(&mut self) -> Expr {
        match self.next_token().kind {
            TokenKind::Identifier => {
                Expr::Identifier
            },
            TokenKind::Keyword(Keyword::False) => {
                Expr::Literal(ExprLiteral::Bool(false))
            },
            TokenKind::Keyword(Keyword::True) => {
                Expr::Literal(ExprLiteral::Bool(true))
            },
            TokenKind::Literal(literal) => match literal {
                Literal::Integer(base) => {
                    Expr::Literal(ExprLiteral::Integer(base))
                },
                Literal::Float(base) => {
                    Expr::Literal(ExprLiteral::Float(base))
                },
                l => unimplemented!("Literal {:?}", l)
            },
            k => unimplemented!("TokenKind {:?}", k)
        }
    }
}
