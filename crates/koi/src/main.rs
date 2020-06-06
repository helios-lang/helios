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
    let mut quiet_mode = false;
    let mut args = std::env::args();

    match (args.next(), args.next(), args.next()) {
        (_, Some(ref opt), _) if opt == "-h" || opt == "--help" => print_usage(),
        (_, Some(ref opt), _) if opt == "-q" || opt == "--quiet" => quiet_mode = true,
        (_, Some(ref opt), _) if opt == "-V" || opt == "--version" => print_version(),
        _ => print_usage()
    }
}
