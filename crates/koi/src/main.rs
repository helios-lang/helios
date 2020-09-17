fn print_usage() {
    println!("{}", include_str!("../usage.txt"));
}

fn print_version() {
    if let Some(version) = option_env!("CARGO_PKG_VERSION") {
        println!("koi {}", version);
    } else {
        eprintln!("Failed to get version.");
    }
}

fn main() {
    // Initialize the logger
    env_logger::init();
    log::info!("Welcome to the koi compiler");

    let mut args = std::env::args();
    args.next(); // Skip path to executable

    match (args.next(), args.next()) {
        (Some(arg), param) => match (&*arg, param) {
            ("-h", _) | ("--help", _) => print_usage(),
            ("-V", _) | ("--version", _) => print_version(),
            ("build", None) => {
                let message = "Missing argument for command `build`";
                log::error!("{}", message);
                eprintln!("ERROR: {}.\n", message);
                print_usage();
            }
            ("build", Some(ref file_name)) => {
                log::trace!("Starting build process...");
                koi_build::build(file_name)
            },
            ("ide", _) => {
                log::trace!("Starting language server...");
                koi_ide_new::start()
            },
            ("repl", _) => {
                log::trace!("Starting REPL...");
                koi_repl::start()
            },
            _ => print_usage()
        }
        _ => print_usage()
    }
}
