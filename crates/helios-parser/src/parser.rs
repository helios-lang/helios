//! Module responsible for parsing Helios source files.

pub(crate) mod event;
pub(crate) mod marker;
pub(crate) mod sink;
pub(crate) mod source;

use helios_diagnostics::Location;
use helios_syntax::SyntaxKind;

use self::event::Event;
use self::marker::Marker;
use self::source::Source;
use crate::lexer::Token;
use crate::{Message, ParserMessage};

const RECOVERY_SET: [SyntaxKind; 1] = [SyntaxKind::Kwd_Let];

/// A lazy, lossless, error-tolerant parser for the Helios programming language.
pub struct Parser<'source, 'tokens, FileId> {
    file_id: FileId,
    source: Source<'tokens, 'source>,
    events: Vec<Event>,
    expected_kinds: Vec<SyntaxKind>,
    messages: Vec<Message<FileId>>,
}

impl<'source, 'tokens, FileId> Parser<'source, 'tokens, FileId>
where
    FileId: Clone + Default,
{
    /// Constructs a new [`Parser`] with a [`Source`] and a channel that sends
    /// [`Diagnostic`]s.
    pub fn new(file_id: FileId, source: Source<'tokens, 'source>) -> Self {
        Self {
            file_id,
            source,
            events: Vec::new(),
            expected_kinds: Vec::new(),
            messages: Vec::new(),
        }
    }

    /// Starts the parsing process.
    ///
    /// This function will attempt to build a concrete syntax tree with the
    /// given source text (no matter how invalid it is). Once done, it will
    /// return a [`Parse`] containing a root green node.
    ///
    /// [`Parse`]: crate::Parse
    pub fn parse(mut self) -> (Vec<Event>, Vec<Message<FileId>>) {
        crate::grammar::root(&mut self);
        (self.events, self.messages)
    }
}

impl<'source, 'tokens, FileId> Parser<'source, 'tokens, FileId>
where
    FileId: Clone + Default,
{
    /// Determines if the next [`SyntaxKind`] is the given `kind`.
    pub(crate) fn is_at(&mut self, kind: SyntaxKind) -> bool {
        self.expected_kinds.push(kind);
        self.peek() == Some(kind)
    }

    pub(crate) fn is_at_either<'a>(
        &mut self,
        kinds: &'a [SyntaxKind],
    ) -> Option<&'a SyntaxKind> {
        self.expected_kinds.extend(kinds);
        self.peek()
            .map_or(None, |kind| kinds.iter().find(|&&it| kind == it))
    }

    /// Peeks the next [`SyntaxKind`] token without consuming it.
    fn peek(&mut self) -> Option<SyntaxKind> {
        self.source.peek_kind()
    }

    /// Adds the next token to the syntax tree (via the [`GreenNodeBuilder`]).
    pub(crate) fn bump(&mut self) {
        self.expected_kinds.clear();
        self.source.next_token().unwrap();
        self.events.push(Event::AddToken)
    }

    /// Starts a new node, returning a [`Marker`].
    pub(crate) fn start(&mut self) -> Marker {
        let pos = self.events.len();
        self.events.push(Event::Placeholder);
        Marker::new(pos)
    }

    pub(crate) fn expect(
        &mut self,
        kind: SyntaxKind,
        context: impl Into<Option<SyntaxKind>>,
    ) {
        if self.is_at(kind) {
            self.bump();
        } else {
            self.error(context);
        }
    }

    pub(crate) fn error(&mut self, context: impl Into<Option<SyntaxKind>>) {
        let current_token = self.source.peek_token();

        let (given, range) =
            if let Some(Token { kind, range, .. }) = current_token {
                (Some(*kind), range.clone())
            } else {
                (None, self.source.last_token_range().unwrap())
            };

        let expected = std::mem::take(&mut self.expected_kinds);
        let expected_len = expected.len();

        let message_kind = if expected_len == 1 {
            ParserMessage::MissingKind {
                context: context.into(),
                expected: expected[0],
            }
        } else {
            ParserMessage::UnexpectedKind {
                context: context.into(),
                given,
                expected,
            }
        };

        self.messages.push(Message::new(
            message_kind,
            Location::new(self.file_id.clone(), range),
        ));

        if expected_len > 1
            && !self.is_at_set(&RECOVERY_SET)
            && !self.is_at_end()
        {
            let m = self.start();
            self.bump();
            m.complete(self, SyntaxKind::Error);
        }
    }

    fn is_at_set(&mut self, set: &[SyntaxKind]) -> bool {
        self.peek().map_or(false, |kind| set.contains(&kind))
    }

    pub(crate) fn is_at_end(&mut self) -> bool {
        self.peek().is_none()
    }
}

#[cfg(test)]
mod tests {
    use crate::check;
    use expect_test::expect;

    #[test]
    fn test_parse_nothing() {
        check(
            "",
            expect![[r#"
                Root@0..0
            "#]],
        );
    }

    #[test]
    fn test_parse_whitespace() {
        check(
            "   ",
            expect![[r#"
                Root@0..3
                  Whitespace@0..3 "   "
            "#]],
        );
    }

    #[test]
    fn test_parse_comment() {
        check(
            "# hello, world!",
            expect![[r##"
                Root@0..15
                  Comment@0..15 "# hello, world!"
            "##]],
        );
    }

    #[test]
    fn test_parse_comment_followed_by_whitespace() {
        check(
            "# hello, world!\n",
            expect![[r##"
                Root@0..16
                  Comment@0..15 "# hello, world!"
                  Newline@15..16 "\n"
            "##]],
        );
    }

    #[test]
    fn test_parse_multiple_comments() {
        check(
            "
# hello, world!
# this is another line
",
            expect![[r##"
                Root@0..40
                  Newline@0..1 "\n"
                  Comment@1..16 "# hello, world!"
                  Newline@16..17 "\n"
                  Comment@17..39 "# this is another line"
                  Newline@39..40 "\n"
            "##]],
        );
    }
}
