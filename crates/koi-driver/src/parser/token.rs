#![allow(dead_code)]

use std::ops::Range;

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub range: Range<usize>,
    pub line: usize,
    pub character: usize,
}

impl Token {
    pub fn with(kind: TokenKind, range: Range<usize>, line: usize, character: usize) -> Self {
        assert!(
            range.end >= range.start,
            format!("Invalid range `{}..{}`", range.start, range.end)
        );
        Self { kind, range, line, character }
    }
}

/// An enum representing all the possible lexeme types.
#[derive(Clone, Debug, PartialEq)]
pub enum TokenKind {
    /// A tag that may represent a variable, type, module, etc.
    Identifier(String),

    /// A reserved identifier.
    Keyword(Keyword),

    /// A literal type represented the same as, or as close to, the Robin
    /// source code.
    Literal(Literal),

    /// A character or delimiter with significant meaning of the structure of
    /// the code.
    Symbol(Symbol),

    /// A line comment starting with two slashes (`//`).
    LineComment { is_doc_comment: bool },

    /// Any whitespace character (e.g. a new-line character).
    Whitespace,

    /// A newline character (`\n` or `\r`).
    Newline,

    /// End of file token.
    Eof,

    /// An unknown token. An error may be raised if such a token is encountered.
    Unknown(char),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Keyword {
    And,
    Do,
    Else,
    Export,
    False,
    Fun,
    If,
    Import,
    Let,
    Match,
    Mut,
    Not,
    Or,
    Then,
    True,
    With,
}

/// Describes the base system used by the number literal encoding.
#[derive(Clone, Debug, PartialEq)]
pub enum NumericBase {
    /// The binary base system (radix = 2). Number literals in binary base start
    /// with `0b`, for example `0b01`.
    Binary,
    /// The octal base system (radix = 8). Number literals in octal base start
    /// with `0o`, for example `0b07`.
    Octal,
    /// The binary base system (radix = 16). Number literals in hexadecimal base
    /// start with `0x`, for example `0b0f`.
    Hexadecimal,
    /// The decimal base system (radix = 10). This is the default base.
    Decimal,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Literal {
    Bool(bool),
    Char(char),
    Float { base: NumericBase, value: f64 },
    Int { base: NumericBase, value: i32 },
    Str { content: String, terminated: bool },
}

impl Literal {
    pub fn description(&self) -> String {
        use Literal::*;
        match self {
            Bool(b)             => format!("{}", b),
            Char(c)             => format!("{}", c),
            Float { value, .. } => format!("{}", value),
            Int { value, .. }   => format!("{}", value),
            Str { content, .. } => content.clone()
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Symbol {
    /// The `&` token.
    Ampersand,
    /// The `*` token.
    Asterisk,
    /// The `@` token.
    At,
    /// The `!` token.
    Bang,
    /// The `!=` token.
    BangEq,
    /// The `^` token.
    Caret,
    /// The `:` token.
    Colon,
    /// The `,` token.
    Comma,
    /// The `$` token.
    Dollar,
    /// The `.` token.
    Dot,
    /// The `–` token.
    EnDash,
    /// The `—` token.
    EmDash,
    /// The `=` token.
    Eq,
    /// The `-` token.
    Minus,
    /// The `%` token.
    Percent,
    /// The `+` token.
    Plus,
    /// The '#' token.
    Pound,
    /// The `?` token.
    Question,
    /// The `;` token.
    Semicolon,
    /// The `£` token.
    Sterling,
    /// The `~` token.
    Tilde,
    /// The `|` token.
    Vertical,
    /// The `/` token.
    ForwardSlash,
    /// The `\` token.
    BackSlash,

    /// The `<` token.
    Lt,
    /// The `<=` token.
    LtEq,
    /// The `>` token.
    Gt,
    /// The `>=` token.
    GtEq,
    /// The `{` token.
    LBrace,
    /// The `}` token.
    RBrace,
    /// The `[` token.
    LBracket,
    /// The `]` token.
    RBracket,
    /// The `(` token.
    LParen,
    /// The `)` token.
    RParen,
}

impl Symbol {
    pub fn from_char(c: char) -> Self {
        use Symbol::*;
        match c {
            '&' => Ampersand,
            '*' => Asterisk,
            '@' => At,
            '!' => Bang,
            '^' => Caret,
            ':' => Colon,
            ',' => Comma,
            '$' => Dollar,
            '.' => Dot,
            '–' => EnDash,
            '—' => EmDash,
            '=' => Eq,
            '-' => Minus,
            '%' => Percent,
            '+' => Plus,
            '#' => Pound,
            '?' => Question,
            ';' => Semicolon,
            '£' => Sterling,
            '~' => Tilde,
            '|' => Vertical,
            '/' => ForwardSlash,
            '\\'=> BackSlash,
            '<' => Lt,
            '>' => Gt,
            '{' => LBrace,
            '}' => RBrace,
            '[' => LBracket,
            ']' => RBracket,
            '(' => LParen,
            ')' => RParen,
            _ => panic!("Cannot convert `{}` to a valid Symbol", c)
        }
    }

    pub fn from_char_with_equal(c: char) -> Self {
        use Symbol::*;
        match c {
            '!' => BangEq,
            '<' => LtEq,
            '>' => GtEq,
            _ => panic!("Not a valid compound token: `{}=`", c)
        }
    }
}
