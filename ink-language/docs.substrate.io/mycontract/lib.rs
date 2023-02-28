#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod mycontract {
    #[ink(storage)]
    pub struct Mycontract {
        value: bool,
    }

    impl Mycontract {
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            Self { value: init_value }
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default())
        }

        #[ink(message)]
        pub fn flip(&mut self) {
            self.value = !self.value;
        }

        #[ink(message)]
        pub fn get(&self) -> bool {
            self.value
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
