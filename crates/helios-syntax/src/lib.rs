mod lang;
mod repr;

use helios_formatting::FormattedString;
pub use lang::HeliosLanguage;
use repr::{Article, HumanReadableRepr};
use std::fmt::{self, Display};

pub type SyntaxNode = rowan::SyntaxNode<HeliosLanguage>;
pub type SyntaxToken = rowan::SyntaxToken<HeliosLanguage>;

/// A convenient way to construct new `SyntaxNode` symbols.
///
/// # Examples
/// ```rust
/// use helios_syntax::Sym;
/// assert_eq!(Sym!["@"], helios_syntax::SyntaxKind::Sym_At);
/// assert_eq!(Sym!["$"], helios_syntax::SyntaxKind::Sym_Dollar);
/// assert_eq!(Sym![">="], helios_syntax::SyntaxKind::Sym_GtEq);
/// assert_eq!(Sym!["<-"], helios_syntax::SyntaxKind::Sym_LThinArrow);
/// ```
#[macro_export]
macro_rules! Sym {
    ["&"] => ($crate::SyntaxKind::Sym_Ampersand);
    ["*"] => ($crate::SyntaxKind::Sym_Asterisk);
    ["@"] => ($crate::SyntaxKind::Sym_At);
    ["!"] => ($crate::SyntaxKind::Sym_Bang);
    ["!="]=> ($crate::SyntaxKind::Sym_BangEq);
    ["^"] => ($crate::SyntaxKind::Sym_Caret);
    [","] => ($crate::SyntaxKind::Sym_Comma);
    ["$"] => ($crate::SyntaxKind::Sym_Dollar);
    ["."] => ($crate::SyntaxKind::Sym_Dot);
    ["—"] => ($crate::SyntaxKind::Sym_EmDash);
    ["–"] => ($crate::SyntaxKind::Sym_EnDash);
    ["="] => ($crate::SyntaxKind::Sym_Eq);
    ["/"] => ($crate::SyntaxKind::Sym_ForwardSlash);
    ["-"] => ($crate::SyntaxKind::Sym_Minus);
    ["%"] => ($crate::SyntaxKind::Sym_Percent);
    ["|"] => ($crate::SyntaxKind::Sym_Pipe);
    ["+"] => ($crate::SyntaxKind::Sym_Plus);
    ["#"] => ($crate::SyntaxKind::Sym_Pound);
    ["?"] => ($crate::SyntaxKind::Sym_Question);
    [";"] => ($crate::SyntaxKind::Sym_Semicolon);
    ["£"] => ($crate::SyntaxKind::Sym_Sterling);
    ["~"] => ($crate::SyntaxKind::Sym_Tilde);

    ["<"] => ($crate::SyntaxKind::Sym_Lt);
    ["<="]=> ($crate::SyntaxKind::Sym_LtEq);
    [">"] => ($crate::SyntaxKind::Sym_Gt);
    [">="]=> ($crate::SyntaxKind::Sym_GtEq);
    ["<-"]=> ($crate::SyntaxKind::Sym_LThinArrow);
    ["->"]=> ($crate::SyntaxKind::Sym_RThinArrow);
    ["=>"]=> ($crate::SyntaxKind::Sym_ThickArrow);

    ["{"] => ($crate::SyntaxKind::Sym_LParen);
    ["}"] => ($crate::SyntaxKind::Sym_RParen);
    ["["] => ($crate::SyntaxKind::Sym_LBracket);
    ["]"] => ($crate::SyntaxKind::Sym_RBracket);
    ["("] => ($crate::SyntaxKind::Sym_LParen);
    [")"] => ($crate::SyntaxKind::Sym_RParen);
}

