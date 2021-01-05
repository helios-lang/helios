/// Prints the usage information of the Helios-LS executable.
fn print_usage() {
    println!("{}", include_str!("../usage.txt"));
}

/// Prints the current version number of the Helios-LS executable.
///
/// This function will print the version number found in the `Cargo.toml`
/// file of this package.
fn print_version() {
    // TODO: We could ignore the hash if we fail to retrieve it
    fn get_env_variables<'a>() -> Option<(&'a str, &'a str)> {
        let version = option_env!("CARGO_PKG_VERSION")?;
        let hash = option_env!("GIT_HASH")?;
        Some((version, hash))
    }

    match get_env_variables() {
        Some((version, hash)) => println!("helios-ls {} ({})", version, hash),
        None => eprintln!("ERROR: Failed to get version."),
    }
}

fn main() {
    env_logger::init();
    let mut args = std::env::args();
    args.next(); // Skip path to executable

    match args.next() {
        Some(arg) => match &*arg {
            "-h" | "--help" => print_usage(),
            "-V" | "--version" => print_version(),
            _ => {
                eprintln!("ERROR: Unrecognised option `{}`", arg);
                print_usage()
            }
        },
        _ => {
            log::trace!("Starting Helios-LS...");
            helios_ls::start()
        }
    }
}
