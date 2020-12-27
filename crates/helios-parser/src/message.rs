use crate::FileId;
use helios_diagnostics::{Diagnostic, Location};
use helios_syntax::SyntaxKind;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Message {
    Parser(ParserMessage),
}

impl From<Message> for Diagnostic<FileId> {
    fn from(message: Message) -> Self {
        match message {
            Message::Parser(message) => message.into(),
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
        found: Option<SyntaxKind>,
        expected: Vec<SyntaxKind>,
    },
}

impl From<ParserMessage> for Message {
    fn from(message: ParserMessage) -> Self {
        Message::Parser(message)
    }
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
                let message = format!("I expected {} here.", expected);
                let description = format!(
                    "I was partway through {} when I got stuck here:",
                    match context {
                        Some(kind) => format!("{}", kind),
                        None => "something".to_string(),
                    }
                );

                Diagnostic::error(error)
                    .location(location)
                    .description(description)
                    .message(message)
            }
            ParserMessage::UnexpectedKind {
                location,
                context,
                found,
                expected,
            } => {
                let title = format!(
                    "Unexpected {}",
                    match found {
                        Some(found) => found.kind(),
                        None => "end of file".to_string(),
                    }
                );

                let description = format!(
                    "I was partway through {} when I got stuck here:",
                    match context {
                        Some(kind) => format!("{}", kind),
                        None => "something".to_string(),
                    }
                );

                let (message, hint) = {
                    if expected.len() == 1 {
                        let expected = expected[0];

                        use SyntaxKind::Identifier;
                        let hint = match (expected, found) {
                            (Identifier, Some(kind)) if kind.is_keyword() => {
                                Some(format!(
                                    "{} cannot be used as an identifier \
                                     because it is a reserved word. Try \
                                     using a different name instead.",
                                    kind.description().expect(
                                        "keywords should have descriptions"
                                    )
                                ))
                            }
                            _ => None,
                        };

                        (format!("I expected {} here.", expected), hint)
                    } else {
                        let mut expected_string = String::from(
                            "I expected one of the following here:\n",
                        );

                        for kind in expected.iter() {
                            expected_string
                                .push_str(&format!("\n    {}", kind));
                        }

                        (expected_string, None)
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