/// All the possible nodes and tokens defined in the Helios grammar.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd)]
#[repr(u16)]
pub enum SyntaxKind {
    Kwd_And,
    Kwd_As,
    Kwd_Case,
    Kwd_Else,
    Kwd_Enum,
    Kwd_For,
    Kwd_Forall,
    Kwd_Func,
    Kwd_If,
    Kwd_Impl,
    Kwd_Import,
    Kwd_In,
    Kwd_Iter,
    Kwd_Let,
    Kwd_Module,
    Kwd_Not,
    Kwd_Of,
    Kwd_Or,
    Kwd_Range,
    Kwd_Record,
    Kwd_Type,
    Kwd_Var,
    Kwd_While,
    Kwd_With,
    Kwd_Yield,

    Sym_Ampersand,
    Sym_Asterisk,
    Sym_At,
    Sym_BackSlash,
    Sym_Bang,
    Sym_BangEq,
    Sym_Caret,
    Sym_Colon,
    Sym_Comma,
    Sym_Dollar,
    Sym_Dot,
    Sym_EmDash,
    Sym_EnDash,
    Sym_Eq,
    Sym_ForwardSlash,
    Sym_Minus,
    Sym_Percent,
    Sym_Pipe,
    Sym_Plus,
    Sym_Question,
    Sym_Semicolon,
    Sym_Sterling,
    Sym_Tilde,

    Sym_Lt,
    Sym_LtEq,
    Sym_Gt,
    Sym_GtEq,
    Sym_LThinArrow,
    Sym_RThinArrow,
    Sym_ThickArrow,
    Sym_Walrus,

    Sym_LBrace,
    Sym_RBrace,
    Sym_LBracket,
    Sym_RBracket,
    Sym_LParen,
    Sym_RParen,

    Lit_Character,
    Lit_Float,
    Lit_Integer,
    Lit_String,

    Exp_Binary,
    Exp_Indented,
    Exp_Literal,
    Exp_Paren,
    Exp_UnaryPrefix,
    Exp_UnaryPostfix,
    Exp_VariableRef,
    Exp_Unnamed,

    Dec_GlobalBinding,

    Comment,
    DocComment,
    Whitespace,

    Indent,
    Dedent,
    Newline,

    Identifier,
    ReservedIdentifier,

    Placeholder,
    UnknownChar,
    Error,
    Root, // this should be last
}

impl SyntaxKind {
    /// Determines if the [`SyntaxKind`] is a discardable token (i.e. syntax
    /// trivia).
    ///
    /// This method doesn't take a reference to a [`SyntaxKind`]. Due to the
    /// fact that [`SyntaxKind`] is one byte in size, it is much more efficient
    /// to pass by value than by reference. A reference is much larger in size
    /// (eight bytes on 64-bit systems), which would have required an unneeded
    /// allocation of memory. Note that [`SyntaxKind`] is `Copy`, so any other
    /// references to the instance is not consumed.
    #[inline]
    pub fn is_trivia(self) -> bool {
        use SyntaxKind::*;
        matches!(self, Comment | DocComment | Whitespace | Newline)
    }

    /// Determines if the [`SyntaxKind`] is a keyword.
    #[inline]
    pub fn is_keyword(self) -> bool {
        self >= SyntaxKind::Kwd_And && self <= SyntaxKind::Kwd_Yield
    }

    /// Determines if the [`SyntaxKind`] is a symbol.
    #[inline]
    pub fn is_symbol(self) -> bool {
        self >= SyntaxKind::Sym_Ampersand && self <= SyntaxKind::Sym_RParen
    }

    #[inline]
    pub fn is_literal(self) -> bool {
        self >= SyntaxKind::Lit_Character && self <= SyntaxKind::Lit_String
    }

    #[inline]
    pub fn is_expression(self) -> bool {
        self >= SyntaxKind::Exp_Binary && self <= SyntaxKind::Exp_Unnamed
    }

    #[inline]
    pub fn is_declaration(self) -> bool {
        self == SyntaxKind::Dec_GlobalBinding
    }

    #[inline]
    pub fn is_comment(self) -> bool {
        self == SyntaxKind::Comment || self == SyntaxKind::DocComment
    }

    #[inline]
    pub fn is_identifier(self) -> bool {
        self == SyntaxKind::Identifier || self == SyntaxKind::ReservedIdentifier
    }

