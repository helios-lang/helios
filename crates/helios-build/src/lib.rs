use colored::*;
use helios_diagnostics::{files::SimpleFiles, Diagnostic};
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
                write!(f, "an IO error occurred: {}", error)
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

    for message in messages_rx.try_iter() {
        let diagnostic = Diagnostic::from(message);

        if !(emitted_ranges.contains(&diagnostic.location)) {
            emitted_ranges.push(diagnostic.location.clone());
            helios_diagnostics::emit(&mut stdout, &files, &diagnostic)
                .expect("Failed to print diagnostic");
        }
    }

    if message_count > 0 {
        Err(Error::BuildError(message_count))
    } else {
        Ok(())
    }
}

/// Starts the build process with the given file content input.
pub fn build(path: &str) {
    println!("{} {}...\n", "Building".green().bold(), path.underline());

    match build_inner(path) {
        Ok(()) => println!("Finished."),
        Err(error) => eprintln!("{}", format!("{}", error).red()),
    }
}
