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

    match (args.next(), args.next(), args.next()) {
        (_, Some(ref opt), _) if opt == "-h" || opt == "--help" =>
            print_usage(),
        (_, Some(ref opt), _) if opt == "-V" || opt == "--version" =>
            print_version(),
        (_, Some(ref cmd), Some(ref file_name)) if cmd == "build" =>
            koi_build::build(file_name),
        (_, Some(ref cmd), None) if cmd == "build" => {
            eprintln!("ERROR: Missing argument for command `build`.\n");
            print_usage();
        },
        (_, Some(ref cmd), _) if cmd == "ide" => koi_ide::run(),
        (_, Some(ref cmd), _) if cmd == "repl" => koi_repl::start(),
        _ => print_usage()
    }
}
