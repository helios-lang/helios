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
        write!(f, "{:?}", self)
    }
}

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

impl std::error::Error for Error {}

pub fn emit<'files, F: Files<'files>>(
    f: &mut dyn Write,
    files: &'files F,
    diagnostic: &Diagnostic<F::FileId>,
) -> Result<()> {
    let file_id = diagnostic.location.file_id;
    let source = files.source(file_id)?;

    let error_range = diagnostic.location.range.clone();
    let error_start = error_range.start;
    let severity = diagnostic.severity;

    let line_index = files.line_index(file_id, error_range.start)?;
    let line_range = files.line_range(file_id, line_index)?;

    let line_number = files.line_number(file_id, line_index)?;
    let col_number = files.column_number(file_id, line_index, error_start)?;

    let (color, header, underline_str) = {
        let make_header = |msg: String| {
            let remaining_len = textwrap::termwidth() - msg.len();
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

    let location_str = format!("-> src/Foo.he:{}:{}", line_number, col_number);
    writeln!(f, "{}", header.color(color))?;
    writeln!(f, "{}\n", location_str.color(color))?;

    if let Some(description) = &diagnostic.description {
        writeln!(f, "{}\n", wrap!(description))?;
    }

    let gutter = format!("{:>4} | ", line_number);
    let line = &source.as_ref()[line_range].trim_end(); // remove trailing LF
    writeln!(f, "{}{}", gutter.dimmed(), line)?;

    let offset = " ".repeat(gutter.len() + error_start);
    let underline_str = underline_str.repeat(error_range.len()).color(color);
    writeln!(f, "{}{}", offset, underline_str)?;

    writeln!(f, "{}\n", wrap!(diagnostic.message))?;

    if let Some(hint) = &diagnostic.hint {
        writeln!(f, "{}\n", wrap!("{}: {}", "Hint".underline(), hint))?;
    }

    Ok(())
}
