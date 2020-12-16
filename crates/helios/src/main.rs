/// Prints a formatted error message to standard error.
fn print_error(message: impl Into<String>) {
    eprintln!("ERROR: {}\n", message.into())
}

/// Prints the usage information of the Helios executable.
fn print_usage() {
    println!("{}", include_str!("../usage.txt"));
}

/// Prints the current version number of the Helios executable.
///
/// This function will print the version number found in the `Cargo.toml`
/// file of this package.
fn print_version() {
    if let Some(version) = option_env!("CARGO_PKG_VERSION") {
        println!("helios {}", version);
    } else {
        eprintln!("ERROR: Failed to get version.");
    }
}

fn main() {
    // Initialize the logger
    env_logger::init();

    let mut args = std::env::args();
    args.next(); // Skip path to executable

    match (args.next(), args.next()) {
        (Some(arg), param) => match (&*arg, param) {
            ("-h", _) | ("--help", _) => print_usage(),
            ("-V", _) | ("--version", _) => print_version(),
            ("build", None) => {
                print_error("Missing argument for subcommand `build`");
                print_usage();
            }
            ("build", Some(ref file_name)) => {
                log::trace!("Starting build process...");
                helios_build::build(file_name)
            }
            ("ide", _) => {
                log::trace!("Starting language server...");
                helios_ide::start()
            }
            ("repl", _) => {
                log::trace!("Starting REPL...");
                helios_repl::start()
            }
            _ => {
                let message =
                    format!("Unrecognised option or subcommand `{}`", arg);
                print_error(message);
                print_usage()
            }
        },
        _ => print_usage(),
    }
}
