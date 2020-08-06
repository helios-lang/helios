use crate::tree::token::{Base, GroupingDelimiter};

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
    RedundantClosingDelimiter(GroupingDelimiter),
}
