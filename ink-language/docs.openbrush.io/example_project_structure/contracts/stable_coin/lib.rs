#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod token {
    // use monorepo_project::traits::stable_coin::*;
    use openbrush::{
        contracts::psp22::extensions::{
            metadata::*,
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

        #[ink(message)]
        pub fn decimals(&self) -> u8 {
            self.metadata.token_decimals()
        }
    }
}