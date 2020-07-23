use crate::errors::LexerError;
use crate::source::Span;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
    pub leading_trivia: Vec<Trivia>,
    pub trailing_trivia: Vec<Trivia>,
}

impl Token {
    pub fn with(kind: TokenKind, span: Span) -> Self {
        Self::with_trivia(kind, span, Vec::new(), Vec::new())
    }

    pub fn with_trivia(kind: TokenKind,
                       span: Span,
                       leading_trivia: Vec<Trivia>,
                       trailing_trivia: Vec<Trivia>) -> Self
    {
        assert! {
            span.end >= span.start,
            format!("invalid token span: {}..{}", span.start, span.end)
        }

        Self { kind, span, leading_trivia, trailing_trivia }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Trivia {
    Spaces(usize),
    Tabs(usize),
    CarriageReturn(usize),
    LineComment { is_doc_comment: bool, span: Span },
}

/// An enum representing all the possible token types.
#[derive(Clone, Debug, Eq, PartialEq)]
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

    /// A line comment starting with two or three forward slashes (`/`).
    LineComment { is_doc_comment: bool },

    /// Signifies the end of the current line (if it is still part of the
    /// current scope).
    Newline,

    /// Signifies the beginning of a new scope.
    Begin,

    /// Signifies the end of a scope.
    End,

    /// Signifies the beginning of a grouping delimiter.
    GroupingStart(GroupingDelimiter),

    /// Signifies the end of a grouping delimiter.
    GroupingEnd(GroupingDelimiter),

    /// A token signifying an error, for example when a string literal is not
    /// terminated properly.
    Error(LexerError),

    /// A missing token.
    Missing(Box<Self>),

    /// An unknown token.
    Unknown(char),

    /// An end of file token.
    Eof,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
    Type,
    Unimplemented,
    Using,
    Val,
    With,
}

impl Keyword {
    pub fn keyword_list() -> Vec<String> {
        vec![
            "and", "as", "case", "def", "else", "enum", "extend", "external",
            "if", "internal", "let", "match", "module", "mut", "not", "or",
            "public", "ref", "return", "struct", "type", "using", "val", "with",
        ].into_iter().map(String::from).collect()
    }
}

/// Describes the base system used by the number literal encoding.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Literal {
    Character,
    Float,
    Integer(Base),
    String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WhitespaceKind {
    Space,
    Tab,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GroupingDelimiter {
    Brace,
    Bracket,
    Paren,
}

impl GroupingDelimiter {
    pub fn description(self) -> String {
        use GroupingDelimiter::*;
        match self {
            Brace => "brace".to_string(),
            Bracket => "bracket".to_string(),
            Paren => "parenthesis".to_string(),
        }
    }

    pub fn closing_char_representation(self) -> char {
        use GroupingDelimiter::*;
        match self {
            Brace => '}',
            Bracket => ']',
            Paren => ')',
        }
    }

    pub fn from_char(c: char) -> Self {
        match c {
            '{' | '}' => Self::Brace,
            '[' | ']' => Self::Bracket,
            '(' | ')' => Self::Paren,
            _ => panic!("Cannot create a GroupingDelimiter from {:?}", c)
        }
    }
}
