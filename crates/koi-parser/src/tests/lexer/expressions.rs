use crate::token::*;
use crate::lexer::*;
use crate::source::Position;
use super::read_from_string;

#[test]
fn test_simple_expressions() {
    create_test! {
        "let x = 1.1",
        vec! {
            Token::with(
                TokenKind::Keyword(Keyword::Let),
                Position::new(0, 0)..Position::new(0, 3)
            ),
            Token::with(
                TokenKind::Identifier("x".to_string()),
                Position::new(0, 4)..Position::new(0, 5)
            ),
            Token::with(
                TokenKind::Symbol(Symbol::Eq),
                Position::new(0, 6)..Position::new(0, 7)
            ),
            Token::with(
                TokenKind::Literal(Literal::Float { base: NumericBase::Decimal, value: 1.1 }),
                Position::new(0, 8)..Position::new(0, 11)
            ),
        }
    }

    create_test! {
        "if a > b then a else b",
        vec! {
            Token::with(
                TokenKind::Keyword(Keyword::If),
                Position::new(0, 0)..Position::new(0, 2)
            ),
            Token::with(
                TokenKind::Identifier("a".to_string()),
                Position::new(0, 3)..Position::new(0, 4)
            ),
            Token::with(
                TokenKind::Symbol(Symbol::Gt),
                Position::new(0, 5)..Position::new(0, 6)
            ),
            Token::with(
                TokenKind::Identifier("b".to_string()),
                Position::new(0, 7)..Position::new(0, 8)
            ),
            Token::with(
                TokenKind::Keyword(Keyword::Then),
                Position::new(0, 9)..Position::new(0, 13)
            ),
            Token::with(
                TokenKind::Identifier("a".to_string()),
                Position::new(0, 14)..Position::new(0, 15)
            ),
            Token::with(
                TokenKind::Keyword(Keyword::Else),
                Position::new(0, 16)..Position::new(0, 20)
            ),
            Token::with(
                TokenKind::Identifier("b".to_string()),
                Position::new(0, 21)..Position::new(0, 22)
            ),
        }
    }

    create_test! {
        "type Point = { x : float, y : float }",
        vec! {
            Token::with(
                TokenKind::Keyword(Keyword::Type),
                Position::new(0, 0)..Position::new(0, 4)
            ),
            Token::with(
                TokenKind::Identifier("Point".to_string()),
                Position::new(0, 5)..Position::new(0, 10)
            ),
            Token::with(
                TokenKind::Symbol(Symbol::Eq),
                Position::new(0, 11)..Position::new(0, 12)
            ),
            Token::with(
                TokenKind::GroupingStart(GroupingDelimiter::Brace),
                Position::new(0, 13)..Position::new(0, 14)
            ),
            Token::with(
                TokenKind::Identifier("x".to_string()),
                Position::new(0, 15)..Position::new(0, 16)
            ),
            Token::with(
                TokenKind::Symbol(Symbol::Colon),
                Position::new(0, 17)..Position::new(0, 18)
            ),
            Token::with(
                TokenKind::Identifier("float".to_string()),
                Position::new(0, 19)..Position::new(0, 24)
            ),
            Token::with(
                TokenKind::Symbol(Symbol::Comma),
                Position::new(0, 24)..Position::new(0, 25)
            ),
            Token::with(
                TokenKind::Identifier("y".to_string()),
                Position::new(0, 26)..Position::new(0, 27)
            ),
            Token::with(
                TokenKind::Symbol(Symbol::Colon),
                Position::new(0, 28)..Position::new(0, 29)
            ),
            Token::with(
                TokenKind::Identifier("float".to_string()),
                Position::new(0, 30)..Position::new(0, 35)
            ),
            Token::with(
                TokenKind::GroupingEnd(GroupingDelimiter::Brace),
                Position::new(0, 36)..Position::new(0, 37)
            ),
        }
    }
}
