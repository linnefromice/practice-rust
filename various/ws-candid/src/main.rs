fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use std::ops::Deref;

    use candid::TypeEnv;
    use candid_parser::{IDLProg, check_prog};

    fn generate_type_env(ast: IDLProg) -> TypeEnv {
        let mut type_env = TypeEnv::new();
        let _ = check_prog(&mut type_env, &ast);
        type_env
    }

    #[test]
    fn test_only_primitive() {
        let did  = r#"
type RequestType = nat;
type ResponseType = text;"#.to_string();
        let ast: IDLProg = did.parse().unwrap();
        let type_env = generate_type_env(ast);
        assert_eq!(type_env.find_type("RequestType").unwrap().deref(), &candid::types::TypeInner::Nat);
        assert_eq!(type_env.find_type("ResponseType").unwrap().deref(), &candid::types::TypeInner::Text);
    }

    #[test]
    fn test_unknown_letter() {
        let did  = r#"
type RequestType = nat;
type ResponseType = UnknownType;"#.to_string();
        let ast: IDLProg = did.parse().unwrap();
        let type_env = generate_type_env(ast);
        assert_eq!(type_env.find_type("RequestType").unwrap().deref(), &candid::types::TypeInner::Nat);
        assert_eq!(type_env.find_type("ResponseType").unwrap().deref(), &candid::types::TypeInner::Unknown);
    }

    #[test]
    fn test_unknown_former() {
        let did  = r#"
type RequestType = UnknownType;
type ResponseType = text;"#.to_string();
        let ast: IDLProg = did.parse().unwrap();
        let type_env = generate_type_env(ast);
        assert_eq!(type_env.find_type("RequestType").unwrap().deref(), &candid::types::TypeInner::Unknown);
        assert_eq!(type_env.find_type("ResponseType").unwrap().deref(), &candid::types::TypeInner::Unknown); // NOTE: Why can't I get Text?
    }
}