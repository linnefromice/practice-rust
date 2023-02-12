struct Empty;
struct Null;

trait DoubleDrop<T> {
    fn double_drop(self, _: T);
}

impl <T, U> DoubleDrop<T> for U {
    fn double_drop(self, _: T) {}
}

pub fn execute() {
    let empty = Empty;
    let null = Null;

    32.double_drop(empty);
    null.double_drop(&"abc");
}