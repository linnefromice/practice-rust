mod custom_attribute;
mod derive_macro;

use custom_attribute::example_func;
use derive_macro::Sample;

fn main() {
    example_func();

    let sample = Sample {
        field1: 1,
        field2: 2,
    };
    println!("field1: {}", sample.get_field1());
    println!("field2: {}", sample.get_field2());
}
