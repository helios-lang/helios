use colored::*;
use std::fmt::{self, Display};
use std::ops::Range;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Location<FileId> {
    pub file_id: FileId,
    pub range: Range<usize>,
}

impl<FileId> Location<FileId> {
    pub fn new(file_id: FileId, range: impl Into<Range<usize>>) -> Self {
        Self {
            file_id,
            range: range.into(),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[repr(u8)]
pub enum Severity {
    Bug = 3,
    Error = 2,
    Warning = 1,
    Note = 0,
}

impl Default for Severity {
    fn default() -> Self {
        Self::Note
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MessageSegment {
    Text(String),
    CodeSnippet(String),
}

impl MessageSegment {
    pub fn text(string: impl Into<String>) -> Self {
        Self::Text(string.into())
    }

    pub fn code_snippet(string: impl Into<String>) -> Self {
        Self::CodeSnippet(string.into())
    }
}

impl Display for MessageSegment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if colored::control::SHOULD_COLORIZE.should_colorize() {
            match self {
                Self::Text(s) => write!(f, "{}", s),
                Self::CodeSnippet(s) => write!(f, "{}", s.yellow()),
            }
        } else {
            match self {
                Self::Text(s) => write!(f, "{}", s),
                Self::CodeSnippet(s) => write!(f, "`{}`", s),
            }
        }
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Message {
    segments: Vec<MessageSegment>,
}

impl Message {
    pub fn from_formatted_string(s: impl Into<String>) -> Self {
        let string: String = s.into();
        let mut segments = Vec::new();
        let mut in_code_snippet = string.starts_with("`");

        for s in string.split("`").filter(|s| !s.is_empty()) {
            if in_code_snippet {
                segments.push(MessageSegment::code_snippet(s))
            } else {
                segments.push(MessageSegment::text(s))
            }

            in_code_snippet = !in_code_snippet;
        }

        Self { segments }
    }
}

impl Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.segments
                .iter()
                .map(|s| format!("{}", s))
                .collect::<Vec<_>>()
                .join("")
        )?;

        Ok(())
    }
}

impl From<String> for Message {
    fn from(string: String) -> Self {
        Self::from_formatted_string(string)
    }
}

impl From<&str> for Message {
    fn from(string: &str) -> Self {
        Self::from_formatted_string(string)
    }
}

pub type Hint = Message;

/// A diagnostic that provides information about a found issue in a Helios
/// source file like errors or warnings.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Diagnostic<FileId> {
    pub location: Location<FileId>,
    pub severity: Severity,
    pub title: String,
    pub description: Option<String>,
    pub message: Message,
    pub hint: Option<Hint>,
}

impl<FileId> Diagnostic<FileId>
where
    FileId: Default,
{
    pub fn new(
        location: Location<FileId>,
        severity: Severity,
        title: impl Into<String>,
        description: impl Into<Option<String>>,
        message: impl Into<Message>,
        hint: impl Into<Option<Hint>>,
    ) -> Self {
        Self {
            location,
            severity,
            title: title.into(),
            description: description.into(),
            message: message.into(),
            hint: hint.into(),
        }
    }

    pub fn bug(title: impl Into<String>) -> Self {
        Self {
            severity: Severity::Bug,
            title: title.into(),
            ..Self::default()
        }
    }

    pub fn error(title: impl Into<String>) -> Self {
        Self {
            severity: Severity::Error,
            title: title.into(),
            ..Self::default()
        }
    }

    pub fn warning(title: impl Into<String>) -> Self {
        Self {
            severity: Severity::Warning,
            title: title.into(),
            ..Self::default()
        }
    }

    pub fn note(title: impl Into<String>) -> Self {
        Self {
            severity: Severity::Note,
            title: title.into(),
            ..Self::default()
        }
    }

    pub fn severity(mut self, severity: Severity) -> Self {
        self.severity = severity;
        self
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn message(mut self, message: impl Into<Message>) -> Self {
        self.message = message.into();
        self
    }

    pub fn location(mut self, location: Location<FileId>) -> Self {
        self.location = location;
        self
    }

    pub fn hint(mut self, hint: impl Into<Hint>) -> Self {
        self.hint = Some(hint.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compare_severity_ok() {
        let mut is_ok = true;

        for severity in &[Severity::Note, Severity::Note, Severity::Warning] {
            is_ok &= *severity < Severity::Error;
        }

        assert!(is_ok);
    }

    #[test]
    fn test_compare_severity_not_ok() {
        let mut is_ok = true;

        for severity in &[Severity::Note, Severity::Error, Severity::Warning] {
            is_ok &= *severity < Severity::Error;
        }

        assert!(!is_ok);
    }

    #[test]
    fn test_message_string_with_line_code_snippets() {
        assert_eq!(
            Message::from_formatted_string(
                "I expected a colon (`:`) or an equal symbol (`=`) here"
            ),
            Message {
                segments: vec![
                    MessageSegment::text("I expected a colon ("),
                    MessageSegment::code_snippet(":"),
                    MessageSegment::text(") or an equal symbol ("),
                    MessageSegment::code_snippet("="),
                    MessageSegment::text(") here"),
                ]
            }
        );

        assert_eq!(
            Message::from_formatted_string(
                "`external` is `not` a `valid` identifier `here`"
            ),
            Message {
                segments: vec![
                    MessageSegment::code_snippet("external"),
                    MessageSegment::text(" is "),
                    MessageSegment::code_snippet("not"),
                    MessageSegment::text(" a "),
                    MessageSegment::code_snippet("valid"),
                    MessageSegment::text(" identifier "),
                    MessageSegment::code_snippet("here"),
                ]
            }
        );

        assert_eq!(
            Message::from_formatted_string(
                "this string doesn't terminate `properly"
            ),
            Message {
                segments: vec![
                    MessageSegment::text("this string doesn't terminate "),
                    MessageSegment::code_snippet("properly"),
                ]
            }
        );
    }
}
