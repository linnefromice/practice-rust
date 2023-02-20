use std::{io::Error, env};

use rust_evm::vm::{Vm, self};

fn run() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let function = args[1].clone();
    let filename = if args.len() == 3 && !args[2].is_empty() { args[2].clone() } else { String::from("bytecode-add") };
    println!("In file {}", filename);
    
    let mut vm = Vm::new_from_file(&filename).unwrap();
    println!("Correctly loaded VM");

    match function.as_str() {
        "debug" => vm::debug(&mut vm),
        "run" => vm::interpret(&mut vm),
        _ => panic!("Expect either 'debug' or 'run' for first parameter")
    }

    Ok(())
}

fn main() {
    run().unwrap();
}