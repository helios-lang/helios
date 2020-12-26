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
    let location = format!("-> src/Foo.he ({:?})\n", diagnostic.location.range);
    let make_header = |msg: String| {
        let remaining_len = 80 - msg.len();
        format!("{}{}", msg, "-".repeat(remaining_len))
    };

    match diagnostic.severity {
        Severity::Bug => {
            let msg = format!("-- Bug: {} ", diagnostic.title);
            writeln!(f, "{}", make_header(msg).magenta())?;
            writeln!(f, "{}", location.magenta())?;
        }
        Severity::Error => {
            let msg = format!("-- Error: {} ", diagnostic.title);
            writeln!(f, "{}", make_header(msg).red())?;
            writeln!(f, "{}", location.red())?;
        }
        Severity::Warning => {
            let msg = format!("-- Warning: {} ", diagnostic.title);
            writeln!(f, "{}", make_header(msg).yellow())?;
            writeln!(f, "{}", location.yellow())?;
        }
        Severity::Note => {
            let msg = format!("-- Note: {} ", diagnostic.title);
            writeln!(f, "{}", make_header(msg).blue())?;
            writeln!(f, "{}", location.blue())?;
        }
    }

    if let Some(description) = &diagnostic.description {
        writeln!(f, "{}\n", description)?;
    }

    let line_index = files
        .line_index(
            diagnostic.location.file_id,
            diagnostic.location.range.start,
        )
        .unwrap();

    let line_range = files
        .line_range(diagnostic.location.file_id, line_index)
        .unwrap();

    let error_start = diagnostic.location.range.start - line_index;
    let error_span = diagnostic.location.range.len();

    let input = files.source(diagnostic.location.file_id)?;
    let input = input.as_ref();

    writeln!(
        f,
        "{}{}",
        format!("{:>4} | ", line_index + 1).dimmed(),
        &input[line_range].trim()
    )?;

    writeln!(
        f,
        "{}{}",
        " ".repeat(7 + error_start),
        "^".repeat(error_span).red()
    )?;

    writeln!(f, "{}\n", diagnostic.message)?;

    if let Some(hint) = &diagnostic.hint {
        writeln!(f, "{}\n", hint)?;
    }

    Ok(())
}
