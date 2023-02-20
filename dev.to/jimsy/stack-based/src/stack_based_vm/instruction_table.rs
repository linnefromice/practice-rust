use std::collections::HashMap;

use super::instruction::Instruction;

pub struct InstructionTable<T>(HashMap<usize, Instruction<T>>);

impl<T> InstructionTable<T> {
    pub fn new() -> InstructionTable<T> {
        InstructionTable(HashMap::new())
    }

    pub fn by_op_code(&self, op_code: usize) -> Option<&Instruction<T>> {
        self.0.get(&op_code)
    }

    pub fn by_name(&self, name: &str) -> Option<&Instruction<T>> {
        self.0
            .values()
            .find(|ref instr| instr.name == name) // FIXME: duplicated name
    }

    pub fn insert(&mut self, instr: Instruction<T>) {
        self.0.insert(instr.op_code, instr);
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use super::super::machine::Machine;

    fn noop(_machine: &mut Machine<usize>, _args: &[usize]) {}

    #[test]
    fn new() {
        let table: InstructionTable<usize> = InstructionTable::new();
        assert!(table.is_empty())
    }

    #[test]
    fn insert() {
        let mut table: InstructionTable<usize> = InstructionTable::new();
        assert!(table.is_empty());
        table.insert(Instruction::new(0, "NOOP", 0, noop));
        assert!(!table.is_empty());
    }

    #[test]
    fn by_op_code() {
        let mut table: InstructionTable<usize> = InstructionTable::new();
        table.insert(Instruction::new(0, "NOOP", 0, noop));
        let instr = table.by_op_code(0).unwrap();
        assert_eq!(instr.name, "NOOP");
    }

    #[test]
    fn by_name() {
        let mut table: InstructionTable<usize> = InstructionTable::new();
        table.insert(Instruction::new(0, "NOOP", 0, noop));
        let instr = table.by_name("NOOP").unwrap();
        assert_eq!(instr.op_code, 0);
    }
}
