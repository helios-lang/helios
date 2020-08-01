#![allow(dead_code)]

use crate::source::TextSpan;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SyntaxToken<'a> {
    pub(crate) _raw: &'a RawSyntaxToken,
    pub span: TextSpan,
}

impl<'a> SyntaxToken<'a> {
    pub fn new(raw: &'a RawSyntaxToken, span: TextSpan) -> Self {
        Self { _raw: raw, span }
    }

    pub fn kind(&self) -> TokenKind {
        self._raw.kind
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct RawSyntaxToken {
    kind: TokenKind,
}

impl RawSyntaxToken {
    pub fn new(kind: TokenKind) -> Self {
        Self { kind }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum TokenKind {
    Eof,
    Identifier,
    Keyword(Keyword),
    Literal,
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
    Type,
    Unimplemented,
    Using,
    Val,
    With,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum SyntaxTrivia {
    Tab(usize),
    Space(usize),
    LineFeed(usize),
    CarriageReturn(usize),
    CarriageReturnLineFeed(usize),
}
