// use crate::node::*;
// use crate::source::*;
// use crate::token::*;

// macro_rules! test_let_expression {
//     (
//         $string:expr,
//         $identifier_start:expr => $identifier_end:expr,
//         $equal_symbol_start:expr => $equal_symbol_end:expr,
//         $expression:expr
//     ) => {
//         test_let_expression! {
//             $string,
//             $crate::token::Token::with(
//                 $crate::token::TokenKind::Identifier,
//                 $crate::source::Span::new(
//                     $crate::source::Position::new(0, $identifier_start, $identifier_start),
//                     $crate::source::Position::new(0, $identifier_end, $identifier_end),
//                 )
//             ),
//             $crate::token::Token::with(
//                 $crate::token::TokenKind::Symbol($crate::token::Symbol::Eq),
//                 $crate::source::Span::new(
//                     $crate::source::Position::new(0, $equal_symbol_start, $equal_symbol_start),
//                     $crate::source::Position::new(0, $equal_symbol_end, $equal_symbol_end),
//                 )
//             ),
//             $expression
//         }
//     };
//     ($string:expr, $identifier:expr, $equal_symbol:expr, $expression:expr) => {
//         create_parser_test! {
//             $string,
//             vec! {
//                 $crate::node::Node::ExpressionNode(
//                     $crate::node::ExpressionNode::LocalBindingNode {
//                         identifier: $identifier,
//                         equal_symbol: $equal_symbol,
//                         expression: Box::new($expression),
//                     }
//                 )
//             }
//         }
//     };
// }

// #[test]
// fn test_let_expressions() {
//     test_let_expression! {
//         "let x = 10.0", 4 => 5, 6 => 7,
//         ExpressionNode::LiteralNode(
//             LiteralNode::Float,
//             Span::new(
//                 Position::new(0, 8, 8),
//                 Position::new(0, 12, 12),
//             )
//         )
//     }

//     test_let_expression! {
//         "let   foo   =   true", 6 => 9, 12 => 13,
//         ExpressionNode::LiteralNode(
//             LiteralNode::Boolean(true),
//             Span::new(
//                 Position::new(0, 16, 16),
//                 Position::new(0, 20, 20),
//             )
//         )
//     }
// }

// #[test]
// fn test_binary_expressions() {
//     create_parser_test! {
//         "10 + 5 * 2 = 40 / 4 * 2",
//         vec! {
//             Node::ExpressionNode(
//                 ExpressionNode::BinaryExpressionNode {
//                     operator: Token::with(
//                         TokenKind::Symbol(Symbol::Eq),
//                         Span::new(
//                             Position::new(0, 11, 11),
//                             Position::new(0, 12, 12),
//                         )
//                     ),
//                     lhs: Box::new(
//                         ExpressionNode::BinaryExpressionNode {
//                             operator: Token::with(
//                                 TokenKind::Symbol(Symbol::Plus),
//                                 Span::new(
//                                     Position::new(0, 3, 3),
//                                     Position::new(0, 4, 4),
//                                 )
//                             ),
//                             lhs: Box::new(
//                                 ExpressionNode::LiteralNode(
//                                     LiteralNode::Integer(Base::Decimal),
//                                     Span::new(
//                                         Position::new(0, 0, 0),
//                                         Position::new(0, 2, 2),
//                                     )
//                                 )
//                             ),
//                             rhs: Box::new(
//                                 ExpressionNode::BinaryExpressionNode {
//                                     operator: Token::with(
//                                         TokenKind::Symbol(Symbol::Asterisk),
//                                         Span::new(
//                                             Position::new(0, 7, 7),
//                                             Position::new(0, 8, 8),
//                                         )
//                                     ),
//                                     lhs: Box::new(
//                                         ExpressionNode::LiteralNode(
//                                             LiteralNode::Integer(Base::Decimal),
//                                             Span::new(
//                                                 Position::new(0, 5, 5),
//                                                 Position::new(0, 6, 6),
//                                             )
//                                         )
//                                     ),
//                                     rhs: Box::new(
//                                         ExpressionNode::LiteralNode(
//                                             LiteralNode::Integer(Base::Decimal),
//                                             Span::new(
//                                                 Position::new(0, 9, 9),
//                                                 Position::new(0, 10, 10),
//                                             )
//                                         )
//                                     ),
//                                 }
//                             ),
//                         }
//                     ),
//                     rhs: Box::new(
//                         ExpressionNode::BinaryExpressionNode {
//                             operator: Token::with(
//                                 TokenKind::Symbol(Symbol::Asterisk),
//                                 Span::new(
//                                     Position::new(0, 20, 20),
//                                     Position::new(0, 21, 21),
//                                 )
//                             ),
//                             lhs: Box::new(
//                                 ExpressionNode::BinaryExpressionNode {
//                                     operator: Token::with(
//                                         TokenKind::Symbol(Symbol::ForwardSlash),
//                                         Span::new(
//                                             Position::new(0, 16, 16),
//                                             Position::new(0, 17, 17),
//                                         )
//                                     ),
//                                     lhs: Box::new(
//                                         ExpressionNode::LiteralNode(
//                                             LiteralNode::Integer(Base::Decimal),
//                                             Span::new(
//                                                 Position::new(0, 13, 13),
//                                                 Position::new(0, 15, 15),
//                                             )
//                                         )
//                                     ),
//                                     rhs: Box::new(
//                                         ExpressionNode::LiteralNode(
//                                             LiteralNode::Integer(Base::Decimal),
//                                             Span::new(
//                                                 Position::new(0, 18, 18),
//                                                 Position::new(0, 19, 19),
//                                             )
//                                         )
//                                     ),
//                                 }
//                             ),
//                             rhs: Box::new(
//                                 ExpressionNode::LiteralNode(
//                                     LiteralNode::Integer(Base::Decimal),
//                                     Span::new(
//                                         Position::new(0, 22, 22),
//                                         Position::new(0, 23, 23)
//                                     )
//                                 )
//                             ),
//                         }
//                     )
//                 }
//             )
//         }
//     }
// }
