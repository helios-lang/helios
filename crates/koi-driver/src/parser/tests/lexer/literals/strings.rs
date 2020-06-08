use crate::parser::token::*;
use crate::parser::lexer::*;
use crate::source::{Position, Source};
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
    let string = r#""""#;
    let mut s = read_from_string(string);
    let s = Source::stream(&mut s);
    test_string!(s, "".to_string(), string.len());

    let string = r#""          ""#;
    let mut s = read_from_string(string);
    let s = Source::stream(&mut s);
    test_string!(s, "          ".to_string(), string.len());

    let string = r#""Hello, world!""#;
    let mut s = read_from_string(string);
    let s = Source::stream(&mut s);
    test_string!(s, "Hello, world!".to_string(), string.len());

    let string = r#""Hello\nWorld""#;
    let mut s = read_from_string(string);
    let s = Source::stream(&mut s);
    test_string!(s, "Hello\nWorld".to_string(), string.len());

    let string = r#""\\\n\t\r""#;
    let mut s = read_from_string(string);
    let s = Source::stream(&mut s);
    test_string!(s, "\\\n\t\r".to_string(), string.len());
}
