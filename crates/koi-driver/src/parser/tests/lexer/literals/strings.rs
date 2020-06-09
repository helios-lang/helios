use crate::parser::token::*;
use crate::parser::lexer::*;
use crate::source::Position;
use super::read_from_string;

macro_rules! test_string {
    ($source:expr, $content:expr, $size:expr) => {
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
                                Literal::Str {
                                    content: $content,
                                    terminated: true,
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
fn test_string_literals() {
    create_string_test!(r#""""#, test_string, "".to_string());

    create_string_test!(r#""          ""#, test_string, "          ".to_string());

    create_string_test!(r#""Hello, world!""#, test_string, "Hello, world!".to_string());

    create_string_test!(r#""Hello\nWorld""#, test_string, "Hello\nWorld".to_string());

    create_string_test!(r#""\\\n\t\r""#, test_string, "\\\n\t\r".to_string());

    let string = r#""a\b\c\de""#;
    create_test! {
        string,
        vec! {
            Token::with(
                TokenKind::Error,
                Position::new(0, 0)..Position::new(0, string.len())
            )
        }
    }

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
