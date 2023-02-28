#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod erc20 {
    use ink::{storage::Mapping};

    #[ink(storage)]
    pub struct Erc20 {
        total_supply: Balance,
        balances: Mapping<AccountId, Balance>,
        allowances: Mapping<(AccountId, AccountId), Balance>
    }

    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        value: Balance,
    }

    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        spender: AccountId,
        value: Balance,
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        InsufficientBalance,
        InsufficientAllowance
    }

    pub type Result<T> = core::result::Result<T, Error>;

    impl Erc20 {
        #[ink(constructor)]
        pub fn new(total_supply: Balance) -> Self {
            let mut balances = Mapping::default();
            let caller = Self::env().caller();
            balances.insert(caller, &total_supply);

            Self::env().emit_event(Transfer {
                from: None,
                to: Some(caller),
                value: total_supply,
            });

            Self {
                total_supply,
                balances,
                allowances: Mapping::default(),
            }
        }

        #[ink(message)]
        pub fn total_supply(&self) -> Balance {
            self.total_supply
        }

        #[ink(message)]
        pub fn balance_of(&self, owner: AccountId) -> Balance {
            self.balances.get(owner).unwrap_or_default()
        }

        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, value: Balance) -> Result<()> {
            let from = self.env().caller();
            self.transfer_from_to(&from, &to, value)
        }

        #[ink(message)]
        pub fn transfer_from(&mut self, from: AccountId, to: AccountId, value: Balance) -> Result<()> {
            let caller = Self::env().caller();
            let allowance = self.allowance(from.clone(), caller.clone());
            if value > allowance {
                return Err(Error::InsufficientAllowance)
            }

            self.transfer_from_to(&from, &to, value)?;

            self.allowances.insert(&(from, caller), &(allowance - value));

            Ok(())
        }

        fn transfer_from_to(
            &mut self,
            from: &AccountId,
            to: &AccountId,
            value: Balance
        ) -> Result<()> {
            let from_balance = self.balance_of(*from);
            if from_balance < value {
                return Err(Error::InsufficientBalance)
            }

            self.balances.insert(&from, &(from_balance - value));
            let to_balance = self.balance_of(*to);
            self.balances.insert(&to, &(to_balance + value));

            Self::env().emit_event(Transfer {
                from: Some(*from),
                to: Some(*to),
                value,
            });

            Ok(())
        }

        #[ink(message)]
        pub fn approve(&mut self, spender: AccountId, value: Balance) -> Result<()> {
            let owner = Self::env().caller();
            self.allowances.insert(&(owner, spender), &value);

            Self::env().emit_event(Approval {
                owner,
                spender,
                value,
            });

            Ok(())
        }

        #[ink(message)]
        pub fn allowance(&self, owner: AccountId, spender: AccountId) -> Balance {
            self.allowances.get((owner, spender)).unwrap_or_default()
        }

    }

    #[cfg(test)]
    mod tests {
        use super::*;

        // We define some helper Accounts to make our tests more readable
        fn default_accounts() -> ink::env::test::DefaultAccounts<Environment> {
            ink::env::test::default_accounts::<Environment>()
        }

        fn alice() -> AccountId {
            default_accounts().alice
        }

        fn bob() -> AccountId {
            default_accounts().bob
        }

        #[ink::test]
        fn new_works() {
            let erc20 = Erc20::new(777);
            assert_eq!(erc20.total_supply(), 777);
        }

        #[ink::test]
        fn balance_works() {
            let erc20 = Erc20::new(100);
            assert_eq!(erc20.total_supply(), 100);
            assert_eq!(erc20.balance_of(alice()), 100);
            assert_eq!(erc20.balance_of(bob()), 0);
        }

        #[ink::test]
        fn transfer_works() {
            let mut contract = Erc20::new(100);
            assert_eq!(contract.balance_of(alice()), 100);
            assert!(contract.transfer(bob(), 10).is_ok());
            assert_eq!(contract.balance_of(bob()), 10);
            assert!(contract.transfer(bob(), 100).is_err());
        }
    }
}
