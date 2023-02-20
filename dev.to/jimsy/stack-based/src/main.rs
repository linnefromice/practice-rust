use stack_based::{stack::Stack};

fn main() {
    println!("Hello, world!");
    let mut model = Stack::<u32>::new();
    println!("{}", model.is_empty());
    model.push(100);
    println!("{}", model.is_empty());
    println!("{}", model.pop());
    println!("{}", model.is_empty());
}
