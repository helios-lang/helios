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

    Identifier,

    LineComment,
    LineDocComment,
    Whitespace,

    Eof,
    Root,
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

    pub fn symbol_from_chars(first: char, second: char) -> Option<Self> {
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

#[test]
fn test_symbol_from_char() {
    use SyntaxKind::*;

    assert_eq!(SyntaxKind::symbol_from_char('&'), Sym_Ampersand);
    assert_eq!(SyntaxKind::symbol_from_char('*'), Sym_Asterisk);
    assert_eq!(SyntaxKind::symbol_from_char('@'), Sym_At);
    assert_eq!(SyntaxKind::symbol_from_char('\\'), Sym_BackSlash);
    assert_eq!(SyntaxKind::symbol_from_char('!'), Sym_Bang);
    assert_eq!(SyntaxKind::symbol_from_char('^'), Sym_Caret);
    assert_eq!(SyntaxKind::symbol_from_char(':'), Sym_Colon);
    assert_eq!(SyntaxKind::symbol_from_char(','), Sym_Comma);
    assert_eq!(SyntaxKind::symbol_from_char('$'), Sym_Dollar);
    assert_eq!(SyntaxKind::symbol_from_char('.'), Sym_Dot);
    assert_eq!(SyntaxKind::symbol_from_char('—'), Sym_EmDash);
    assert_eq!(SyntaxKind::symbol_from_char('–'), Sym_EnDash);
    assert_eq!(SyntaxKind::symbol_from_char('='), Sym_Eq);
    assert_eq!(SyntaxKind::symbol_from_char('/'), Sym_ForwardSlash);
    assert_eq!(SyntaxKind::symbol_from_char('-'), Sym_Minus);
    assert_eq!(SyntaxKind::symbol_from_char('%'), Sym_Percent);
    assert_eq!(SyntaxKind::symbol_from_char('|'), Sym_Pipe);
    assert_eq!(SyntaxKind::symbol_from_char('+'), Sym_Plus);
    assert_eq!(SyntaxKind::symbol_from_char('#'), Sym_Pound);
    assert_eq!(SyntaxKind::symbol_from_char('?'), Sym_Question);
    assert_eq!(SyntaxKind::symbol_from_char(';'), Sym_Semicolon);
    assert_eq!(SyntaxKind::symbol_from_char('£'), Sym_Sterling);
    assert_eq!(SyntaxKind::symbol_from_char('~'), Sym_Tilde);
    assert_eq!(SyntaxKind::symbol_from_char('<'), Sym_Lt);
    assert_eq!(SyntaxKind::symbol_from_char('>'), Sym_Gt);
    assert_eq!(SyntaxKind::symbol_from_char('{'), Sym_LBrace);
    assert_eq!(SyntaxKind::symbol_from_char('}'), Sym_RBrace);
    assert_eq!(SyntaxKind::symbol_from_char('['), Sym_LBracket);
    assert_eq!(SyntaxKind::symbol_from_char(']'), Sym_RBracket);
    assert_eq!(SyntaxKind::symbol_from_char('('), Sym_LParen);
    assert_eq!(SyntaxKind::symbol_from_char(')'), Sym_RParen);
}

#[test]
fn test_symbol_composed_from_chars() {
    use SyntaxKind::*;

    assert_eq!(SyntaxKind::symbol_from_chars('!', '='), Some(Sym_BangEq));
    assert_eq!(SyntaxKind::symbol_from_chars('<', '='), Some(Sym_LtEq));
    assert_eq!(SyntaxKind::symbol_from_chars('>', '='), Some(Sym_GtEq));
    assert_eq!(SyntaxKind::symbol_from_chars('<', '-'), Some(Sym_LThinArrow));
    assert_eq!(SyntaxKind::symbol_from_chars('-', '>'), Some(Sym_RThinArrow));
    assert_eq!(SyntaxKind::symbol_from_chars('=', '>'), Some(Sym_ThickArrow));
}
