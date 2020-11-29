use tokio::runtime::Runtime;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

struct KoiLanguageServer {
    client: Client,
}

#[tower_lsp::async_trait]
impl LanguageServer for KoiLanguageServer {
    async fn initialize(
        &self,
        _: InitializeParams,
    ) -> Result<InitializeResult> {
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

        // use koi_syntax_old::token::Keyword;
        // let keywords: Vec<CompletionItem> = Keyword::keyword_list()
        //     .into_iter()
        //     .map(|keyword| CompletionItem {
        //         label: keyword.clone(),
        //         kind: Some(CompletionItemKind::Keyword),
        //         insert_text: Some(keyword + " "),
        //         detail: Some("Koi keyword".to_string()),
        //         ..CompletionItem::default()
        //     })
        //     .collect();

        let primitive_types: Vec<CompletionItem> = vec![
            "Bool", "Char", "String", "Float", "Float32", "Float64", "Int",
            "Int8", "Int16", "Int32", "Int64", "UInt", "UInt8", "UInt16",
            "UInt32", "UInt64", "Optional", "Result",
        ]
        .into_iter()
        .map(|r#type| CompletionItem {
            label: r#type.to_string(),
            kind: Some(CompletionItemKind::Struct),
            insert_text: match r#type {
                "Optional" => Some("Optional(of ${1:???})".to_string()),
                "Result" => Some("Result(of ${1:???}, ${2:???})".to_string()),
                _ => None,
            },
            insert_text_format: match r#type {
                "Optional" => Some(InsertTextFormat::Snippet),
                "Result" => Some(InsertTextFormat::Snippet),
                _ => None,
            },
            ..CompletionItem::default()
        })
        .collect();

        let special_identifiers: Vec<CompletionItem> =
            vec!["True", "False", "Some", "None", "Ok", "Err", "Self"]
                .into_iter()
                .map(|ident| CompletionItem {
                    label: ident.to_string(),
                    kind: Some(CompletionItemKind::Struct),
                    insert_text: match ident {
                        "Some" | "Ok" | "Err" => {
                            Some(format!("{}(${{1:???}})", ident))
                        },
                        _ => None,
                    },
                    insert_text_format: match ident {
                        "Some" | "Ok" | "Err" => {
                            Some(InsertTextFormat::Snippet)
                        },
                        _ => None,
                    },
                    ..CompletionItem::default()
                })
                .collect();

        Ok(params.context.map(|context| match context.trigger_kind {
            CompletionTriggerKind::Invoked => CompletionResponse::Array(
                [
                    // &keywords[..],
                    &primitive_types[..],
                    &special_identifiers[..],
                ]
                .concat(),
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
        LspService::new(|client| KoiLanguageServer { client });

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
