//! REPL support for the Helios programming language.

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

    let (diagnostics_tx, _diagnostics_rx) = flume::unbounded();

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
                helios_parser::parse(&input, diagnostics_tx.clone());
            println!("{}", parse_result.debug_tree());
        }

        // for diagnostic in diagnostics_rx.try_iter() {
        //     println!("{:?}: {}", diagnostic.severity, diagnostic.title);
        //     for detail in diagnostic.details {
        //         println!("  {:?}: {}", detail.range, detail.message);
        //     }
        // }

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
