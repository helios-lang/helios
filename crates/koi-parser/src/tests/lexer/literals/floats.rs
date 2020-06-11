use crate::token::*;
use crate::lexer::*;
use crate::source::Position;
use super::read_from_string;

macro_rules! test_float {
    ($string:expr, $value:expr) => {
        test_float!($string, $value, NumericBase::Decimal)
    };
    ($string:expr, $value:expr, $base:expr) => {
        create_test! {
            $string,
            vec! {
                Token::with(
                    TokenKind::Literal(
                        Literal::Float {
                            base: $base,
                            value: $value
                        }
                    ),
                    Position::new(0, 0)..Position::new(0, $string.len())
                )
            }
        }
    }
}

#[test]
fn test_float_literals() {
    test_float!("0.0", 0.0);
    test_float!("1.1", 1.1);
    test_float!("10.01", 10.01);
    test_float!("100_000.000_001", 100_000.000_001);
    test_float!("1.7976931348623157", 1.7976931348623157);
}
