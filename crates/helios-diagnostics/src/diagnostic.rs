use colored::*;
use std::fmt::{self, Display};
use text_size::TextRange;

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
    InlineCodeSnippet(String),
}

impl MessageSegment {
    pub fn text(string: impl Into<String>) -> Self {
        Self::Text(string.into())
    }

    pub fn code_snippet(string: impl Into<String>) -> Self {
        Self::CodeSnippet(string.into())
    }

    pub fn inline_code_snippet(string: impl Into<String>) -> Self {
        Self::InlineCodeSnippet(string.into())
    }
}

impl Display for MessageSegment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Text(s) => write!(f, "{}", s),
            Self::CodeSnippet(s) => write!(f, "{}", s.yellow()),
            Self::InlineCodeSnippet(s) => write!(f, "{}", s.yellow()),
        }
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Message {
    segments: Vec<MessageSegment>,
}

impl Message {
    pub fn string_with_inline_code_snippets(s: impl Into<String>) -> Self {
        let string: String = s.into();
        let mut segments = Vec::new();
        let mut in_code_snippet = string.starts_with("`");

        for s in string.split("`").filter(|s| !s.is_empty()) {
            if in_code_snippet {
                segments.push(MessageSegment::inline_code_snippet(s))
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
        Self::string_with_inline_code_snippets(string)
    }
}

impl From<&str> for Message {
    fn from(string: &str) -> Self {
        Self::string_with_inline_code_snippets(string)
    }
}

/// A suggestion that may be added to a [`Diagnostic`].
///
/// For now, this type is merely an alias to [`String`].
pub type Hint = String;

/// A diagnostic that provides information about a found problem in a Helios
/// source file like errors or warnings.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Diagnostic {
    /// The severity of the diagnostic.
    severity: Severity,
    title: String,
    description: Option<String>,
    message: Message,
    range: TextRange,
    hint: Option<Hint>,
}

impl Diagnostic {
    pub fn new(
        severity: Severity,
        title: impl Into<String>,
        description: impl Into<Option<String>>,
        message: impl Into<Message>,
        range: impl Into<TextRange>,
        hint: impl Into<Option<Hint>>,
    ) -> Self {
        Self {
            severity,
            title: title.into(),
            description: description.into(),
            message: message.into(),
            range: range.into(),
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

    pub fn range(mut self, range: impl Into<TextRange>) -> Self {
        self.range = range.into();
        self
    }

    pub fn hint(mut self, hint: impl Into<Hint>) -> Self {
        self.hint = Some(hint.into());
        self
    }
}

impl Display for Diagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.severity {
            Severity::Error => {
                writeln!(f, "{}", format!("-- Error: {}", self.title).red())?
            }
            _ => todo!(),
        }

        writeln!(f, "{}", "-> src/Errors.he:##:##".dimmed())?;

        if let Some(description) = &self.description {
            writeln!(f, "\n{}", description)?;
        }

        writeln!(f, "\n{}", self.message)?;

        if let Some(hint) = &self.hint {
            writeln!(f, "\n{}", hint)?;
        }

        Ok(())
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
            Message::string_with_inline_code_snippets(
                "I expected a colon (`:`) or an equal symbol (`=`) here"
            ),
            Message {
                segments: vec![
                    MessageSegment::text("I expected a colon ("),
                    MessageSegment::inline_code_snippet(":"),
                    MessageSegment::text(") or an equal symbol ("),
                    MessageSegment::inline_code_snippet("="),
                    MessageSegment::text(") here"),
                ]
            }
        );

        assert_eq!(
            Message::string_with_inline_code_snippets(
                "`external` is `not` a `valid` identifier `here`"
            ),
            Message {
                segments: vec![
                    MessageSegment::inline_code_snippet("external"),
                    MessageSegment::text(" is "),
                    MessageSegment::inline_code_snippet("not"),
                    MessageSegment::text(" a "),
                    MessageSegment::inline_code_snippet("valid"),
                    MessageSegment::text(" identifier "),
                    MessageSegment::inline_code_snippet("here"),
                ]
            }
        );

        assert_eq!(
            Message::string_with_inline_code_snippets(
                "this string doesn't terminate `properly"
            ),
            Message {
                segments: vec![
                    MessageSegment::text("this string doesn't terminate "),
                    MessageSegment::inline_code_snippet("properly"),
                ]
            }
        );
    }
}
