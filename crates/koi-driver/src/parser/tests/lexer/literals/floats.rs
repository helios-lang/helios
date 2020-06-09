use crate::parser::token::*;
use crate::parser::lexer::*;
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
    test_float!("0.0", FloatValue::Value(0.0));
    test_float!("1.1", FloatValue::Value(1.1));
    test_float!("10.01", FloatValue::Value(10.01));
    test_float!("100_000.000_001", FloatValue::Value(100_000.000_001));
    // Close to std::f64::MAX
    test_float!("1.7976931348623157", FloatValue::Value(1.7976931348623157));
}
