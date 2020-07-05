use crate::node::*;
use crate::source::*;
use crate::token::*;

macro_rules! test_let_expression {
    (
        $string:expr,
        $identifier_start:expr => $identifier_end:expr,
        $equal_symbol_start:expr => $equal_symbol_end:expr,
        $expression:expr
    ) => {
        test_let_expression! {
            $string,
            $crate::token::Token::with(
                $crate::token::TokenKind::Identifier,
                $crate::source::Span::new(
                    $crate::source::Position::new(0, $identifier_start, $identifier_start),
                    $crate::source::Position::new(0, $identifier_end, $identifier_end),
                )
            ),
            $crate::token::Token::with(
                $crate::token::TokenKind::Symbol($crate::token::Symbol::Eq),
                $crate::source::Span::new(
                    $crate::source::Position::new(0, $equal_symbol_start, $equal_symbol_start),
                    $crate::source::Position::new(0, $equal_symbol_end, $equal_symbol_end),
                )
            ),
            $expression
        }
    };
    ($string:expr, $identifier:expr, $equal_symbol:expr, $expression:expr) => {
        let mut local_binding = $crate::node::LocalBindingNode::new();

        local_binding
            .identifier($identifier)
            .equal_symbol($equal_symbol)
            .expression($expression);

        create_parser_test! {
            $string,
            vec! {
                $crate::node::Node::ExpressionNode(
                    $crate::node::ExpressionNode::LocalBindingNode(local_binding)
                )
            }
        }
    };
}

#[test]
fn test_let_expressions() {
    test_let_expression! {
        "let x = 10.0", 4 => 5, 6 => 7,
        ExpressionNode::LiteralNode(
            LiteralNode::Float(
                Token::with(
                    TokenKind::Literal(
                        Literal::Float(crate::token::Base::Decimal)
                    ),
                    Span::new(
                        Position::new(0, 8, 8),
                        Position::new(0, 12, 12),
                    )
                )
            )
        )
    }

    test_let_expression! {
        "let   foo   =   true", 6 => 9, 12 => 13,
        ExpressionNode::LiteralNode(
            LiteralNode::Boolean(true)
        )
    }
}

#[test]
fn test_binary_expressions() {
    let mut binary_expr1 = BinaryExpressionNode::new();
    let mut binary_expr2 = BinaryExpressionNode::new();
    let mut binary_expr3 = BinaryExpressionNode::new();
    let mut binary_expr4 = BinaryExpressionNode::new();
    let mut binary_expr5 = BinaryExpressionNode::new();

    binary_expr1
        .operator(
            Token::with(
                TokenKind::Symbol(Symbol::ForwardSlash),
                Span::new(
                    Position::new(0, 16, 16),
                    Position::new(0, 17, 17)
                )
            )
        )
        .lhs(ExpressionNode::LiteralNode(
            LiteralNode::Integer(
                Token::with(
                    TokenKind::Literal(Literal::Integer(Base::Decimal)),
                    Span::new(
                        Position::new(0, 13, 13),
                        Position::new(0, 15, 15)
                    )
                )
            )
        ))
        .rhs(ExpressionNode::LiteralNode(
            LiteralNode::Integer(
                Token::with(
                    TokenKind::Literal(Literal::Integer(Base::Decimal)),
                    Span::new(
                        Position::new(0, 18, 18),
                        Position::new(0, 19, 19)
                    )
                )
            )
        ));

    binary_expr2
        .operator(
            Token::with(
                TokenKind::Symbol(Symbol::Asterisk),
                Span::new(
                    Position::new(0, 20, 20),
                    Position::new(0, 21, 21)
                )
            )
        )
        .lhs(ExpressionNode::BinaryExpressionNode(binary_expr1))
        .rhs(ExpressionNode::LiteralNode(
            LiteralNode::Integer(
                Token::with(
                    TokenKind::Literal(Literal::Integer(Base::Decimal)),
                    Span::new(
                        Position::new(0, 22, 22),
                        Position::new(0, 23, 23)
                    )
                )
            )
        ));

    binary_expr3
        .operator(
            Token::with(
                TokenKind::Symbol(Symbol::Asterisk),
                Span::new(
                    Position::new(0, 7, 7),
                    Position::new(0, 8, 8)
                )
            )
        )
        .lhs(ExpressionNode::LiteralNode(
            LiteralNode::Integer(
                Token::with(
                    TokenKind::Literal(Literal::Integer(Base::Decimal)),
                    Span::new(
                        Position::new(0, 5, 5),
                        Position::new(0, 6, 6)
                    )
                )
            )
        ))
        .rhs(ExpressionNode::LiteralNode(
            LiteralNode::Integer(
                Token::with(
                    TokenKind::Literal(Literal::Integer(Base::Decimal)),
                    Span::new(
                        Position::new(0, 9, 9),
                        Position::new(0, 10, 10)
                    )
                )
            )
        ));

    binary_expr4
        .operator(
            Token::with(
                TokenKind::Symbol(Symbol::Plus),
                Span::new(
                    Position::new(0, 3, 3),
                    Position::new(0, 4, 4)
                )
            )
        )
        .lhs(ExpressionNode::LiteralNode(
            LiteralNode::Integer(
                Token::with(
                    TokenKind::Literal(Literal::Integer(Base::Decimal)),
                    Span::new(
                        Position::new(0, 0, 0),
                        Position::new(0, 2, 2)
                    )
                )
            )
        ))
        .rhs(ExpressionNode::BinaryExpressionNode(binary_expr3));

    binary_expr5
        .operator(
            Token::with(
                TokenKind::Symbol(Symbol::Eq),
                Span::new(
                    Position::new(0, 11, 11),
                    Position::new(0, 12, 12)
                )
            )
        )
        .lhs(ExpressionNode::BinaryExpressionNode(binary_expr4))
        .rhs(ExpressionNode::BinaryExpressionNode(binary_expr2));

    create_parser_test! {
        "10 + 5 * 2 = 40 / 4 * 2",
        vec! {
            Node::ExpressionNode(
                ExpressionNode::BinaryExpressionNode(binary_expr5)
            )
        }
    }
}
