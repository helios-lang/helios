use helios_diagnostics::Diagnostic;
use helios_syntax::SyntaxKind;
use text_size::TextRange;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Message {
    Parser(ParserMessage),
}

impl From<Message> for Diagnostic {
    fn from(message: Message) -> Self {
        match message {
            Message::Parser(message) => message.into(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ParserMessage {
    UnexpectedToken {
        found: Option<SyntaxKind>,
        expected: Vec<SyntaxKind>,
        range: TextRange,
    },
}

impl From<ParserMessage> for Message {
    fn from(message: ParserMessage) -> Self {
        Message::Parser(message)
    }
}

impl From<ParserMessage> for Diagnostic {
    fn from(message: ParserMessage) -> Self {
        match message {
            ParserMessage::UnexpectedToken {
                found,
                expected,
                range,
            } => {
                let found = match found {
                    Some(found) => {
                        format!(
                            "{}",
                            found
                                .human_readable_repr()
                                .omitting()
                                .article()
                                .code_repr()
                                .example()
                        )
                    }
                    None => "end of file".to_string(),
                };

                let expected_string = {
                    if expected.len() == 1 {
                        format!(
                            "I expected {} here.",
                            expected[0].human_readable_repr()
                        )
                    } else {
                        let mut expected_string = String::from(
                            "I expected one of the follow here:\n",
                        );

                        for kind in expected.iter() {
                            expected_string.push_str(
                                &format!(
                                    "\n    {}",
                                    kind.human_readable_repr()
                                )
                            );
                        }

                        expected_string
                    }
                };

                Diagnostic::error(format!("Unexpected {}", found))
                    .range(range)
                    .description(
                        "I was partway through something when I got stuck here:",
                    )
                    .message(expected_string)
            }
        }
    }
}
