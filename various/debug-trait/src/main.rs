use std::vec;

#[derive(Debug, PartialEq, Clone)]
struct Stats {
    player: String,
    goal: u8,
    assist: u8,
    is_starting: bool,
}

fn main() {
    let val = vec![1, 2, 3, 4, 5];
    // println!("{}", val); // `Vec<{integer}>` doesn't implement `std::fmt::Display`
    println!("{:?}", val);

    let val = (1, 2, 3, 4, 5);
    // println!("{}", val); // `({integer}, {integer}, {integer}, {integer}, {integer})` doesn't implement `std::fmt::Display`
    println!("{:?}", val);

    let val = ((120, "String".to_string()), true);
    // println!("{}", val); // `(({integer}, String), bool)` doesn't implement `std::fmt::Display` the trait `std::fmt::Display`
    println!("{:?}", val);

    let val = Stats {
        player: "Messi".to_string(),
        goal: 2,
        assist: 1,
        is_starting: true,
    };
    // println!("{}", val); // `Stats` doesn't implement `std::fmt::Display` the trait `std::fmt::Display` is not implemented for `Stats`
    println!("{:?}", val);
}
