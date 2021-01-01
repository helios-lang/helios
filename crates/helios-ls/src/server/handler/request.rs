use super::*;
use crate::state::StateSnapshot;

pub fn initialize(
    _: StateSnapshot,
    _: InitializeParams,
) -> Result<InitializeResult> {
    let server_info = ServerInfo {
        name: "Helios-LS".to_string(),
        version: Some(env!("CARGO_PKG_VERSION").into()),
    };

    let capabilities = ServerCapabilities {
        text_document_sync: Some(TextDocumentSyncCapability::Kind(
            TextDocumentSyncKind::Incremental,
        )),
        hover_provider: Some(HoverProviderCapability::Simple(true)),
        completion_provider: Some(CompletionOptions {
            resolve_provider: Some(true),
            trigger_characters: Some(vec![".".into()]),
            work_done_progress_options: WorkDoneProgressOptions {
                work_done_progress: Some(false),
            },
        }),
        ..ServerCapabilities::default()
    };

    Ok(InitializeResult {
        server_info: Some(server_info),
        capabilities,
    })
}

pub fn shutdown(_: StateSnapshot, _: ()) -> Result<()> {
    log::trace!("Shutting down...");
    Ok(())
}
