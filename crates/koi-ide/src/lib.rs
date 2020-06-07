//! An implementation of a language server for the Koi programming language.

mod actors;

pub use actors::{Receiver, Responder};
use lsp_types;
use serde::{Serialize, Deserialize};
use serde_json;
use std::io::prelude::Write;
use std::sync::mpsc::Sender;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "method")]
pub enum LspMessage {
    #[serde(rename = "initialize")]
    InitializeRequest {
        id: usize,
        params: lsp_types::InitializeParams,
    },

    #[serde(rename = "initialized")]
    InitializedNotification,

    #[serde(rename = "shutdown")]
    ShutdownRequest,

    #[serde(rename = "exit")]
    ExitNotification,

    #[serde(rename = "textDocument/didOpen")]
    TextDocumentDidOpenNotification {
        params: lsp_types::DidOpenTextDocumentParams
    },

    #[serde(rename = "textDocument/didChange")]
    TextDocumentDidChangeNotification {
        params: lsp_types::DidChangeTextDocumentParams,
    },

    #[serde(rename = "textDocument/didSave")]
    TextDocumentDidSaveNotification {
        params: lsp_types::DidSaveTextDocumentParams,
    },

    #[serde(rename = "textDocument/completion")]
    TextDocumentCompletionRequest {
        id: usize,
        params: lsp_types::CompletionParams,
    },

    #[serde(rename = "textDocument/hover")]
    TextDocumentHoverRequest {
        id: usize,
        params: lsp_types::TextDocumentPositionParams,
    }
}

pub enum LspResponse {
    InitializeResult {
        id: usize,
    },

    CompletionList {
        id: usize,
        params: lsp_types::CompletionParams,
    },

    HoverResult {
        id: usize,
        params: lsp_types::Hover,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Capabilities {
    capabilities: lsp_types::ServerCapabilities
}

#[derive(Debug, Serialize, Deserialize)]
struct JsonRpcResponse<T> {
    jsonrpc: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<usize>,
    result: T,
}

impl<T> JsonRpcResponse<T> {
    pub fn new<U: Into<Option<usize>>>(id: U, result: T) -> Self {
        Self { jsonrpc: "2.0".to_string(), id: id.into(), result }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct JsonRpcNotification<T> {
    jsonrpc: String,
    method: String,
    params: T,
}

impl<T> JsonRpcNotification<T> {
    pub fn new<S: Into<String>>(method: S, params: T) -> Self {
        Self { jsonrpc: "2.0".to_string(), method: method.into(), params }
    }
}

pub fn run() {
    let responder = koi_actor::spawn(Responder);
    let receiver = koi_actor::spawn(Receiver::with(responder.channel));
    println!("Established connection");
    start(receiver.channel);
}

pub fn start(receiver: Sender<LspMessage>) {
    loop {
        let mut header_buffer = String::new();
        match std::io::stdin().read_line(&mut header_buffer) {
            Ok(_) => {
                let mut header_contents = header_buffer.splitn(2, ": ");
                match (header_contents.next(), header_contents.next()) {
                    (Some("Content-Length"), Some(msg)) => send_message(&receiver, msg),
                    _ => unimplemented!()
                }
            },
            Err(error) => eprintln!("An error occurred: {}", error)
        }
    }
}

fn send_message(receiver: &Sender<LspMessage>, msg: &str) {
    match deserialize_message(msg) {
        Ok(msg) => receiver.send(msg).expect("Failed to send"),
        Err(err) => println!("*** [WARN] received unsupported message: {}", err)
    }
}

fn deserialize_message(value: &str) -> serde_json::Result<LspMessage> {
    use std::io::prelude::*;

    let value: usize = value.trim_end().parse().unwrap();
    let mut buffer = vec![0u8; value + 2];
    std::io::stdin().read_exact(&mut buffer).unwrap();
    let buffer_string = String::from_utf8(buffer).unwrap();

    println!("--> {}", buffer_string.as_str().trim());

    serde_json::from_str::<LspMessage>(&buffer_string)
}

pub fn send_jsonrpc_response<T, U>(id: U, result: T)
    where T: Serialize, U: Into<Option<usize>>
{
    let response = JsonRpcResponse::new(id, result);
    let response =
        serde_json::to_string(&response)
            .expect("Failed to serialize JSON RPC response.");

    println!("<-- {}", response);

    print!("Content-Length: {}\r\n\r\n", response.len());
    print!("{}", response);

    let _ = std::io::stdout().flush();
}

pub fn send_jsonrpc_response_raw<S: Into<String>>(reponse: S) {
    let response = reponse.into();

    println!("<-- {}", response);

    print!("Content-Length: {}\r\n\r\n", response.len());
    print!("{}", response);

    let _ = std::io::stdout().flush();
}

pub fn send_jsonrpc_notification<S, T>(method: S, params: T)
    where S: Into<String>, T: Serialize
{
    let response = JsonRpcNotification::new(method, params);
    let response =
        serde_json::to_string(&response)
            .expect("Failed to serialize JSON RPC notification.");

    println!("<-- {}", response);

    print!("Content-Length: {}\r\n\r\n", response.len());
    print!("{}", response);
    let _ = std::io::stdout().flush();
}
