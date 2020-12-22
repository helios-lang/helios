use std::ops::Range;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[repr(u8)]
pub enum Severity {
    Bug = 3,
    Error = 2,
    Warning = 1,
    Note = 0,
}

/// A diagnostic that provides information about a found problem in a Helios
/// source file like errors or warnings.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Diagnostic {
    /// The severity of the diagnostic.
    pub severity: Severity,
    /// A short summary of the diagnostic found.
    pub title: String,
    /// A collection of [`SubDiagnostic`]s that describe the problem in detail.
    pub details: Vec<SubDiagnostic>,
    /// A collection of possible suggestions to fix the problem.
    pub suggestions: Vec<Suggestion>,
}

impl Diagnostic {
    /// Constructs a new [`Diagnostic`] with the given severity, title, details
    /// and suggestions.
    pub fn new(
        severity: Severity,
        title: impl Into<String>,
        details: impl Into<Option<Vec<SubDiagnostic>>>,
        suggestions: impl Into<Option<Vec<Suggestion>>>,
    ) -> Self {
        Self {
            severity,
            title: title.into(),
            details: details.into().unwrap_or_default(),
            suggestions: suggestions.into().unwrap_or_default(),
        }
    }

    /// Constructs a new [`Diagnostic`] with the [`Error`] severity.
    ///
    /// [`Error`]: crate::Severity::Error
    pub fn error(title: impl Into<String>) -> Self {
        Self::new(Severity::Error, title, None, None)
    }

    /// Constructs a new [`Diagnostic`] with the [`Warning`] severity.
    ///
    /// [`Warning`]: crate::Severity::Warning
    pub fn warning(title: impl Into<String>) -> Self {
        Self::new(Severity::Warning, title, None, None)
    }

    /// Constructs a new [`Diagnostic`] with the [`Note`] severity.
    ///
    /// [`Note`]: crate::Severity::Note
    pub fn note(title: impl Into<String>) -> Self {
        Self::new(Severity::Note, title, None, None)
    }

    /// Attaches an additional detail describing the diagnostic.
    pub fn detail(
        mut self,
        message: impl Into<String>,
        range: Range<usize>,
    ) -> Self {
        self.details.push(SubDiagnostic::new(message, range));
        self
    }

    /// Attaches an additional suggestion for the diagnostic.
    pub fn suggestion(mut self, suggestion: impl Into<String>) -> Self {
        self.suggestions.push(suggestion.into());
        self
    }
}

/// Additional information that may be added to a [`Diagnostic`].
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SubDiagnostic {
    pub message: String,
    pub range: Range<usize>,
}

impl SubDiagnostic {
    /// Constructs a new [`SubDiagnostic`] with the given message and range.
    pub fn new(message: impl Into<String>, range: Range<usize>) -> Self {
        Self {
            message: message.into(),
            range,
        }
    }
}

/// A suggestion that may be added to a [`Diagnostic`].
///
/// For now, this type is merely an alias to [`String`].
pub type Suggestion = String;

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
    fn test_error_diagnostic_with_details_and_suggestions() {
        let diagnostic = Diagnostic::error("An error message")
            .detail("... which is caused by this section of code", 0..3)
            .detail("... and also this section of code", 6..10)
            .suggestion("try doing XYZ");

        assert_eq!(
            diagnostic,
            Diagnostic {
                severity: Severity::Error,
                title: "An error message".to_string(),
                details: vec![
                    SubDiagnostic {
                        message: "... which is caused by this section of code"
                            .to_string(),
                        range: 0..3
                    },
                    SubDiagnostic {
                        message: "... and also this section of code"
                            .to_string(),
                        range: 6..10
                    }
                ],
                suggestions: vec!["try doing XYZ".to_string()],
            }
        );
    }
}
