use json_typegen::json_typegen;

json_typegen!("User", r#"
{
    "name": "John Doe",
    "age": 43,
    "phones": [
        "+44 1234567",
        "+44 2345678"
    ]
}"#);

fn main() {
    let user = User {
        name: "John Doe".to_string(),
        age: 43,
        phones: vec!["+44 1234567".to_string(), "+44 2345678".to_string()]
    };

    // let user = (r#"
    // {
    //     "name": "John Doe",
    //     "age": 43,
    //     "phones": [
    //         "+44 1234567",
    //         "+44 2345678"
    //     ]
    // }"#).unwrap();
    dbg!(user);
}
