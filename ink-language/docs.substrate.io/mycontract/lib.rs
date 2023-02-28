#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod mycontract {
    use ink::storage::Mapping;

    #[ink(storage)]
    pub struct Mycontract {
        my_map: Mapping<AccountId, u32>
    }

    impl Mycontract {
        #[ink(constructor)]
        pub fn new(count: u32) -> Self {
            let mut my_map = Mapping::default();
            let caller = Self::env().caller();
            my_map.insert(&caller, &count);

            Self { my_map }
        }

        #[ink(message)]
        pub fn get(&self) -> u32 {
            let caller = Self::env().caller();
            self.my_map.get(&caller).unwrap_or_default()
        }
    }
}
