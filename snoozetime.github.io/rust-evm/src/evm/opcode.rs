#[derive(Debug)]
pub enum Opcode {
    STOP(usize), // 0x00
    ADD(usize), // 0x01
    MUL(usize), // 0x02
    SLT(usize), // 0x12
    JUMP(usize), // 0x56
    JUMPI(usize), // 0x57
    PUSH1(usize, u8), // 0x60
    PUSH2(usize, u8, u8), // 0x61
    // PUSH32(usize, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8) // 0x7f
    PRINT(usize), // 0xbb
    EOF,
}

impl Opcode {
    pub fn describe(&self) {
        match self {
            Opcode::STOP(line) => println!("0x{:x}\tSTOP\tHalts operation", line),
            Opcode::ADD(line) => println!("0x{:x}\tADD\tAddition operation", line),
            Opcode::MUL(line) => println!("0x{:x}\tMUL\tMultiplication operation", line),
            Opcode::SLT(line) => println!("0x{:x}\tSLT\tint256 less-than operation", line),
            Opcode::JUMP(line) => println!("0x{:x}\tJUMP\t$pc := dst mark that pc is only assigned if dst is a valid jumpdest", line),
            Opcode::JUMPI(line) => println!("0x{:x}\tJUMPI\t$pc := condition ? dst : $pc + 1", line),
            Opcode::PUSH1(line, x) => println!("0x{:x}\tPUSH1\tPlace 1-byte item on the stack 0x{:x}", line, x),
            Opcode::PUSH2(line, x0, x1) => println!("0x{:x}\tPUSH2\tPlace 2-bytes item on the stack 0x{:x} 0x{:x}", line, x0, x1),
            Opcode::PRINT(line) => println!("0x{:x}\tPRINT\tspecial instruction", line),
            _ => println!("Unknown opcode")
        }
    }
}
