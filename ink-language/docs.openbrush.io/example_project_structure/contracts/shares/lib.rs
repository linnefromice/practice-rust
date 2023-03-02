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

        #[ink(message)]
        pub fn caller(&self) -> AccountId {
            self.env().caller()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        fn default_accounts() -> ink::env::test::DefaultAccounts<ink::env::DefaultEnvironment> {
            ink::env::test::default_accounts::<ink::env::DefaultEnvironment>()
        }

        #[ink::test]
        fn new_works() {
            let coin = SharesContract::new(
                Some(String::from("sample coin")),
                Some(String::from("SAMPLE")),
                18,
            );

            assert_eq!(coin.token_name().unwrap(), String::from("sample coin"));
            assert_eq!(coin.token_symbol().unwrap(), String::from("SAMPLE"));
            assert_eq!(coin.total_supply(), 0);
        }

        #[ink::test]
        fn mint_works() {
            let accounts = default_accounts();
            assert_eq!(accounts.alice, AccountId::from([0x01; 32]));
            assert_eq!(accounts.bob, AccountId::from([0x02; 32]));
            assert_eq!(accounts.charlie, AccountId::from([0x03; 32]));
            ink::env::debug_println!("{:?}", accounts.alice);
            ink::env::debug_println!("{:?}", accounts.bob);
            ink::env::debug_println!("{:?}", accounts.charlie);
        }
    }
}
