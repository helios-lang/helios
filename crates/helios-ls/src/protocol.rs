use serde::{Deserialize, Serialize};
use std::fmt::{self, Debug, Display};
use std::io::{self, BufRead, Write};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
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

impl From<&str> for RequestId {
    fn from(s: &str) -> Self {
        RequestId::String(s.to_string())
    }
}

impl From<String> for RequestId {
    fn from(s: String) -> Self {
        RequestId::String(s)
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
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Request {
    /// The request id.
    pub(crate) id: RequestId,
    /// The method to be invoked.
    pub(crate) method: String,
    /// The method's params.
    #[serde(default = "serde_json::Value::default")]
    #[serde(skip_serializing_if = "serde_json::Value::is_null")]
    pub(crate) params: serde_json::Value,
}

impl Request {
    #[allow(dead_code)]
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

    #[allow(dead_code)]
    pub fn new_without_params(
        id: impl Into<RequestId>,
        method: impl Into<String>,
    ) -> Self {
        Self::new(id, method, serde_json::Value::Null)
    }

    pub fn is_initialize(&self) -> bool {
        self.method == "initialize"
    }

    #[allow(dead_code)]
    pub fn is_shutdown(&self) -> bool {
        self.method == "shutdown"
    }
}

/// A response message sent as a result of a request.
///
/// If a request doesn't provide a result value the receiver of a request still
/// needs to return a response message to conform to the JSON RPC specification.
/// The result property of the `Response` should be set to `null` in this case
/// to signal a successful request.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Response {
    /// The request id.
    pub(crate) id: RequestId,
    /// The result of a request. This member is **REQUIRED** on success. This
    /// member **MUST NOT** exist if there was an error invoking the method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) result: Option<serde_json::Value>,
    /// The error object in case a request fails.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) error: Option<ResponseError>,
}

impl Response {
    pub fn new_ok(id: impl Into<RequestId>, result: impl Serialize) -> Self {
        Self {
            id: id.into(),
            result: Some(serde_json::to_value(result).unwrap()),
            error: None,
        }
    }

    #[allow(dead_code)]
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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResponseError {
    /// A number indicating the error type that occurred.
    pub(crate) code: i32,
    /// A string providing a short description of the error.
    pub(crate) message: String,
    /// A primitive or structured value that contains additional information
    /// about the error. Can be omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) data: Option<serde_json::Value>,
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
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Notification {
    /// The method to be invoked.
    pub(crate) method: String,
    /// The notification's params.
    #[serde(default = "serde_json::Value::default")]
    #[serde(skip_serializing_if = "serde_json::Value::is_null")]
    pub(crate) params: serde_json::Value,
}

impl Notification {
    #[allow(dead_code)]
    pub fn new(method: impl Into<String>, params: impl Serialize) -> Self {
        Self {
            method: method.into(),
            params: serde_json::to_value(params).unwrap(),
        }
    }

    pub fn is_exit(&self) -> bool {
        self.method == "exit"
    }
}

/// An enumeration that can either be a [`Request`], a [`Response`] or a
/// [`Notification`] as defined by the JSON-RPC.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Message {
    Request(Request),
    Response(Response),
    Notification(Notification),
}

impl Message {
    pub fn read(reader: &mut impl BufRead) -> io::Result<Option<Self>> {
        if let Some(input) = read_message(reader)? {
            let input = serde_json::from_str(&input)?;
            Ok(Some(input))
        } else {
            Ok(None)
        }
    }

