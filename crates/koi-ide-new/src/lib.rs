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
        Ok(InitializeResult::default())
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::Info, "Server successfully initialized")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
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
