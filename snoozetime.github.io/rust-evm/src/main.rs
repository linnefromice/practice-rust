use std::io::Error;

use rust_evm::vm::{Vm, self};

fn run() -> Result<(), Error> {
    let filename = "bytecode-add";
    let mut vm = Vm::new_from_file(&filename).unwrap();

    // vm::debug(&mut vm);
    vm::interpret(&mut vm);

    Ok(())
}

fn main() {
    run().unwrap();
}