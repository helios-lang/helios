//! Parsing Helios source files.
//!
//! The showrunner of this module is the [`parse`] function. It is responsible
//! for parsing a given input and returning a concrete syntax tree (CST) with
//! the [`rowan`] library.
//!
//! [`rowan`]: https://docs.rs/rowan/0.10.0/rowan

mod event;
mod expr;
mod marker;
mod sink;
mod source;

use self::event::Event;
use self::marker::Marker;
use self::sink::Sink;
use crate::lexer::{Lexer, Token};
use helios_syntax::{SyntaxKind, SyntaxNode};
use rowan::GreenNode;
use source::Source;

/// Parse the given source text into a [`Parse`].
pub fn parse(source: &str) -> Parse {
    let tokens = Lexer::new(source).collect::<Vec<_>>();
    let parser = Parser::new(&tokens);
    let events = parser.parse();
    let sink = Sink::new(&tokens, events);

    Parse {
        green_node: sink.finish(),
    }
}

/// The result of parsing a source text.
pub struct Parse {
    /// The root green node of the syntax tree.
    green_node: GreenNode,
}

impl Parse {
    /// Returns a formatted string representation of the syntax tree.
    pub fn debug_tree(&self) -> String {
        let syntax_node = SyntaxNode::new_root(self.green_node.clone());
        let formatted = format!("{:#?}", syntax_node);

        // trims newline at the end
        formatted[0..formatted.len() - 1].to_string()
    }
}

/// A lazy, lossless, error-tolerant parser for the Helios programming language.
struct Parser<'tokens, 'source> {
    source: Source<'tokens, 'source>,
    events: Vec<Event>,
}

impl<'tokens, 'source> Parser<'tokens, 'source> {
    /// Construct a new [`Parser`] with a given slice of [`Token`]s.
    pub fn new(tokens: &'tokens [Token<'source>]) -> Self {
        Self {
            source: Source::new(tokens),
            events: Vec::new(),
        }
    }

    /// Start the parsing process.
    ///
    /// This function will attempt to build a concrete syntax tree of the given
    /// source text (no matter how invalid it is). Once done, it will return a
    /// [`ParserResult`] containing the root green node.
    pub fn parse(mut self) -> Vec<Event> {
        let marker = self.start();
        expr::parse_expr(&mut self, 0);
        marker.complete(&mut self, SyntaxKind::Root);

        self.events
    }
}

impl<'tokens, 'source> Parser<'tokens, 'source> {
    /// Determines if the next [`SyntaxKind`] is the given `kind`.
    fn is_at(&mut self, kind: SyntaxKind) -> bool {
        self.peek() == Some(kind)
    }

    /// Peeks the next [`SyntaxKind`] token without consuming it.
    fn peek(&mut self) -> Option<SyntaxKind> {
        self.source.peek_kind()
    }

    /// Adds the next token to the syntax tree (via the [`GreenNodeBuilder`]).
    fn bump(&mut self) {
        self.source.next_token().unwrap();
        self.events.push(Event::AddToken)
    }

    /// Starts a new node, returning a [`Marker`].
    fn start(&mut self) -> Marker {
        let pos = self.events.len();
        self.events.push(Event::Placeholder);
        Marker::new(pos)
    }
}

#[cfg(test)]
fn check(input: &str, expected_tree: expect_test::Expect) {
    let parse = parse(input);
    expected_tree.assert_eq(&parse.debug_tree());
}

#[cfg(test)]
mod tests {
    use super::*;
    use expect_test::expect;

    #[test]
    fn test_parse_nothing() {
        check("", expect![[r#"Root@0..0"#]]);
    }

    #[test]
    fn test_parse_whitespace() {
        check(
            "   ",
            expect![[r#"
Root@0..3
  Whitespace@0..3 "   ""#]],
        );
    }

    #[test]
    fn test_parse_comment() {
        check(
            "// hello, world!",
            expect![[r#"
Root@0..16
  Comment@0..16 "// hello, world!""#]],
        );
    }

    #[test]
    fn test_parse_comment_followed_by_whitespace() {
        check(
            "// hello, world!\n",
            expect![[r#"
Root@0..17
  Comment@0..16 "// hello, world!"
  Whitespace@16..17 "\n""#]],
        );
    }

    #[test]
    fn test_parse_multiple_comments() {
        check(
            "
// hello, world!
// this is another line
",
            expect![[r#"
Root@0..42
  Whitespace@0..1 "\n"
  Comment@1..17 "// hello, world!"
  Whitespace@17..18 "\n"
  Comment@18..41 "// this is another line"
  Whitespace@41..42 "\n""#]],
        );
    }
}
