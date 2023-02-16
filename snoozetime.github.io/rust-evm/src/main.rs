use std::{io::{Error, Read}, fs::File, num::ParseIntError};

fn decode(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..(s.len()-1))
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i+2], 16))
        .collect()
}

#[derive(Debug)]
enum Opcode {
    STOP, // 0x00
    ADD, // 0x01
    MUL, // 0x02
    PUSH1(u8), // 0x60
    PUSH2(u8, u8), // 0x61
    // PUSH32(u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8) // 0x7f
    EOF,
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

    fn next(&mut self) -> Option<Opcode> {
        if self.pc >= self.code.len() {
            return Some(Opcode::EOF);
        }

        match self.code[self.pc] {
            0x00 => {
                self.pc += 1;
                Some(Opcode::STOP)
            },
            0x01 => {
                self.pc += 1;
                Some(Opcode::ADD)
            },
            0x02 => {
                self.pc += 1;
                Some(Opcode::MUL)
            },
            0x60 => {
                let value = self.code[self.pc+1];
                self.pc += 2;
                Some(Opcode::PUSH1(value))
            },
            0x61 => {
                let value0 = self.code[self.pc+1];
                let value1 = self.code[self.pc+2];
                self.pc += 3;
                Some(Opcode::PUSH2(value0, value1))
            },
            _ => { self.pc += 1; None }
        }
    }
}

fn run() -> Result<(), Error> {
    let filename = "bytecode-addtion";
    let mut vm = Vm::new_from_file(&filename).unwrap();

    loop {
        match vm.next() {
            Some(Opcode::EOF) => break,
            Some(x) => println!("{:?}", x),
            None => {}
        }
    }

    Ok(())
}

fn main() {
    run().unwrap();
}
