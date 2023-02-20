pub struct Instruction<T> {
    pub op_code: usize,
    pub name: String,
    pub arity: usize,
    pub fun: InstructionFn<T>
}

pub type Machine<T> = T; // temp: machine
pub type InstructionFn<T> = fn(machine: &mut Machine<T>, args: &[usize]);

impl<T> Instruction<T> {
    pub fn new(op_code: usize, name: &str, arity: usize, fun: InstructionFn<T>) -> Instruction<T> {
        Instruction {
            op_code,
            name: String::from(name),
            arity,
            fun
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    struct Operand(i64);

    fn noop(_machine: &mut Machine<Operand>,_args: &[usize]) {}

    #[test]
    fn new() {
        let operand = Instruction::new(13, "noop", 7, noop);
        assert_eq!(operand.op_code, 13);
        assert_eq!(operand.name, "noop".to_string());
        assert_eq!(operand.arity, 7);
    }
}