use crate::tree::token::{Base, GroupingDelimiter};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum LexerError {
    EmptyCharLiteral,
    UnterminatedCharLiteral,
    UnknownEscapeChar(char),
    IllegalTabCharInCharLiteral,
    MultipleCodepointsInCharLiteral,
    MultiLineSpanningChar,
    UnterminatedStringLiteral,
    UnsupportedFloatLiteralBase(Base),
    RedundantClosingDelimiter(GroupingDelimiter),
}
