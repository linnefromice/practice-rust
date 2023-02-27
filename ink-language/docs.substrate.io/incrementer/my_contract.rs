#[ink::contract]
mod my_contract {
    
    #[ink(storage)]
    pub struct MyContract {
        number: u32,
    }

    impl MyContract {
        #[ink(constructor)]
        pub fn new(init_value: u32) -> Self {
            Self {
                number: init_value
            }
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            Self {
                number: Default::default(),
            }
        }
    }
}