    pub fn write(self, writer: &mut impl Write) -> io::Result<()> {
        #[derive(Serialize)]
        struct JsonRpc {
            jsonrpc: &'static str,
            #[serde(flatten)]
            pub(crate) message: Message,
        }

        let content = serde_json::to_string(&JsonRpc {
            jsonrpc: "2.0",
            message: self,
        })?;

        write_message(writer, &content)
    }
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

fn invalid_data(
    error: impl Into<Box<dyn std::error::Error + Send + Sync>>,
) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, error)
}

fn read_message(reader: &mut impl BufRead) -> io::Result<Option<String>> {
    macro_rules! invalid_data {
        ($($tt:tt)*) => (invalid_data(format!($($tt)*)))
    }

    let mut header_buffer = String::new();
    let mut content_length = None::<usize>;

    // Process the header of the JSON-RPC message
    loop {
        header_buffer.clear();

        // An empty header means no input; we return `None` here to signify
        // that we successfully read the input
        if reader.read_line(&mut header_buffer)? == 0 {
            return Ok(None);
        }

        // The header MUST be separated by a CRLF sequence
        if !header_buffer.ends_with("\r\n") {
            return Err(invalid_data!("malformed header: {:?}", header_buffer));
        }

        // We'll ignore the CRLF sequence and split the field and value
        let header_buffer = &header_buffer[..header_buffer.len() - 2];
        if header_buffer.is_empty() {
            break;
        }

        let mut header_parts = header_buffer.splitn(2, ": ");

        // If we get a valid field, we'll process it, otherwise we'll bail with
        // an error message. If we don't find anything, we'll assume to have
        // finished processing the header and break the loop.
        match header_parts.next() {
            None => break,
            Some("Content-Length") => {
                let value = header_parts.next().ok_or_else(|| {
                    invalid_data!("missing value for `Content-Length` field")
                })?;

                let parsed_value = value.parse().map_err(invalid_data)?;
                content_length = Some(parsed_value)
            }
            Some("Content-Type") => {
                log::warn!(
                    "The `Content-Type` field is unsupported at the moment. \
                     Defaulting to `application/vscode-jsonrpc; charset=utf-8`."
                );
                break;
            }
            Some(field) => {
                return Err(invalid_data!("unrecognized field: {:?}", field))
            }
        }
    }

    let content_length = content_length
        .ok_or_else(|| invalid_data!("missing `Content-Length` value"))?;
    let mut buffer = header_buffer.into_bytes();
    buffer.resize(content_length, 0);
    reader.read_exact(&mut buffer)?;

    let buffer = String::from_utf8(buffer).map_err(invalid_data)?;
    log::trace!("-> {}", buffer);

    Ok(Some(buffer))
}

fn write_message(writer: &mut impl Write, message: &str) -> io::Result<()> {
    log::trace!("<- {}", message);

    write!(writer, "Content-Length: {}\r\n\r\n", message.len())?;
    writer.write_all(message.as_bytes())?;
    writer.flush()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const CONTENT: &str = r#"{"jsonrpc":"2.0","id":0,"method":"shutdown"}"#;
    const CONTENT_LEN: usize = CONTENT.len();

    #[test]
    fn test_message_from_valid_input() {
        macro_rules! check {
            ($buffer:expr, $expected:expr) => {
                assert_eq!(
                    Message::read(&mut $buffer.as_bytes()).unwrap(),
                    Some($expected.into())
                )
            };
        }

        // Without `Content-Type` header field
        let header = format!("Content-Length: {}\r\n\r\n", CONTENT_LEN);
        let request = header + CONTENT;
        check!(request, Request::new_without_params(0, "shutdown"));

        // With `Content-Type` header field
        let header = format!("Content-Length: {}\r\nContent-Type: \"application/vscode-jsonrpc; charset=utf-8\"\r\n", CONTENT_LEN);
        let request = header + CONTENT;
        check!(request, Request::new_without_params(0, "shutdown"));
    }

    #[test]
    fn test_message_from_invalid_input() {
        // Missing header
        assert!(Message::read(&mut CONTENT.as_bytes()).is_err());

        // Missing `Content-Length` value
        let header = format!("Content-Length: \r\n\r\n");
        let request = header + CONTENT;
        assert!(Message::read(&mut request.as_bytes()).is_err());

        // Missing header fields
        let header = format!("\r\n\r\n");
        let request = header + CONTENT;
        assert!(Message::read(&mut request.as_bytes()).is_err());

        // Invalid header fields
        let header = format!("Foo: abc\r\nBar: def\r\n");
        let request = header + CONTENT;
        assert!(Message::read(&mut request.as_bytes()).is_err());

        // Malformed header
        let header = format!("abcdef\r\n");
        let request = header + CONTENT;
        assert!(Message::read(&mut request.as_bytes()).is_err());

        // Malformed input
        assert!(Message::read(&mut "<INVALID-INPUT>".as_bytes()).is_err());
    }
}
