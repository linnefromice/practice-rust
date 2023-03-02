#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod shares {
    use monorepo_project::traits::shares::*;
    use openbrush::traits::{Storage, String};
    use openbrush::{
        modifiers,
        contracts::{
            ownable::*,
            psp22::extensions::{
                metadata::*,
                mintable::*,
                burnable::*,
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

    impl PSP22 for SharesContract {}
    impl Ownable for SharesContract {}
    impl PSP22Metadata for SharesContract {}
    impl PSP22Mintable for SharesContract {
        // override
        #[ink(message)]
        #[modifiers(only_owner)]
        fn mint(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error> {
            self._mint_to(account, amount)
        }
    }
    impl PSP22Burnable for SharesContract {
        // override
        #[ink(message)]
        #[modifiers(only_owner)]
        fn burn(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error> {
            self._burn_from(account, amount)
        }
    }
    // It forces the compiler to check that you implemented all super traits
    impl Shares for SharesContract {}

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
    }
}
