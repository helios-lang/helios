use super::{LspMessage, LspResponse};
use koi_actor::Actor;
use koi_driver::{Ast, Position, Source, tokenize};
use lsp_types::Url;
use std::collections::{HashMap, VecDeque};
use std::sync::mpsc::Sender;

pub struct Receiver {
    responder_channel: Sender<LspResponse>,
    database: HashMap<String, (i64, Ast)>,
}

impl Receiver {
    pub fn with(responder_channel: Sender<LspResponse>) -> Self {
        Receiver { responder_channel, database: HashMap::new() }
    }

    fn generate_tokens(&self, path: Url) -> Result<Ast, String> {
        match path.to_file_path() {
            Ok(path) => match Source::file(path) {
                Ok(source) => {
                    Ok(tokenize(source))
                },
                Err(error) => Err(format!("Failed to load file from source: {}", error))
            },
            Err(_) => Err(format!("Failed to convert `{}` to file path.", path))
        }
    }

    fn cache_tokens<V: Into<Option<i64>>>(&mut self, url: Url, version: V) {
        let version = version.into().unwrap_or(0);
        let key = url.to_string();
        match self.database.get(&key) {
            Some((cached_version, _)) => {
                if cached_version != &version {
                    match self.generate_tokens(url) {
                        Ok(tokens) => {
                            self.database.insert(key, (version, tokens));
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
                        self.database.insert(key, (version, tokens));
                    },
                    Err(error) => {
                        eprintln!("Failed to generate tokens: {}", error)
                    }
                }
            }
        }
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
                self.cache_tokens(params.text_document.uri, params.text_document.version);
            },
            LspMessage::TextDocumentDidChangeNotification { .. } => {
                // The text document has been modified
            },
            LspMessage::TextDocumentDidSaveNotification { params } => {
                self.cache_tokens(params.text_document.uri, params.text_document.version);
            },
            LspMessage::TextDocumentCompletionRequest { .. } => {
                // A completion request has been sent
            },
            LspMessage::TextDocumentHoverRequest { id, params } => {
                match self.database.get(params.text_document.uri.as_str()) {
                    Some((_, tokens)) => for token in tokens {
                        if token.range.contains(
                            &Position::new(
                                params.position.line as usize,
                                params.position.character as usize
                            )
                        ) {
                            self.responder_channel
                                .clone()
                                .send(LspResponse::HoverResult {
                                    id,
                                    params: lsp_types::Hover {
                                        contents: lsp_types::HoverContents::Scalar(
                                            lsp_types::MarkedString::from_language_code(
                                                "koi".to_string(),
                                                format!("{:?}", token.kind)
                                            )
                                        ),
                                        range: Some(
                                            lsp_types::Range::new(
                                                lsp_types::Position::new(
                                                    token.range.start.line as u64,
                                                    token.range.start.character as u64
                                                ),
                                                lsp_types::Position::new(
                                                    token.range.end.line as u64,
                                                    token.range.end.character as u64
                                                )
                                            )
                                        )
                                    }
                                })
                                .expect("Failed to send `HoverRequest` message to Responder.")
                        }
                    },
                    None => {
                        eprintln!("Error: No AST has been cached for the given file url.");
                    }
                }
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
