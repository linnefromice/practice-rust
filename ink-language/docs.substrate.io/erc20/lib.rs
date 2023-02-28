#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod erc20 {
    #[ink(storage)]
    pub struct Erc20 {
        value: bool,
    }

    impl Erc20 {
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

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn default_works() {
            let erc20 = Erc20::default();
            assert_eq!(erc20.get(), false);
        }

        #[ink::test]
        fn it_works() {
            let mut erc20 = Erc20::new(false);
            assert_eq!(erc20.get(), false);
            erc20.flip();
            assert_eq!(erc20.get(), true);
        }
    }
}
