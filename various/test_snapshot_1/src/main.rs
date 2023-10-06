fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use std::fs;

    #[test]
    fn it_works() {
        let actual = r#"fn main() {
    println!("Hello, world!");
    println!("Hello, sample_code!");
}"#;
        let expected = fs::read_to_string("assets/sample_code.rs").unwrap();

        assert_eq!(actual, expected);
    }
}