use tokio::runtime::Runtime;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

struct HeliosLanguageServer {
    client: Client,
}

#[tower_lsp::async_trait]
impl LanguageServer for HeliosLanguageServer {
    async fn initialize(
        &self,
        _: InitializeParams,
    ) -> Result<InitializeResult> {
        Ok(InitializeResult {
            server_info: Some(ServerInfo {
                name: "Helios-LS".to_string(),
                version: Some(env!("CARGO_PKG_VERSION").to_string()),
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
                    },
                }),
                rename_provider: Some(RenameProviderCapability::Simple(true)),
                ..ServerCapabilities::default()
            },
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

    async fn completion(
        &self,
        params: CompletionParams,
    ) -> Result<Option<CompletionResponse>> {
        self.client
            .log_message(MessageType::Info, format!("{:?}", params))
            .await;

        let keywords = helios_syntax::KEYWORDS
            .iter()
            .map(|keyword| {
                let keyword = keyword.to_string();
                CompletionItem {
                    label: keyword.clone(),
                    kind: Some(CompletionItemKind::Keyword),
                    insert_text: Some(keyword + " "),
                    detail: Some("Helios keyword".to_string()),
                    ..CompletionItem::default()
                }
            })
            .collect::<Vec<_>>();

        let special_identifiers: Vec<CompletionItem> =
            vec!["True", "False", "Some", "None", "Ok", "Err"]
                .into_iter()
                .map(|ident| CompletionItem {
                    label: ident.to_string(),
                    kind: Some(CompletionItemKind::Struct),
                    insert_text: match ident {
                        "Some" | "Ok" | "Err" => {
                            Some(format!("{}(${{1:???}})", ident))
                        }
                        _ => None,
                    },
                    insert_text_format: match ident {
                        "Some" | "Ok" | "Err" => {
                            Some(InsertTextFormat::Snippet)
                        }
                        _ => None,
                    },
                    ..CompletionItem::default()
                })
                .collect();

        Ok(params.context.map(|context| match context.trigger_kind {
            CompletionTriggerKind::Invoked => CompletionResponse::Array(
                [&keywords[..], &special_identifiers[..]].concat(),
            ),
            _ => CompletionResponse::Array(special_identifiers),
        }))
    }

    async fn completion_resolve(
        &self,
        item: CompletionItem,
    ) -> Result<CompletionItem> {
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

    let (service, messages) =
        LspService::new(|client| HeliosLanguageServer { client });

    Server::new(stdin, stdout)
        .interleave(messages)
        .serve(service)
        .await;
}

/// Starts the connection between the client and server via the Language Server
/// Protocol.
///
/// This function initializes and starts a [`tokio`] runtime, panicking if it
/// has failed to initialize.
///
/// [`tokio`]: https://docs.rs/tokio/0.2.24/tokio
pub fn start() {
    let mut runtime = Runtime::new().expect("Failed to start tokio runtime");
    runtime.block_on(__start());
}
