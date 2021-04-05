use clap::Clap;
use helios::build::HeliosBuildOpts;
use helios::repl::HeliosReplOpts;

#[derive(Clap)]
#[clap(version = "0.2.0")]
struct HeliosOpts {
    /// Enables quiet mode (no output to stdout)
    #[clap(short = 'q', long = "quiet")]
    quiet: bool,
    /// Prints diagnostic output to stdout
    #[clap(short = 'v', long = "verbose")]
    verbose: bool,
    #[clap(subcommand)]
    subcommand: HeliosSubcommand,
}

#[derive(Clap)]
enum HeliosSubcommand {
    Build(HeliosBuildOpts),
    Repl(HeliosReplOpts),
}

fn main() {
    env_logger::init();

    let opts = HeliosOpts::parse();
    println!("quiet: {}", opts.quiet);
    println!("verbose: {}", opts.verbose);

    match opts.subcommand {
        HeliosSubcommand::Build(build_opts) => {
            log::trace!("Starting build process...");
            helios::build::build(&*build_opts.file);
        }
        HeliosSubcommand::Repl(_repl_opts) => {
            log::trace!("Starting REPL...");
            helios::repl::start();
        }
    }
}
