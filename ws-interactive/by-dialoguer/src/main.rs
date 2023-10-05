use clap::{Parser, Subcommand};
use dialoguer::{Confirm, FuzzySelect, theme::ColorfulTheme, MultiSelect};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    operation: Operation
}

#[derive(Subcommand)]
enum Operation {
    Confirm,
    MultiSelect,
}

fn main() {
    let cli = Cli::parse();

    println!("Hello, world!");
    match &cli.operation {
        Operation::Confirm => confirm(),
        Operation::MultiSelect => multi_select()
    }
}

fn confirm() {
    if Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Do you want to continue?")
        .wait_for_newline(true)
        .interact()
        .unwrap()
    {
        println!("Confirmed");
    } else {
        println!("Not confirmed");
    }

    if Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Do you really really want to continue?")
        .default(true)
        .show_default(false)
        .wait_for_newline(true)
        .interact()
        .unwrap()
    {
        println!("Looks like you want to continue");
    } else {
        println!("nevermind then :(");
    }

    match Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt(r#"Do you really really really really really want to continue?"#)
        .default(true)
        .wait_for_newline(true)
        .interact_opt()
        .unwrap()
    {
        Some(true) => println!("Looks like you want to continue"),
        Some(false) => println!("nevermind then :("),
        None => println!("Ok, we can start over later"),
    }
}

fn multi_select() {
    let items = &[
        "Play Station",
        "Nintendo Switch",
        "Xbox",
        "Steam",
        "Social Game"
    ];

    let selections = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Pick your favorite game console")
        .items(items)
        .interact()
        .unwrap();
    if selections.is_empty() {
        println!("You did not select anything :(");
    } else {
        println!("You selected these things:");
        for selection in selections {
            println!("  {}", items[selection]);
        }
    }

    let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Pick your favorite game console")
        .default(0)
        .items(items)
        .interact()
        .unwrap();
    println!("You picked: {}", items[selection])
}
