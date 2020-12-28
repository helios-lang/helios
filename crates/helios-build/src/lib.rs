use colored::*;
use helios_diagnostics::files::SimpleFiles;
use helios_diagnostics::{Diagnostic, Severity};
use std::fmt::Display;

type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug, Eq, PartialEq)]
enum Error {
    BuildError(usize),
    IoError(String),
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self::IoError(error.to_string())
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BuildError(count) => {
                write!(
                    f,
                    "Failed to build due to {} previous error{}",
                    count,
                    if *count == 1 { "" } else { "s" }
                )
            }
            Self::IoError(error) => {
                write!(f, "An IO error occurred: {}", error)
            }
        }
    }
}

fn build_inner(path: &str) -> Result<()> {
    let mut stdout = std::io::stdout();
    let (messages_tx, messages_rx) = flume::unbounded();

    let source = std::fs::read_to_string(path)?;
    let mut files = SimpleFiles::new();

    let file_id = files.add(path, source);
    let file = files.get(file_id).unwrap();

    let parse = helios_parser::parse(file_id, file.source(), messages_tx);
    println!("{}", parse.debug_tree().cyan());

    let message_count = messages_rx.len();
    let mut emitted_ranges = Vec::new();
    let mut severities = Vec::new();

    for message in messages_rx.try_iter() {
        let diagnostic = Diagnostic::from(message);
        severities.push(diagnostic.severity);

        if !(emitted_ranges.contains(&diagnostic.location)) {
            emitted_ranges.push(diagnostic.location.clone());
            helios_diagnostics::emit(&mut stdout, &files, &diagnostic)
                .expect("Failed to print diagnostic");
        }
    }

    // An empty vector (i.e. no messages to report) or a vector of severities
    // lower in importance than error is okay
    let is_ok = {
        severities.is_empty()
            || severities
                .iter()
                .any(|severity| *severity < Severity::Error)
    };

    if is_ok {
        Ok(())
    } else {
        Err(Error::BuildError(message_count))
    }
}

/// Starts the build process with the given path to a file.
pub fn build(path: &str) {
    println!("{} {}...\n", "Building".green().bold(), path.underline());

    match build_inner(path) {
        Ok(()) => println!("{}", "Finished building".green().bold()),
        Err(error) => {
            eprintln!("{}", format!("{}", error).red().bold());
            std::process::exit(1)
        },
    }
}
