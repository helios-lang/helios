use crate::decl::Decl;
use crate::expr::{Expr, ExprLiteral};
use crate::lexer::{Lexer, LexerMode};
use crate::token::*;

pub type Ast = Vec<AstNode>;

#[derive(Debug)]
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

    fn expect(&mut self, kind: TokenKind) -> bool {
        if self.check(kind) {
            self.next_token();
            true
        } else {
            false
        }
    }

    fn consume<S: Into<String>>(&mut self, kind: TokenKind, message: S) -> Token {
        if self.check(kind) {
            self.next_token().unwrap()
        } else {
            panic!("Error: {}", message.into())
        }
    }
}

impl Parser {
    fn program(&mut self) -> AstNode {
        AstNode::Expr(self.expression())
    }

    fn expression(&mut self) -> Expr {
        if self.expect(TokenKind::Keyword(Keyword::Let)) {
            return self.let_expression();
        }

        self.equality_expression()
    }

    fn let_expression(&mut self) -> Expr {
        let identifier = match self.next_token() {
            Some(token) => match token.kind {
                TokenKind::Identifier(s) => Some(s),
                k => panic!("Unexpected {:?}", k),
            },
            None => panic!("Unexpected EOF")
        };

        self.consume(TokenKind::Symbol(Symbol::Eq), "Expected `=` after binding name");
        let rhs = self.expression();

        Expr::LocalBinding(identifier, Box::new(rhs))
    }

    fn equality_expression(&mut self) -> Expr {
        let mut lhs = self.comparison_expression();

        while self.check_all(&[
            TokenKind::Symbol(Symbol::BangEq),
            TokenKind::Symbol(Symbol::Eq),
        ]) {
            let operator = self.next_token().unwrap();
            let rhs = self.comparison_expression();
            lhs = Expr::Binary(operator, Box::new(lhs), Box::new(rhs))
        }

        lhs
    }

    fn comparison_expression(&mut self) -> Expr {
        let mut lhs = self.additive_expression();

        while self.check_all(&[
            TokenKind::Symbol(Symbol::Lt),
            TokenKind::Symbol(Symbol::LtEq),
            TokenKind::Symbol(Symbol::Gt),
            TokenKind::Symbol(Symbol::GtEq),
        ]) {
            let operator = self.next_token().unwrap();
            let rhs = self.additive_expression();
            lhs = Expr::Binary(operator, Box::new(lhs), Box::new(rhs))
        }

        lhs
    }

    fn additive_expression(&mut self) -> Expr {
        let mut lhs = self.multiplicative_expression();

        while self.check_all(&[
            TokenKind::Symbol(Symbol::Plus),
            TokenKind::Symbol(Symbol::Minus),
        ]) {
            let operator = self.next_token().unwrap();
            let rhs = self.multiplicative_expression();
            lhs = Expr::Binary(operator, Box::new(lhs), Box::new(rhs))
        }

        lhs
    }

    fn multiplicative_expression(&mut self) -> Expr {
        let mut lhs = self.unary_expression();

        while self.check_all(&[
            TokenKind::Symbol(Symbol::Asterisk),
            TokenKind::Symbol(Symbol::ForwardSlash),
        ]) {
            let operator = self.next_token().unwrap();
            let rhs = self.unary_expression();
            lhs = Expr::Binary(operator, Box::new(lhs), Box::new(rhs))
        }

        lhs
    }

    fn unary_expression(&mut self) -> Expr {
        while self.check_all(&[
            TokenKind::Symbol(Symbol::Bang),
            TokenKind::Symbol(Symbol::Minus),
        ]) {
            let operator = self.next_token().unwrap();
            let rhs = self.additive_expression();
            return Expr::Unary(operator, Box::new(rhs))
        }

        self.primary()
    }

    fn primary(&mut self) -> Expr {
        if let Some(token) = self.next_token() {
            match token.kind {
                TokenKind::Keyword(Keyword::False) => {
                    Expr::Literal(ExprLiteral::Bool(false))
                },
                TokenKind::Keyword(Keyword::True) => {
                    Expr::Literal(ExprLiteral::Bool(true))
                },
                TokenKind::Literal(literal) => match literal {
                    Literal::Int { value, .. } => {
                        Expr::Literal(ExprLiteral::Int(value))
                    },
                    Literal::Float { value, .. } => {
                        Expr::Literal(ExprLiteral::Float(value))
                    },
                    l => unimplemented!("Literal {:?}", l)
                },
                TokenKind::GroupingStart(GroupingDelimiter::Paren) => {
                    self.lexer.push_mode(LexerMode::Grouping);
                    let expr = self.expression();

                    if self.expect(TokenKind::GroupingEnd(GroupingDelimiter::Paren)) {
                        self.lexer.pop_mode();
                    } else {
                        eprintln!("Missing parenthesis grouping end delimiter!");
                    }

                    Expr::Grouping(Box::new(expr))
                },
                k => unimplemented!("Kind {:?}", k)
            }
        } else {
            panic!("Unexpected EOF")
        }
    }
}
