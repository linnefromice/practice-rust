use std::fmt;

#[derive(Debug)]
struct MinMax(i64, i64);

impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} {})", self.0, self.1)
    }
}

fn main_display() {
    let minmax = MinMax(0, 14);

    println!("Compare structures");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);
}

fn main() {
    main_display();
}
