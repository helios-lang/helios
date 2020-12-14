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
mod source;

use self::event::Event;
use self::sink::Sink;
use crate::lexer::{Lexeme, Lexer};
use koi_syntax::{SyntaxKind, SyntaxNode};
use rowan::GreenNode;
use source::Source;

pub fn parse(source: &str) -> ParserResult {
    let lexemes = Lexer::new(source).collect::<Vec<_>>();
    let parser = Parser::new(&lexemes);
    let events = parser.parse();
    let sink = Sink::new(&lexemes, events);

    ParserResult {
        green_node: sink.finish(),
    }
}

/// A lazy, lossless, error-tolerant parser for the Koi programming language.
struct Parser<'lexemes, 'source> {
    source: Source<'lexemes, 'source>,
    events: Vec<Event>,
}

impl<'lexemes, 'source> Parser<'lexemes, 'source> {
    /// Construct a new [`Parser`] with a given slice of [`Lexeme`]s.
    pub fn new(lexemes: &'lexemes [Lexeme<'source>]) -> Self {
        Self {
            source: Source::new(lexemes),
            events: Vec::new(),
        }
    }

    /// Start the parsing process.
    ///
    /// This function will attempt to build a concrete syntax tree of the given
    /// source text (no matter how invalid it is). Once done, it will return a
    /// [`ParserResult`] containing the root green node.
    pub fn parse(mut self) -> Vec<Event> {
        self.start_node(SyntaxKind::Root.into());
        expr::parse_expr(&mut self, 0);
        self.finish_node();

        self.events
    }
}

impl<'lexeme, 'source> Parser<'lexeme, 'source> {
    /// Peeks the next [`SyntaxKind`] token without consuming it.
    fn peek(&mut self) -> Option<SyntaxKind> {
        self.source.peek_kind()
    }

    /// Adds the next token to the syntax tree (via the [`GreenNodeBuilder`]).
    fn bump(&mut self) {
        let Lexeme { kind, text } = self.source.next_lexeme().unwrap();
        self.events.push(Event::AddToken {
            kind: *kind,
            text: (*text).into(),
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
    let parse_result = parse(input);
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
