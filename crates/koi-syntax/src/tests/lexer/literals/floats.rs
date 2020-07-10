use crate::token::*;
use super::read_from_string;

macro_rules! test_float {
    ($string:expr) => {
        create_lexer_test! {
            $string,
            vec! {
                $crate::token::Token::with(
                    $crate::token::TokenKind::Literal(Literal::Float),
                    $crate::source::Span::new(
                        $crate::source::Position::new(0, 0, 0),
                        $crate::source::Position::new(0, $string.len(), $string.len())
                    )
                ),
                $crate::token::Token::with(
                    $crate::token::TokenKind::Eof,
                    $crate::source::Span::new(
                        $crate::source::Position::new(0, $string.len(), $string.len()),
                        $crate::source::Position::new(0, $string.len(), $string.len())
                    )
                ),
            }
        }
    };
}

#[test]
fn test_float_literals() {
    test_float!("0.0");
    test_float!("1.1");
    test_float!("10.01");
    test_float!("100_000.000_001");
    test_float!("1.7976931348623157");
}
