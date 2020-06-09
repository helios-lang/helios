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
    create_test!(r#""""#, test_string, "".to_string());

    create_test!(r#""          ""#, test_string, "          ".to_string());

    create_test!(r#""Hello, world!""#, test_string, "Hello, world!".to_string());

    create_test!(r#""Hello\nWorld""#, test_string, "Hello\nWorld".to_string());

    create_test!(r#""\\\n\t\r""#, test_string, "\\\n\t\r".to_string());
}
