use structopt::StructOpt;

mod cli;

fn main() {
    cli::CommandLineArgs::from_args();
}