    pub fn human_readable_repr(self) -> HumanReadableRepr {
        HumanReadableRepr {
            article: self.article(),
            qualifier: self.qualifier(),
            description: self.description(),
            kind: self.kind(),
            code_repr: self.code_repr(),
            example: self.example(),
        }
    }

    pub fn article(self) -> Article {
        match self {
            kind if kind.is_keyword() => Article::The,
            SyntaxKind::Sym_Ampersand
            | SyntaxKind::Sym_Asterisk
            | SyntaxKind::Sym_At
            | SyntaxKind::Sym_Bang
            | SyntaxKind::Sym_EmDash
            | SyntaxKind::Sym_EnDash
            | SyntaxKind::Sym_Eq
            | SyntaxKind::Sym_LBrace
            | SyntaxKind::Sym_LBracket
            | SyntaxKind::Sym_LParen
            | SyntaxKind::Lit_Integer
            | SyntaxKind::Exp_Indented
            | SyntaxKind::Exp_Unnamed
            | SyntaxKind::Indent
            | SyntaxKind::Identifier
            | SyntaxKind::UnknownChar
            | SyntaxKind::Error => Article::An,
            _ => Article::A,
        }
    }

    pub fn qualifier(self) -> Option<String> {
        let s = match self {
            SyntaxKind::Sym_LBrace => "opening curly",
            SyntaxKind::Sym_LBracket => "opening square",
            SyntaxKind::Sym_LParen => "opening",
            SyntaxKind::Sym_RBrace => "closing curly",
            SyntaxKind::Sym_RBracket => "closing square",
            SyntaxKind::Sym_RParen => "closing",
            _ => return None,
        };

        Some(s.to_string())
    }

