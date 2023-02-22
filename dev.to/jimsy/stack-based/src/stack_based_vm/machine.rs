use crate::stack::Stack;
use super::{
    instruction_table::{InstructionTable, self},
    code::Code, frame::Frame
};

pub struct Machine<'a, T> {
    code: Code<T>,
    instruction_table: &'a InstructionTable<T>,
    ip: usize,
    operand_stack: Stack<T>,
    call_stack: Stack<Frame>
}

impl<'a, T> Machine<'a, T> {
    pub fn new(code: Code<T>, instruction_table: &'a InstructionTable<T>) -> Machine<'a, T> {
        let frame = Frame::new(code.code.len());
        let mut call_stack = Stack::new();
        call_stack.push(frame);

        Machine {
            code,
            instruction_table,
            ip: 0,
            operand_stack: Stack::new(),
            call_stack,
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

    pub fn jump(&mut self, label: &str) {
        self.ip = self
            .code
            .get_label_ip(label)
            .expect(&format!("Attempted to jump to unknown label {}", label));
    }

    pub fn call(&mut self, label: &str) {
        self.call_stack.push(Frame::new(self.ip));
        self.jump(label);
    }

    pub fn ret(&mut self) {
        let frame = self.call_stack.pop();
        self.ip = frame.return_address;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{stack_based_vm::{instruction_table::InstructionTable, instruction::Instruction, code::Builder}, write_many_table::WriteManyTable};

    fn push(machine: &mut Machine<usize>, args: &[usize]) {
        let args = &machine.code.data[args[0]];
        machine.operand_stack.push(*args);
    }

    fn add(machine: &mut Machine<usize>, _args: &[usize]) {
        let rhs = machine.operand_pop();
        let lhs = machine.operand_pop();
        machine.operand_stack.push(lhs + rhs);
    }

    fn instruction_table() -> InstructionTable<usize> {
        let mut it = InstructionTable::new();
        it.insert(Instruction::new(1, "push", 1, push));
        it.insert(Instruction::new(2, "add", 0, add));
        it
    }

    #[test]
    fn new() {
        let it = instruction_table();
        let builder: Builder<usize> = Builder::new(&it);
        let machine = Machine::new(Builder::from(builder), &it);
        assert_eq!(machine.ip, 0);
        assert!(machine.operand_stack.is_empty());
    }

    #[test]
    fn addition_example() {
        let it = instruction_table();
        let mut builder = Builder::new(&it);
        builder.push("push", vec![2]);
        builder.push("push", vec![3]);
        builder.push("add", vec![]);
        let code = Builder::from(builder);
        let mut machine = Machine::new(code, &it);
        machine.run();
        let result = machine.operand_pop();
        assert_eq!(result, 5);
    }
}