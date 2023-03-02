#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod shares {
    use openbrush::traits::{Storage, String};
    use openbrush::{
        contracts::{
            ownable::*,
            psp22::extensions::{
                metadata::*,
            }
        }
    };

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct SharesContract {
        #[storage_field]
        psp22: psp22::Data,
        #[storage_field]
        ownable: ownable::Data,
        #[storage_field]
        metadata: metadata::Data,
    }

    impl SharesContract {
        #[ink(constructor)]
        pub fn new(
            name: Option<String>,
            symbol: Option<String>,
            decimals: u8,
        ) -> Self {
            Self {
                psp22: psp22::Data::default(),
                ownable: ownable::Data::default(),
                metadata: metadata::Data {
                    name,
                    symbol,
                    decimals,
                    _reserved: None,
                },
            }
        }

        #[ink(message)]
        pub fn share_name(&self) -> Option<String> {
            self.metadata.token_name()
        }
        #[ink(message)]
        pub fn share_symbol(&self) -> Option<String> {
            self.metadata.token_symbol()
        }
    }
}
