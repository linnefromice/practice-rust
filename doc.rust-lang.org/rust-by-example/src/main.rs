#[derive(Debug)]
struct MinMax(i64, i64);

fn main_display() {
    let minmax = MinMax(0, 14);

    println!("Compare structures");
    println!("Debug: {:?}", minmax);
}

fn main() {
    main_display();
}
