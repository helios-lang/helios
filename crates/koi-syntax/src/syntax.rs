use crate::lang::Language;

pub(crate) type SyntaxNode = rowan::SyntaxNode<Language>;

#[macro_export]
macro_rules! Sym {
    ["&"] => (SyntaxKind::Sym_Ampersand);
    ["*"] => (SyntaxKind::Sym_Asterisk);
    ["@"] => (SyntaxKind::Sym_At);
    ["!"] => (SyntaxKind::Sym_Bang);
    ["!="]=> (SyntaxKind::Sym_BangEq);
    ["^"] => (SyntaxKind::Sym_Caret);
    [","] => (SyntaxKind::Sym_Comma);
    ["$"] => (SyntaxKind::Sym_Dollar);
    ["."] => (SyntaxKind::Sym_Dot);
    ["—"] => (SyntaxKind::Sym_EmDash);
    ["–"] => (SyntaxKind::Sym_EnDash);
    ["="] => (SyntaxKind::Sym_Eq);
    ["/"] => (SyntaxKind::Sym_ForwardSlash);
    ["-"] => (SyntaxKind::Sym_Minus);
    ["%"] => (SyntaxKind::Sym_Percent);
    ["|"] => (SyntaxKind::Sym_Pipe);
    ["+"] => (SyntaxKind::Sym_Plus);
    ["#"] => (SyntaxKind::Sym_Pound);
    ["?"] => (SyntaxKind::Sym_Question);
    [";"] => (SyntaxKind::Sym_Semicolon);
    ["£"] => (SyntaxKind::Sym_Sterling);
    ["~"] => (SyntaxKind::Sym_Tilde);

    ["<"] => (SyntaxKind::Sym_Lt);
    ["<="]=> (SyntaxKind::Sym_LtEq);
    [">"] => (SyntaxKind::Sym_Gt);
    [">="]=> (SyntaxKind::Sym_GtEq);
    ["<-"]=> (SyntaxKind::Sym_LThinArrow);
    ["->"]=> (SyntaxKind::Sym_RThinArrow);
    ["=>"]=> (SyntaxKind::Sym_ThickArrow);
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(u16)]
pub enum SyntaxKind {
    Kwd_Alias,
    Kwd_And,
    Kwd_As,
    Kwd_Const,
    Kwd_Else,
    Kwd_Extend,
    Kwd_External,
    Kwd_For,
    Kwd_Function,
    Kwd_If,
    Kwd_Import,
    Kwd_In,
    Kwd_Interface,
    Kwd_Internal,
    Kwd_Let,
    Kwd_Match,
    Kwd_Module,
    Kwd_Not,
    Kwd_Of,
    Kwd_Or,
    Kwd_Public,
    Kwd_Ref,
    Kwd_Return,
    Kwd_Take,
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
    Exp_Unary,

    LineComment,
    LineDocComment,
    Whitespace,

    Identifier,
    Error,
    Root, // this should be last
}

impl SyntaxKind {
    pub fn symbol_from_char(c: char) -> Self {
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

    pub fn symbol_from_two_chars(first: char, second: char) -> Option<Self> {
        match (first, second) {
            ('!', '=') => Some(SyntaxKind::Sym_BangEq),
            ('<', '=') => Some(SyntaxKind::Sym_LtEq),
            ('>', '=') => Some(SyntaxKind::Sym_GtEq),
            ('<', '-') => Some(SyntaxKind::Sym_LThinArrow),
            ('-', '>') => Some(SyntaxKind::Sym_RThinArrow),
            ('=', '>') => Some(SyntaxKind::Sym_ThickArrow),
            _ => None,
        }
    }
}

impl From<SyntaxKind> for rowan::SyntaxKind {
    fn from(kind: SyntaxKind) -> Self {
        Self(kind as u16)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_symbol_from_char() {
        macro_rules! expect {
            ($c:expr => $kind:ident) => {{
                assert_eq!(SyntaxKind::symbol_from_char($c), SyntaxKind::$kind);
            }};
        }

        expect!('&' => Sym_Ampersand);
        expect!('*' => Sym_Asterisk);
        expect!('@' => Sym_At);
        expect!('\\'=> Sym_BackSlash);
        expect!('!' => Sym_Bang);
        expect!('^' => Sym_Caret);
        expect!(':' => Sym_Colon);
        expect!(',' => Sym_Comma);
        expect!('$' => Sym_Dollar);
        expect!('.' => Sym_Dot);
        expect!('—' => Sym_EmDash);
        expect!('–' => Sym_EnDash);
        expect!('=' => Sym_Eq);
        expect!('/' => Sym_ForwardSlash);
        expect!('-' => Sym_Minus);
        expect!('%' => Sym_Percent);
        expect!('|' => Sym_Pipe);
        expect!('+' => Sym_Plus);
        expect!('#' => Sym_Pound);
        expect!('?' => Sym_Question);
        expect!(';' => Sym_Semicolon);
        expect!('£' => Sym_Sterling);
        expect!('~' => Sym_Tilde);
        expect!('<' => Sym_Lt);
        expect!('>' => Sym_Gt);
        expect!('{' => Sym_LBrace);
        expect!('}' => Sym_RBrace);
        expect!('[' => Sym_LBracket);
        expect!(']' => Sym_RBracket);
        expect!('(' => Sym_LParen);
        expect!(')' => Sym_RParen);
    }

    #[test]
    fn test_symbol_from_two_chars() {
        macro_rules! expect {
            ($a:expr, $b:expr => $kind:ident) => {{
                assert_eq!(
                    SyntaxKind::symbol_from_two_chars($a, $b),
                    Some(SyntaxKind::$kind)
                );
            }};
        }

        expect!('!', '=' => Sym_BangEq);
        expect!('<', '=' => Sym_LtEq);
        expect!('>', '=' => Sym_GtEq);
        expect!('<', '-' => Sym_LThinArrow);
        expect!('-', '>' => Sym_RThinArrow);
        expect!('=', '>' => Sym_ThickArrow);
    }
}
