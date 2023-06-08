fn main() {
    println!("Hello, world!");
}


#[cfg(test)]
mod tests {
    use std::{fs::File, io::Read};

    use ic_web3_rs::ethabi::Contract;

  #[test]
  fn test_contract_functions() {
    let mut file = File::open("res/ERC20.json").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let contract = Contract::load(contents.as_bytes()).unwrap();

    let transfer_func = contract.functions.get("transfer").unwrap();
    assert_eq!(
      format!("{:?}", transfer_func),
      r#"[Function { name: "transfer", inputs: [Param { name: "to", kind: Address, internal_type: Some("address") }, Param { name: "amount", kind: Uint(256), internal_type: Some("uint256") }], outputs: [Param { name: "", kind: Bool, internal_type: Some("bool") }], constant: None, state_mutability: NonPayable }]"#
    );
    let balance_of_func = contract.functions.get("balanceOf").unwrap();
    assert_eq!(
      format!("{:?}", balance_of_func),
      r#"[Function { name: "balanceOf", inputs: [Param { name: "account", kind: Address, internal_type: Some("address") }], outputs: [Param { name: "", kind: Uint(256), internal_type: Some("uint256") }], constant: None, state_mutability: View }]"#
    );

    assert_eq!(
      transfer_func[0].signature(),
      r#"transfer(address,uint256):(bool)"#
    );
    assert_eq!(
      balance_of_func[0].signature(),
      r#"balanceOf(address):(uint256)"#
    );
  }
}
