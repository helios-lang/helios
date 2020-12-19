//! Module responsible for parsing Helios source files.

pub(crate) mod event;
pub(crate) mod marker;
pub(crate) mod sink;
pub(crate) mod source;

use self::event::Event;
use self::marker::Marker;
use self::source::Source;
use helios_syntax::SyntaxKind;

/// A lazy, lossless, error-tolerant parser for the Helios programming language.
pub struct Parser<'tokens, 'source> {
    source: Source<'tokens, 'source>,
    events: Vec<Event>,
}

impl<'tokens, 'source> Parser<'tokens, 'source> {
    /// Construct a new [`Parser`] with a [`Source`].
    pub fn new(source: Source<'tokens, 'source>) -> Self {
        Self {
            source,
            events: Vec::new(),
        }
    }

    /// Starts the parsing process.
    ///
    /// This function will attempt to build a concrete syntax tree with the
    /// given source text (no matter how invalid it is). Once done, it will
    /// return a [`Parse`] containing a root green node.
    ///
    /// [`Parse`]: crate::Parse
    pub fn parse(mut self) -> Vec<Event> {
        crate::grammar::root(&mut self);
        self.events
    }
}

impl<'tokens, 'source> Parser<'tokens, 'source> {
    /// Determines if the next [`SyntaxKind`] is the given `kind`.
    pub(crate) fn is_at(&mut self, kind: SyntaxKind) -> bool {
        self.peek() == Some(kind)
    }

    /// Peeks the next [`SyntaxKind`] token without consuming it.
    pub(crate) fn peek(&mut self) -> Option<SyntaxKind> {
        self.source.peek_kind()
    }

    /// Adds the next token to the syntax tree (via the [`GreenNodeBuilder`]).
    pub(crate) fn bump(&mut self) {
        self.source.next_token().unwrap();
        self.events.push(Event::AddToken)
    }

    /// Starts a new node, returning a [`Marker`].
    pub(crate) fn start(&mut self) -> Marker {
        let pos = self.events.len();
        self.events.push(Event::Placeholder);
        Marker::new(pos)
    }
}

#[cfg(test)]
mod tests {
    use crate::check;
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
