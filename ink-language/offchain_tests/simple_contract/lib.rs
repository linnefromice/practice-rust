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
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn default_works() {
            let simple_contract = SimpleContract::default();
            assert_eq!(simple_contract.get(), 0);
        }
    }
}
