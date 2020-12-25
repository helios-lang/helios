//! REPL support for the Helios programming language.

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

fn start_main_loop() -> io::Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut input = String::new();

    let (messages_tx, messages_rx) = flume::unbounded();

    for (i, line) in LOGO_BANNER.iter().enumerate() {
        match i {
            2 => println!("{}Version {}", line, env!("CARGO_PKG_VERSION")),
            3 => println!("{}{}", line, env!("CARGO_PKG_REPOSITORY")),
            4 => println!("{}Type #exit to exit, #help for help", line),
            _ => println!("{}", line.trim_end()),
        }
    }

    loop {
        write!(stdout, "> ")?;
        stdout.flush()?;

        stdin.read_line(&mut input)?;

        if input.trim().is_empty() {
            continue;
        }

        if input.trim().starts_with("#") {
            let input = input[1..].trim();
            match input {
                "exit" => break,
                "help" => println!("Help is not available at the moment"),
                command => eprintln!("! Unknown command `{}`", command),
            }
        } else {
            let parse_result =
                helios_parser::parse(&input, messages_tx.clone());
            println!("{}", parse_result.debug_tree());
        }

        for message in messages_rx.try_iter() {
            eprintln!("{}", Diagnostic::from(message));
        }

        println!();
        input.clear();
    }

    Ok(())
}

/// Starts a new REPL session.
pub fn start() {
    match start_main_loop() {
        Ok(_) => println!("Goodbye"),
        Err(error) => eprintln!("An error occurred: {}", error),
    }
}
