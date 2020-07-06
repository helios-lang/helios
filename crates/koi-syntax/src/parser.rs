use crate::lexer::{Lexer, LexerMode};
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
    /// Peeks the next token without consuming it.
    fn peek(&mut self) -> Option<Token> {
        if self.peeked_token.is_none() {
            self.peeked_token = Some(self.lexer.next_token());
        }

        self.peeked_token.clone()
    }

    /// Retrieves the next token.
    fn next_token(&mut self) -> Token {
        match self.peeked_token.take() {
            Some(token) => token,
            None => self.lexer.next_token()
        }
    }

    /// Checks if the next token is of the expected `TokenKind`.
    fn check(&mut self, kind: TokenKind) -> bool {
        match self.peek() {
            Some(token) => token.kind == kind,
            None => false
        }
    }

    /// Checks if the next token is any one of the expected `TokenKind`s.
    fn check_all(&mut self, kinds: &[TokenKind]) -> bool {
        for kind in kinds {
            if self.check(kind.clone()) {
                return true;
            }
        }

        false
    }

    /// Consumes the next token if it is of the expected `TokenKind`, otherwise
    /// returns a `Missing` token.
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

        panic!("Unhandled case: consuming when peeking gives None value");
    }

    /// Consumes the next token if it is of the expected `TokenKind`, otherwise
    /// does NOT move to the next token.
    fn consume_optional(&mut self, kind: TokenKind) -> bool {
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
        self.consume_optional(TokenKind::Newline);

        if self.consume_optional(TokenKind::Eof) {
            return Node::Eof;
        }

        Node::ExpressionNode(self.parse_expression())
    }

    fn parse_expression(&mut self) -> ExpressionNode {
        if self.consume_optional(TokenKind::Eof) {
            return ExpressionNode::MissingExpressionNode(self.lexer.current_pos());
        }

        if self.consume_optional(TokenKind::Newline) {
            return ExpressionNode::Unexpected(TokenKind::Newline, self.lexer.current_pos());
        }

        if self.consume_optional(TokenKind::Keyword(Keyword::Let)) {
            return self.parse_let_expression();
        }

        if self.consume_optional(TokenKind::Keyword(Keyword::If)) {
            return self.parse_if_expression();
        }

        self.parse_binary_expression(0)
    }

    fn parse_let_expression(&mut self) -> ExpressionNode {
        ExpressionNode::LocalBindingNode {
            identifier: self.consume(TokenKind::Identifier),
            equal_symbol: self.consume(TokenKind::Symbol(Symbol::Eq)),
            expression: Box::new(self.parse_expression_block()),
        }
    }

    fn parse_if_expression(&mut self) -> ExpressionNode {
        ExpressionNode::IfExpressionNode {
            pattern: Box::new(self.parse_expression()),
            then_keyword: self.consume(TokenKind::Keyword(Keyword::Then)),
            expression: Box::new(self.parse_expression_block()),
            else_clause: {
                self.consume_optional(TokenKind::Newline);
                if self.consume_optional(TokenKind::Keyword(Keyword::Else)) {
                    Some(Box::new(self.parse_else_clause()))
                } else {
                    None
                }
            },
        }
    }

    fn parse_else_clause(&mut self) -> ExpressionNode {
        if self.check(TokenKind::Newline) {
            return ExpressionNode::MissingExpressionNode(self.lexer.current_pos());
        }

        if self.consume_optional(TokenKind::Keyword(Keyword::If)) {
            return self.parse_if_expression();
        }

        self.parse_expression_block()
    }

    fn parse_expression_block(&mut self) -> ExpressionNode {
        if self.consume_optional(TokenKind::Begin) {
            return self.parse_expression_block_list();
        }

        self.parse_expression()
    }

    fn parse_expression_block_list(&mut self) -> ExpressionNode {
        if self.consume_optional(TokenKind::End) {
            return ExpressionNode::MissingExpressionNode(self.lexer.current_pos());
        }

        ExpressionNode::BlockExpressionNode {
            expressions: {
                let mut expressions = Vec::new();
                expressions.push(Box::new(self.parse_expression()));

                while self.consume_optional(TokenKind::Newline) {
                    expressions.push(Box::new(self.parse_expression()));
                }

                expressions
            },
            end_token: self.consume(TokenKind::End),
        }
    }

    fn parse_binary_expression(&mut self, min_precedence: u8) -> ExpressionNode {
        if self.check(TokenKind::Begin) {
            return self.parse_expression_block();
        }

        let mut lhs = self.parse_unary_expression();

        loop {
            let operator = match self.peek() {
                Some(Token { kind: TokenKind::Symbol(symbol), .. }) => symbol,
                _ => break
            };

            if let Some((left_precedence, right_precedence)) = infix_binding_power(operator) {
                if left_precedence < min_precedence {
                    break;
                }

                lhs = ExpressionNode::BinaryExpressionNode {
                    operator: self.next_token(),
                    rhs: Box::new(self.parse_binary_expression(right_precedence)),
                    lhs: Box::new(lhs.clone()),
                };
                continue;
            }

            break;
        }

        lhs
    }

    fn parse_unary_expression(&mut self) -> ExpressionNode {
        if let Some(Token { kind: TokenKind::Symbol(symbol), .. }) = self.peek() {
            let token = self.next_token();

            if let Some(right_precedence) = prefix_binding_power(symbol) {
                return ExpressionNode::UnaryExpressionNode {
                    operator: token,
                    expression: Box::new(self.parse_binary_expression(right_precedence)),
                };
            }

            return ExpressionNode::Unexpected(TokenKind::Symbol(symbol), self.lexer.current_pos());
        }

        self.parse_primary()
    }

    fn parse_primary(&mut self) -> ExpressionNode {
        let token = self.next_token();
        match &token.kind {
            TokenKind::Identifier => {
                ExpressionNode::Identifier(token.span)
            },
            TokenKind::Keyword(Keyword::False) => {
                ExpressionNode::LiteralNode(LiteralNode::Boolean(false), token.span)
            },
            TokenKind::Keyword(Keyword::True) => {
                ExpressionNode::LiteralNode(LiteralNode::Boolean(true), token.span)
            },
            TokenKind::Keyword(Keyword::Unimplemented) => {
                ExpressionNode::UnimplementedExpressionNode
            },
            TokenKind::Literal(literal) => match literal {
                Literal::Integer(base) => {
                    ExpressionNode::LiteralNode(LiteralNode::Integer(base.clone()), token.span)
                },
                Literal::Float(_) => {
                    ExpressionNode::LiteralNode(LiteralNode::Float, token.span)
                },
                l => unimplemented!("Literal {:?}", l)
            },
            TokenKind::GroupingStart(delimiter) => {
                self.lexer.push_mode(LexerMode::Grouping);

                let grouped_expression = ExpressionNode::GroupedExpressionNode {
                    start_delimiter: token.clone(),
                    expression: Box::new(self.parse_expression()),
                    end_delimiter: self.consume(TokenKind::GroupingEnd(delimiter.clone())),
                };

                self.lexer.pop_mode();
                grouped_expression
            },
            TokenKind::Newline | TokenKind::Eof => {
                ExpressionNode::MissingExpressionNode(self.lexer.current_pos())
            },
            TokenKind::Error(error) => {
                ExpressionNode::Error(error.clone())
            },
            k => ExpressionNode::Unexpected(k.clone(), self.lexer.current_pos())
        }
    }
}

/// Determines the prefix binding power of the given symbol. Currently, the only
/// legal prefix symbols are `Symbol::Minus` and `Symbol::Bang`.
fn prefix_binding_power(symbol: Symbol) -> Option<u8> {
    let power = match symbol {
        Symbol::Minus | Symbol::Bang => 9,
        _ => return None,
    };

    Some(power)
}

/// Determines the infix binding power of the given symbol. A higher binding
/// power means higher precedence, meaning that it is more likely to hold onto
/// its adjacent operands.
fn infix_binding_power(symbol: Symbol) -> Option<(u8, u8)> {
    let power = match symbol {
        Symbol::LThinArrow => (2, 1),
        Symbol::Eq | Symbol::BangEq => (3, 2),
        Symbol::Lt | Symbol::Gt | Symbol::LtEq | Symbol::GtEq => (4, 5),
        Symbol::Plus | Symbol::Minus => (6, 7),
        Symbol::Asterisk | Symbol::ForwardSlash => (8, 9),
        _ => return None,
    };

    Some(power)
}
