pub mod diagnostic;
pub mod files;

use colored::*;
use std::{fmt::Display, io::Write};

pub use crate::diagnostic::*;
pub use crate::files::*;

use crate::files::FileInspector;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Error {
    MissingFile,
    OutOfBounds { given: usize, max: usize },
    IoError(String),
    FmtError(std::fmt::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingFile => write!(f, "missing file"),
            Self::OutOfBounds { given, max } => write!(
                f,
                "the provided index ({given}) is outside the maximum index of {max}",
            ),
            Self::IoError(error) => {
                write!(f, "an IO error occurred: {error}")
            }
            Self::FmtError(error) =>{
                write!(f, "a formatting error occurred: {error}")
            }
        }
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self::IoError(error.to_string())
    }
}

impl From<std::fmt::Error> for Error {
    fn from(error: std::fmt::Error) -> Self {
        Self::FmtError(error)
    }
}

pub fn emit<'a, F: FileInspector<'a>>(
    f: &mut dyn Write,
    inspector: &'a F,
    diagnostic: &Diagnostic<F::FileId>,
) -> Result<()> {
    let file_id = diagnostic.location.file_id;
    let file_name = inspector.name(file_id)?;
    let source = inspector.source(file_id)?;

    let severity = diagnostic.severity;
    let error_range = diagnostic.location.range.clone();
    let error_start = error_range.start;
    let error_end = error_range.end;

    let line_index = inspector.line_index(file_id, error_range.start)?;
    let line_range = inspector.line_range(file_id, line_index)?;
    let line_number = line_index + 1;

    let column_start = inspector.column_number(file_id, error_start)?;
    let column_end = inspector.column_number(file_id, error_end)?;

    let (color, header, underline) = {
        let make_header = |msg: String| {
            let remaining_len = textwrap::termwidth()
                .checked_sub(msg.len())
                .unwrap_or_default();
            format!("{msg}{}", "-".repeat(remaining_len))
        };

        match severity {
            Severity::Bug => {
                let msg = format!("-- Bug: {} ", diagnostic.title);
                let header = make_header(msg);
                (Color::Magenta, header, "^")
            }
            Severity::Error => {
                let msg = format!("-- Error: {} ", diagnostic.title);
                let header = make_header(msg);
                (Color::Red, header, "^")
            }
            Severity::Warning => {
                let msg = format!("-- Warning: {} ", diagnostic.title);
                let header = make_header(msg);
                (Color::Yellow, header, "~")
            }
            Severity::Note => {
                let msg = format!("-- Note: {} ", diagnostic.title);
                let header = make_header(msg);
                (Color::Blue, header, "-")
            }
        }
    };

    macro_rules! wrap {
        ($formatting:literal, $( $args:expr ),* $(,)?) => {
            textwrap::fill(
                &format!($formatting, $( $args ),*),
                textwrap::Options::with_termwidth(),
            )
        };
        ($item:expr) => {
            textwrap::fill(
                &format!("{}", $item),
                textwrap::Options::with_termwidth(),
            )
        };
    }

    let location_str = format!("-> {file_name}:{line_number}:{column_start}");
    writeln!(f, "{}", header.color(color))?;
    writeln!(f, "{}\n", location_str.color(color))?;

    if let Some(description) = &diagnostic.description {
        writeln!(f, "{}\n", wrap!(description))?;
    }

    let gutter = format!("{line_number:>4} | ");
    let line = &source.as_ref()[line_range].trim_end(); // remove trailing LF
    writeln!(f, "{}{line}", gutter.dimmed())?;

    // `column_start` is indexed by 1
    let offset = " ".repeat(gutter.len() + column_start - 1);
    // The difference of the column positions, or 1, whichever is larger
    let underline_count = std::cmp::max(1, column_end - column_start);
    // Underline string repeated `underline_count` times
    let underline = underline.repeat(underline_count).color(color);
    writeln!(f, "{offset}{underline}")?;

    writeln!(f, "{}\n", wrap!(diagnostic.message).trim_end())?;

    if let Some(hint) = &diagnostic.hint {
        writeln!(f, "{}\n", wrap!("{}: {hint}", "Hint".underline()))?;
    }

    Ok(())
}
