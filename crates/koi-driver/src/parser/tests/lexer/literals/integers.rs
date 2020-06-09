use crate::parser::token::*;
use crate::parser::lexer::*;
use crate::source::{Position, Source};
use super::read_from_string;

macro_rules! test_integer {
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
    create_numeric_test! {
        "0",
        test_integer,
        IntValue::Value(0),
        NumericBase::Decimal
    }

    create_numeric_test! {
        "9",
        test_integer,
        IntValue::Value(9),
        NumericBase::Decimal
    }

    create_numeric_test! {
        "10",
        test_integer,
        IntValue::Value(10),
        NumericBase::Decimal
    }

    create_numeric_test! {
        "99",
        test_integer,
        IntValue::Value(99),
        NumericBase::Decimal
    }

    create_numeric_test! {
        "100",
        test_integer,
        IntValue::Value(100),
        NumericBase::Decimal
    }

    create_numeric_test! {
        "1_000",
        test_integer,
        IntValue::Value(1_000),
        NumericBase::Decimal
    }

    create_numeric_test! {
        "1_000_000",
        test_integer,
        IntValue::Value(1_000_000),
        NumericBase::Decimal
    }

    create_numeric_test! {
        "9_090_909",
        test_integer,
        IntValue::Value(9_090_909),
        NumericBase::Decimal
    }

    create_numeric_test! {
        "1234567890",
        test_integer,
        IntValue::Value(1234567890),
        NumericBase::Decimal
    }

    create_numeric_test! {
        "2147483647",
        test_integer,
        IntValue::Value(2147483647),
        NumericBase::Decimal
    }
}

#[test]
fn test_binary_integer_literals() {
    create_numeric_test! {
        "0b0",
        test_integer,
        IntValue::Value(0),
        NumericBase::Binary
    }

    create_numeric_test! {
        "0b1",
        test_integer,
        IntValue::Value(1),
        NumericBase::Binary
    }

    create_numeric_test! {
        "0b10",
        test_integer,
        IntValue::Value(2),
        NumericBase::Binary
    }

    create_numeric_test! {
        "0b11",
        test_integer,
        IntValue::Value(3),
        NumericBase::Binary
    }

    create_numeric_test! {
        "0b100",
        test_integer,
        IntValue::Value(4),
        NumericBase::Binary
    }

    create_numeric_test! {
        "0b1_000",
        test_integer,
        IntValue::Value(8),
        NumericBase::Binary
    }

    create_numeric_test! {
        "0b1_000_000",
        test_integer,
        IntValue::Value(64),
        NumericBase::Binary
    }

    create_numeric_test! {
        "0b1_010_101",
        test_integer,
        IntValue::Value(85),
        NumericBase::Binary
    }

    create_numeric_test! {
        "0b101010101",
        test_integer,
        IntValue::Value(341),
        NumericBase::Binary
    }

    create_numeric_test! {
        "0b1111111111111111111111111111111",
        test_integer,
        IntValue::Value(std::i32::MAX),
        NumericBase::Binary
    }
}

#[test]
fn test_octal_integer_literals() {
    create_numeric_test! {
        "0o0",
        test_integer,
        IntValue::Value(0),
        NumericBase::Octal
    }

    create_numeric_test! {
        "0o7",
        test_integer,
        IntValue::Value(7),
        NumericBase::Octal
    }

    create_numeric_test! {
        "0o10",
        test_integer,
        IntValue::Value(8),
        NumericBase::Octal
    }

    create_numeric_test! {
        "0o77",
        test_integer,
        IntValue::Value(63),
        NumericBase::Octal
    }

    create_numeric_test! {
        "0o100",
        test_integer,
        IntValue::Value(64),
        NumericBase::Octal
    }

    create_numeric_test! {
        "0o1_000",
        test_integer,
        IntValue::Value(512),
        NumericBase::Octal
    }

    create_numeric_test! {
        "0o1_000_000",
        test_integer,
        IntValue::Value(262144),
        NumericBase::Octal
    }

    create_numeric_test! {
        "0o7_070_707",
        test_integer,
        IntValue::Value(1864135),
        NumericBase::Octal
    }

    create_numeric_test! {
        "0o1234567",
        test_integer,
        IntValue::Value(342391),
        NumericBase::Octal
    }

    create_numeric_test! {
        "0o17777777777",
        test_integer,
        IntValue::Value(std::i32::MAX),
        NumericBase::Octal
    }
}

#[test]
fn test_hexadecimal_integer_literals() {
    create_numeric_test! {
        "0x0",
        test_integer,
        IntValue::Value(0),
        NumericBase::Hexadecimal
    }

    create_numeric_test! {
        "0xf",
        test_integer,
        IntValue::Value(15),
        NumericBase::Hexadecimal
    }

    create_numeric_test! {
        "0x10",
        test_integer,
        IntValue::Value(16),
        NumericBase::Hexadecimal
    }

    create_numeric_test! {
        "0xff",
        test_integer,
        IntValue::Value(255),
        NumericBase::Hexadecimal
    }

    create_numeric_test! {
        "0x100",
        test_integer,
        IntValue::Value(256),
        NumericBase::Hexadecimal
    }

    create_numeric_test! {
        "0x1_000",
        test_integer,
        IntValue::Value(4096),
        NumericBase::Hexadecimal
    }

    create_numeric_test! {
        "0x1_000_000",
        test_integer,
        IntValue::Value(16777216),
        NumericBase::Hexadecimal
    }

    create_numeric_test! {
        "0xf_0f0_f0f",
        test_integer,
        IntValue::Value(252645135),
        NumericBase::Hexadecimal
    }

    create_numeric_test! {
        "0x7FFFFFFF",
        test_integer,
        IntValue::Value(std::i32::MAX),
        NumericBase::Hexadecimal
    }
}

#[test]
fn test_overflowed_integer_literals() {
    create_numeric_test! {
        "2147483648",
        test_integer,
        IntValue::Overflowed,
        NumericBase::Decimal
    }
}
