use crate::token::*;
use crate::lexer::*;
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
    test_integer!("0", 0);
    test_integer!("9", 9);
    test_integer!("10", 10);
    test_integer!("99", 99);
    test_integer!("100", 100);
    test_integer!("1_000", 1_000);
    test_integer!("1_000_000", 1_000_000);
    test_integer!("9_090_909", 9_090_909);
    test_integer!("1234567890", 1234567890);
    test_integer!("2147483647", 2147483647);
}

#[test]
fn test_binary_integer_literals() {
    test_integer!("0b0", 0, NumericBase::Binary);
    test_integer!("0b1", 1, NumericBase::Binary);
    test_integer!("0b10", 2, NumericBase::Binary);
    test_integer!("0b11", 3, NumericBase::Binary);
    test_integer!("0b100", 4, NumericBase::Binary);
    test_integer!("0b1_000", 8, NumericBase::Binary);
    test_integer!("0b1_000_000", 64, NumericBase::Binary);
    test_integer!("0b1_010_101", 85, NumericBase::Binary);
    test_integer!("0b101010101", 341, NumericBase::Binary);
    test_integer!("0b1111111111111111111111111111111", std::i32::MAX, NumericBase::Binary);
}

#[test]
fn test_octal_integer_literals() {
    test_integer!("0o0", 0, NumericBase::Octal);
    test_integer!("0o7", 7, NumericBase::Octal);
    test_integer!("0o10", 8, NumericBase::Octal);
    test_integer!("0o77", 63, NumericBase::Octal);
    test_integer!("0o100", 64, NumericBase::Octal);
    test_integer!("0o1_000", 512, NumericBase::Octal);
    test_integer!("0o1_000_000", 262144, NumericBase::Octal);
    test_integer!("0o7_070_707", 1864135, NumericBase::Octal);
    test_integer!("0o1234567", 342391, NumericBase::Octal);
    test_integer!("0o17777777777", std::i32::MAX, NumericBase::Octal);
}

#[test]
fn test_hexadecimal_integer_literals() {
    test_integer!("0x0", 0, NumericBase::Hexadecimal);
    test_integer!("0xf", 15, NumericBase::Hexadecimal);
    test_integer!("0x10", 16, NumericBase::Hexadecimal);
    test_integer!("0xff", 255, NumericBase::Hexadecimal);
    test_integer!("0x100", 256, NumericBase::Hexadecimal);
    test_integer!("0x1_000", 4096, NumericBase::Hexadecimal);
    test_integer!("0x1_000_000", 16777216, NumericBase::Hexadecimal);
    test_integer!("0xf_0f0_f0f", 252645135, NumericBase::Hexadecimal);
    test_integer!("0x7FFFFFFF", std::i32::MAX, NumericBase::Hexadecimal);
}

#[test]
fn test_overflowed_integer_literals() {
    create_test! {
        "2147483648",
        vec! {
            Token::with(
                TokenKind::Error(LexerError::OverflowedIntegerLiteral),
                Position::new(0, 0)..Position::new(0, 10)
            )
        }
    }
}
