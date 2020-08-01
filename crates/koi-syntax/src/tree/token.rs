#![allow(dead_code)]

use crate::errors::LexerError;
use crate::source::TextSpan;
use std::rc::Rc;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SyntaxToken<'a> {
    raw: &'a Rc<RawSyntaxToken>,
    pub(crate) span: TextSpan,
    pub(crate) leading_trivia: Vec<SyntaxTrivia>,
    pub(crate) trailing_trivia: Vec<SyntaxTrivia>,
}

impl<'a> SyntaxToken<'a> {
    pub fn with(raw: &'a Rc<RawSyntaxToken>, span: TextSpan) -> Self {
        Self::with_trivia(raw, span, Vec::new(), Vec::new())
    }

    pub fn with_trivia(raw: &'a Rc<RawSyntaxToken>,
                       span: TextSpan,
                       leading_trivia: Vec<SyntaxTrivia>,
                       trailing_trivia: Vec<SyntaxTrivia>) -> Self
    {
        Self { raw, span, leading_trivia, trailing_trivia }
    }

    pub fn span(&self) -> TextSpan {
        self.span
    }

    pub fn full_span(&self) -> TextSpan {
        TextSpan::from_bounds(
            self.leading_trivia.first().map_or(0, |trivia| trivia.len()),
            self.trailing_trivia.last().map_or(0, |trivia| trivia.len())
        )
    }

    pub fn kind(&self) -> TokenKind {
        self.raw.kind
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct RawSyntaxToken {
    kind: TokenKind,
    text: String,
}

impl RawSyntaxToken {
    pub fn with(kind: TokenKind, text: String) -> Self {
        Self { kind, text }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum TokenKind {
    /// A tag that identifies a variable, type, module, etc.
    Identifier,

    /// A reserved identifier.
    Keyword(Keyword),

    /// A literal type.
    Literal(Literal),

    /// A character or delimiter with significant meaning of the structure of
    /// the code.
    Symbol(Symbol),

    /// A token signifying an error, for example when a string literal is not
    /// terminated properly.
    Error(LexerError),

    /// An unknown token.
    Unknown(char),

    /// An end of file token.
    Eof,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum SyntaxTrivia {
    Tab(usize),
    Space(usize),
    LineFeed(usize),
    CarriageReturn(usize),
    CarriageReturnLineFeed(usize),
    LineComment { is_doc_comment: bool, len: usize },
}

impl SyntaxTrivia {
    pub fn len(&self) -> usize {
        match self {
            Self::Tab(n)
                | Self::Space(n)
                | Self::LineFeed(n)
                | Self::CarriageReturn(n) => *n,
            Self::CarriageReturnLineFeed(n) => *n * 2,
            Self::LineComment { len, .. } => *len
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Keyword {
    And,
    As,
    Case,
    Def,
    Else,
    Enum,
    External,
    If,
    Internal,
    Let,
    Match,
    Module,
    Mut,
    Not,
    Or,
    Public,
    Ref,
    Return,
    Struct,
    Trait,
    Type,
    Unimplemented,
    Using,
    Var,
    With,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Literal {
    Character,
    Float,
    Integer(Base),
    String,
}

/// Describes the base system used by the number literal encoding.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Base {
    /// The binary base system (radix = 2). Number literals in binary base start
    /// with `0b`, for example `0b01`.
    Binary,

    /// The octal base system (radix = 8). Number literals in octal base start
    /// with `0o`, for example `0o07`.
    Octal,

    /// The hexadecimal base system (radix = 16). Number literals in hexadecimal
    /// base start with `0x`, for example `0x0f`.
    Hexadecimal,

    /// The decimal base system (radix = 10). This is the default base.
    Decimal,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
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
    /// The `<-` token.
    LThinArrow,
    /// The `->` token.
    RThinArrow,
    /// The `=>` token.
    ThickArrow,

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
            _ => panic!("Character `{}` is not a valid Symbol", c)
        }
    }

    pub fn compose(first: char, second: char) -> Option<Self> {
        match (first, second) {
            ('!', '=') => Some(Self::BangEq),
            ('<', '=') => Some(Self::LtEq),
            ('>', '=') => Some(Self::GtEq),
            ('<', '-') => Some(Self::LThinArrow),
            ('-', '>') => Some(Self::RThinArrow),
            ('=', '>') => Some(Self::ThickArrow),
            _ => None
        }
    }
}
