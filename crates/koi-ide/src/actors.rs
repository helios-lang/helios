use super::{LspMessage, LspResponse};
use koi_actor::Actor;
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
            LspMessage::TextDocumentHoverRequest { .. } => {
                // A hover request has been sent
            }
        }
    }
}

pub struct Responder;

impl Actor for Responder {
    type InMessage = LspResponse;

    fn poll_messages(&mut self, messages: &mut VecDeque<Self::InMessage>) {
        use super::{send_jsonrpc_response, Capabilities};

        match messages.pop_front().expect("Failed to get next message") {
            LspResponse::InitializeResult { id } => {
                use lsp_types::*;
                let capabilities = ServerCapabilities {
                    text_document_sync: Some(TextDocumentSyncCapability::Kind(
                        TextDocumentSyncKind::Incremental,
                    )),
                    hover_provider: Some(true),
                    completion_provider: Some(CompletionOptions {
                        resolve_provider: Some(true),
                        trigger_characters: Some(vec![".".to_string()]),
                        work_done_progress_options: WorkDoneProgressOptions {
                            work_done_progress: Some(false)
                        }
                    }),
                    rename_provider: Some(RenameProviderCapability::Simple(true)),
                    ..ServerCapabilities::default()
                };

                send_jsonrpc_response(id, Capabilities { capabilities });
            },
            LspResponse::CompletionList { .. } => {
                // Unimplemented...
            },
            LspResponse::HoverResult { .. } => {
                // Unimplemented...
            }
        }
    }
}