    pub fn description(self) -> Option<String> {
        let s = match self {
            // keywords
            SyntaxKind::Kwd_And => "and",
            SyntaxKind::Kwd_As => "as",
            SyntaxKind::Kwd_Case => "case",
            SyntaxKind::Kwd_Else => "else",
            SyntaxKind::Kwd_Enum => "enum",
            SyntaxKind::Kwd_For => "for",
            SyntaxKind::Kwd_Forall => "forall",
            SyntaxKind::Kwd_Func => "func",
            SyntaxKind::Kwd_If => "if",
            SyntaxKind::Kwd_Impl => "impl",
            SyntaxKind::Kwd_Import => "import",
            SyntaxKind::Kwd_In => "in",
            SyntaxKind::Kwd_Iter => "iter",
            SyntaxKind::Kwd_Let => "let",
            SyntaxKind::Kwd_Module => "module",
            SyntaxKind::Kwd_Not => "not",
            SyntaxKind::Kwd_Of => "of",
            SyntaxKind::Kwd_Or => "or",
            SyntaxKind::Kwd_Range => "range",
            SyntaxKind::Kwd_Record => "record",
            SyntaxKind::Kwd_Type => "type",
            SyntaxKind::Kwd_Var => "var",
            SyntaxKind::Kwd_While => "while",
            SyntaxKind::Kwd_With => "with",
            SyntaxKind::Kwd_Yield => "yield",
            // symbols
            SyntaxKind::Sym_Ampersand => "ampersand",
            SyntaxKind::Sym_Asterisk => "asterisk",
            SyntaxKind::Sym_At => "at",
            SyntaxKind::Sym_BackSlash => "backslash",
            SyntaxKind::Sym_Bang => "exclamation mark",
            SyntaxKind::Sym_BangEq => "not equal",
            SyntaxKind::Sym_Caret => "caret",
            SyntaxKind::Sym_Colon => "colon",
            SyntaxKind::Sym_Comma => "comma",
            SyntaxKind::Sym_Dollar => "dollar",
            SyntaxKind::Sym_Dot => "dot",
            SyntaxKind::Sym_EmDash => "em-dash",
            SyntaxKind::Sym_EnDash => "en-dash",
            SyntaxKind::Sym_Eq => "equals",
            SyntaxKind::Sym_ForwardSlash => "forward slash",
            SyntaxKind::Sym_Minus => "minus",
            SyntaxKind::Sym_Percent => "percent",
            SyntaxKind::Sym_Pipe => "pipe",
            SyntaxKind::Sym_Plus => "plus",
            SyntaxKind::Sym_Question => "question mark",
            SyntaxKind::Sym_Semicolon => "semicolon",
            SyntaxKind::Sym_Sterling => "sterling",
            SyntaxKind::Sym_Tilde => "tilde",
            SyntaxKind::Sym_Lt => "less than",
            SyntaxKind::Sym_LtEq => "less than equal",
            SyntaxKind::Sym_Gt => "greater than",
            SyntaxKind::Sym_GtEq => "greater than equal",
            SyntaxKind::Sym_LThinArrow => "leftwards thin arrow",
            SyntaxKind::Sym_RThinArrow => "rightwards thin arrow",
            SyntaxKind::Sym_ThickArrow => "thick arrow",
            SyntaxKind::Sym_Walrus => "walrus",
            SyntaxKind::Sym_LBrace | SyntaxKind::Sym_RBrace => "brace",
            SyntaxKind::Sym_LBracket | SyntaxKind::Sym_RBracket => "bracket",
            SyntaxKind::Sym_LParen | SyntaxKind::Sym_RParen => "parenthesis",
            // literals
            SyntaxKind::Lit_Character => "character",
            SyntaxKind::Lit_Float => "float",
            SyntaxKind::Lit_Integer => "integer",
            SyntaxKind::Lit_String => "string",
            // expressions
            SyntaxKind::Exp_Binary => "binary",
            SyntaxKind::Exp_Indented => "indented",
            SyntaxKind::Exp_Literal => "literal",
            SyntaxKind::Exp_Paren => "parenthesized",
            SyntaxKind::Exp_UnaryPrefix => "prefixed unary",
            SyntaxKind::Exp_UnaryPostfix => "postfixed unary",
            SyntaxKind::Exp_VariableRef => "variable reference",
            // declarations
            SyntaxKind::Dec_GlobalBinding => "global binding",
            // other
            SyntaxKind::DocComment => "documentation",
            SyntaxKind::ReservedIdentifier => "reserved",
            _ => return None,
        };

        Some(s.to_string())
    }

    pub fn kind(self) -> String {
        let s = match self {
            kind if kind.is_keyword() => "keyword",
            kind if kind.is_symbol() => "symbol",
            kind if kind.is_literal() => "literal",
            kind if kind.is_expression() => "expression",
            kind if kind.is_declaration() => "declaration",
            kind if kind.is_comment() => "comment",
            kind if kind.is_identifier() => "identifier",
            SyntaxKind::Indent => "indent",
            SyntaxKind::Dedent => "dedent",
            SyntaxKind::Newline => "new line",
            SyntaxKind::Whitespace => "whitespace",
            SyntaxKind::Placeholder => "placeholder",
            SyntaxKind::UnknownChar => "unknown character",
            SyntaxKind::Error => "error",
            _ => unreachable!("Unreachable kind: {:?}", self),
        };

        s.to_string()
    }

