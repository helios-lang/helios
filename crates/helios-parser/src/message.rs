use crate::FileId;
use helios_diagnostics::{Diagnostic, Location};
use helios_formatting::FormattedText;
use helios_syntax::SyntaxKind;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Message {
    Lexer(LexerMessage),
    Parser(ParserMessage),
}

impl From<Message> for Diagnostic<FileId> {
    fn from(message: Message) -> Self {
        match message {
            Message::Lexer(message) => message.into(),
            Message::Parser(message) => message.into(),
        }
    }
}

impl From<LexerMessage> for Message {
    fn from(message: LexerMessage) -> Self {
        Message::Lexer(message)
    }
}

impl From<ParserMessage> for Message {
    fn from(message: ParserMessage) -> Self {
        Message::Parser(message)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LexerMessage {
    UnknownCharacter {
        location: Location<FileId>,
        character: char,
    },
}

impl From<LexerMessage> for Diagnostic<FileId> {
    fn from(message: LexerMessage) -> Self {
        match message {
            LexerMessage::UnknownCharacter {
                location,
                character,
            } => {
                let description = FormattedText::default()
                    .text("I encountered a token I don't know how to handle:");

                let message = FormattedText::default()
                    .text("The character ")
                    .code(format!("{:?}", character))
                    .text(" is not a valid token. Did you mean to write it?");

                Diagnostic::error("Unknown character")
                    .location(location)
                    .description(description)
                    .message(message)
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ParserMessage {
    MissingKind {
        location: Location<FileId>,
        context: Option<SyntaxKind>,
        expected: SyntaxKind,
    },
    UnexpectedKind {
        location: Location<FileId>,
        context: Option<SyntaxKind>,
        given: Option<SyntaxKind>,
        expected: Vec<SyntaxKind>,
    },
}

impl From<ParserMessage> for Diagnostic<FileId> {
    fn from(message: ParserMessage) -> Self {
        match message {
            ParserMessage::MissingKind {
                location,
                context,
                expected,
            } => {
                let error = format!(
                    "Missing {}{}",
                    expected.description().map(|s| s + " ").unwrap_or_default(),
                    expected.kind()
                );

                let description = FormattedText::default().text(format!(
                    "I was partway through {} when I got stuck here:",
                    context.map_or("something".to_string(), |context| {
                        context.to_string()
                    })
                ));

                let message = FormattedText::default()
                    .text(format!("I expected {} here.", expected));

                Diagnostic::error(error)
                    .location(location)
                    .description(description)
                    .message(message)
            }
            ParserMessage::UnexpectedKind {
                location,
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

                let description = FormattedText::default().text(format!(
                    "I was partway through {} when I got stuck here:",
                    context.map_or("something".to_string(), |context| {
                        context.to_string()
                    })
                ));

                let (message, hint) = {
                    if expected.len() == 1 {
                        let expected = expected[0];

                        let message = FormattedText::default()
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
                                    FormattedText::default().code(description)
                                ))
                            }
                            _ => None,
                        };

                        (message, hint)
                    } else {
                        let message = FormattedText::default()
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
