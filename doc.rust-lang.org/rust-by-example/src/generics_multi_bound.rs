use std::fmt::{Debug, Display};

fn compare_prints<T: Debug + Display>(t: &T) {
    println!("Debug: {:?}", t);
    println!("Display: {}", t);
}

fn compare_types<T: Debug, U: Debug>(t: &T, u: &U) {
    println!("Debug: {:?}, {:?}", t, u);
}

pub fn execute() {
    let string_val = "words";
    let arr_val = [1, 2, 3];
    let vec = vec![1, 2, 3];

    compare_prints(&string_val);
    compare_types(&arr_val, &vec);
}

// impl <A: TraitB + TraitC, D: TraitE + TraitF> MyTrait<A, D> for YourType {}

// impl <A, D> MyTrait<A, D> for YourType where
//     A: TraitB + TraitC,
//     D: TraitE + TraitF {}
