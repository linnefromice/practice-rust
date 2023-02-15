struct Val {
    val: f64,
}
struct GenVal<T> {
    gen_val: T
}

impl Val {
    fn value(&self) -> &f64 {
        &self.val
    }
}

impl<T> GenVal<T> {
    fn value(&self) -> &T {
        &self.gen_val
    }
}

pub fn execute() {
    let x = Val { val: 3.0 };
    let y = GenVal { gen_val: 3.0 };

    println!("{}, {}", x.value(), y.value());
}