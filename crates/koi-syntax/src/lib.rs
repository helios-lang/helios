mod lang;

use lang::Language;

pub type SyntaxNode = rowan::SyntaxNode<Language>;

/// A convenient way to construct a new `SyntaxNode`.
///
/// # Examples
/// ```rust
/// use koi_syntax::Sym;
/// assert_eq!(Sym!["$"], koi_syntax::SyntaxKind::Sym_Dollar);
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

/// All the possible nodes and tokens defined in the Koi grammar.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(u16)]
pub enum SyntaxKind {
    Kwd_Alias,
    Kwd_And,
    Kwd_As,
    Kwd_Else,
    Kwd_Extend,
    Kwd_External,
    Kwd_For,
    Kwd_Function,
    Kwd_If,
    Kwd_Import,
    Kwd_In,
    Kwd_Interface,
    Kwd_Let,
    Kwd_Match,
    Kwd_Module,
    Kwd_Not,
    Kwd_Of,
    Kwd_Or,
    Kwd_Public,
    Kwd_Ref,
    Kwd_Return,
    Kwd_Type,
    Kwd_Unimplemented,
    Kwd_Var,
    Kwd_Where,
    Kwd_While,
    Kwd_With,

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
    Sym_Pound,
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
    Exp_Paren,
    Exp_UnaryPrefix,
    Exp_UnaryPostfix,

    LineComment,
    LineDocComment,
    Whitespace,

    Identifier,
    Error,
    Root, // this should be last
}

impl From<SyntaxKind> for rowan::SyntaxKind {
    fn from(kind: SyntaxKind) -> Self {
        Self(kind as u16)
    }
}

/// A list of all the keywords defined in the Koi grammar.
pub fn keyword_list() -> Vec<String> {
    [
        "alias",
        "and",
        "as",
        "else",
        "extend",
        "external",
        "for",
        "function",
        "if",
        "import",
        "in",
        "interface",
        "let",
        "match",
        "module",
        "not",
        "of",
        "or",
        "public",
        "ref",
        "return",
        "type",
        "var",
        "where",
        "while",
        "with",
    ]
    .iter()
    .map(|s| String::from(*s))
    .collect()
}

/// Create a new symbol variant of [`SyntaxKind`] that corresponds to the given
/// character.
///
/// This function panics if an invalid character is given.
///
/// # Examples
///
/// ```rust
/// use koi_syntax::{symbol_from_char, SyntaxKind};
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
        '#' => SyntaxKind::Sym_Pound,
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
        _ => panic!("Character `{}` is not a valid Symbol", c),
    }
}

/// Create a new symbol variant of [`SyntaxKind`] that corresponds to the given
/// sequence of characters.
///
/// # Examples
///
/// ```rust
/// use koi_syntax::{symbol_from_chars, SyntaxKind};
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
        check!('#' => Sym_Pound);
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
    }
}
