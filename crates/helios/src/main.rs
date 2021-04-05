use clap::Clap;
use helios::build::HeliosBuildOpts;
use helios::repl::HeliosReplOpts;

#[derive(Clap)]
#[clap(version = "0.2.0")]
#[allow(unused)]
struct HeliosOpts {
    /// Enables quiet mode (no output to stdout)
    #[clap(short = 'q', long = "quiet")]
    quiet: bool,
    /// The verbosity of the output to stdout
    #[clap(short = 'v', long = "verbose")]
    verbose: bool,
    /// Recognized subcommands
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

    match opts.subcommand {
        HeliosSubcommand::Build(build_opts) => {
            log::trace!("Starting build process...");
            helios::build::build(&*build_opts.file);
        }
        HeliosSubcommand::Repl(_repl_opts) => {
            log::trace!("Starting new REPL session...");
            helios::repl::start();
        }
    }
}
