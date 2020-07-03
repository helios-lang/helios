#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LexerError {
    BadIndent { expected: usize, found: usize },

    OverflowedIntLiteral,
    OverflowedFloatLiteral,

    EmptyCharLiteral,
    UnterminatedCharLiteral,
    UnknownEscapeChar(char),
    IllegalTabCharInCharLiteral,
    MultipleCodepointsInCharLiteral,
    MultiLineSpanningChar,

    UnterminatedStringLiteral,
}
