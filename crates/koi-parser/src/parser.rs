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

mod expr;

use crate::lexer::Lexer;
use koi_syntax::{SyntaxKind, SyntaxNode};
use rowan::{Checkpoint, GreenNode, GreenNodeBuilder};
use std::iter::Peekable;

/// A lazy, lossless, error-tolerant parser for the Koi programming language.
pub struct Parser {
    lexer: Peekable<Lexer>,
    builder: GreenNodeBuilder<'static>,
}

impl Parser {
    /// Construct a new [`Parser`] with a given source text.
    pub fn new(source: String) -> Self {
        Self {
            lexer: Lexer::new(source).peekable(),
            builder: GreenNodeBuilder::new(),
        }
    }

    /// Start the parsing process.
    ///
    /// This function will attempt to build a concrete syntax tree of the given
    /// source text (no matter how invalid it is). Once done, it will return a
    /// [`ParserResult`] containing the root green node.
    pub fn parse(mut self) -> ParserResult {
        self.builder.start_node(SyntaxKind::Root.into());
        expr::parse_expr(&mut self, 0);
        self.builder.finish_node();

        ParserResult {
            green_node: self.builder.finish(),
        }
    }
}

impl Parser {
    /// Peeks the next [`SyntaxKind`] token without consuming it.
    pub(crate) fn peek(&mut self) -> Option<SyntaxKind> {
        self.lexer.peek().map(|lexeme| lexeme.kind)
    }

    /// Adds the next token to the syntax tree (via the [`GreenNodeBuilder`]).
    fn bump(&mut self) {
        let lexeme = self.lexer.next().expect("Failed to get next token");
        self.builder.token(
            lexeme.clone().kind.into(),
            lexeme.clone().text.clone().into(),
        )
    }

    fn start_node_at(&mut self, checkpoint: Checkpoint, kind: SyntaxKind) {
        self.builder.start_node_at(checkpoint, kind.into());
    }

    fn finish_node(&mut self) {
        self.builder.finish_node();
    }

    fn checkpoint(&mut self) -> Checkpoint {
        self.builder.checkpoint()
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
    let parse_result = Parser::new(input.to_string()).parse();
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
