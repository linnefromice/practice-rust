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

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            let mycontract = Mycontract::default();
            assert_eq!(mycontract.get(), false);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let mut mycontract = Mycontract::new(false);
            assert_eq!(mycontract.get(), false);
            mycontract.flip();
            assert_eq!(mycontract.get(), true);
        }
    }
}
