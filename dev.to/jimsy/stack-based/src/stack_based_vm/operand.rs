use super::{machine::Machine, instruction_table::InstructionTable, instruction::Instruction};

#[derive(Clone, PartialEq)]
enum Operand {
    I(i64),
    S(String),
}

impl Operand {
    fn to_i(&self) -> Option<i64> {
        match self {
            &Operand::I(i) => Some(i),
            _ => None,
        }
    }

    fn to_s(&self) -> Option<&str> {
        match self {
            &Operand::S(ref s) => Some(s),
            _ => None,
        }
    }
}

impl From<i64> for Operand {
    fn from(value: i64) -> Self {
        Operand::I(value)
    }
}

impl From<&str> for Operand {
    fn from(value: &str) -> Self {
        Operand::S(value.to_string())
    }
}

fn push(machine: &mut Machine<Operand>, args: &[usize]) {
    let arg = machine.get_data(args[0]).clone();
    machine.operand_push(arg)
}

fn add(machine: &mut Machine<Operand>, args: &[usize]) {
    let rhs = machine.operand_pop().to_i().unwrap();
    let lhs = machine.operand_pop().to_i().unwrap();
    machine.operand_push(Operand::I(lhs + rhs));
}

fn call(machine: &mut Machine<Operand>, args: &[usize]) {
    let label = machine.get_data(args[0]).clone();
    machine.call(label.to_s().unwrap());
}

fn ret(machine: &mut Machine<Operand>, _args: &[usize]) {
    machine.ret();
}

fn instruction_table() -> InstructionTable<Operand> {
    let mut it = InstructionTable::new();
    it.insert(Instruction::new(0, "push", 1, push));
    it.insert(Instruction::new(1, "add", 0, add));
    it.insert(Instruction::new(2, "call", 1, call));
    it.insert(Instruction::new(3, "ret", 0, ret));
    it
}

#[cfg(test)]
mod test {
    use crate::stack_based_vm::{code::{Builder, Code}, machine::Machine};

    use super::{instruction_table, Operand};

    #[test]
    fn example() {
        let it = instruction_table();
        let mut builder = Builder::new(&it);
        builder.push("push", vec![Operand::from(3)]);
        builder.push("push", vec![Operand::from(4)]);
        builder.push("call", vec![Operand::from("add_fun")]);
        builder.push("ret", vec![]);
        builder.label("add_fun");
        builder.push("add", vec![]);
        builder.push("ret", vec![]);

        let mut machine: Machine<Operand> = Machine::new(Builder::from(builder), &it);
        machine.run();
        let result = machine.operand_pop().to_i().unwrap();
        assert_eq!(result, 7);
    }
}