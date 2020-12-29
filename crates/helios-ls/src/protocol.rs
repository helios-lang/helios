#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use std::fmt::{self, Debug, Display};
use std::io::{self, Write};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RequestId {
    Integer(i32),
    String(String),
}

impl From<i32> for RequestId {
    fn from(integer: i32) -> RequestId {
        RequestId::Integer(integer)
    }
}

impl From<String> for RequestId {
    fn from(string: String) -> Self {
        RequestId::String(string)
    }
}

impl Display for RequestId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RequestId::Integer(i) => Display::fmt(i, f),
            RequestId::String(s) => Debug::fmt(s, f),
        }
    }
}

/// A request message to describe a request between the client and the server.
/// Every processed request must send a response back to the sender of the
/// request.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Request {
    /// The request id.
    id: RequestId,
    /// The method to be invoked.
    method: String,
    /// The method's params.
    #[serde(default = "serde_json::Value::default")]
    #[serde(skip_serializing_if = "serde_json::Value::is_null")]
    params: serde_json::Value,
}

impl Request {
    pub fn new(
        id: impl Into<RequestId>,
        method: impl Into<String>,
        params: impl Serialize,
    ) -> Self {
        Self {
            id: id.into(),
            method: method.into(),
            params: serde_json::to_value(params).unwrap(),
        }
    }
}

/// A response message sent as a result of a request.
///
/// If a request doesn't provide a result value the receiver of a request still
/// needs to return a response message to conform to the JSON RPC specification.
/// The result property of the `Response` should be set to `null` in this case
/// to signal a successful request.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Response {
    /// The request id.
    id: RequestId,
    /// The result of a request. This member is **REQUIRED** on success. This
    /// member **MUST NOT** exist if there was an error invoking the method.
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<serde_json::Value>,
    /// The error object in case a request fails.
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<ResponseError>,
}

impl Response {
    pub fn new_ok(id: impl Into<RequestId>, result: impl Serialize) -> Self {
        Self {
            id: id.into(),
            result: Some(serde_json::to_value(result).unwrap()),
            error: None,
        }
    }

    pub fn new_error(
        id: impl Into<RequestId>,
        code: ErrorCode,
        message: impl Into<String>,
    ) -> Self {
        let error = ResponseError {
            code: code as i32,
            message: message.into(),
            data: None,
        };

        Self {
            id: id.into(),
            result: None,
            error: Some(error),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ResponseError {
    /// A number indicating the error type that occurred.
    code: i32,
    /// A string providing a short description of the error.
    message: String,
    /// A primitive or structured value that contains additional information
    /// about the error. Can be omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<serde_json::Value>,
}

#[derive(Clone, Copy, Debug)]
#[allow(unused)]
pub enum ErrorCode {
    // Defined by JSON RPC
    ParseError = -32700,
    InvalidRequest = -32600,
    MethodNotFound = -32601,
    InvalidParams = -32602,
    InternalError = -32603,
}

/// A notification message.
///
/// A processed notification message must not send a response back. They work
/// like events.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Notification {
    /// The method to be invoked.
    method: String,
    /// The notification's params.
    #[serde(default = "serde_json::Value::default")]
    #[serde(skip_serializing_if = "serde_json::Value::is_null")]
    params: serde_json::Value,
}

impl Notification {
    pub fn new(method: impl Into<String>, params: impl Serialize) -> Self {
        Self {
            method: method.into(),
            params: serde_json::to_value(params).unwrap(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Message {
    Request(Request),
    Response(Response),
    Notification(Notification),
}

impl From<Request> for Message {
    fn from(request: Request) -> Message {
        Message::Request(request)
    }
}

impl From<Response> for Message {
    fn from(response: Response) -> Message {
        Message::Response(response)
    }
}

impl From<Notification> for Message {
    fn from(notification: Notification) -> Message {
        Message::Notification(notification)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JsonRpc {
    jsonrpc: &'static str,
    #[serde(flatten)]
    message: Message,
}

impl JsonRpc {
    pub fn with(message: impl Into<Message>) -> Self {
        Self {
            jsonrpc: "2.0",
            message: message.into(),
        }
    }

    pub fn write(self, w: &mut impl Write) -> io::Result<()> {
        let text = serde_json::to_string(&self)?;

        write!(w, "Content-Length: {}\r\n\r\n", text.len())?;
        w.write_all(text.as_bytes())?;
        w.flush()?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jsonrpc_to_string() {
        let request = Request::new(0, "shutdown", serde_json::Value::Null);
        let message = serde_json::to_string(&JsonRpc::with(request)).unwrap();
        let content = r#"{"jsonrpc":"2.0","id":0,"method":"shutdown"}"#;
        assert_eq!(content.to_string(), message);
    }

    #[test]
    fn test_jsonrpc_write() {
        let mut buffer = Vec::new();
        let request = Request::new(0, "shutdown", serde_json::Value::Null);
        JsonRpc::with(request).write(&mut buffer).unwrap();

        let content = r#"{"jsonrpc":"2.0","id":0,"method":"shutdown"}"#;
        let header = format!("Content-Length: {}\r\n\r\n", content.len());
        let request = header + content;
        assert_eq!(&request.as_bytes(), &buffer);
    }
}
