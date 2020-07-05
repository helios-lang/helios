use crate::lexer::Lexer;
use crate::node::*;
use crate::source::Span;
use crate::token::*;

pub type Ast = Vec<Node>;

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
    fn parse_program(&mut self) -> Node {
        self.try_consume(TokenKind::Newline);

        // TODO: This may never be called
        if self.try_consume(TokenKind::Eof) {
            return Node::Eof;
        }

        Node::ExpressionNode(self.parse_expression())
    }

    fn parse_expression(&mut self) -> ExpressionNode {
        if self.try_consume(TokenKind::Eof) {
            return ExpressionNode::Missing;
        }

        if self.try_consume(TokenKind::Newline) {
            return ExpressionNode::Unexpected(TokenKind::Newline);
        }

        if self.try_consume(TokenKind::Keyword(Keyword::Let)) {
            return self.parse_let_expression();
        }

        if self.try_consume(TokenKind::Keyword(Keyword::If)) {
            return self.parse_if_expression();
        }

        self.parse_equality_expression()
    }

    fn parse_let_expression(&mut self) -> ExpressionNode {
        let mut local_binding = LocalBindingNode::new();

        local_binding
            .identifier(self.consume(TokenKind::Identifier))
            .equal_symbol(self.consume(TokenKind::Symbol(Symbol::Eq)))
            .expression(self.parse_expression_block());

        ExpressionNode::LocalBindingNode(local_binding)
    }

    fn parse_if_expression(&mut self) -> ExpressionNode {
        let mut if_expression = IfExpressionNode::new();

        if_expression
            .pattern(self.parse_expression())
            .then_keyword(self.consume(TokenKind::Keyword(Keyword::Then)))
            .expression(self.parse_expression_block());

        self.try_consume(TokenKind::Newline);
        if self.try_consume(TokenKind::Keyword(Keyword::Else)) {
            if_expression.else_clause(self.parse_else_clause());
        }

        ExpressionNode::IfExpressionNode(if_expression)
    }

    fn parse_else_clause(&mut self) -> ExpressionNode {
        if self.try_consume(TokenKind::Keyword(Keyword::If)) {
            self.parse_if_expression()
        } else {
            self.parse_expression_block()
        }
    }

    fn parse_expression_block(&mut self) -> ExpressionNode {
        if self.try_consume(TokenKind::Begin) {
            return self.parse_expression_block_list();
        }

        self.parse_expression()
    }

    fn parse_expression_block_list(&mut self) -> ExpressionNode {
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

        ExpressionNode::BlockExpression(expressions)
    }

    fn parse_equality_expression(&mut self) -> ExpressionNode {
        let mut lhs = self.parse_comparison_expression();

        while self.check_all(&[
            TokenKind::Symbol(Symbol::BangEq),
            TokenKind::Symbol(Symbol::Eq),
        ]) {
            let operator = self.next_token();
            let rhs = self.parse_comparison_expression();
            lhs = ExpressionNode::BinaryExpression(operator, Box::new(lhs), Box::new(rhs))
        }

        lhs
    }

    fn parse_comparison_expression(&mut self) -> ExpressionNode {
        let mut lhs = self.parse_additive_expression();

        while self.check_all(&[
            TokenKind::Symbol(Symbol::Lt),
            TokenKind::Symbol(Symbol::LtEq),
            TokenKind::Symbol(Symbol::Gt),
            TokenKind::Symbol(Symbol::GtEq),
        ]) {
            let operator = self.next_token();
            let rhs = self.parse_additive_expression();
            lhs = ExpressionNode::BinaryExpression(operator, Box::new(lhs), Box::new(rhs))
        }

        lhs
    }

    fn parse_additive_expression(&mut self) -> ExpressionNode {
        let mut lhs = self.parse_multiplicative_expression();

        while self.check_all(&[
            TokenKind::Symbol(Symbol::Plus),
            TokenKind::Symbol(Symbol::Minus),
        ]) {
            let operator = self.next_token();
            let rhs = self.parse_multiplicative_expression();
            lhs = ExpressionNode::BinaryExpression(operator, Box::new(lhs), Box::new(rhs))
        }

        lhs
    }

    fn parse_multiplicative_expression(&mut self) -> ExpressionNode {
        let mut lhs = self.parse_unary_expression();

        while self.check_all(&[
            TokenKind::Symbol(Symbol::Asterisk),
            TokenKind::Symbol(Symbol::ForwardSlash),
        ]) {
            let operator = self.next_token();
            let rhs = self.parse_unary_expression();
            lhs = ExpressionNode::BinaryExpression(operator, Box::new(lhs), Box::new(rhs))
        }

        lhs
    }

    fn parse_unary_expression(&mut self) -> ExpressionNode {
        while self.check_all(&[
            TokenKind::Symbol(Symbol::Bang),
            TokenKind::Symbol(Symbol::Minus),
        ]) {
            let operator = self.next_token();
            let rhs = self.parse_additive_expression();
            return ExpressionNode::UnaryExpression(operator, Box::new(rhs))
        }

        self.parse_primary()
    }

    fn parse_primary(&mut self) -> ExpressionNode {
        let token = self.next_token();
        match &token.kind {
            TokenKind::Identifier => {
                ExpressionNode::Identifier
            },
            TokenKind::Keyword(Keyword::False) => {
                ExpressionNode::LiteralNode(LiteralNode::Boolean(false))
            },
            TokenKind::Keyword(Keyword::True) => {
                ExpressionNode::LiteralNode(LiteralNode::Boolean(true))
            },
            TokenKind::Literal(literal) => match literal {
                Literal::Integer(_) => {
                    ExpressionNode::LiteralNode(LiteralNode::Integer(token))
                },
                Literal::Float(_) => {
                    ExpressionNode::LiteralNode(LiteralNode::Float(token))
                },
                l => unimplemented!("Literal {:?}", l)
            },
            k => ExpressionNode::Unexpected(k.clone())
        }
    }
}
