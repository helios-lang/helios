use colored::*;
use helios_diagnostics::files::SimpleFiles;
use helios_diagnostics::{Diagnostic, Severity};
use std::fmt::Display;

/// Compiling support for Helios files
#[derive(clap::Parser)]
pub struct HeliosBuildOpts {
    /// The entry point file for the program to be built
    pub file: String,
}

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
                let suffix = if *count == 1 { "" } else { "s" };
                write!(
                    f,
                    "Failed to build due to {count} previous error{suffix}"
                )
            }
            Self::IoError(error) => {
                write!(f, "Failed to build due to an IO error: {error}")
            }
        }
    }
}

fn __build(path: &str) -> Result<()> {
    let source = std::fs::read_to_string(path)?;
    let mut stdout = std::io::stdout();
    let mut files = SimpleFiles::new();

    let file_id = files.add(path, source);
    let file = files.get(file_id).unwrap();

    let parse = helios_parser::parse(file_id, file.source());
    println!("{}", parse.debug_tree().cyan());

    let mut emitted_ranges = Vec::new();
    let mut severities = Vec::new();

    for message in parse.messages() {
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

    let message_count = emitted_ranges.len();

    if is_ok {
        Ok(())
    } else {
        Err(Error::BuildError(message_count))
    }
}

/// Starts the build process with the given path to a file.
pub fn build(path: &str) {
    println!("\n{} {}\n", "Building".green().bold(), path.underline());

    if let Err(error) = __build(path) {
        let error = format!("{}", error).red().bold();
        eprintln!("{}", error);
        std::process::exit(1);
    }

    println!("{}", "Finished building".green().bold());
}
