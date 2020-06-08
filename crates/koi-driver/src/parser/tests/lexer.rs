use crate::parser::token::*;
use crate::parser::lexer::*;
use crate::source::{Position, Source};

fn read_from_string(s: &str) -> &[u8] {
    s.as_bytes()
}

macro_rules! test_integer {
    ($source:expr, $base:expr, $value:expr, $size:expr) => {
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
                                Literal::Int {
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
fn test_decimal_integer_literals() {
    let number = "0";
    let mut s = read_from_string(number);
    let s = Source::stream(&mut s);
    test_integer!(s, NumericBase::Decimal, 0, number.len());

    let number = "9";
    let mut s = read_from_string(number);
    let s = Source::stream(&mut s);
    test_integer!(s, NumericBase::Decimal, 9, number.len());

    let number = "10";
    let mut s = read_from_string(number);
    let s = Source::stream(&mut s);
    test_integer!(s, NumericBase::Decimal, 10, number.len());

    let number = "99";
    let mut s = read_from_string(number);
    let s = Source::stream(&mut s);
    test_integer!(s, NumericBase::Decimal, 99, number.len());

    let number = "100";
    let mut s = read_from_string(number);
    let s = Source::stream(&mut s);
    test_integer!(s, NumericBase::Decimal, 100, number.len());

    let number = "1_000";
    let mut s = read_from_string(number);
    let s = Source::stream(&mut s);
    test_integer!(s, NumericBase::Decimal, 1_000, number.len());

    let number = "1_000_000";
    let mut s = read_from_string(number);
    let s = Source::stream(&mut s);
    test_integer!(s, NumericBase::Decimal, 1_000_000, number.len());

    let number = "9_090_909";
    let mut s = read_from_string(number);
    let s = Source::stream(&mut s);
    test_integer!(s, NumericBase::Decimal, 9_090_909, number.len());

    let number = "1234567890";
    let mut s = read_from_string(number);
    let s = Source::stream(&mut s);
    test_integer!(s, NumericBase::Decimal, 1234567890, number.len());

    let number = "2147483647";
    let mut s = read_from_string(number);
    let s = Source::stream(&mut s);
    test_integer!(s, NumericBase::Decimal, std::i32::MAX, number.len());
}

#[test]
fn test_binary_integer_literals() {
    let number = "0b0";
    let mut s = read_from_string(number);
    let s = Source::stream(&mut s);
    test_integer!(s, NumericBase::Binary, 0, number.len());

    let number = "0b1";
    let mut s = read_from_string(number);
    let s = Source::stream(&mut s);
    test_integer!(s, NumericBase::Binary, 1, number.len());

    let number = "0b10";
    let mut s = read_from_string(number);
    let s = Source::stream(&mut s);
    test_integer!(s, NumericBase::Binary, 2, number.len());

    let number = "0b11";
    let mut s = read_from_string(number);
    let s = Source::stream(&mut s);
    test_integer!(s, NumericBase::Binary, 3, number.len());

    let number = "0b100";
    let mut s = read_from_string(number);
    let s = Source::stream(&mut s);
    test_integer!(s, NumericBase::Binary, 4, number.len());

    let number = "0b1_000";
    let mut s = read_from_string(number);
    let s = Source::stream(&mut s);
    test_integer!(s, NumericBase::Binary, 8, number.len());

    let number = "0b1_000_000";
    let mut s = read_from_string(number);
    let s = Source::stream(&mut s);
    test_integer!(s, NumericBase::Binary, 64, number.len());

    let number = "0b1_010_101";
    let mut s = read_from_string(number);
    let s = Source::stream(&mut s);
    test_integer!(s, NumericBase::Binary, 85, number.len());

    let number = "0b101010101";
    let mut s = read_from_string(number);
    let s = Source::stream(&mut s);
    test_integer!(s, NumericBase::Binary, 341, number.len());

    let number = "0b1111111111111111111111111111111";
    let mut s = read_from_string(number);
    let s = Source::stream(&mut s);
    test_integer!(s, NumericBase::Binary, std::i32::MAX, number.len());
}

#[test]
fn test_octal_integer_literals() {
    let number = "0o0";
    let mut s = read_from_string(number);
    let s = Source::stream(&mut s);
    test_integer!(s, NumericBase::Octal, 0, number.len());

    let number = "0o7";
    let mut s = read_from_string(number);
    let s = Source::stream(&mut s);
    test_integer!(s, NumericBase::Octal, 7, number.len());

    let number = "0o10";
    let mut s = read_from_string(number);
    let s = Source::stream(&mut s);
    test_integer!(s, NumericBase::Octal, 8, number.len());

    let number = "0o77";
    let mut s = read_from_string(number);
    let s = Source::stream(&mut s);
    test_integer!(s, NumericBase::Octal, 63, number.len());

    let number = "0o100";
    let mut s = read_from_string(number);
    let s = Source::stream(&mut s);
    test_integer!(s, NumericBase::Octal, 64, number.len());

    let number = "0o1_000";
    let mut s = read_from_string(number);
    let s = Source::stream(&mut s);
    test_integer!(s, NumericBase::Octal, 512, number.len());

    let number = "0o1_000_000";
    let mut s = read_from_string(number);
    let s = Source::stream(&mut s);
    test_integer!(s, NumericBase::Octal, 262144, number.len());

    let number = "0o7_070_707";
    let mut s = read_from_string(number);
    let s = Source::stream(&mut s);
    test_integer!(s, NumericBase::Octal, 1864135, number.len());

    let number = "0o1234567";
    let mut s = read_from_string(number);
    let s = Source::stream(&mut s);
    test_integer!(s, NumericBase::Octal, 342391, number.len());

    let number = "0o17777777777";
    let mut s = read_from_string(number);
    let s = Source::stream(&mut s);
    test_integer!(s, NumericBase::Octal, std::i32::MAX, number.len());
}
