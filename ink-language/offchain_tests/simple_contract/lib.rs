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
        use sp_core::Pair;
        use ink::env::DefaultEnvironment;
        use ink_e2e::SubstrateConfig;

        fn default_accounts() -> ink::env::test::DefaultAccounts<DefaultEnvironment> {
            ink::env::test::default_accounts::<DefaultEnvironment>()
        }
        fn get_account_balance(id: AccountId) -> Balance {
            ink::env::test::get_account_balance::<DefaultEnvironment>(id).unwrap()
        }
        fn set_account_balance(id: AccountId, balance: Balance) {
            ink::env::test::set_account_balance::<DefaultEnvironment>(id, balance);
        }
        fn create_signer_from_seed(seed: &str) -> ink_e2e::PairSigner::<SubstrateConfig, sp_core::sr25519::Pair> {
            let valid_public = <sp_core::sr25519::Pair as sp_core::Pair>::from_string_with_seed(seed, None).unwrap();
            ink_e2e::PairSigner::<SubstrateConfig, sp_core::sr25519::Pair>::new(valid_public.0)
        }
        fn create_signer() -> ink_e2e::PairSigner::<SubstrateConfig, sp_core::sr25519::Pair> {
            let pair = <sp_core::sr25519::Pair as sp_core::Pair>::generate();
            ink_e2e::PairSigner::<SubstrateConfig, sp_core::sr25519::Pair>::new(pair.0)
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

        #[ink::test]
        fn env_new_account() {
            let seed =
			    "remember fiber forum demise paper uniform squirrel feel access exclude casual effort";
            let valid_public = <sp_core::sr25519::Pair as sp_core::Pair>::from_string_with_seed(seed, None).unwrap();
            // ink::env::debug_println!("{:?}", valid_public.0.public());
            // ink::env::debug_println!("{:?}", valid_public.1.unwrap());
            let new_signer = ink_e2e::PairSigner::<
                SubstrateConfig,
                sp_core::sr25519::Pair
            >::new(valid_public.0);
            // ink::env::debug_println!("{:?}", new_signer.account_id());
            let new_account_id = AccountId::try_from(new_signer.account_id().0).unwrap();
            // ink::env::debug_println!("{:?}", new_account_id);

            set_account_balance(new_account_id, 0);
            assert_eq!(get_account_balance(new_account_id), 0);
            set_account_balance(new_account_id, 25_000);
            assert_eq!(get_account_balance(new_account_id), 25_000);
        }

        #[ink::test]
        fn env_transfer_in() {
            let seed1 =
                "remember fiber forum demise paper uniform squirrel feel access exclude casual effort";

            let signer1 = create_signer_from_seed(seed1);
            let signer2 = create_signer();
            let signer1_id = AccountId::try_from(signer1.account_id().0).unwrap();
            let signer2_id = AccountId::try_from(signer2.account_id().0).unwrap();
            set_account_balance(signer1_id, 10_000);
            set_account_balance(signer2_id, 5_000);
            assert_eq!(get_account_balance(signer1_id), 10_000);
            assert_eq!(get_account_balance(signer2_id), 5_000);
            ink::env::test::set_caller::<DefaultEnvironment>(signer1_id);
            ink::env::test::set_callee::<DefaultEnvironment>(signer2_id);
            ink::env::test::transfer_in::<DefaultEnvironment>(7_500);
            assert_eq!(get_account_balance(signer1_id), 2_500);
            assert_eq!(get_account_balance(signer2_id), 12_500);
        }

        #[ink::test]
        fn env_pay_with_call() {
            use ink::codegen::Env;

            let accounts = default_accounts();
            let signer1 = create_signer();
            let signer2 = create_signer();
            let signer1_id = AccountId::try_from(signer1.account_id().0).unwrap();
            let signer2_id = AccountId::try_from(signer2.account_id().0).unwrap();
            set_account_balance(signer1_id, 0);
            set_account_balance(signer2_id, 1_000_000);

            ink::env::test::set_caller::<DefaultEnvironment>(signer1_id);
            let contract = SimpleContract::default();
            
            // ink::env::debug_println!("{:?}", signer1_id);
            // ink::env::debug_println!("{:?}", signer2_id);
            // ink::env::debug_println!("{:?}", contract.env().account_id());
            // ink::env::debug_println!("{:?}", accounts.alice);
            assert_eq!(get_account_balance(contract.env().account_id()), 1_000_000);
            assert_eq!(get_account_balance(accounts.alice), 1_000_000); // why same id?

            ink::env::test::set_caller::<DefaultEnvironment>(signer2_id);
            ink::env::pay_with_call!(contract.caller(), 1_000);

            assert_eq!(get_account_balance(signer2_id), 999_000);
            assert_eq!(get_account_balance(accounts.alice), 1_001_000);
            assert_eq!(get_account_balance(contract.env().account_id()), 1_001_000); // why same id?
        }
    }
}
