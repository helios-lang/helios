//! REPL support for the Helios programming language.

use colored::*;
use helios_diagnostics::files::SimpleFiles;
use helios_diagnostics::Diagnostic;
use std::io::{self, Write};

const LOGO_BANNER: &[&str] = &[
    r"          __   __     __              ",
    r"         / /  / /__  / /_)__  ___     ",
    r"        / /__/ / _ \/ / / _ \(_ /_    ",
    r"       / ,__, / ,__/ / / _/ /__) /    ",
    r"      /_/  /_/\___/_/_/\___/____/     ",
    r"",
];

fn print_logo_banner() -> io::Result<()> {
    for (i, line) in LOGO_BANNER.iter().enumerate() {
        match i {
            2 => println!(
                "{}{}",
                line.yellow().bold(),
                format!(
                    "Version {} ({})",
                    env!("CARGO_PKG_VERSION"),
                    env!("GIT_HASH")
                )
                .italic(),
            ),
            3 => println!(
                "{}{}",
                line.yellow().bold(),
                env!("CARGO_PKG_REPOSITORY").italic()
            ),
            4 => println!(
                "{}{}",
                line.yellow().bold(),
                format!(
                    "Type {} to exit, {} for help",
                    "#exit".blue(),
                    "#help".blue()
                )
                .italic()
            ),
            _ => println!("{}", line.trim_end().yellow().bold()),
        }
    }

    Ok(())
}

fn start_main_loop() -> io::Result<()> {
    print_logo_banner()?;

    let stdin = io::stdin();
    let mut stdout = io::stdout();

    let mut input = String::new();
    let mut files = SimpleFiles::new();
    let (messages_tx, messages_rx) = flume::unbounded();

    loop {
        write!(stdout, "{}", "> ".blue())?;
        stdout.flush()?;

        stdin.read_line(&mut input)?;

        if input.trim().is_empty() {
            continue;
        }

        if input.trim().starts_with("#") {
            let input = input[1..].trim();
            match input {
                "exit" => break,
                "help" => {
                    println!("{}", "Help is not available at the moment".blue())
                }
                command => {
                    let msg = format!("Unknown command: `{}`", command).red();
                    eprintln!("{}", msg)
                }
            }

            println!()
        } else {
            let file_id = files.add("<repl>", input.to_string());
            let file = files.get(file_id).unwrap();

            let parse_result = helios_parser::parse(
                file_id,
                file.source(),
                messages_tx.clone(),
            );

            println!("{}", parse_result.debug_tree().cyan());
        }

        let mut emitted_ranges = Vec::new();
        for message in messages_rx.try_iter() {
            let diagnostic = Diagnostic::from(message);
            if !(emitted_ranges.contains(&diagnostic.location)) {
                emitted_ranges.push(diagnostic.location.clone());
                helios_diagnostics::emit(&mut stdout, &files, &diagnostic)
                    .expect("Failed to print diagnostic");
            }
        }

        input.clear();
    }

    Ok(())
}

/// Starts a new REPL session.
pub fn start() {
    match start_main_loop() {
        Ok(_) => println!("{}", "Goodbye...".blue()),
        Err(error) => eprintln!("An error occurred: {}", error),
    }
}
