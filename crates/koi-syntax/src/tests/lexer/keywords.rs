use crate::token::*;
use super::read_from_string;

macro_rules! test_keyword {
    ($string:expr, $keyword:expr) => {
        test_keyword!($string, $keyword, $string.len())
    };
    ($string:expr, $keyword:expr, $size:expr) => {
        create_lexer_test! {
            $string,
            vec! {
                $crate::token::Token::with(
                    $crate::token::TokenKind::Keyword($keyword),
                    $crate::source::Span::new(
                        $crate::source::Position::new(0, 0, 0),
                        $crate::source::Position::new(0, $size, $size)
                    )
                )
            }
        }
    };
}

#[test]
fn test_keywords() {
    test_keyword!("and",    Keyword::And);
    test_keyword!("def",    Keyword::Def);
    test_keyword!("do",     Keyword::Do);
    test_keyword!("else",   Keyword::Else);
    test_keyword!("false",  Keyword::False);
    test_keyword!("if",     Keyword::If);
    test_keyword!("let",    Keyword::Let);
    test_keyword!("match",  Keyword::Match);
    test_keyword!("not",    Keyword::Not);
    test_keyword!("or",     Keyword::Or);
    test_keyword!("then",   Keyword::Then);
    test_keyword!("true",   Keyword::True);
    test_keyword!("type",   Keyword::Type);
    test_keyword!("using",  Keyword::Using);
    test_keyword!("with",   Keyword::With);
    test_keyword!("???",    Keyword::Unimplemented);
}
