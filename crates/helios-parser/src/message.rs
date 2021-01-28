use helios_diagnostics::{Diagnostic, Location};
use helios_formatting::FormattedString;
use helios_syntax::SyntaxKind;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Message<FileId> {
    kind: MessageKind,
    location: Location<FileId>,
}

impl<FileId> Message<FileId>
where
    FileId: Clone + Default,
{
    pub fn new(
        kind: impl Into<MessageKind>,
        location: Location<FileId>,
    ) -> Self {
        Self {
            kind: kind.into(),
            location,
        }
    }

    pub fn generate_diagnostic(&self) -> Diagnostic<FileId> {
        match &self.kind {
            MessageKind::Lexer(it) => it.diagnostic(self.location.clone()),
            MessageKind::Parser(it) => it.diagnostic(self.location.clone()),
        }
    }
}

impl<FileId> From<Message<FileId>> for Diagnostic<FileId>
where
    FileId: Clone + Default,
{
    fn from(message: Message<FileId>) -> Self {
        message.generate_diagnostic()
    }
}

impl<FileId> From<&Message<FileId>> for Diagnostic<FileId>
where
    FileId: Clone + Default,
{
    fn from(message: &Message<FileId>) -> Self {
        message.generate_diagnostic()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MessageKind {
    Lexer(LexerMessage),
    Parser(ParserMessage),
}

impl From<LexerMessage> for MessageKind {
    fn from(message: LexerMessage) -> Self {
        MessageKind::Lexer(message)
    }
}

impl From<ParserMessage> for MessageKind {
    fn from(message: ParserMessage) -> Self {
        MessageKind::Parser(message)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LexerMessage {
    UnknownCharacter(char),
    InvalidIndentation { expected: usize, found: usize },
}

impl LexerMessage {
    pub fn diagnostic<FileId>(
        &self,
        location: Location<FileId>,
    ) -> Diagnostic<FileId>
    where
        FileId: Default,
    {
        match self {
            LexerMessage::UnknownCharacter(character) => {
                let description = FormattedString::default()
                    .text("I encountered a token I don't know how to handle:");

                let message = FormattedString::default()
                    .text("The character ")
                    .code(format!("{:?}", character))
                    .text(" is not a valid token. Did you mean to write it?");

                Diagnostic::error("Unknown character")
                    .location(location)
                    .description(description)
                    .message(message)
            }
            LexerMessage::InvalidIndentation { .. } => {
                todo!()
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ParserMessage {
    MissingKind {
        context: Option<SyntaxKind>,
        expected: SyntaxKind,
    },
    UnexpectedKind {
        context: Option<SyntaxKind>,
        given: Option<SyntaxKind>,
        expected: Vec<SyntaxKind>,
    },
}

impl ParserMessage {
    pub fn diagnostic<FileId>(
        &self,
        location: Location<FileId>,
    ) -> Diagnostic<FileId>
    where
        FileId: Default,
    {
        match self {
            ParserMessage::MissingKind { context, expected } => {
                let error = format!(
                    "Missing {}{}",
                    expected.description().map(|s| s + " ").unwrap_or_default(),
                    expected.kind()
                );

                let description = FormattedString::default().text(format!(
                    "I was partway through {} when I got stuck here:",
                    context.map_or("something".to_string(), |context| {
                        context.to_string()
                    })
                ));

                let message = FormattedString::default()
                    .text(format!("I expected {} here.", expected));

                Diagnostic::error(error)
                    .location(location)
                    .description(description)
                    .message(message)
            }
            ParserMessage::UnexpectedKind {
                context,
                given,
                expected,
            } => {
                let title = format!(
                    "Unexpected {}",
                    given.map_or("end of file".to_string(), |given| {
                        given.kind()
                    })
                );

                let description = FormattedString::default().text(format!(
                    "I was partway through {} when I got stuck here:",
                    context.map_or("something".to_string(), |context| {
                        context.to_string()
                    })
                ));

                let (message, hint) = {
                    if expected.len() == 1 {
                        let expected = expected[0];

                        let message = FormattedString::default()
                            .text(format!("I expected {} here.", expected));

                        let hint = match (expected, given) {
                            (SyntaxKind::Identifier, Some(kind))
                                if kind.is_keyword() =>
                            {
                                let description = kind.description().expect(
                                    "keywords should have descriptions",
                                );

                                Some(format!(
                                    "It looks like you're trying to use the \
                                     reserved keyword {} as an identifier! Try \
                                     using a different name instead.",
                                    FormattedString::default()
                                        .code(description)
                                ))
                            }
                            _ => None,
                        };

                        (message, hint)
                    } else {
                        let message = FormattedString::default()
                            .text("I expected one of the following here:")
                            .list(
                                expected
                                    .iter()
                                    .map(|kind| {
                                        kind.human_readable_repr().into()
                                    })
                                    .collect::<Vec<_>>(),
                            );

                        (message, None)
                    }
                };

                if let Some(hint) = hint {
                    Diagnostic::error(title)
                        .location(location)
                        .description(description)
                        .message(message)
                        .hint(hint)
                } else {
                    Diagnostic::error(title)
                        .location(location)
                        .description(description)
                        .message(message)
                }
            }
        }
    }
}
