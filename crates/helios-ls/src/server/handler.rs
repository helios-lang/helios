use lsp_types::*;

pub trait Handler {
    fn initialize(&self, params: InitializeParams) -> InitializeResult;
    fn shutdown(&self);

    fn initialized(&self, params: InitializedParams) {
        let _ = params;
        log::info!("Unimplemented method: initialized");
    }
}
