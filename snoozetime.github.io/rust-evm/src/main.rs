use std::{io::{Error, Read}, fs::File, num::ParseIntError};

use ethereum_types::U256;

fn decode(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..(s.len()-1))
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i+2], 16))
        .collect()
}

#[derive(Debug)]
enum Opcode {
    STOP(usize), // 0x00
    ADD(usize), // 0x01
    MUL(usize), // 0x02
    PUSH1(usize, u8), // 0x60
    PUSH2(usize, u8, u8), // 0x61
    // PUSH32(usize, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8) // 0x7f
    EOF,
}

impl Opcode {
    fn describe(&self) {
        match self {
            Opcode::STOP(line) => println!("0x{:x}\tADD\tHalts operation", line),
            Opcode::ADD(line) => println!("0x{:x}\tADD\tAddition operation", line),
            Opcode::MUL(line) => println!("0x{:x}\tMUL\tMultiplication operation", line),
            Opcode::PUSH1(line, x) => println!("0x{:x}\tPUSH1\tPlace 1-byte item on the stack 0x{:x}", line, x),
            Opcode::PUSH2(line, x0, x1) => println!("0x{:x}\tPUSH2\tPlace 2-bytes item on the stack 0x{:x} 0x{:x}", line, x0, x1),
            _ => println!("Unknown opcode")
        }
    }
}

struct Vm {
    code: Vec<u8>, // smart contract code
    pc: usize,
    stack: Vec<U256>,
}

impl Vm {
    fn new_from_file(filename: &str) -> Result<Vm, Error> {
        let mut f = File::open(filename)?;
        let mut buffer = String::new();
        f.read_to_string(&mut buffer)?;
        let code = decode(&buffer).unwrap();
        Ok(Vm { code, pc: 0, stack: Vec::new() })
    }

    fn next(&mut self) -> Option<Opcode> {
        if self.pc >= self.code.len() {
            return Some(Opcode::EOF);
        }

        let addr = self.pc;
        match self.code[addr] {
            0x00 => {
                self.pc += 1;
                Some(Opcode::STOP(addr))
            },
            0x01 => {
                self.pc += 1;
                Some(Opcode::ADD(addr))
            },
            0x02 => {
                self.pc += 1;
                Some(Opcode::MUL(addr))
            },
            0x60 => {
                let value = self.code[self.pc+1];
                self.pc += 2;
                Some(Opcode::PUSH1(addr, value))
            },
            0x61 => {
                let value0 = self.code[self.pc+1];
                let value1 = self.code[self.pc+2];
                self.pc += 3;
                Some(Opcode::PUSH2(addr, value0, value1))
            },
            _ => { self.pc += 1; None }
        }
    }

    fn interpret(&mut self) {
        let maybe_op = self.next();

        match &maybe_op {
            Some(x) => x.describe(),
            None => {}
        }

        match &maybe_op {
            Some(x) => {
                match x {
                    Opcode::PUSH1(_, value) => {
                        self.stack.push(U256::from(*value));
                    },
                    Opcode::ADD(_) => {
                        let v1 = self.stack.pop().unwrap();
                        let v2 = self.stack.pop().unwrap();
                        self.stack.push(v1 + v2);
                    },
                    _ => {}
                }
            },
            None => {}
        }
    }

    fn print_stack(&self) {
        self.stack
            .iter()
            .enumerate()
            .rev()
            .for_each(|(i, x)| {
                let mut bytes = vec![0;32];
                x.to_big_endian(&mut bytes);
                println!("|{}:\t{:?}|", i, bytes)
            })
    }
}

fn run() -> Result<(), Error> {
    let filename = "bytecode-add";
    let mut vm = Vm::new_from_file(&filename).unwrap();

    // -> fn debug
    // loop {
    //     match vm.next() {
    //         Some(Opcode::EOF) => break,
    //         Some(x) => x.describe(),
    //         None => {}
    //     }
    // }

    // -> fn interpret
    loop {
        vm.interpret();
        if vm.pc >= vm.code.len() {
            break;
        }
    }
    vm.print_stack();

    Ok(())
}

fn main() {
    run().unwrap();
}
