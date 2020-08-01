use crate::tree::token::Base;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum LexerError {
    UnsupportedFloatLiteralBase(Base),
    EmptyCharLiteral,
    UnterminatedCharLiteral,
    UnknownEscapeChar(char),
    IllegalTabCharInCharLiteral,
    MultipleCodepointsInCharLiteral,
    MultiLineSpanningChar,
    UnterminatedStringLiteral,
}
