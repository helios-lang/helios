use crate::parser::token::*;
use crate::parser::lexer::*;
use crate::source::{Position, Source};
use super::read_from_string;

macro_rules! test_float {
    ($source:expr, $value:expr, $base:expr, $size:expr) => {
        match $source {
            Ok(source) => {
                let mut tokens = Vec::new();
                let mut lexer = Lexer::with(source);

                loop {
                    match lexer.next_token() {
                        Some(token) => tokens.push(token),
                        None => break
                    }
                }

                assert_eq! {
                    tokens,
                    vec! {
                        Token::with(
                            TokenKind::Literal(
                                Literal::Float {
                                    base: $base,
                                    value: $value,
                                }
                            ),
                            Position::new(0, 0)..Position::new(0, $size)
                        )
                    }
                }
            },
            Err(error) => panic!("Failed to create Source from stream: {}", error)
        }
    }
}

#[test]
fn test_float_literals() {
    create_numeric_test! {
        "0.0",
        test_float,
        FloatValue::Value(0.0)
    }

    create_numeric_test! {
        "1.1",
        test_float,
        FloatValue::Value(1.1)
    }

    create_numeric_test! {
        "10.01",
        test_float,
        FloatValue::Value(10.01)
    }

    create_numeric_test! {
        "10_000.000_100",
        test_float,
        FloatValue::Value(10_000.000_100)
    }

    // Close to std::f64::MAX
    create_numeric_test! {
        "1.7976931348623157",
        test_float,
        FloatValue::Value(1.7976931348623157)
    }
}
