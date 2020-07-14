use crate::token::Base;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LexerError {
    BadIndent { expected: usize, found: usize },

    UnsupportedFloatLiteralBase(Base),

    EmptyCharLiteral,
    UnterminatedCharLiteral,
    UnknownEscapeChar(char),
    IllegalTabCharInCharLiteral,
    MultipleCodepointsInCharLiteral,
    MultiLineSpanningChar,

    UnterminatedStringLiteral,
}
