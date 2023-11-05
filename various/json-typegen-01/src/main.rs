
// json_typegen!("SampleUser", r#"
// {
//     "name": "John Doe",
//     "age": 43,
//     "phones": [
//         "+44 1234567",
//         "+44 2345678"
//     ]
// }"#);

use std::fs;

use json_typegen_shared::{codegen, Options};

fn main() {
    // let args: Vec<String> = std::env::args().collect();
    // if args.len() != 2 {
    //     println!("Usage: {} <json_file>", args[0]);
    //     std::process::exit(1);
    // }

    // let user = SampleUser {
    //     name: "John Doe".to_string(),
    //     age: 43,
    //     phones: vec!["+44 1234567".to_string(), "+44 2345678".to_string()]
    // };
    // dbg!(user);

    // let file = File::open("src/response.json").unwrap();
    let response_str = fs::read_to_string("src/response.json").unwrap();
    let generated = codegen("SampleUser", &response_str, Options::default());
    match generated {
        Ok(generated) => {
            println!("{}", generated);
        }
        Err(e) => {
            println!("{}", e);
        }
    }
}
