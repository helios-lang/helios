#![allow(dead_code)]

use crate::lexer::Lexer;
use crate::tree::node::*;
use crate::tree::token::*;

pub type ParserOut = SyntaxTree;

pub struct Parser {
    lexer: Lexer,
    peeked_token: Option<SyntaxToken>,
}

impl Parser {
    pub fn with(lexer: Lexer) -> Self {
        Self { lexer, peeked_token: None }
    }

    pub fn parse(&mut self) -> ParserOut {
        let mut nodes = Vec::new();

        while !self.lexer.is_at_end() {
            nodes.push(self.parse_program());
        }

        SyntaxTree(nodes)
    }
}

impl Parser {
    /// Peeks the next token without consuming it.
    fn peek(&mut self) -> Option<SyntaxToken> {
        if self.peeked_token.is_none() {
            self.peeked_token = Some(self.lexer.next_token());
        }

        self.peeked_token.clone()
    }

    /// Retrieves the next token.
    fn next_token(&mut self) -> SyntaxToken {
        match self.peeked_token.take() {
            Some(token) => token,
            None => self.lexer.next_token()
        }
    }

    /// Checks if the next token is of the expected `TokenKind`.
    fn check(&mut self, kind: TokenKind) -> bool {
        match self.peek() {
            Some(token) => token.kind() == kind,
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
    fn consume(&mut self, kind: TokenKind) -> SyntaxToken {
        let pos = self.lexer.current_pos();
        if let Some(token) = self.peek() {
            if token.kind() == kind {
                return self.next_token();
            } else {
                return SyntaxToken::missing(kind, pos);
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
        Node::ExpressionNode(self.parse_expression())
    }

    fn parse_expression(&mut self) -> Box<dyn ExpressionNode> {
        self.parse_binary_expression(0)
    }

    fn parse_binary_expression(&mut self,
                               min_precedence: u8) -> Box<dyn ExpressionNode>
    {
        let mut lhs = self.parse_unary_expression();

        loop {
            let operator = match self.peek().map(|it| it.kind()) {
                Some(TokenKind::Symbol(symbol)) => symbol,
                _ => break,
            };

            if let Some((left, right)) = infix_binding_power(operator) {
                if left < min_precedence {
                    break;
                }

                lhs = Box::new(BinaryExpressionNode {
                    operator: self.next_token(),
                    lhs: lhs.clone(),
                    rhs: self.parse_binary_expression(right),
                });

                continue;
            }

            break;
        }

        lhs
    }

    fn parse_unary_expression(&mut self) -> Box<dyn ExpressionNode> {
        if let Some(TokenKind::Symbol(symbol)) = self.peek().map(|it| it.kind()) {
            let token = self.next_token();

            if let Some(right_precedence) = prefix_binding_power(symbol) {
                return Box::new(UnaryExpressionNode {
                    operator: token,
                    operand: self.parse_binary_expression(right_precedence),
                });
            }

            return Box::new(UnexpectedTokenNode {
                token: self.lexer.next_token()
            });
        }

        self.parse_primary()
    }

    fn parse_primary(&mut self) -> Box<dyn ExpressionNode> {
        let token = self.next_token();
        match &token.kind() {
            TokenKind::Identifier => {
                Box::new(IdentifierExpressionNode { identifier: token })
            },
            TokenKind::Literal(_) => {
                Box::new(LiteralExpressionNode { literal: token })
            },
            _ => Box::new(UnexpectedTokenNode { token }),
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
        Symbol::Semicolon => (1, 2),
        Symbol::LThinArrow => (3, 2),
        Symbol::Eq | Symbol::BangEq => (4, 3),
        Symbol::Lt | Symbol::Gt | Symbol::LtEq | Symbol::GtEq => (5, 6),
        Symbol::Plus | Symbol::Minus => (7, 8),
        Symbol::Asterisk | Symbol::ForwardSlash => (9, 10),
        _ => return None,
    };

    Some(power)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::source::TextSpan;
    use std::sync::Arc;

    macro_rules! create_parser_test {
        ($string:expr, $expected:expr) => {
            let source = $string;
            let mut parser = Parser::with(Lexer::with(source.to_string()));
            let tree_output = format!("{:?}", parser.parse());

            assert_eq!(tree_output, format!("{:?}", $expected));
        };
    }

    #[test]
    fn test_parser_unary_expression() {
        create_parser_test!("-1", SyntaxTree(vec! {
            Node::ExpressionNode(Box::new(UnaryExpressionNode {
                operator: SyntaxToken::with(
                    Arc::new(RawSyntaxToken::with(
                        TokenKind::Symbol(Symbol::Minus),
                        "-".to_string(),
                    )),
                    TextSpan::new(0, 1),
                ),
                operand: Box::new(LiteralExpressionNode {
                    literal: SyntaxToken::with(
                        Arc::new(RawSyntaxToken::with(
                            TokenKind::Literal(Literal::Integer(Base::Decimal)),
                            "1".to_string(),
                        )),
                        TextSpan::new(1, 1),
                    ),
                }),
            })),
        }));

        create_parser_test!("- 404_040", SyntaxTree(vec! {
            Node::ExpressionNode(Box::new(UnaryExpressionNode {
                operator: SyntaxToken::with_trivia(
                    Arc::new(RawSyntaxToken::with(
                        TokenKind::Symbol(Symbol::Minus),
                        "-".to_string(),
                    )),
                    TextSpan::new(0, 1),
                    Vec::new(),
                    vec![SyntaxTrivia::Space(1)],
                ),
                operand: Box::new(LiteralExpressionNode {
                    literal: SyntaxToken::with(
                        Arc::new(RawSyntaxToken::with(
                            TokenKind::Literal(Literal::Integer(Base::Decimal)),
                            "404_040".to_string(),
                        )),
                        TextSpan::new(2, 7),
                    ),
                }),
            })),
        }));

        create_parser_test!("!True", SyntaxTree(vec! {
            Node::ExpressionNode(Box::new(UnaryExpressionNode {
                operator: SyntaxToken::with(
                    Arc::new(RawSyntaxToken::with(
                        TokenKind::Symbol(Symbol::Bang),
                        "!".to_string(),
                    )),
                    TextSpan::new(0, 1),
                ),
                operand: Box::new(IdentifierExpressionNode {
                    identifier: SyntaxToken::with(
                        Arc::new(RawSyntaxToken::with(
                            TokenKind::Identifier,
                            "True".to_string(),
                        )),
                        TextSpan::new(1, 4),
                    ),
                }),
            })),
        }));
    }

    #[test]
    fn test_parser_binary_expression() {
        let source = "1 + 1";
        let mut parser = Parser::with(Lexer::with(source.to_string()));
        let tree_output = format!("{:?}", parser.parse());

        assert_eq!(tree_output, format!("{:?}", SyntaxTree(vec! {
            Node::ExpressionNode(Box::new(BinaryExpressionNode {
                operator: SyntaxToken::with_trivia(
                    Arc::new(RawSyntaxToken::with(
                        TokenKind::Symbol(Symbol::Plus),
                        "+".to_string(),
                    )),
                    TextSpan::new(2, 1),
                    Vec::new(),
                    vec![SyntaxTrivia::Space(1)],
                ),
                lhs: Box::new(LiteralExpressionNode {
                    literal: SyntaxToken::with_trivia(
                        Arc::new(RawSyntaxToken::with(
                            TokenKind::Literal(Literal::Integer(Base::Decimal)),
                            "1".to_string(),
                        )),
                        TextSpan::new(0, 1),
                        Vec::new(),
                        vec![SyntaxTrivia::Space(1)],
                    ),
                }),
                rhs: Box::new(LiteralExpressionNode {
                    literal: SyntaxToken::with(
                        Arc::new(RawSyntaxToken::with(
                            TokenKind::Literal(Literal::Integer(Base::Decimal)),
                            "1".to_string(),
                        )),
                        TextSpan::new(4, 1),
                    ),
                }),
            })),
        })));
    }
}
