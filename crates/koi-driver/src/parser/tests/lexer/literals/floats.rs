use crate::parser::token::*;
use crate::parser::lexer::*;
use crate::source::{Position, Source};
use super::read_from_string;

macro_rules! test_float {
    ($source:expr, $value:expr, $size:expr) => {
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
                                    base: NumericBase::Decimal,
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
    let number = "0.0";
    let mut s = read_from_string(number);
    let s = Source::stream(&mut s);
    test_float!(s, 0.0, number.len());

    let number = "1.1";
    let mut s = read_from_string(number);
    let s = Source::stream(&mut s);
    test_float!(s, 1.1, number.len());

    let number = "10.01";
    let mut s = read_from_string(number);
    let s = Source::stream(&mut s);
    test_float!(s, 10.01, number.len());

    let number = "10_000.000_100";
    let mut s = read_from_string(number);
    let s = Source::stream(&mut s);
    test_float!(s, 10_000.000_100, number.len());

    // Close to std::f64::MAX
    let number = "1.7976931348623157";
    let mut s = read_from_string(number);
    let s = Source::stream(&mut s);
    test_float!(s, 1.7976931348623157, number.len());
}
