use crate::token::*;
use crate::lexer::*;
use crate::source::Position;
use super::read_from_string;

macro_rules! test_character {
    ($string:expr, $character:expr) => {
        test_character!($string, $character, $string.len())
    };
    ($string:expr, $character:expr, $size:expr) => {
        create_test! {
            $string,
            vec! {
                Token::with(
                    TokenKind::Literal(Literal::Char($character)),
                    Position::new(0, 0)..Position::new(0, $size)
                )
            }
        }
    }
}

#[test]
fn test_character_literals() {
    test_character!(r#"' '"#, ' ');
    test_character!(r#"'a'"#, 'a');
    test_character!(r#"'\''"#, '\'');
    test_character!(r#"'\n'"#, '\n');
    test_character!(r#"'\r'"#, '\r');
    test_character!(r#"'\t'"#, '\t');
    test_character!(r#"'\0'"#, '\0');
}

#[test]
fn test_invalid_character_literals() {
    create_test! {
        r#"''"#,
        vec! {
            Token::with(
                TokenKind::Error(LexerError::EmptyCharLiteral),
                Position::new(0, 0)..Position::new(0, 2)
            )
        }
    }

    create_test! {
        r#"'\ '"#,
        vec! {
            Token::with(
                TokenKind::Error(LexerError::UnknownEscapeChar(' ')),
                Position::new(0, 0)..Position::new(0, 4)
            )
        }
    }

    create_test! {
        r#"'abc'"#,
        vec! {
            Token::with(
                TokenKind::Error(LexerError::MultipleCodepointsInCharLiteral),
                Position::new(0, 0)..Position::new(0, 5)
            )
        }
    }

    create_test! {
r#"'
'"#,
        vec! {
            Token::with(
                TokenKind::Error(LexerError::MultiLineSpanningChar),
                Position::new(0, 0)..Position::new(1, 1)
            )
        }
    }
}
