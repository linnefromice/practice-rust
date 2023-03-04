#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod token {
    use monorepo_project::traits::stable_coin::*;
    use openbrush::{
        contracts::psp22::extensions::{
            metadata::*,
            mintable::*,
        },
        traits::{Storage, String},
    };

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct StableCoinContract {
        #[storage_field]
        psp22: psp22::Data,
        #[storage_field]
        metadata: metadata::Data,
    }

    impl PSP22 for StableCoinContract {}
    impl PSP22Metadata for StableCoinContract {}
    impl PSP22Mintable for StableCoinContract {}
    impl StableCoin for StableCoinContract {}

    impl StableCoinContract {
        #[ink(constructor)]
        pub fn new(
            name: Option<String>,
            symbol: Option<String>,
            decimals: u8,
        ) -> Self {
            Self {
                psp22: psp22::Data::default(),
                metadata: metadata::Data {
                    name,
                    symbol,
                    decimals,
                    _reserved: None,
                },
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn new_works() {
            let coin = StableCoinContract::new(
                Some(String::from("sample coin")),
                Some(String::from("SAMPLE")),
                18,
            );

            assert_eq!(coin.token_name().unwrap(), String::from("sample coin"));
            assert_eq!(coin.token_symbol().unwrap(), String::from("SAMPLE"));
            assert_eq!(coin.total_supply(), 0);
        }
    }
}