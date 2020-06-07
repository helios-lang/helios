use super::{LspMessage, LspResponse};
use koi_actor::Actor;
use koi_driver::Driver;
use std::collections::VecDeque;
use std::sync::mpsc::Sender;
pub struct Receiver {
    responder_channel: Sender<LspResponse>,
}

impl Receiver {
    pub fn with(responder_channel: Sender<LspResponse>) -> Self {
        Receiver { responder_channel }
    }
}

impl Actor for Receiver {
    type InMessage = LspMessage;

    fn poll_messages(&mut self, messages: &mut VecDeque<Self::InMessage>) {
        match messages.pop_front().expect("Failed to get next message") {
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
            LspMessage::TextDocumentDidOpenNotification { .. } => {
                // A text document has been opened
            },
            LspMessage::TextDocumentDidChangeNotification { .. } => {
                // The text document has been modified
            },
            LspMessage::TextDocumentDidSaveNotification { .. } => {
                // The text document was saved
            },
            LspMessage::TextDocumentCompletionRequest { .. } => {
                // A completion request has been sent
            },
            LspMessage::TextDocumentHoverRequest { id, params } => {
                eprintln!("::: {}", params.text_document.uri);
                let driver = Driver::with(params.text_document.uri.as_str());
                let text = match driver.tokenize_source() {
                    Ok(_) => "???".to_string(),
                    Err(mut diagnostics) => match diagnostics.pop() {
                        Some(diagnostic) => diagnostic.code,
                        None => "???".to_string()
                    },
                };

                self.responder_channel
                    .clone()
                    .send(LspResponse::HoverResult {
                        id,
                        params: lsp_types::Hover {
                            contents: lsp_types::HoverContents::Scalar(
                                lsp_types::MarkedString::from_language_code(
                                    "koi".to_string(),
                                    text
                                )
                            ),
                            range: Some(
                                lsp_types::Range::new(
                                    params.position,
                                    lsp_types::Position::new(
                                        params.position.line,
                                        params.position.character + 5
                                    )
                                )
                            )
                        }
                    })
                    .expect("Failed to send `HoverResult` message to responder");
            }
        }
    }
}
