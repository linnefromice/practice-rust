use crate::{write_once_table::WriteOnceTable, table::Table};

use super::instruction_table::InstructionTable;

pub struct Code<T> {
    pub symbols: Vec<(usize, String)>,
    pub code: Vec<usize>,
    pub data: Vec<T>,
    pub labels: Vec<(usize, String)>
}

impl<T> Code<T> {
    pub fn get_label_ip(&self, name: &str) -> Option<usize> {
        for label in self.labels.as_slice() {
            if label.1 == name {
                return Some(label.0);
            }
        }
        None
    }
}

pub struct Builder<'a, T: 'a> {
    pub instruction_table: &'a InstructionTable<T>,
    pub instructions: Vec<usize>,
    pub labels: WriteOnceTable<usize>,
    pub data: Vec<T>,
}

impl<'a, T: PartialEq> Builder<'a, T> {
    pub fn new(instruction_table: &'a InstructionTable<T>) -> Builder<T> {
        let mut labels = WriteOnceTable::new();
        // labels.insert("main", 0);
        Builder {
            instruction_table: &instruction_table,
            instructions: vec![],
            labels,
            data: vec![]
        }
    }

    pub fn from(builder: Builder<T>) -> Code<T> {
        let symbols = builder.instruction_table.symbols();
        let code = builder.instructions;
        let data = builder.data;
        let mut labels = vec![];
        for key in builder.labels.keys() {
            let idx = builder.labels.get(&key).unwrap();
            labels.push((*idx, key.clone()));
        }
        labels.sort_by(|lhs, rhs| {
            lhs.0.cmp(&rhs.0)
        });

        Code {
            symbols,
            code,
            data,
            labels
        }
    }

    pub fn push(&mut self, name: &str, args: Vec<T>) {
        let instr = self
            .instruction_table
            .by_name(name)
            .expect(&format!("Unable to find instruction with name {:?}", name));

        if args.len() != instr.arity {
            panic!("Instruction {} has arity of {}, but you provided {} arguments", instr.name, instr.arity, args.len())
        }

        self.instructions.push(instr.op_code);
        self.instructions.push(instr.arity);

        for arg in args {
            let pos = self.push_data(arg);
            self.instructions.push(pos);
        }
    }

    pub fn label(&mut self, name: &str) {
        let idx = self.instructions.len();
        self.labels.insert(name, idx)
    }

    fn push_data(&mut self, data: T) -> usize {
        let pos = self.data.iter().position(|d| d == &data);
        match pos {
            Some(pos) => pos,
            None => {
                self.data.push(data);
                self.data.len() - 1
            }
        }
    }
}
