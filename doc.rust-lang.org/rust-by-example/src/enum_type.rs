use crate::enum_type::List::*;

enum List {
    Cons(u32, Box<List>),
    Nil,
}

impl List {
    fn new() -> List {
        Nil
    }

    fn prepend(self, elem: u32) -> List {
        Cons(elem, Box::new(self))
    }

    fn len(&self) -> u32 {
        match *self {
            Cons(_, ref tail) => 1 + tail.len(),
            Nil => 0,
        }
    }

    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                format!("{}, {}", head, tail.stringify())
            },
            Nil => {
                format!("Nil")
            },
        }
    }
}

pub fn execute_enum_type() -> () {
    let list = List::new();
    let list = list.prepend(1);
    let list = list.prepend(2);
    let list = list.prepend(3);

    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());
}