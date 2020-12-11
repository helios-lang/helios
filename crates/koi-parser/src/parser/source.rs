use koi_syntax::SyntaxKind;

use crate::lexer::Lexeme;

pub(super) struct Source<'lexemes, 'source> {
    lexemes: &'lexemes [Lexeme<'source>],
    cursor: usize,
}

impl<'lexemes, 'source> Source<'lexemes, 'source> {
    pub(super) fn new(lexemes: &'lexemes [Lexeme<'source>]) -> Self {
        Self { lexemes, cursor: 0 }
    }

    pub(super) fn next_lexeme(&mut self) -> Option<&'lexemes Lexeme<'source>> {
        self.eat_trivia();

        let lexeme = self.lexemes.get(self.cursor)?;
        self.cursor += 1;

        Some(lexeme)
    }

    pub(super) fn peek_kind(&mut self) -> Option<SyntaxKind> {
        self.eat_trivia();
        self.peek_kind_raw()
    }

    fn eat_trivia(&mut self) {
        while self.peek_kind_raw() == Some(SyntaxKind::Whitespace) {
            self.cursor += 1;
        }
    }

    fn peek_kind_raw(&self) -> Option<SyntaxKind> {
        self.lexemes
            .get(self.cursor)
            .map(|Lexeme { kind, .. }| *kind)
    }
}
