use crate::parser::token::*;
use crate::parser::lexer::*;
use crate::source::Position;
use super::read_from_string;

macro_rules! test_integer {
    ($string:expr, $value:expr) => {
        test_integer!($string, $value, NumericBase::Decimal)
    };
    ($string:expr, $value:expr, $base:expr) => {
        create_test! {
            $string,
            vec! {
                Token::with(
                    TokenKind::Literal(
                        Literal::Int {
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
fn test_decimal_integer_literals() {
    test_integer!("0", IntValue::Value(0));
    test_integer!("9", IntValue::Value(9));
    test_integer!("10", IntValue::Value(10));
    test_integer!("99", IntValue::Value(99));
    test_integer!("100", IntValue::Value(100));
    test_integer!("1_000", IntValue::Value(1_000));
    test_integer!("1_000_000", IntValue::Value(1_000_000));
    test_integer!("9_090_909", IntValue::Value(9_090_909));
    test_integer!("1234567890", IntValue::Value(1234567890));
    test_integer!("2147483647", IntValue::Value(2147483647));
}

#[test]
fn test_binary_integer_literals() {
    test_integer!("0b0", IntValue::Value(0), NumericBase::Binary);
    test_integer!("0b1", IntValue::Value(1), NumericBase::Binary);
    test_integer!("0b10", IntValue::Value(2), NumericBase::Binary);
    test_integer!("0b11", IntValue::Value(3), NumericBase::Binary);
    test_integer!("0b100", IntValue::Value(4), NumericBase::Binary);
    test_integer!("0b1_000", IntValue::Value(8), NumericBase::Binary);
    test_integer!("0b1_000_000", IntValue::Value(64), NumericBase::Binary);
    test_integer!("0b1_010_101", IntValue::Value(85), NumericBase::Binary);
    test_integer!("0b101010101", IntValue::Value(341), NumericBase::Binary);
    test_integer!("0b1111111111111111111111111111111", IntValue::Value(std::i32::MAX), NumericBase::Binary);
}

#[test]
fn test_octal_integer_literals() {
    test_integer!("0o0", IntValue::Value(0), NumericBase::Octal);
    test_integer!("0o7", IntValue::Value(7), NumericBase::Octal);
    test_integer!("0o10", IntValue::Value(8), NumericBase::Octal);
    test_integer!("0o77", IntValue::Value(63), NumericBase::Octal);
    test_integer!("0o100", IntValue::Value(64), NumericBase::Octal);
    test_integer!("0o1_000", IntValue::Value(512), NumericBase::Octal);
    test_integer!("0o1_000_000", IntValue::Value(262144), NumericBase::Octal);
    test_integer!("0o7_070_707", IntValue::Value(1864135), NumericBase::Octal);
    test_integer!("0o1234567", IntValue::Value(342391), NumericBase::Octal);
    test_integer!("0o17777777777", IntValue::Value(std::i32::MAX), NumericBase::Octal);
}

#[test]
fn test_hexadecimal_integer_literals() {
    test_integer!("0x0", IntValue::Value(0), NumericBase::Hexadecimal);
    test_integer!("0xf", IntValue::Value(15), NumericBase::Hexadecimal);
    test_integer!("0x10", IntValue::Value(16), NumericBase::Hexadecimal);
    test_integer!("0xff", IntValue::Value(255), NumericBase::Hexadecimal);
    test_integer!("0x100", IntValue::Value(256), NumericBase::Hexadecimal);
    test_integer!("0x1_000", IntValue::Value(4096), NumericBase::Hexadecimal);
    test_integer!("0x1_000_000", IntValue::Value(16777216), NumericBase::Hexadecimal);
    test_integer!("0xf_0f0_f0f", IntValue::Value(252645135), NumericBase::Hexadecimal);
    test_integer!("0x7FFFFFFF", IntValue::Value(std::i32::MAX), NumericBase::Hexadecimal);
}

#[test]
fn test_overflowed_integer_literals() {
    test_integer!("2147483648", IntValue::Overflowed, NumericBase::Decimal);
}
