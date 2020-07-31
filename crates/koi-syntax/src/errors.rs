type Base = ();

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
