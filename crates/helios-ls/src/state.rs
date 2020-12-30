use crate::server::Handler;
use lsp_types::*;

pub struct GlobalState;

impl GlobalState {
    pub fn new() -> Self {
        Self
    }
}

impl Handler for GlobalState {
    fn initialize(&self, _: InitializeParams) -> InitializeResult {
        let server_info = ServerInfo {
            name: "Helios-LS".to_string(),
            version: Some(env!("CARGO_PKG_VERSION").to_string()),
        };

        let capabilities = ServerCapabilities {
            text_document_sync: Some(TextDocumentSyncCapability::Kind(
                TextDocumentSyncKind::Incremental,
            )),
            hover_provider: Some(HoverProviderCapability::Simple(true)),
            completion_provider: Some(CompletionOptions {
                resolve_provider: Some(true),
                trigger_characters: Some(vec![".".to_string()]),
                work_done_progress_options: WorkDoneProgressOptions {
                    work_done_progress: Some(false),
                },
            }),
            ..ServerCapabilities::default()
        };

        InitializeResult {
            server_info: Some(server_info),
            capabilities,
        }
    }

    fn shutdown(&self) {
        log::info!("Shutdown")
    }
}
