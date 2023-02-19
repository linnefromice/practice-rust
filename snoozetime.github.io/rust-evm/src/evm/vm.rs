use std::{num::ParseIntError, io::{Error, Read, self}, fs::File};
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
    pub stack: Vec<U256>,
    at_end: bool
}

impl Vm {
    pub fn new(binary: Vec<u8>) -> Vm {
        Vm { code: binary, pc: 0, stack: Vec::new(), at_end: false }
    }

    pub fn new_from_file(filename: &str) -> Result<Vm, Error> {
        let mut f = File::open(filename)?;
        let mut buffer = String::new();
        f.read_to_string(&mut buffer)?;
        let code = decode(&buffer).unwrap();
        Ok(Vm::new(code))
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
            0x12 => {
                self.pc += 1;
                Some(Opcode::SLT(addr))
            },
            0x56 => {
                self.pc += 1;
                Some(Opcode::JUMP(addr))
            },
            0x57 => {
                self.pc += 1;
                Some(Opcode::JUMPI(addr))
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
            0xbb => {
                self.pc += 1;
                Some(Opcode::PRINT(addr))
            },
            _ => { self.pc += 1; None }
        }
    }

    pub fn interpret(&mut self) {
        let maybe_op = self.next();

        // for debug
        match &maybe_op {
            Some(x) => x.describe(),
            None => {}
        }

        match &maybe_op {
            Some(x) => {
                match x {
                    Opcode::STOP(_) => {
                        self.at_end = true;
                    },
                    Opcode::PUSH1(_, value) => {
                        self.stack.push(U256::from(*value));
                    },
                    Opcode::ADD(_) => {
                        let v1 = self.stack.pop().unwrap();
                        let v2 = self.stack.pop().unwrap();
                        self.stack.push(v1 + v2);
                    },
                    Opcode::SLT(_) => {
                        let lhs = self.stack.pop().unwrap();
                        let rhs = self.stack.pop().unwrap();
                        if lhs < rhs {
                            self.stack.push(U256::from(0x01));
                        } else {
                            self.stack.push(U256::from(0x00));
                        }
                    },
                    Opcode::JUMP(_) => {
                        let jump_location = self.stack.pop().unwrap();
                        self.pc = jump_location.as_u64() as usize;
                    }
                    Opcode::JUMPI(_) => {
                        let then_addr = self.stack.pop().unwrap();
                        let cond = self.stack.pop().unwrap();
                        if !cond.is_zero() {
                            self.pc = then_addr.as_u64() as usize;
                        }
                    },
                    Opcode::PRINT(_addr) => {
                        let v = self.stack.pop().unwrap();
                        let mut bytes = vec![0;32];
                        v.to_big_endian(&mut bytes);
                        println!("PRINT\t{:?}|", bytes)
                    },
                    Opcode::EOF => {
                        self.at_end = true;
                    }
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

    fn print_debug(&self) {
        println!("pc:{}\n", self.pc);
        println!("Stack:");
        self.print_stack();
    }
}

pub fn debug(vm: &mut Vm) {
    loop {
        if vm.at_end {
            break;
        }

        let mut input = String::new();
        io::stdin().read_line(&mut input).ok().expect("Couldn't read line");

        match input.as_str() {
            "c\n" => vm.interpret(),
            "s\n" => vm.print_debug(),
            "q\n" => break,
            _ => println!("Please type either c, s or q"),
        }
    }
}

pub fn interpret(vm: &mut Vm) {
    while !vm.at_end {
        vm.interpret();
    }
    vm.print_stack();
}