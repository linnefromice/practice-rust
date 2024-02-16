mod functions_test {
    use macros_3::generate_something;

    generate_something!("hello", String, String);

    #[test]
    #[should_panic(expected = "not implemented")]
    fn test() {
        assert_eq!(hello("World.".to_string()), "hello");
    }
}
