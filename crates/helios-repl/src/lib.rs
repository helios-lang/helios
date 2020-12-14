//! REPL support for the Helios programming language.

use std::io::{self, Write};

fn start_main_loop() -> io::Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut input = String::new();

    println!("Welcome to the REPL. Type :exit to quit.\n");

    loop {
        write!(stdout, "> ")?;
        stdout.flush()?;

        stdin.read_line(&mut input)?;

        if input.trim() == ":exit" {
            break;
        }

        let parse_result = helios_parser::parse(&input);
        println!("{}", parse_result.debug_tree());

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
