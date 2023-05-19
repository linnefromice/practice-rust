use trace::trace;

#[trace]
fn example_func() {
    println!("Hello, world!");
}

fn main() {
    example_func();
}