    pub fn code_repr(self) -> Option<String> {
        let s = match self {
            SyntaxKind::Sym_Ampersand => "&",
            SyntaxKind::Sym_Asterisk => "*",
            SyntaxKind::Sym_At => "@",
            SyntaxKind::Sym_BackSlash => "\\",
            SyntaxKind::Sym_Bang => "!",
            SyntaxKind::Sym_BangEq => "!=",
            SyntaxKind::Sym_Caret => "^",
            SyntaxKind::Sym_Colon => ":",
            SyntaxKind::Sym_Comma => ",",
            SyntaxKind::Sym_Dollar => "$",
            SyntaxKind::Sym_Dot => ".",
            SyntaxKind::Sym_EmDash => "—",
            SyntaxKind::Sym_EnDash => "–",
            SyntaxKind::Sym_Eq => "=",
            SyntaxKind::Sym_ForwardSlash => "/",
            SyntaxKind::Sym_Minus => "-",
            SyntaxKind::Sym_Percent => "%",
            SyntaxKind::Sym_Pipe => "|",
            SyntaxKind::Sym_Plus => "+",
            SyntaxKind::Sym_Question => "?",
            SyntaxKind::Sym_Semicolon => ";",
            SyntaxKind::Sym_Sterling => "£",
            SyntaxKind::Sym_Tilde => "~",
            SyntaxKind::Sym_Lt => "<",
            SyntaxKind::Sym_LtEq => "<=",
            SyntaxKind::Sym_Gt => ">",
            SyntaxKind::Sym_GtEq => ">=",
            SyntaxKind::Sym_LThinArrow => "<-",
            SyntaxKind::Sym_RThinArrow => "->",
            SyntaxKind::Sym_ThickArrow => "=>",
            SyntaxKind::Sym_Walrus => ":=",
            SyntaxKind::Sym_LBrace => "{",
            SyntaxKind::Sym_RBrace => "}",
            SyntaxKind::Sym_LBracket => "[",
            SyntaxKind::Sym_RBracket => "]",
            SyntaxKind::Sym_LParen => "(",
            SyntaxKind::Sym_RParen => ")",
            _ => return None,
        };

        Some(s.to_string())
    }

    pub fn example(self) -> Option<String> {
        let s = match self {
            SyntaxKind::Lit_Character => "'a'",
            SyntaxKind::Lit_Float => "123.456",
            SyntaxKind::Lit_Integer => "123",
            SyntaxKind::Lit_String => r#""hello, world!""#,
            SyntaxKind::Identifier => "foo",
            _ => return None,
        };

        Some(s.to_string())
    }
}

impl From<SyntaxKind> for rowan::SyntaxKind {
    fn from(kind: SyntaxKind) -> Self {
        Self(kind as u16)
    }
}

impl Display for SyntaxKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", FormattedString::from(self.human_readable_repr()))
    }
}

/// An array of all the keywords defined in the Helios grammar.
pub const KEYWORDS: &[&str] = &[
    "and", "as", "case", "else", "enum", "for", "forall", "func", "if", "impl",
    "import", "in", "iter", "let", "module", "not", "of", "or", "range",
    "record", "return", "test", "trait", "type", "var", "while", "with",
    "yield",
];

/// Creates a new symbol variant of [`SyntaxKind`] that corresponds to the given
/// character.
///
/// This function panics if an invalid character is given.
///
/// # Examples
///
/// ```rust
/// use helios_syntax::{symbol_from_char, SyntaxKind};
///
/// assert_eq!(symbol_from_char('@'), SyntaxKind::Sym_At);
/// assert_eq!(symbol_from_char('%'), SyntaxKind::Sym_Percent);
/// assert_eq!(symbol_from_char('$'), SyntaxKind::Sym_Dollar);
/// ```
pub fn symbol_from_char(c: char) -> SyntaxKind {
    match c {
        '&' => SyntaxKind::Sym_Ampersand,
        '*' => SyntaxKind::Sym_Asterisk,
        '@' => SyntaxKind::Sym_At,
        '\\' => SyntaxKind::Sym_BackSlash,
        '!' => SyntaxKind::Sym_Bang,
        '^' => SyntaxKind::Sym_Caret,
        ':' => SyntaxKind::Sym_Colon,
        ',' => SyntaxKind::Sym_Comma,
        '$' => SyntaxKind::Sym_Dollar,
        '.' => SyntaxKind::Sym_Dot,
        '—' => SyntaxKind::Sym_EmDash,
        '–' => SyntaxKind::Sym_EnDash,
        '=' => SyntaxKind::Sym_Eq,
        '/' => SyntaxKind::Sym_ForwardSlash,
        '-' => SyntaxKind::Sym_Minus,
        '%' => SyntaxKind::Sym_Percent,
        '|' => SyntaxKind::Sym_Pipe,
        '+' => SyntaxKind::Sym_Plus,
        '?' => SyntaxKind::Sym_Question,
        ';' => SyntaxKind::Sym_Semicolon,
        '£' => SyntaxKind::Sym_Sterling,
        '~' => SyntaxKind::Sym_Tilde,
        '<' => SyntaxKind::Sym_Lt,
        '>' => SyntaxKind::Sym_Gt,
        '{' => SyntaxKind::Sym_LBrace,
        '}' => SyntaxKind::Sym_RBrace,
        '[' => SyntaxKind::Sym_LBracket,
        ']' => SyntaxKind::Sym_RBracket,
        '(' => SyntaxKind::Sym_LParen,
        ')' => SyntaxKind::Sym_RParen,
        _ => panic!("Character `{c}` is not a valid Symbol"),
    }
}

