use std::error::Error;
use std::fmt::{self, Display};

#[derive(Debug, Clone)]
pub struct ProtocolError(pub(crate) String);

impl Error for ProtocolError {}

impl Display for ProtocolError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.0, f)
    }
}
