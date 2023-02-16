use std::{io::{Error, Read}, fs::File};

fn run() -> Result<(), Error> {
    let filename = "bytecode-addtion";
    let mut f = File::open(filename)?;
    let mut buffer = String::new();
    f.read_to_string(&mut buffer)?;

    println!("{}", buffer);
    Ok(())
}

fn main() {
    run().unwrap();
}
