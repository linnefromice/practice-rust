#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod simple_contract {
    #[ink(storage)]
    pub struct SimpleContract {
        value: u32,
    }

    impl SimpleContract {
        #[ink(constructor)]
        pub fn new(init_value: u32) -> Self {
            Self { value: init_value }
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default())
        }

        /// Simply returns the current value of our `bool`.
        #[ink(message)]
        pub fn get(&self) -> u32 {
            self.value
        }

        #[ink(message)]
        pub fn caller(&self) -> AccountId {
            self.env().caller()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use ink::env::DefaultEnvironment;

        fn default_accounts() -> ink::env::test::DefaultAccounts<DefaultEnvironment> {
            ink::env::test::default_accounts::<DefaultEnvironment>()
        }
        fn get_account_balance(id: AccountId) -> Balance {
            ink::env::test::get_account_balance::<DefaultEnvironment>(id).unwrap()
        }

        #[ink::test]
        fn default_works() {
            let contract = SimpleContract::default();
            assert_eq!(contract.get(), 0);
        }

        #[ink::test]
        fn env_events_after_instanced_contract() {
            let _ = SimpleContract::default();

            let events = ink::env::test::recorded_events();
            assert_eq!(events.count(), 0);
            // let last_event = events.last().unwrap();
            // ink::env::debug_println!("{:?}", last_event.topics);
            // ink::env::debug_println!("{:?}", last_event.data);
        }

        #[ink::test]
        fn env_accounts_works() {
            let accounts = default_accounts();
            assert_eq!(accounts.alice, AccountId::from([0x01; 32]));
            assert_eq!(accounts.bob, AccountId::from([0x02; 32]));
            assert_eq!(accounts.charlie, AccountId::from([0x03; 32]));
            assert_eq!(accounts.django, AccountId::from([0x04; 32]));
            assert_eq!(accounts.eve, AccountId::from([0x05; 32]));
            assert_eq!(accounts.frank, AccountId::from([0x06; 32]));
            // ink::env::debug_println!("{:?}", accounts.alice);
            // ink::env::debug_println!("{:?}", accounts.bob);
            // ink::env::debug_println!("{:?}", accounts.charlie);
            // ink::env::debug_println!("{:?}", accounts.django);
            // ink::env::debug_println!("{:?}", accounts.eve);
            // ink::env::debug_println!("{:?}", accounts.frank);
        }

        #[ink::test]
        fn env_caller() {
            let accounts = default_accounts();
            let contract = SimpleContract::default();
            assert_eq!(contract.caller(), accounts.alice);

            ink::env::test::set_caller::<DefaultEnvironment>(accounts.bob);
            assert_eq!(contract.caller(), accounts.bob);

            ink::env::test::set_caller::<DefaultEnvironment>(accounts.charlie);
            assert_eq!(contract.caller(), accounts.charlie);
        }

        #[ink::test]
        fn env_get_account_balance() {
            let accounts = default_accounts();
            assert_eq!(get_account_balance(accounts.alice), 1_000_000);
            assert_eq!(get_account_balance(accounts.bob), 1_000);
            assert_eq!(get_account_balance(accounts.charlie), 1_000);

            let _ = SimpleContract::default();
            assert_eq!(get_account_balance(accounts.alice), 1_000_000);
            assert_eq!(get_account_balance(accounts.bob), 1_000);
            assert_eq!(get_account_balance(accounts.charlie), 1_000);

            ink::env::test::set_account_balance::<DefaultEnvironment>(accounts.charlie, 50_000);
            assert_eq!(get_account_balance(accounts.charlie), 50_000);
        }
    }
}
