use colored::*;
use std::fmt::{self, Display};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FormattedTextSegment {
    LineBreak,
    Text(String),
    Code(String),
    CodeBlock(String),
    List(Vec<FormattedText>),
}

impl FormattedTextSegment {
    pub fn text(text: impl Display) -> Self {
        Self::Text(text.to_string())
    }

    pub fn code(code: impl Display) -> Self {
        Self::Code(code.to_string())
    }

    pub fn code_block(code: impl Display) -> Self {
        Self::CodeBlock(code.to_string())
    }

    pub fn list(list: impl Into<Vec<FormattedText>>) -> Self {
        Self::List(list.into())
    }
}

impl Display for FormattedTextSegment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let colorize = colored::control::SHOULD_COLORIZE.should_colorize();

        match self {
            Self::LineBreak => write!(f, "\n\n"),
            Self::Text(text) => write!(f, "{}", text),
            Self::Code(code) => {
                if colorize {
                    write!(f, "{}", code.yellow())
                } else {
                    write!(f, "{}", code)
                }
            }
            Self::CodeBlock(block) => {
                if colorize {
                    write!(f, "    {}", block.yellow())
                } else {
                    write!(f, "    {}", block)
                }
            }
            Self::List(lines) => {
                for line in lines {
                    write!(f, "    {}\n", line)?;
                }

                Ok(())
            }
        }
    }
}

impl From<String> for FormattedTextSegment {
    fn from(string: String) -> Self {
        FormattedTextSegment::Text(string)
    }
}

impl From<&str> for FormattedTextSegment {
    fn from(string: &str) -> Self {
        FormattedTextSegment::Text(string.to_string())
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct FormattedText {
    segments: Vec<FormattedTextSegment>,
}

impl FormattedText {
    pub fn new(segments: impl Into<Vec<FormattedTextSegment>>) -> Self {
        Self {
            segments: segments.into(),
        }
    }

    pub fn push(&mut self, segment: impl Into<FormattedTextSegment>) {
        self.segments.push(segment.into())
    }

    pub fn text(mut self, text: impl Display) -> Self {
        self.segments.push(FormattedTextSegment::text(text));
        self
    }

    pub fn code(mut self, code: impl Display) -> Self {
        self.segments.push(FormattedTextSegment::code(code));
        self
    }

    pub fn code_block(mut self, code: impl Display) -> Self {
        self.segments.push(FormattedTextSegment::LineBreak);
        self.segments.push(FormattedTextSegment::code_block(code));
        self.segments.push(FormattedTextSegment::LineBreak);
        self
    }

    pub fn list(mut self, list: impl Into<Vec<FormattedText>>) -> Self {
        self.segments.push(FormattedTextSegment::LineBreak);
        self.segments.push(FormattedTextSegment::list(list));
        self.segments.push(FormattedTextSegment::LineBreak);
        self
    }

    pub fn line_break(mut self) -> Self {
        self.segments.push(FormattedTextSegment::LineBreak);
        self
    }
}

impl From<String> for FormattedText {
    fn from(s: String) -> Self {
        Self::new([s.into()])
    }
}

impl From<&str> for FormattedText {
    fn from(s: &str) -> Self {
        Self::new([s.into()])
    }
}

impl Display for FormattedText {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for segment in &self.segments {
            write!(f, "{}", segment)?;
        }

        Ok(())
    }
}
