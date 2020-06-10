use crate::Position;
use std::ops::Range;
use std::fmt::{self, Display};

#[derive(Debug)]
pub struct Diagnostic {
    pub message: String,
    pub range: Range<Position>,
}

impl Diagnostic {
    pub fn new<S: Into<String>>(message: S, range: Range<Position>) -> Self {
        Self { message: message.into(), range }
    }
}

impl Display for Diagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} to {} => {}", self.range.start, self.range.end, self.message)
    }
}
