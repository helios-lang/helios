pub mod diagnostic;
pub mod files;

pub use crate::diagnostic::*;
use colored::*;
use files::Files;
use std::{fmt::Display, io::Write};

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
                "the provided index ({}) is outside the maximum index of {}",
                given, max
            ),
            Self::IoError(error) => {
                write!(f, "an IO error occurred: {}", error)
            }
            Self::FmtError(error) => {
                write!(f, "a formatting error occurred: {}", error)
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

pub fn emit<'files, F: Files<'files>>(
    f: &mut dyn Write,
    files: &'files F,
    diagnostic: &Diagnostic<F::FileId>,
) -> Result<()> {
    let file_id = diagnostic.location.file_id;
    let source = files.source(file_id)?;

    let severity = diagnostic.severity;
    let error_range = diagnostic.location.range.clone();
    let error_start = error_range.start;
    let error_end = error_range.end;

    let line_index = files.line_index(file_id, error_range.start)?;
    let line_range = files.line_range(file_id, line_index)?;

    let line_number = files.line_number(file_id, line_index)?;
    let column_start = files.column_number(file_id, line_index, error_start)?;
    let column_end = files.column_number(file_id, line_index, error_end)?;

    let (color, header, underline) = {
        let make_header = |msg: String| {
            let remaining_len = textwrap::termwidth()
                .checked_sub(msg.len())
                .unwrap_or_default();
            format!("{}{}", msg, "-".repeat(remaining_len))
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
    };

    let location_str =
        format!("-> src/Foo.he:{}:{}", line_number, column_start);
    writeln!(f, "{}", header.color(color))?;
    writeln!(f, "{}\n", location_str.color(color))?;

    if let Some(description) = &diagnostic.description {
        writeln!(f, "{}\n", wrap!(description))?;
    }

    let gutter = format!("{:>4} | ", line_number);
    let line = &source.as_ref()[line_range].trim_end(); // remove trailing LF
    writeln!(f, "{}{}", gutter.dimmed(), line)?;

    // `column_start` is indexed by 1
    let offset = " ".repeat(gutter.len() + column_start - 1);
    // The difference of the column positions, or 1, whichever is larger
    let underline_count = std::cmp::max(1, column_end - column_start);
    // Underline string repeated `underline_count` times
    let underline = underline.repeat(underline_count).color(color);
    writeln!(f, "{}{}", offset, underline)?;

    writeln!(f, "{}\n", wrap!(diagnostic.message).trim_end())?;

    if let Some(hint) = &diagnostic.hint {
        writeln!(f, "{}\n", wrap!("{}: {}", "Hint".underline(), hint))?;
    }

    Ok(())
}
