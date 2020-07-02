#![allow(unused_imports)]
#![allow(unused_variables)]

use super::{LspMessage, LspResponse};
use koi_actor::Actor;
use koi_driver::tokenize;
use koi_parser::{Ast, token};
use koi_parser::source::{Position, Source};
use koi_parser::reporter::{Diagnosis, Reporter};
use lsp_types::Url;
use std::collections::{HashMap, VecDeque};
use std::sync::mpsc::Sender;

struct IdeReporter;

impl Reporter for IdeReporter {
    fn report(&mut self, diagnosis: Diagnosis) {
        eprintln!(">>> ERROR: {:?}", diagnosis);
    }
}

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
                    Ok(tokenize(source, Box::new(IdeReporter), true))
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
                            tokens.iter().for_each(|token| eprintln!("{:?}", token));
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
        // if let Some((_, tokens)) = self.token_database.get(params.text_document.uri.as_str()) {
        //     for token in tokens {
        //         if token.range.contains(&Position::new(
        //             params.position.line as usize,
        //             params.position.character as usize
        //         )) {
        //             // use token::TokenKind;
        //             match token.kind {
        //                 // TokenKind::Keyword(_)
        //                 // | TokenKind::Symbol(_)
        //                 // | TokenKind::Eof => {
        //                 //     self.responder_channel
        //                 //         .clone()
        //                 //         .send(LspResponse::HoverResult { id, params: None })
        //                 //         .expect("Failed to send `HoverRequest` message to Responder.")
        //                 // },
        //                 _ => {
        //                     self.responder_channel
        //                         .clone()
        //                         .send(LspResponse::HoverResult {
        //                             id,
        //                             params: Some(lsp_types::Hover {
        //                                 contents: lsp_types::HoverContents::Scalar(
        //                                     lsp_types::MarkedString::from_language_code(
        //                                         "koi".to_string(),
        //                                         format!("{:?}", token.kind)
        //                                     )
        //                                 ),
        //                                 range: Some(
        //                                     lsp_types::Range::new(
        //                                         lsp_types::Position::new(
        //                                             token.range.start.line as u64,
        //                                             token.range.start.character as u64
        //                                         ),
        //                                         lsp_types::Position::new(
        //                                             token.range.end.line as u64,
        //                                             token.range.end.character as u64
        //                                         )
        //                                     )
        //                                 )
        //                             })
        //                         })
        //                         .expect("Failed to send `HoverRequest` message to Responder.")
        //                 }
        //             }
        //         }
        //     }
        // } else {
        //     eprintln!("Error: No AST has been cached for the given file url.");
        // }
    }

    fn publish_diagnostics(&self, uri: lsp_types::Url) {
        // if let Some((version, tokens)) = self.token_database.get(uri.as_str()) {
        //     let mut diagnostics = Vec::new();

        //     for token in tokens {
        //         let range = lsp_types::Range::new(
        //             lsp_types::Position::new(
        //                 token.range.start.line as u64,
        //                 token.range.start.character as u64
        //             ),
        //             lsp_types::Position::new(
        //                 token.range.end.line as u64,
        //                 token.range.end.character as u64
        //             ),
        //         );

        //         match token.kind {
        //             token::TokenKind::Error(error) => {
        //                 let related_information = error
        //                     .related_information()
        //                     .map(|message| vec![
        //                         lsp_types::DiagnosticRelatedInformation {
        //                             location: lsp_types::Location {
        //                                 uri: uri.clone(),
        //                                 range,
        //                             },
        //                             message
        //                         }
        //                     ]);

        //                 diagnostics.push(lsp_types::Diagnostic {
        //                     range,
        //                     severity: Some(lsp_types::DiagnosticSeverity::Error),
        //                     source: Some("koi".to_string()),
        //                     message: error.message(),
        //                     code: Some(lsp_types::NumberOrString::String(error.code())),
        //                     related_information,
        //                     ..lsp_types::Diagnostic::default()
        //                 });
        //             },
        //             token::TokenKind::Unexpected(c) => {
        //                 diagnostics.push(lsp_types::Diagnostic {
        //                     range,
        //                     severity: Some(lsp_types::DiagnosticSeverity::Error),
        //                     source: Some("koi".to_string()),
        //                     message: format!("Unexpected character {:?} (U+{:04X})", c, c as u32),
        //                     code: Some(lsp_types::NumberOrString::String("E0012".to_string())),
        //                     ..lsp_types::Diagnostic::default()
        //                 });
        //             },
        //             _ => {}
        //         }
        //     }

        //     self.responder_channel
        //         .clone()
        //         .send(LspResponse::PublishDiagnostics {
        //             params: lsp_types::PublishDiagnosticsParams {
        //                 uri, diagnostics, version: Some(*version)
        //             }
        //         })
        //         .expect("Failed to send `PublishDiagnostics` notification to Responder.");
        // }
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
                let keywords: Vec<CompletionItem> = token::Keyword::keyword_list()
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
