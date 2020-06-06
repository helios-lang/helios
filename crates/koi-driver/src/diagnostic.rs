#[derive(Debug)]
pub struct Diagnostic {
    pub code: String,
    pub message: String,
}

impl Diagnostic {
    pub fn new<S: Into<String>>(code: S, message: S) -> Self {
        Self { code: code.into(), message: message.into() }
    }
}
