use clap::Parser;

use helios::build::HeliosBuildOpts;
use helios::repl::HeliosReplOpts;

#[derive(Parser)]
#[clap(version = "0.2.0")]
struct HeliosOpts {
    /// Enables quiet mode (no output to stdout)
    #[clap(short, long)]
    quiet: bool,
    /// The verbosity of the output to stdout
    #[clap(short, long)]
    verbose: bool,
    /// Recognized subcommands
    #[clap(subcommand)]
    subcommand: HeliosSubcommand,
}

#[derive(Parser)]
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
            helios::build::build(&build_opts.file);
        }
        HeliosSubcommand::Repl(_repl_opts) => {
            log::trace!("Starting new REPL session...");
            helios::repl::start();
        }
    }
}
