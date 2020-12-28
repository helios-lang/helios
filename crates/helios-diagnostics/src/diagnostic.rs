use helios_formatting::FormattedString;
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

/// A diagnostic that provides information about a found issue in a Helios
/// source file like errors or warnings.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Diagnostic<FileId> {
    pub location: Location<FileId>,
    pub severity: Severity,
    pub title: String,
    pub description: Option<FormattedString>,
    pub message: FormattedString,
    pub hint: Option<FormattedString>,
}

impl<FileId> Diagnostic<FileId>
where
    FileId: Default,
{
    pub fn new(
        location: Location<FileId>,
        severity: Severity,
        title: impl Into<String>,
        description: impl Into<Option<FormattedString>>,
        message: impl Into<FormattedString>,
        hint: impl Into<Option<FormattedString>>,
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

    pub fn description(
        mut self,
        description: impl Into<FormattedString>,
    ) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn message(mut self, message: impl Into<FormattedString>) -> Self {
        self.message = message.into();
        self
    }

    pub fn location(mut self, location: Location<FileId>) -> Self {
        self.location = location;
        self
    }

    pub fn hint(mut self, hint: impl Into<FormattedString>) -> Self {
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
}
