use crate::parser::token::*;
use crate::parser::lexer::*;
use crate::source::Position;
use super::read_from_string;

macro_rules! test_string {
    ($string:expr, $content:expr) => {
        test_string!($string, $content, $string.len())
    };
    ($string:expr, $content:expr, $size:expr) => {
        create_test! {
            $string,
            vec! {
                Token::with(
                    TokenKind::Literal(
                        Literal::Str {
                            content: $content,
                            terminated: true,
                        }
                    ),
                    Position::new(0, 0)..Position::new(0, $size)
                )
            }
        }
    }
}

#[test]
fn test_string_literals() {
    test_string!(r#""""#, "".to_string());
    test_string!(r#""   ""#, "   ".to_string());
    test_string!(r#""Hello, world!""#, "Hello, world!".to_string());
    test_string!(r#""Hello\nworld!""#, "Hello\nworld!".to_string());
    test_string!(r#""\\\n\t\r""#, "\\\n\t\r".to_string());

    create_test! {
r#""This is the first line. \
This is the second line. \
This is the third line. \
This is the fourth line. \
This is the last line.""#,
        vec! {
            Token::with(
                TokenKind::Literal(Literal::Str {
                    content:
                        "This is the first line. \
                         This is the second line. \
                         This is the third line. \
                         This is the fourth line. \
                         This is the last line.".to_string(),
                    terminated: true,
                }),
                Position::new(0, 0)..Position::new(4, 23)
            )
        }
    }

    create_test! {
r#""\
    Hello, world! My name is \
    PAL. I am a friendly computer \
    looking for no harm.\
""#,
        vec! {
            Token::with(
                TokenKind::Literal(Literal::Str {
                    content:
                        "Hello, world! My name is PAL. I am a friendly \
                         computer looking for no harm.".to_string(),
                    terminated: true,
                }),
                Position::new(0, 0)..Position::new(4, 1)
            )
        }
    }
}

#[test]
fn test_unterminated_string_literals() {
    create_test!(
        r#""Hello, world!"#,
        vec! {
            Token::with(
                TokenKind::Literal(Literal::Str {
                    content: "Hello, world!".to_string(),
                    terminated: false,
                }),
                Position::new(0, 0)..Position::new(0, 14)
            )
        }
    );
}

#[test]
fn test_invalid_string_literals() {
    create_test! {
        r#""a\b\c\de""#,
        vec! {
            Token::with(
                TokenKind::Error,
                Position::new(0, 0)..Position::new(0, 10)
            )
        }
    }

    create_test! {
        r#""Hello. \World""#,
        vec! {
            Token::with(
                TokenKind::Error,
                Position::new(0, 0)..Position::new(0, 15)
            )
        }
    }
}
