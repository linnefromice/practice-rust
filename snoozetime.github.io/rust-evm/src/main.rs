use std::{io::{Error, Read}, fs::File, num::ParseIntError};

fn decode(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..(s.len()-1))
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i+2], 16))
        .collect()
}

struct Vm {
    code: Vec<u8>, // smart contract code
    pc: usize,
}

impl Vm {
    fn new_from_file(filename: &str) -> Result<Vm, Error> {
        let mut f = File::open(filename)?;
        let mut buffer = String::new();
        f.read_to_string(&mut buffer)?;
        let code = decode(&buffer).unwrap();
        Ok(Vm { code, pc: 0 })
    }
}


fn run() -> Result<(), Error> {
    let filename = "bytecode-addtion";
    let vm = Vm::new_from_file(&filename).unwrap();

    for b in &vm.code {
        println!("0x{:x}", b);
    }

    Ok(())
}

fn main() {
    run().unwrap();
}
