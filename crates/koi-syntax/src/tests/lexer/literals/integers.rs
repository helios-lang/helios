use crate::token::*;
use super::read_from_string;

macro_rules! test_integer {
    ($string:expr, $base:expr) => {
        create_lexer_test! {
            $string,
            vec! {
                $crate::token::Token::with(
                    $crate::token::TokenKind::Literal(Literal::Integer($base)),
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
fn test_decimal_integer_literals() {
    test_integer!("0", Base::Decimal);
    test_integer!("9", Base::Decimal);
    test_integer!("10", Base::Decimal);
    test_integer!("99", Base::Decimal);
    test_integer!("100", Base::Decimal);
    test_integer!("1_000", Base::Decimal);
    test_integer!("1_000_000", Base::Decimal);
    test_integer!("9_090_909", Base::Decimal);
    test_integer!("1234567890", Base::Decimal);
    test_integer!("2147483647", Base::Decimal);
}

#[test]
fn test_binary_integer_literals() {
    test_integer!("0b0", Base::Binary);
    test_integer!("0b1", Base::Binary);
    test_integer!("0b10", Base::Binary);
    test_integer!("0b11", Base::Binary);
    test_integer!("0b100", Base::Binary);
    test_integer!("0b1_000", Base::Binary);
    test_integer!("0b1_000_000", Base::Binary);
    test_integer!("0b1_010_101", Base::Binary);
    test_integer!("0b101010101", Base::Binary);
    test_integer!("0b1111111111111111111111111111111", Base::Binary);
}

#[test]
fn test_octal_integer_literals() {
    test_integer!("0o0", Base::Octal);
    test_integer!("0o7", Base::Octal);
    test_integer!("0o10", Base::Octal);
    test_integer!("0o77", Base::Octal);
    test_integer!("0o100", Base::Octal);
    test_integer!("0o1_000", Base::Octal);
    test_integer!("0o1_000_000", Base::Octal);
    test_integer!("0o7_070_707", Base::Octal);
    test_integer!("0o1234567", Base::Octal);
    test_integer!("0o17777777777", Base::Octal);
}

#[test]
fn test_hexadecimal_integer_literals() {
    test_integer!("0x0", Base::Hexadecimal);
    test_integer!("0xf", Base::Hexadecimal);
    test_integer!("0x10", Base::Hexadecimal);
    test_integer!("0xff", Base::Hexadecimal);
    test_integer!("0x100", Base::Hexadecimal);
    test_integer!("0x1_000", Base::Hexadecimal);
    test_integer!("0x1_000_000", Base::Hexadecimal);
    test_integer!("0xf_0f0_f0f", Base::Hexadecimal);
    test_integer!("0x7FFFFFFF", Base::Hexadecimal);
}
