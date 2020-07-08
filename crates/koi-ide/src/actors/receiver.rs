#![allow(unused_imports)]
#![allow(unused_variables)]

use super::{LspMessage, LspResponse};
use koi_actor::Actor;
use koi_syntax::Ast;
use koi_syntax::source::{Position, Source};
use lsp_types::Url;
use std::collections::{HashMap, VecDeque};
use std::sync::mpsc::Sender;

pub struct Receiver {
    responder_channel: Sender<LspResponse>,
    token_database: HashMap<String, (i64, Ast)>,
}

impl Receiver {
    pub fn with(responder_channel: Sender<LspResponse>) -> Self {
        Receiver { responder_channel, token_database: HashMap::new(), }
    }

    fn generate_tokens(&self, path: Url) -> Result<Ast, String> {
        match path.to_file_path() {
            Ok(path) => match Source::file(path) {
                Ok(source) => {
                    Ok(koi_driver::parse(source))
                },
                Err(error) => Err(format!("Failed to load file from source: {}", error))
            },
            Err(_) => Err(format!("Failed to convert `{}` to file path.", path))
        }
    }

    fn cache_tokens<V: Into<Option<i64>>>(&mut self, url: Url, version: V) {
        let version = version.into().unwrap_or(0);
        let key = url.to_string();
        match self.token_database.get(&key) {
            Some((cached_version, _)) => {
                if cached_version != &version {
                    match self.generate_tokens(url) {
                        Ok(tokens) => {
                            // eprintln!("{:#?}", tokens.nodes());
                            self.token_database.insert(key, (version, tokens));
                        },
                        Err(error) => {
                            eprintln!("Failed to generate tokens: {}", error);
                        }
                    }
                }
            },
            None => {
                match self.generate_tokens(url) {
                    Ok(tokens) => {
                        self.token_database.insert(key, (version, tokens));
                    },
                    Err(error) => {
                        eprintln!("Failed to generate tokens: {}", error)
                    }
                }
            }
        }
    }

    fn send_hover_response(&self, id: usize, params: lsp_types::TextDocumentPositionParams) {
        eprintln!("Receiver::send_hover_request")
    }

    fn publish_diagnostics(&self, uri: lsp_types::Url) {
        eprintln!("Receiver::publish_diagnostics")
    }

    fn process_message(&mut self, message: LspMessage) {
        match message {
            LspMessage::InitializeRequest { id, .. } => {
                self.responder_channel
                    .clone()
                    .send(LspResponse::InitializeResult { id })
                    .expect("Failed to send `InitializeResult` message to Responder");
            },
            LspMessage::InitializedNotification => {
                // The connection has been successfully initialized
            },
            LspMessage::ShutdownRequest => {
                // The client has asked us to shutdown
            },
            LspMessage::ExitNotification => {
                // The client has asked us to exit now
            },
            LspMessage::TextDocumentDidOpenNotification { params, .. } => {
                let uri = params.text_document.uri;
                let version = params.text_document.version;
                self.cache_tokens(uri.clone(), version);
                self.publish_diagnostics(uri);
            },
            LspMessage::TextDocumentDidChangeNotification { .. } => {
                // The text document has been modified
            },
            LspMessage::TextDocumentDidSaveNotification { params } => {
                let uri = params.text_document.uri;
                let version = params.text_document.version;
                self.cache_tokens(uri.clone(), version);
                self.publish_diagnostics(uri);
            },
            LspMessage::TextDocumentCompletionRequest { id, params } => {
                use lsp_types::{CompletionItem, CompletionItemKind};
                use koi_syntax::token::Keyword;
                let keywords: Vec<CompletionItem> = Keyword::keyword_list()
                    .into_iter()
                    .map(|keyword| {
                        lsp_types::CompletionItem {
                            label: keyword,
                            kind: Some(CompletionItemKind::Keyword),
                            ..CompletionItem::default()
                        }
                    })
                    .collect();
                let primitive_types: Vec<CompletionItem> =
                    vec![
                        "bool",
                        "char",
                        "float",
                        "int",
                        "uint",
                        "string",
                    ]
                    .into_iter()
                    .map(|r#type| {
                        CompletionItem {
                            label: r#type.to_string(),
                            kind: Some(lsp_types::CompletionItemKind::Struct),
                            ..CompletionItem::default()
                        }
                    })
                    .collect();

                match params.context {
                    Some(context) => match context.trigger_kind {
                        lsp_types::CompletionTriggerKind::Invoked => {
                            self.responder_channel
                            .clone()
                            .send(LspResponse::CompletionList {
                                id,
                                params: Some(
                                    lsp_types::CompletionResponse::Array(
                                        [
                                            &keywords[..],
                                            &primitive_types[..]
                                        ].concat()
                                    )
                                )
                            })
                            .expect("Failed to send `CompletionRequest` message to Responder");
                        },
                        lsp_types::CompletionTriggerKind::TriggerCharacter => {
                            eprintln! {
                                "completionRequest with trigger character: {:?}",
                                context.trigger_character
                            }
                        },
                        _ => {}
                    },
                    None => {}
                }
            },
            LspMessage::TextDocumentHoverRequest { id, params } => {
                self.send_hover_response(id, params);
            }
        }
    }
}

impl Actor for Receiver {
    type InMessage = LspMessage;

    fn poll_messages(&mut self, messages: &mut VecDeque<Self::InMessage>) {
        if let Some(message) = messages.pop_front() {
            self.process_message(message);
        }
    }
}
