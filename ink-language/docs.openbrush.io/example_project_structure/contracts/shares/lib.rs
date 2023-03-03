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
            let mut instance = Self {
                psp22: psp22::Data::default(),
                ownable: ownable::Data::default(),
                metadata: metadata::Data {
                    name,
                    symbol,
                    decimals,
                    _reserved: None,
                },
            };
            instance._init_with_owner(Self::env().caller());
            instance
        }

        #[ink(message)]
        pub fn caller(&self) -> AccountId {
            self.env().caller()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use openbrush::contracts::traits::errors;

        fn default_accounts() -> ink::env::test::DefaultAccounts<ink::env::DefaultEnvironment> {
            ink::env::test::default_accounts::<ink::env::DefaultEnvironment>()
        }

        #[ink::test]
        fn new_works() {
            let accounts = default_accounts();
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.bob);
            let contract = SharesContract::new(
                Some(String::from("sample coin")),
                Some(String::from("SAMPLE")),
                18,
            );

            assert_eq!(contract.token_name().unwrap(), String::from("sample coin"));
            assert_eq!(contract.token_symbol().unwrap(), String::from("SAMPLE"));
            assert_eq!(contract.total_supply(), 0);
            assert_eq!(contract.owner(), accounts.bob);
            // let events = ink::env::test::recorded_events();
            // assert_eq!(events.count(), 0);
            // ink::env::test::advance_block::<ink::env::DefaultEnvironment>();
            // let events = ink::env::test::recorded_events();
            // assert_eq!(events.count(), 0);
        }

        #[ink::test]
        fn mint_works() {
            let accounts = default_accounts();
            let alice = accounts.alice;
            let bob = accounts.bob;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(bob);
            let mut contract = SharesContract::new(
                Some(String::from("sample coin")),
                Some(String::from("SAMPLE")),
                8,
            );

            // by owner
            assert!(contract.mint(bob, 10_000_000).is_ok());
            assert!(contract.mint(alice, 5_000_000).is_ok());
            assert_eq!(contract.balance_of(bob), 10_000_000);
            assert_eq!(contract.balance_of(alice), 5_000_000);
            assert_eq!(contract.total_supply(), 15_000_000);

            // by not owner
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(alice);
            assert_eq!(
                contract.mint(bob, 2_500_000).unwrap_err(),
                errors::PSP22Error::from(errors::OwnableError::CallerIsNotOwner)
            );
            assert_eq!(contract.balance_of(bob), 10_000_000);
            assert_eq!(contract.balance_of(alice), 5_000_000);
            assert_eq!(contract.total_supply(), 15_000_000);
        }

        #[ink::test]
        fn burn_works() {
            let accounts = default_accounts();
            let alice = accounts.alice;
            let bob = accounts.bob;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(bob);
            let mut contract = SharesContract::new(
                Some(String::from("sample coin")),
                Some(String::from("SAMPLE")),
                8,
            );
            assert!(contract.mint(bob, 10_000_000).is_ok());
            assert!(contract.mint(alice, 5_000_000).is_ok());

            // by owner
            assert!(contract.burn(bob, 1_000_000).is_ok());
            assert!(contract.burn(alice, 3_000_000).is_ok());
            assert_eq!(contract.balance_of(bob), 9_000_000);
            assert_eq!(contract.balance_of(alice), 2_000_000);
            assert_eq!(contract.total_supply(), 11_000_000);

            // by not owner
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(alice);
            assert_eq!(
                contract.burn(bob, 500_000).unwrap_err(),
                errors::PSP22Error::from(errors::OwnableError::CallerIsNotOwner)
            );
            assert_eq!(contract.balance_of(bob), 9_000_000);
            assert_eq!(contract.balance_of(alice), 2_000_000);
            assert_eq!(contract.total_supply(), 11_000_000);
        }
    }
}