/// Creates a new symbol variant of [`SyntaxKind`] that corresponds to the given
/// sequence of characters.
///
/// # Examples
///
/// ```rust
/// use helios_syntax::{symbol_from_chars, SyntaxKind};
///
/// assert_eq!(symbol_from_chars(&['!', '=']), Some(SyntaxKind::Sym_BangEq));
/// assert_eq!(symbol_from_chars(&['>', '=']), Some(SyntaxKind::Sym_GtEq));
/// assert_eq!(symbol_from_chars(&['?', '?']), None);
/// ```
pub fn symbol_from_chars(chars: &[char]) -> Option<SyntaxKind> {
    match chars {
        ['!', '='] => Some(SyntaxKind::Sym_BangEq),
        ['<', '='] => Some(SyntaxKind::Sym_LtEq),
        ['>', '='] => Some(SyntaxKind::Sym_GtEq),
        ['<', '-'] => Some(SyntaxKind::Sym_LThinArrow),
        ['-', '>'] => Some(SyntaxKind::Sym_RThinArrow),
        ['=', '>'] => Some(SyntaxKind::Sym_ThickArrow),
        [':', '='] => Some(SyntaxKind::Sym_Walrus),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! check {
        ([$( $cs:expr ),+ $(,)?] => $kind:ident) => {{
            assert_eq!(symbol_from_chars(&[$($cs),*]), Some(SyntaxKind::$kind));
        }};
        ($c:expr => $kind:ident) => {{
            assert_eq!(symbol_from_char($c), SyntaxKind::$kind);
        }};
    }

    #[test]
    fn test_symbol_from_char() {
        check!('&' => Sym_Ampersand);
        check!('*' => Sym_Asterisk);
        check!('@' => Sym_At);
        check!('\\'=> Sym_BackSlash);
        check!('!' => Sym_Bang);
        check!('^' => Sym_Caret);
        check!(':' => Sym_Colon);
        check!(',' => Sym_Comma);
        check!('$' => Sym_Dollar);
        check!('.' => Sym_Dot);
        check!('—' => Sym_EmDash);
        check!('–' => Sym_EnDash);
        check!('=' => Sym_Eq);
        check!('/' => Sym_ForwardSlash);
        check!('-' => Sym_Minus);
        check!('%' => Sym_Percent);
        check!('|' => Sym_Pipe);
        check!('+' => Sym_Plus);
        check!('?' => Sym_Question);
        check!(';' => Sym_Semicolon);
        check!('£' => Sym_Sterling);
        check!('~' => Sym_Tilde);
        check!('<' => Sym_Lt);
        check!('>' => Sym_Gt);
        check!('{' => Sym_LBrace);
        check!('}' => Sym_RBrace);
        check!('[' => Sym_LBracket);
        check!(']' => Sym_RBracket);
        check!('(' => Sym_LParen);
        check!(')' => Sym_RParen);
    }

    #[test]
    fn test_symbol_from_two_chars() {
        check!(['!', '='] => Sym_BangEq);
        check!(['<', '='] => Sym_LtEq);
        check!(['>', '='] => Sym_GtEq);
        check!(['<', '-'] => Sym_LThinArrow);
        check!(['-', '>'] => Sym_RThinArrow);
        check!(['=', '>'] => Sym_ThickArrow);
        check!([':', '='] => Sym_Walrus);
    }

    #[test]
    fn test_is_trivia() {
        assert!(SyntaxKind::Comment.is_trivia());
        assert!(SyntaxKind::DocComment.is_trivia());
        assert!(SyntaxKind::Whitespace.is_trivia());

        assert!(!SyntaxKind::Kwd_And.is_trivia());
        assert!(!SyntaxKind::Sym_Ampersand.is_trivia());
        assert!(!SyntaxKind::Lit_Character.is_trivia());
        assert!(!SyntaxKind::Root.is_trivia());
    }

    #[test]
    fn test_is_symbol() {
        assert!(SyntaxKind::Sym_Ampersand.is_symbol());
        assert!(SyntaxKind::Sym_Asterisk.is_symbol());
        assert!(SyntaxKind::Sym_Tilde.is_symbol());
        assert!(SyntaxKind::Sym_LParen.is_symbol());
        assert!(SyntaxKind::Sym_RParen.is_symbol());

        assert!(!SyntaxKind::Kwd_And.is_symbol());
        assert!(!SyntaxKind::Lit_Character.is_symbol());
        assert!(!SyntaxKind::Root.is_symbol());
    }

    #[test]
    fn test_syntax_kind_human_readable_repr() {
        fn check(kind: SyntaxKind, input: impl Into<String>) {
            assert_eq!(format!("{}", kind.human_readable_repr()), input.into());
        }

        use SyntaxKind::*;

        check(Kwd_And, "the and keyword");
        check(Kwd_Enum, "the enum keyword");
        check(Kwd_Impl, "the impl keyword");
        check(Kwd_Module, "the module keyword");
        check(Kwd_Record, "the record keyword");
        check(Kwd_Yield, "the yield keyword");

        check(Sym_Ampersand, "an ampersand symbol (`&`)");
        check(Sym_ForwardSlash, "a forward slash symbol (`/`)");
        check(Sym_Lt, "a less than symbol (`<`)");
        check(Sym_LtEq, "a less than equal symbol (`<=`)");
        check(Sym_Walrus, "a walrus symbol (`:=`)");

        check(Sym_LBrace, "an opening curly brace symbol (`{`)");
        check(Sym_LBracket, "an opening square bracket symbol (`[`)");
        check(Sym_LParen, "an opening parenthesis symbol (`(`)");
        check(Sym_RBrace, "a closing curly brace symbol (`}`)");
        check(Sym_RBracket, "a closing square bracket symbol (`]`)");
        check(Sym_RParen, "a closing parenthesis symbol (`)`)");

        check(Lit_Character, "a character literal (such as `'a'`)");
        check(Lit_Float, "a float literal (such as `123.456`)");
        check(Lit_Integer, "an integer literal (such as `123`)");
        check(Lit_String, "a string literal (such as `\"hello, world!\"`)");

        check(Exp_Binary, "a binary expression");
        check(Exp_Indented, "an indented expression");
        check(Exp_Literal, "a literal expression");
        check(Exp_Paren, "a parenthesized expression");
        check(Exp_UnaryPrefix, "a prefixed unary expression");
        check(Exp_UnaryPostfix, "a postfixed unary expression");
        check(Exp_VariableRef, "a variable reference expression");
        check(Exp_Unnamed, "an expression");

        check(Dec_GlobalBinding, "a global binding declaration");

        check(Comment, "a comment");
        check(DocComment, "a documentation comment");
        check(Whitespace, "a whitespace");

        check(Indent, "an indent");
        check(Dedent, "a dedent");
        check(Newline, "a new line");

        check(Identifier, "an identifier (such as `foo`)");
        check(ReservedIdentifier, "a reserved identifier");
        check(Placeholder, "a placeholder");
        check(Error, "an error");
    }
}
