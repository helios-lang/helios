use super::*;
use crate::state::StateSnapshot;
// use helios_query::*;
// use std::sync::Arc;

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

pub fn completion(
    _snapshot: StateSnapshot,
    _: CompletionParams,
) -> Result<Option<CompletionResponse>> {
    // let file_id = 0;
    // let mut completion_items = Vec::new();
    // let bindings: Arc<Vec<BindingId>> = snapshot.db.all_bindings(file_id);
    // let source: Arc<String> = snapshot.db.source(file_id);
    // log::trace!("SOURCE => {:?}", source);

    // for binding in bindings.iter() {
    //     let binding_data: BindingData =
    //         snapshot.db.lookup_intern_binding(*binding);
    //     log::info!("{:?} => {:?}", binding, binding_data);

    //     let completion_item = CompletionItem {
    //         label: binding_data.identifier,
    //         kind: Some(CompletionItemKind::Variable),
    //         ..CompletionItem::default()
    //     };

    //     completion_items.push(completion_item);
    // }

    // if completion_items.is_empty() {
    //     Ok(None)
    // } else {
    //     Ok(Some(completion_items.into()))
    // }

    Ok(None)
}

pub fn hover(_: StateSnapshot, _: HoverParams) -> Result<Option<Hover>> {
    Ok(None)
}
