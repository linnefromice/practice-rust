use std::{num::ParseIntError, io::{Error, Read}, fs::File};
use ethereum_types::U256;

use crate::opcode::Opcode;

fn decode(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..(s.len()-1))
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i+2], 16))
        .collect()
}

pub struct Vm {
    code: Vec<u8>, // smart contract code
    pc: usize,
    stack: Vec<U256>,
}

impl Vm {
    pub fn new_from_file(filename: &str) -> Result<Vm, Error> {
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

pub fn debug(vm: &mut Vm) {
    loop {
        match vm.next() {
            Some(Opcode::EOF) => break,
            Some(x) => x.describe(),
            None => {}
        }
    }
}

pub fn interpret(vm: &mut Vm) {
    loop {
        vm.interpret();
        if vm.pc >= vm.code.len() {
            break;
        }
    }
    vm.print_stack();
}