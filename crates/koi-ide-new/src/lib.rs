use tokio::runtime::Runtime;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

struct KoiBackend {
    client: Client,
}

#[tower_lsp::async_trait]
impl LanguageServer for KoiBackend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            server_info: Some(ServerInfo {
                name: "KoiLS".to_string(),
                version: Some("0.1.5".to_string()),
            }),
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::Incremental,
                )),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                completion_provider: Some(CompletionOptions {
                    resolve_provider: Some(true),
                    trigger_characters: Some(vec![".".to_string()]),
                    work_done_progress_options: WorkDoneProgressOptions {
                        work_done_progress: Some(false),
                    }
                }),
                rename_provider: Some(RenameProviderCapability::Simple(true)),
                ..ServerCapabilities::default()
            }
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::Info, "Server successfully initialized")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        self.client
            .log_message(MessageType::Info, "Shutting down server...")
            .await;

        Ok(())
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        self.client
            .log_message(MessageType::Info, format!("{:?}", params))
            .await;
    }

    async fn did_save(&self, params: DidSaveTextDocumentParams) {
        self.client
            .log_message(MessageType::Info, format!("{:?}", params))
            .await;
    }

    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        self.client
            .log_message(MessageType::Info, format!("{:?}", params))
            .await;

        Ok(Some(CompletionResponse::Array(vec![
            CompletionItem::new_simple("foo".to_string(), "Foo detail".to_string()),
            CompletionItem::new_simple("bar".to_string(), "Bar detail".to_string()),
        ])))
    }

    async fn completion_resolve(&self, item: CompletionItem) -> Result<CompletionItem> {
        Ok(item)
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        self.client
            .log_message(MessageType::Info, format!("{:?}", params))
            .await;

        Ok(None)
    }
}

async fn __start() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, messages) = LspService::new(|client| KoiBackend { client });

    Server::new(stdin, stdout)
        .interleave(messages)
        .serve(service)
        .await;
}

/// Starts the connection between the client and server via the Language Server
/// Protocol.
///
/// This function initializes and starts a `tokio` runtime, panicking if it has
/// failed to initialize.
pub fn start() {
    let mut runtime = Runtime::new().expect("Failed to start tokio runtime");
    runtime.block_on(__start());
}
