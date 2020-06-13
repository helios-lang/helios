use super::LspResponse;
use koi_actor::Actor;
use std::collections::VecDeque;
pub struct Responder;

impl Actor for Responder {
    type InMessage = LspResponse;

    fn poll_messages(&mut self, messages: &mut VecDeque<Self::InMessage>) {
        use super::{send_jsonrpc_response, send_jsonrpc_notification, Capabilities};

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
            LspResponse::CompletionList { id, params: result } => {
                send_jsonrpc_response(id, result);
            },
            LspResponse::HoverResult { id, params: result } => {
                send_jsonrpc_response(id, result);
            },
            LspResponse::PublishDiagnostics { params } => {
                send_jsonrpc_notification("textDocument/publishDiagnostics", params);
            }
        }
    }
}
