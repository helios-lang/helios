use crate::parser::token::*;
use crate::parser::lexer::*;
use crate::source::Position;
use super::read_from_string;

macro_rules! test_symbol {
    ($string:expr, $symbol:expr) => {
        test_symbol!($string, $symbol, $string.len())
    };
    ($string:expr, $symbol:expr, $size:expr) => {
        create_test! {
            $string,
            vec! {
                Token::with(
                    TokenKind::Symbol($symbol),
                    Position::new(0, 0)..Position::new(0, $size)
                )
            }
        }
    }
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
    test_symbol!("{",   Symbol::LBrace);
    test_symbol!("}",   Symbol::RBrace);
    test_symbol!("[",   Symbol::LBracket);
    test_symbol!("]",   Symbol::RBracket);
    test_symbol!("(",   Symbol::LParen);
    test_symbol!(")",   Symbol::RParen);
}
