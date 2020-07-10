use crate::token::*;
use crate::source::{Position, Span};
use super::read_from_string;

macro_rules! test_symbol {
    ($string:expr, $symbol:expr) => {
        test_symbol!($string, $symbol, $string.len())
    };
    ($string:expr, $symbol:expr, $size:expr) => {
        create_lexer_test! {
            $string,
            vec! {
                $crate::token::Token::with(
                    $crate::token::TokenKind::Symbol($symbol),
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
fn test_symbols() {
    test_symbol!("&",   Symbol::Ampersand);
    test_symbol!("*",   Symbol::Asterisk);
    test_symbol!("@",   Symbol::At);
    test_symbol!("!",   Symbol::Bang);
    test_symbol!("!=",  Symbol::BangEq);
    test_symbol!("^",   Symbol::Caret);
    test_symbol!(":",   Symbol::Colon);
    test_symbol!(",",   Symbol::Comma);
    test_symbol!("$",   Symbol::Dollar);
    test_symbol!(".",   Symbol::Dot);
    test_symbol!("–",   Symbol::EnDash, 1);
    test_symbol!("—",   Symbol::EmDash, 1);
    test_symbol!("=",   Symbol::Eq);
    test_symbol!("-",   Symbol::Minus);
    test_symbol!("%",   Symbol::Percent);
    test_symbol!("+",   Symbol::Plus);
    test_symbol!("#",   Symbol::Pound);
    test_symbol!("?",   Symbol::Question);
    test_symbol!(";",   Symbol::Semicolon);
    test_symbol!("£",   Symbol::Sterling, 1);
    test_symbol!("~",   Symbol::Tilde);
    test_symbol!("|",   Symbol::Vertical);
    test_symbol!("/",   Symbol::ForwardSlash);
    test_symbol!("\\",  Symbol::BackSlash);
    test_symbol!("<",   Symbol::Lt);
    test_symbol!("<=",  Symbol::LtEq);
    test_symbol!(">",   Symbol::Gt);
    test_symbol!(">=",  Symbol::GtEq);
}

#[test]
fn test_valid_composed_symbols() {
    test_symbol!("!=",  Symbol::BangEq);
    test_symbol!("<=",  Symbol::LtEq);
    test_symbol!(">=",  Symbol::GtEq);
    test_symbol!("<-",  Symbol::LThinArrow);
    test_symbol!("->",  Symbol::RThinArrow);
    test_symbol!("=>",  Symbol::ThickArrow);
}

#[test]
fn test_invalid_composed_symbols() {
    create_lexer_test! {
        "=!",
        vec! {
            Token::with(
                TokenKind::Symbol(Symbol::Eq),
                Span::new(Position::new(0, 0, 0), Position::new(0, 1, 1))
            ),
            Token::with(
                TokenKind::Symbol(Symbol::Bang),
                Span::new(Position::new(0, 1, 1), Position::new(0, 2, 2))
            ),
            Token::with(
                TokenKind::Eof,
                Span::new(Position::new(0, 2, 2), Position::new(0, 2, 2))
            ),
        }
    }

    create_lexer_test! {
        "=<",
        vec! {
            Token::with(
                TokenKind::Symbol(Symbol::Eq),
                Span::new(Position::new(0, 0, 0), Position::new(0, 1, 1))
            ),
            Token::with(
                TokenKind::Symbol(Symbol::Lt),
                Span::new(Position::new(0, 1, 1), Position::new(0, 2, 2))
            ),
            Token::with(
                TokenKind::Eof,
                Span::new(Position::new(0, 2, 2), Position::new(0, 2, 2))
            ),
        }
    }

    create_lexer_test! {
        "-<",
        vec! {
            Token::with(
                TokenKind::Symbol(Symbol::Minus),
                Span::new(Position::new(0, 0, 0), Position::new(0, 1, 1))
            ),
            Token::with(
                TokenKind::Symbol(Symbol::Lt),
                Span::new(Position::new(0, 1, 1), Position::new(0, 2, 2))
            ),
            Token::with(
                TokenKind::Eof,
                Span::new(Position::new(0, 2, 2), Position::new(0, 2, 2))
            ),
        }
    }

    create_lexer_test! {
        ">-",
        vec! {
            Token::with(
                TokenKind::Symbol(Symbol::Gt),
                Span::new(Position::new(0, 0, 0), Position::new(0, 1, 1))
            ),
            Token::with(
                TokenKind::Symbol(Symbol::Minus),
                Span::new(Position::new(0, 1, 1), Position::new(0, 2, 2))
            ),
            Token::with(
                TokenKind::Eof,
                Span::new(Position::new(0, 2, 2), Position::new(0, 2, 2))
            ),
        }
    }
}

#[test]
fn test_misleading_symbols() {
    create_lexer_test! {
        ";",
        vec! {
            Token::with(
                TokenKind::Unknown('\u{037e}'),
                Span::new(Position::new(0, 0, 0), Position::new(0, 1, 1))
            ),
            Token::with(
                TokenKind::Eof,
                Span::new(Position::new(0, 1, 1), Position::new(0, 1, 1))
            ),
        }
    }

    create_lexer_test! {
        "–",
        vec! {
            Token::with(
                TokenKind::Symbol(Symbol::EnDash),
                Span::new(Position::new(0, 0, 0), Position::new(0, 1, 1))
            ),
            Token::with(
                TokenKind::Eof,
                Span::new(Position::new(0, 1, 1), Position::new(0, 1, 1))
            ),
        }
    }

    create_lexer_test! {
        "—",
        vec! {
            Token::with(
                TokenKind::Symbol(Symbol::EmDash),
                Span::new(Position::new(0, 0, 0), Position::new(0, 1, 1))
            ),
            Token::with(
                TokenKind::Eof,
                Span::new(Position::new(0, 1, 1), Position::new(0, 1, 1))
            ),
        }
    }

    create_lexer_test! {
        "–>",
        vec! {
            Token::with(
                TokenKind::Symbol(Symbol::EnDash),
                Span::new(Position::new(0, 0, 0), Position::new(0, 1, 1))
            ),
            Token::with(
                TokenKind::Symbol(Symbol::Gt),
                Span::new(Position::new(0, 1, 1), Position::new(0, 2, 2))
            ),
            Token::with(
                TokenKind::Eof,
                Span::new(Position::new(0, 2, 2), Position::new(0, 2, 2))
            ),
        }
    }

    create_lexer_test! {
        "—>",
        vec! {
            Token::with(
                TokenKind::Symbol(Symbol::EmDash),
                Span::new(Position::new(0, 0, 0), Position::new(0, 1, 1))
            ),
            Token::with(
                TokenKind::Symbol(Symbol::Gt),
                Span::new(Position::new(0, 1, 1), Position::new(0, 2, 2))
            ),
            Token::with(
                TokenKind::Eof,
                Span::new(Position::new(0, 2, 2), Position::new(0, 2, 2))
            ),
        }
    }

    create_lexer_test! {
        "<–",
        vec! {
            Token::with(
                TokenKind::Symbol(Symbol::Lt),
                Span::new(Position::new(0, 0, 0), Position::new(0, 1, 1))
            ),
            Token::with(
                TokenKind::Symbol(Symbol::EnDash),
                Span::new(Position::new(0, 1, 1), Position::new(0, 2, 2))
            ),
            Token::with(
                TokenKind::Eof,
                Span::new(Position::new(0, 2, 2), Position::new(0, 2, 2))
            ),
        }
    }

    create_lexer_test! {
        "<—",
        vec! {
            Token::with(
                TokenKind::Symbol(Symbol::Lt),
                Span::new(Position::new(0, 0, 0), Position::new(0, 1, 1))
            ),
            Token::with(
                TokenKind::Symbol(Symbol::EmDash),
                Span::new(Position::new(0, 1, 1), Position::new(0, 2, 2))
            ),
            Token::with(
                TokenKind::Eof,
                Span::new(Position::new(0, 2, 2), Position::new(0, 2, 2))
            ),
        }
    }
}

// #[test]
// fn test_erroneous_grouping_delimiters() {
//     create_lexer_test! {
//         "{",
//         vec! {
//             Token::with(
//                 TokenKind::GroupingStart(GroupingDelimiter::Brace),
//                 Span::new(Position::new(0, 0, 0), Position::new(0, 1, 1))
//             ),
//             Token::with(
//                 TokenKind::Error(LexerError::UnclosedDelimiter(GroupingDelimiter::Brace)),
//                 Span::new(Position::new(0, 0, 0), Position::new(0, 1, 1))
//             ),
//         }
//     }

//     create_lexer_test! {
//         "[",
//         vec! {
//             Token::with(
//                 TokenKind::GroupingStart(GroupingDelimiter::Bracket),
//                 Position::new(0, 0)..Position::new(0, 1)
//             ),
//             Token::with(
//                 TokenKind::Error(LexerError::UnclosedDelimiter(GroupingDelimiter::Bracket)),
//                 Position::new(0, 0)..Position::new(0, 1)
//             ),
//         }
//     }

//     create_lexer_test! {
//         "(",
//         vec! {
//             Token::with(
//                 TokenKind::GroupingStart(GroupingDelimiter::Paren),
//                 Position::new(0, 0)..Position::new(0, 1)
//             ),
//             Token::with(
//                 TokenKind::Error(LexerError::UnclosedDelimiter(GroupingDelimiter::Paren)),
//                 Position::new(0, 0)..Position::new(0, 1)
//             ),
//         }
//     }

//     create_lexer_test! {
//         "}",
//         vec! {
//             Token::with(
//                 TokenKind::Error(LexerError::RedundantClosingDelimiter(GroupingDelimiter::Brace)),
//                 Position::new(0, 0)..Position::new(0, 1)
//             ),
//         }
//     }

//     create_lexer_test! {
//         "]",
//         vec! {
//             Token::with(
//                 TokenKind::Error(LexerError::RedundantClosingDelimiter(GroupingDelimiter::Bracket)),
//                 Position::new(0, 0)..Position::new(0, 1)
//             ),
//         }
//     }

//     create_lexer_test! {
//         ")",
//         vec! {
//             Token::with(
//                 TokenKind::Error(LexerError::RedundantClosingDelimiter(GroupingDelimiter::Paren)),
//                 Position::new(0, 0)..Position::new(0, 1)
//             ),
//         }
//     }
// }
