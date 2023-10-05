use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    operation: Operation
}

#[derive(Subcommand)]
enum Operation {
    Do,
    Did,
    Doing,
    Done,
}

fn main() {
    let cli = Cli::parse();

    println!("Hello, world!");
    match &cli.operation {
        Operation::Do => println!("You do it!"),
        Operation::Did => println!("You did it."),
        Operation::Doing => println!("You doing it now!"),
        Operation::Done => println!("You have done it..."),
    }
}
