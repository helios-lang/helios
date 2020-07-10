use crate::node::*;
// use crate::source::*;
// use crate::token::*;

#[test]
fn test_expressions() {
    create_parser_test!("", vec![Node::Eof]);

    // -- THIS PRODUCES A STACK OVERFLOW
    // create_parser_test! {
    //     "1.0",
    //     vec! {
    //         Node::ExpressionNode(Box::new(LiteralExpressionNode {
    //             literal: Token::with(
    //                 TokenKind::Literal(Literal::Float),
    //                 Span::new(Position::new(0, 0, 0), Position::new(0, 3, 3))
    //             )
    //         }))
    //     }
    // }

    // -- THIS PRODUCES A STACK OVERFLOW
    // create_parser_test! {
    //     "let x = 10.0",
    //     vec! {
    //         Node::ExpressionNode(
    //             Box::new(LocalBindingExpressionNode {
    //                 let_keyword: Token::with(
    //                     TokenKind::Keyword(Keyword::Let),
    //                     Span::new(Position::new(0, 0, 0), Position::new(0, 3, 3))
    //                 ),
    //                 identifier: Token::with(
    //                     TokenKind::Identifier,
    //                     Span::new(Position::new(0, 4, 4), Position::new(0, 5, 5))
    //                 ),
    //                 equal_symbol: Token::with(
    //                     TokenKind::Symbol(Symbol::Eq),
    //                     Span::new(Position::new(0, 6, 6), Position::new(0, 7, 7))
    //                 ),
    //                 expression: Box::new(LiteralExpressionNode {
    //                     literal: Token::with(
    //                         TokenKind::Literal(Literal::Float),
    //                         Span::new(Position::new(0, 8, 8), Position::new(0, 12, 12))
    //                     )
    //                 }),
    //             })
    //         )
    //     }
    // }
}
