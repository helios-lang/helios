use crate::errors::LexerError;
use crate::source::TextSpan;
use std::borrow::Borrow;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SyntaxToken {
    pub(crate) raw: Rc<RawSyntaxToken>,
    span: TextSpan,
    is_missing: bool,
    pub(crate) leading_trivia: Vec<SyntaxTrivia>,
    pub(crate) trailing_trivia: Vec<SyntaxTrivia>,
}

impl SyntaxToken {
    /// Constructs a new `SyntaxToken` with no leading or trailing trivia.
    pub fn with(raw: Rc<RawSyntaxToken>, span: TextSpan) -> Self {
        Self::with_trivia(raw, span, Vec::new(), Vec::new())
    }

    /// Constructs a new `SyntaxToken` with leading and trailing trivia.
    pub fn with_trivia(
        raw: Rc<RawSyntaxToken>,
        span: TextSpan,
        leading_trivia: Vec<SyntaxTrivia>,
        trailing_trivia: Vec<SyntaxTrivia>,
    ) -> Self {
        Self {
            raw,
            span,
            is_missing: false,
            leading_trivia,
            trailing_trivia,
        }
    }

    /// Constructs a new missing `SyntaxToken` with the given `TokenKind` and
    /// its position.
    pub fn missing(kind: TokenKind, pos: usize) -> Self {
        Self::missing_with_trivia(kind, pos, Vec::new(), Vec::new())
    }

    pub fn missing_with_trivia(
        kind: TokenKind,
        pos: usize,
        leading_trivia: Vec<SyntaxTrivia>,
        trailing_trivia: Vec<SyntaxTrivia>,
    ) -> Self {
        Self {
            raw: Rc::new(RawSyntaxToken::with(kind, String::new())),
            span: TextSpan::zero_width(pos),
            is_missing: true,
            leading_trivia,
            trailing_trivia,
        }
    }

    /// The span of the token.
    ///
    /// This span does not include any leading or trailing trivia.
    pub fn span(&self) -> TextSpan {
        self.span
    }

    /// The full span of the token.
    ///
    /// A token's full span is it's normal span, plus the span of any leading
    /// and trailing trivia it may have.
    pub fn full_span(&self) -> TextSpan {
        let leading_trivia_len = self
            .leading_trivia
            .iter()
            .fold(0, |acc, trivia| trivia.len() + acc);

        let trailing_trivia_len = self
            .trailing_trivia
            .iter()
            .fold(0, |acc, trivia| trivia.len() + acc);

        TextSpan::from_bounds(
            self.span().start() - leading_trivia_len,
            self.span().end() + trailing_trivia_len,
        )
    }

    /// The kind of the token.
    pub fn kind(&self) -> TokenKind {
        self.raw.kind.clone()
    }

    /// The source-representation of the token.
    pub fn text(&self) -> String {
        self.raw.text.clone()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RawSyntaxToken {
    pub(crate) kind: TokenKind,
    pub(crate) text: String,
}

impl RawSyntaxToken {
    /// Constructs a new `RawSyntaxToken` with a kind and text (its
    /// source-representation).
    pub fn with<S: Into<String>>(kind: TokenKind, text: S) -> Self {
        Self {
            kind,
            text: text.into(),
        }
    }
}

impl Hash for RawSyntaxToken {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.text.hash(state)
    }
}

impl Borrow<String> for RawSyntaxToken {
    fn borrow(&self) -> &String {
        &self.text
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
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

    /// Signifies the beginning of a grouping delimiter.
    GroupingStart(GroupingDelimiter),

    /// Signifies the end of a grouping delimiter.
    GroupingEnd(GroupingDelimiter),

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
            Self::LineComment { len, .. } => *len,
        }
    }
}

#[rustfmt::skip]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Keyword {
    Alias,
    And,
    As,
    Const,
    Else,
    Extend,
    External,           // not reserved
    For,
    Function,
    If,
    Import,
    In,
    Interface,
    Internal,           // not reserved
    Let,
    Match,
    Module,
    Not,
    Of,
    Or,
    Public,
    Ref,
    Return,
    Take,               // not reserved
    Type,
    Unimplemented,
    Var,
    Where,
    While,
    With,
}

impl Keyword {
    pub fn keyword_list() -> Vec<String> {
        vec![
            "alias", "and", "as", "const", "else", "extend", "external", "for",
            "function", "if", "import", "in", "internal", "let", "match",
            "module", "not", "of", "or", "public", "ref", "return", "type",
            "var", "where", "while", "with",
        ]
        .into_iter()
        .map(String::from)
        .collect()
    }
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

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum GroupingDelimiter {
    Brace,
    Bracket,
    Paren,
}

impl GroupingDelimiter {
    pub fn from_char(c: char) -> Self {
        match c {
            '{' | '}' => Self::Brace,
            '[' | ']' => Self::Bracket,
            '(' | ')' => Self::Paren,
            _ => panic!("Invalid grouping delimiter: {:?}", c),
        }
    }
}

impl Symbol {
    #[rustfmt::skip]
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
            _ => panic!("Character `{}` is not a valid Symbol", c),
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
            _ => None,
        }
    }
}
