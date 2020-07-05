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
