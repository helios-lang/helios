use super::{LspMessage, LspResponse};
use koi_actor::Actor;
use koi_driver::{Ast, Position, Source, tokenize};
use std::collections::VecDeque;
use std::sync::mpsc::Sender;

pub struct Receiver {
    responder_channel: Sender<LspResponse>,
    tokens: Option<Ast>,
}

impl Receiver {
    pub fn with(responder_channel: Sender<LspResponse>) -> Self {
        Receiver { responder_channel, tokens: None }
    }

    fn process_message(&mut self, message: LspMessage) {
        match message {
            LspMessage::InitializeRequest { id, .. } => {
                self.responder_channel
                    .clone()
                    .send(LspResponse::InitializeResult { id })
                    .expect("Failed to send `InitializeResult` message to Responder");
            },
            LspMessage::InitializedNotification => {
                // The client has been initialized
            },
            LspMessage::ShutdownRequest => {
                // The client has asked us to shutdown
            },
            LspMessage::ExitNotification => {
                // The client has asked us to exit now
            },
            LspMessage::TextDocumentDidOpenNotification { params, .. } => {
                // A text document has been opened
                match params.text_document.uri.to_file_path() {
                    Ok(path) => match Source::file(path) {
                        Ok(source) => {
                            let tokens = tokenize(source);
                            // tokens.iter().for_each(|token| eprintln!("{:?}", token));
                            self.tokens = Some(tokens);
                        },
                        Err(error) => eprintln!("Failed to load file from source: {}", error)
                    },
                    Err(_) => eprintln!("Failed to convert `{}` to file path.", params.text_document.uri)
                }
            },
            LspMessage::TextDocumentDidChangeNotification { .. } => {
                // The text document has been modified
            },
            LspMessage::TextDocumentDidSaveNotification { params } => {
                match params.text_document.uri.to_file_path() {
                    Ok(path) => match Source::file(path) {
                        Ok(source) => {
                            let tokens = tokenize(source);
                            // tokens.iter().for_each(|token| eprintln!("{:?}", token));
                            self.tokens = Some(tokens);
                        },
                        Err(error) => eprintln!("Failed to load file from source: {}", error)
                    },
                    Err(_) => eprintln!("Failed to convert `{}` to file path.", params.text_document.uri)
                }
            },
            LspMessage::TextDocumentCompletionRequest { .. } => {
                // A completion request has been sent
            },
            LspMessage::TextDocumentHoverRequest { id, params } => {
                if let Some(tokens) = &self.tokens {
                    for token in tokens {
                        if
                            token.range.contains(
                                &Position::new(
                                    params.position.line as usize,
                                    params.position.character as usize
                                )
                            )
                        {
                            self.responder_channel
                                .clone()
                                .send(LspResponse::HoverResult {
                                    id,
                                    params: lsp_types::Hover {
                                        contents: lsp_types::HoverContents::Scalar(
                                            lsp_types::MarkedString::from_language_code(
                                                "koi".to_string(),
                                                format!("{:?}", token.kind)
                                            )
                                        ),
                                        range: Some(
                                            lsp_types::Range::new(
                                                lsp_types::Position::new(
                                                    token.range.start.line as u64,
                                                    token.range.start.character as u64
                                                ),
                                                lsp_types::Position::new(
                                                    token.range.end.line as u64,
                                                    token.range.end.character as u64
                                                )
                                            )
                                        )
                                    }
                                })
                                .expect("Failed to send `HoverRequest` message to Responder.")
                        }
                    }
                }
            }
        }
    }
}

impl Actor for Receiver {
    type InMessage = LspMessage;

    fn poll_messages(&mut self, messages: &mut VecDeque<Self::InMessage>) {
        if let Some(message) = messages.pop_front() {
            self.process_message(message);
        }
    }
}
