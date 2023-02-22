use crate::stack::Stack;
use super::{
    instruction_table::{InstructionTable, self},
    code::Code
};

pub struct Machine<'a, T> {
    code: Code<T>,
    instruction_table: &'a InstructionTable<T>,
    ip: usize,
    operand_stack: Stack<T>
}

impl<'a, T> Machine<'a, T> {
    pub fn new(code: Code<T>, instruction_table: &'a InstructionTable<T>) -> Machine<'a, T> {
        Machine {
            code,
            instruction_table,
            ip: 0,
            operand_stack: Stack::new()
        }
    }

    pub fn run(&mut self) {
        loop {
            if self.ip == self.code.code.len() {
                break;
            }

            let op_code = self.next_code();
            let arity = self.next_code();

            let instr = self
                .instruction_table
                .by_op_code(op_code)
                .unwrap_or_else(|| panic!("Unable to find instruction with op code {}", op_code));

            let mut args: Vec<usize> = vec![];

            for _i in 0..arity {
                args.push(self.next_code());
            }

            let fun = instr.fun;
            fun(self, args.as_slice());
        }
    }

    #[inline]
    fn next_code(&mut self) -> usize {
        let code = self.code.code[self.ip];
        self.ip += 1;
        code
    }

    pub fn operand_push(&mut self, value: T) {
        self.operand_stack.push(value);
    }

    pub fn operand_pop(&mut self) -> T {
        self.operand_stack.pop()
    }

    pub fn get_data(&self, idx: usize) -> &T {
        self.code
            .data
            .get(idx)
            .expect(&format!("Constant data is not present at index {}.", idx))
    }
}