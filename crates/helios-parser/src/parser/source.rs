use std::ops::Range;

use helios_syntax::SyntaxKind;

use crate::lexer::Token;

/// An abstraction over traversing a slice of [`Token`]s.
///
/// This structure is responsible for providing [`Parser`] tokens. It consumes
/// irrelevant tokens (called *trivia*) such as line comments and whitespace.
/// This makes it easier to use [`Parser`] without having to handle such tokens.
///
/// Note that trivia tokens are not completely discarded. The [`Sink`]
/// structure, in short, is able to create a syntax tree with the [`Event`]s
/// produced by the [`Parser`]. As it does this, it knows when trivia tokens
/// have been skipped and "glues" or "inserts" them into the final syntax tree.
/// Refer to [`Sink`]'s documentation for more information on how it does this.
///
/// [`Event`]: crate::parser::event::Parser
/// [`Parser`]: crate::parser::Parser
/// [`Sink`]: crate::parser::sink::Sink
pub struct Source<'source, 'tokens> {
    tokens: &'tokens [Token<'source>],
    cursor: usize,
}

impl<'source, 'tokens> Source<'source, 'tokens> {
    /// Construct a new [`Source`] with a given slice of [`Token`]s.
    pub fn new(tokens: &'tokens [Token<'source>]) -> Self {
        Self { tokens, cursor: 0 }
    }

    pub fn next_token(&mut self) -> Option<&'tokens Token<'source>> {
        self.eat_trivia();

        let token = self.tokens.get(self.cursor)?;
        self.cursor += 1;

        Some(token)
    }

    #[allow(dead_code)]
    pub(crate) fn last_token_range(&self) -> Option<Range<usize>> {
        self.tokens.last().map(|Token { range, .. }| range.clone())
    }

    pub fn peek_kind(&mut self) -> Option<SyntaxKind> {
        self.eat_trivia();
        self.peek_kind_raw()
    }

    pub fn peek_token(&mut self) -> Option<&Token> {
        self.eat_trivia();
        self.peek_token_raw()
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
        self.peek_token_raw().map(|Token { kind, .. }| *kind)
    }

    fn peek_token_raw(&self) -> Option<&Token> {
        self.tokens.get(self.cursor)
    }
}
