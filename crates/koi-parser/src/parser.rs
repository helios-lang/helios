//! Parsing Koi source files.
//!
//! The showrunner of this module is the [`Parser`] type. It is responsible for
//! parsing an input (a `String`) and returning a concrete syntax tree (CST)
//! using the [`rowan`] library.
//!
//! Refer to [`Parser`] and [`ParserResult`] for more information on how parsing
//! is done.
//!
//! [`rowan`]: https://docs.rs/rowan/0.10.0/rowan

mod event;
mod expr;
mod sink;

use self::event::Event;
use self::sink::Sink;
use crate::lexer::{Lexeme, Lexer};
use koi_syntax::{SyntaxKind, SyntaxNode};
use rowan::GreenNode;
use std::iter::Peekable;

/// A lazy, lossless, error-tolerant parser for the Koi programming language.
pub struct Parser<'src> {
    lexer: Peekable<Lexer<'src>>,
    events: Vec<Event>,
}

impl<'src> Parser<'src> {
    /// Construct a new [`Parser`] with a given source text.
    pub fn new(source: &'src str) -> Self {
        Self {
            lexer: Lexer::new(source).peekable(),
            events: Vec::new(),
        }
    }

    /// Start the parsing process.
    ///
    /// This function will attempt to build a concrete syntax tree of the given
    /// source text (no matter how invalid it is). Once done, it will return a
    /// [`ParserResult`] containing the root green node.
    pub fn parse(mut self) -> ParserResult {
        self.start_node(SyntaxKind::Root.into());
        expr::parse_expr(&mut self, 0);
        self.finish_node();

        let sink = Sink::new(self.events);

        ParserResult {
            green_node: sink.finish(),
        }
    }
}

impl<'src> Parser<'src> {
    /// Peeks the next [`SyntaxKind`] token without consuming it.
    fn peek(&mut self) -> Option<SyntaxKind> {
        self.lexer.peek().map(|lexeme| lexeme.kind)
    }

    /// Adds the next token to the syntax tree (via the [`GreenNodeBuilder`]).
    fn bump(&mut self) {
        let Lexeme { kind, text } = self.lexer.next().unwrap();
        self.events.push(Event::AddToken {
            kind,
            text: text.into(),
        })
    }

    fn start_node(&mut self, kind: SyntaxKind) {
        self.events.push(Event::StartNode { kind });
    }

    fn start_node_at(&mut self, checkpoint: usize, kind: SyntaxKind) {
        self.events.push(Event::StartNodeAt { kind, checkpoint });
    }

    fn finish_node(&mut self) {
        self.events.push(Event::FinishNode);
    }

    fn checkpoint(&mut self) -> usize {
        self.events.len()
    }
}

/// The result of parsing a source text.
pub struct ParserResult {
    /// The root green node of the syntax tree.
    green_node: GreenNode,
}

impl ParserResult {
    /// Returns a formatted string representation of the syntax tree.
    pub fn debug_tree(&self) -> String {
        let syntax_node = SyntaxNode::new_root(self.green_node.clone());
        let formatted = format!("{:#?}", syntax_node);

        // trims newline at the end
        formatted[0..formatted.len() - 1].to_string()
    }
}

#[cfg(test)]
fn check(input: &str, expected_tree: expect_test::Expect) {
    let parse_result = Parser::new(input).parse();
    expected_tree.assert_eq(&parse_result.debug_tree());
}

#[cfg(test)]
mod tests {
    use super::*;
    use expect_test::expect;

    #[test]
    fn test_parse_nothing() {
        check("", expect![[r#"Root@0..0"#]]);
    }
}
