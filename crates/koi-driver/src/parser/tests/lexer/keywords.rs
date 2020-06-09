use crate::parser::token::*;
use crate::parser::lexer::*;
use crate::source::Position;
use super::read_from_string;

macro_rules! test_keyword {
    ($source:expr, $keyword:expr, $size:expr) => {
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
                            TokenKind::Keyword($keyword),
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
fn test_keywords() {
    create_string_test!("and", test_keyword, Keyword::And);
    create_string_test!("def", test_keyword, Keyword::Def);
    create_string_test!("do", test_keyword, Keyword::Do);
    create_string_test!("else", test_keyword, Keyword::Else);
    create_string_test!("false", test_keyword, Keyword::False);
    create_string_test!("if", test_keyword, Keyword::If);
    create_string_test!("let", test_keyword, Keyword::Let);
    create_string_test!("match", test_keyword, Keyword::Match);
    create_string_test!("not", test_keyword, Keyword::Not);
    create_string_test!("or", test_keyword, Keyword::Or);
    create_string_test!("then", test_keyword, Keyword::Then);
    create_string_test!("true", test_keyword, Keyword::True);
    create_string_test!("type", test_keyword, Keyword::Type);
    create_string_test!("using", test_keyword, Keyword::Using);
    create_string_test!("with", test_keyword, Keyword::With);
    create_string_test!("???", test_keyword, Keyword::Unimplemented);
}
