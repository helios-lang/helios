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
                ),
                $crate::token::Token::with(
                    $crate::token::TokenKind::Eof,
                    $crate::source::Span::new(
                        $crate::source::Position::new(0, $size, $size),
                        $crate::source::Position::new(0, $size, $size)
                    )
                ),
            }
        }
    };
}

#[test]
fn test_keywords() {
    test_keyword!("and",    Keyword::And);
    test_keyword!("def",    Keyword::Def);
    test_keyword!("else",   Keyword::Else);
    test_keyword!("enum",   Keyword::Enum);
    test_keyword!("if",     Keyword::If);
    test_keyword!("let",    Keyword::Let);
    test_keyword!("match",  Keyword::Match);
    test_keyword!("module", Keyword::Module);
    test_keyword!("mut",    Keyword::Mut);
    test_keyword!("not",    Keyword::Not);
    test_keyword!("or",     Keyword::Or);
    test_keyword!("public", Keyword::Public);
    test_keyword!("ref",    Keyword::Ref);
    test_keyword!("return", Keyword::Return);
    test_keyword!("struct", Keyword::Struct);
    test_keyword!("trait",  Keyword::Trait);
    test_keyword!("type",   Keyword::Type);
    test_keyword!("using",  Keyword::Using);
    test_keyword!("var",    Keyword::Var);
    test_keyword!("with",   Keyword::With);
    test_keyword!("???",    Keyword::Unimplemented);
}
