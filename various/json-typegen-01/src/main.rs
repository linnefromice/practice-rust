
// json_typegen!("SampleUser", r#"
// {
//     "name": "John Doe",
//     "age": 43,
//     "phones": [
//         "+44 1234567",
//         "+44 2345678"
//     ]
// }"#);

use std::{fs, option};

use json_typegen_shared::{codegen, Options, ImportStyle};

fn type_gen_from_file(name: String, path: String, options: Options) -> Result<String, String> {
    let response_str = fs::read_to_string(path).unwrap();
    let generated = codegen(&name, &response_str, options);
    match generated {
        Ok(generated) => Ok(generated),
        Err(e) => Err(format!("{}", e))
    }
}

fn type_gen_from_url(name: String, url: String, options: Options) -> Result<String, String> {
    let generated = codegen(&name, &url, options);
    match generated {
        Ok(generated) => Ok(generated),
        Err(e) => Err(format!("{}", e))
    }
}

fn main() {
    let res = type_gen_from_file("SampleUser".to_string(), "src/response.json".to_string(), Options::default());
    println!("{}", res.unwrap());
    let res = type_gen_from_url("SampleUser2".to_string(), "https://api.coingecko.com/api/v3/simple/price?ids=dai&vs_currencies=usd".to_string(), Options::default());
    println!("{}", res.unwrap());

    let mut options = Options::default();
    options.use_default_for_missing_fields = true; // serde(default)
    options.deny_unknown_fields = true; // #[serde(deny_unknown_fields)]
    // type_visibility = pub struct
    // field_visibility = pub (field_key) in struct
    // options.derives = "Default, Debug, Clone, PartialEq".into();
    options.derives = "Debug, Clone, candid::CandidType, candid::Deserialize, serde::Serialize, chainsight_cdk_macros::StableMemoryStorable".into();
    options.import_style = ImportStyle::QualifiedPaths;
    // AssumeExisting: 存在することを想定して module 名を落とす
    // QualifiedPaths: 記述通りの full path にする
    // AddImports: use を追加する (:: で分割される)
    options.collect_additional = true; // 以下が追加される
    // #[serde(flatten)]
    // additional_fields: std::collections::HashMap<String, serde_json::Value>,
    let res = type_gen_from_file("SampleUser".to_string(), "src/response.json".to_string(), options);
    println!("{}", res.unwrap());
}
