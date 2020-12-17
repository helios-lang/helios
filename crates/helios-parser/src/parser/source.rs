use helios_syntax::SyntaxKind;

use crate::lexer::Token;

pub(super) struct Source<'tokens, 'source> {
    tokens: &'tokens [Token<'source>],
    cursor: usize,
}

impl<'tokens, 'source> Source<'tokens, 'source> {
    pub(super) fn new(tokens: &'tokens [Token<'source>]) -> Self {
        Self { tokens, cursor: 0 }
    }

    pub(super) fn next_token(&mut self) -> Option<&'tokens Token<'source>> {
        self.eat_trivia();

        let token = self.tokens.get(self.cursor)?;
        self.cursor += 1;

        Some(token)
    }

    pub(super) fn peek_kind(&mut self) -> Option<SyntaxKind> {
        self.eat_trivia();
        self.peek_kind_raw()
    }

    fn eat_trivia(&mut self) {
        while self.at_trivia() {
            self.cursor += 1;
        }
    }

    fn at_trivia(&self) -> bool {
        self.peek_kind_raw().map_or(false, SyntaxKind::is_trivia)
    }

    fn peek_kind_raw(&self) -> Option<SyntaxKind> {
        self.tokens.get(self.cursor).map(|Token { kind, .. }| *kind)
    }
}
