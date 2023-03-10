use super::super::{
    machine::Machine,
    instruction_table::InstructionTable,
    instruction::Instruction,
    code:: Builder,
};

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
    fn from(i: i64) -> Self {
        Operand::I(i)
    }
}

impl From<&str> for Operand {
    fn from(s: &str) -> Self {
        Operand::S(s.to_string())
    }
}

fn push(machine: &mut Machine<Operand>, args: &[usize]) {
    let arg = machine.get_data(args[0]).clone();
    machine.operand_push(arg);
}

fn jump(machine: &mut Machine<Operand>, args: &[usize]) {
    let label = machine.get_data(args[0]).clone();
    machine.jump(label.to_s().unwrap());
}

fn jump_if(machine: &mut Machine<Operand>, args: &[usize]) {
    let condition = machine.operand_pop().to_i().unwrap();
    if condition != 0 {
        let label = machine.get_data(args[0]).clone();
        machine.jump(label.to_s().unwrap());
    }
}

fn instruction_table() -> InstructionTable<Operand> {
    let mut it = InstructionTable::new();
    it.insert(Instruction::new(0, "push", 1, push));
    it.insert(Instruction::new(1, "jump_if", 1, jump_if));
    it.insert(Instruction::new(2, "jump", 1, jump));
    it
}

fn conditional_program(condition: Operand) -> Operand {
    let it = instruction_table();
    let mut builder = Builder::new(&it);
    builder.push("push", vec![condition]);
    builder.push("jump_if", vec![Operand::from("if_true")]);
    builder.push("push", vec![Operand::from("it was false")]);
    builder.push("jump", vec![Operand::from("end")]);
    builder.label("if_true");
    builder.push("push", vec![Operand::from("it was true")]);
    builder.label("end");
    let code = Builder::from(builder);
    let mut machine: Machine<Operand> = Machine::new(code, &it);
    machine.run();
    machine.operand_pop()
}

#[test]
fn condition_true() {
    let result = conditional_program(Operand::from(1));
    assert_eq!(result.to_s().unwrap(), "it was true")
}

#[test]
fn condition_false() {
    let result = conditional_program(Operand::from(0));
    assert_eq!(result.to_s().unwrap(), "it was false")
}