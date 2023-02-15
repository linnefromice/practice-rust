use structopt::StructOpt;

mod cli;
mod tasks;

fn main() {
    println!("{:#?}", cli::CommandLineArgs::from_args());
}
