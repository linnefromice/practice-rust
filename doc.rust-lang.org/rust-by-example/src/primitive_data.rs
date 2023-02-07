use std::fmt;

fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (int_param, bool_param) = pair;

    (bool_param, int_param)
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Matrix({}, {}, {}, {})", self.0, self.1, self.2, self.3)
    }
}

pub fn execute_data() {
    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{}", matrix);
    println!("{:?}", matrix);
}