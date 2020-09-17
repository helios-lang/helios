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
    let mut args = std::env::args();
    args.next(); // Skip path to executable

    match (args.next(), args.next()) {
        (Some(arg), param) => match (&*arg, param) {
            ("-h", _) | ("--help", _) => print_usage(),
            ("-V", _) | ("--version", _) => print_version(),
            ("build", None) => {
                eprintln!("ERROR: Missing argument for command `build`.\n");
                print_usage();
            }
            ("build", Some(ref file_name)) => koi_build::build(file_name),
            ("ide", _) => koi_ide::run(),
            ("repl", _) => koi_repl::start(),
            _ => print_usage()
        }
        _ => print_usage()
    }
}
