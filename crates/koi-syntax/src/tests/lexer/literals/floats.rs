use crate::token::*;
use super::read_from_string;

macro_rules! test_float {
    ($string:expr, $base:expr) => {
        create_lexer_test! {
            $string,
            vec! {
                $crate::token::Token::with(
                    $crate::token::TokenKind::Literal(Literal::Float($base)),
                    $crate::source::Span::new(
                        $crate::source::Position::new(0, 0, 0),
                        $crate::source::Position::new(0, $string.len(), $string.len())
                    )
                )
            }
        }
    };
}

#[test]
fn test_float_literals() {
    test_float!("0.0", Base::Decimal);
    test_float!("1.1", Base::Decimal);
    test_float!("10.01", Base::Decimal);
    test_float!("100_000.000_001", Base::Decimal);
    test_float!("1.7976931348623157", Base::Decimal);
}
