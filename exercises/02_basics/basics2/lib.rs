// basic2
//
// This exercise teaches you about using Mappings in ink!
//
// Mappings are like HashMaps but optimized for blockchain storage.
// They allow you to store key-value pairs efficiently.
//
// Your task: Implement a simple balance tracking contract using Mapping
//
// Execute `inklings verify 02_basics/basic2` when you think you're done!
#![allow(unexpected_cfgs)]
#[ink::contract]
mod basic2 {
    use ink::primitives::H160;
    use ink::storage::Mapping;
    #[derive(Debug, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub enum Error {
        /// Insufficient balance for operation
        InsufficientBalance,
        /// Transfer amount is zero
        ZeroAmount,

        /// Overflow in arithmetic operations
        Overflow,
    }
    pub type Result<T> = core::result::Result<T, Error>;
    #[ink(storage)]
    pub struct Basic2 {
        // TODO: Add a Mapping from H160 to Balance (u128)
        // Hint: Mapping<H160, Balance>
        todo!()
    }
    impl Basic2 {
        /// Constructor that initializes the contract
        #[ink(constructor)]
        pub fn new() -> Self {
            // TODO: Initialize the mapping using Mapping::default()
            todo!()
        }
        /// Get the balance of an account
        #[ink(message)]
        pub fn balance_of(&self, account: H160) -> Balance {
            // TODO: Get the balance from the mapping
            // Use .get() which returns Option<Balance>
            // Return 0 if the account is not found (use .unwrap_or(0))
            todo!()
        }
        /// Set balance for the caller
        #[ink(message)]
        pub fn mint(&mut self, amount: Balance) -> Result<()> {
            if amount == 0 {
                return Err(Error::ZeroAmount);
            }

            let caller = self.env().caller();
            let current_balance = self.balance_of(caller);

            // TODO: Calculate new balance (current_balance + amount)
            // TODO: Insert the new balance into the mapping for the caller
            // Hint: self.balances.insert(caller, &new_balance);
            todo!()
        }
        /// Burn (remove) tokens from caller's balance
        #[ink(message)]
        pub fn burn(&mut self, amount: Balance) -> Result<()> {
            if amount == 0 {
                return Err(Error::ZeroAmount);
            }

            let caller = self.env().caller();
            let current_balance = self.balance_of(caller);

            // TODO: Check if caller has enough balance
            // If not, return Error::InsufficientBalance

            // TODO: Calculate new balance (current_balance - amount)
            // TODO: Insert the new balance into the mapping
            todo!()
        }
        /// Transfer tokens from caller to another account
        #[ink(message)]
        pub fn transfer(&mut self, to: H160, amount: Balance) -> Result<()> {
            if amount == 0 {
                return Err(Error::ZeroAmount);
            }

            let caller = self.env().caller();
            let from_balance = self.balance_of(caller);

            // TODO: Check if caller has enough balance
            // If not, return Error::InsufficientBalance

            // TODO: Get the recipient's current balance

            // TODO: Update both balances:
            // - Subtract amount from caller's balance
            // - Add amount to recipient's balance
            // - Insert both updated balances into the mapping

            todo!()
        }
    }
    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn new_works() {
            let contract = Basic2::new();
            let accounts = ink::env::test::default_accounts();
            assert_eq!(contract.balance_of(accounts.alice), 0);
        }

        #[ink::test]
        fn mint_works() {
            let mut contract = Basic2::new();
            let accounts = ink::env::test::default_accounts();
            ink::env::test::set_caller(accounts.alice);
            assert_eq!(contract.mint(100), Ok(()));
            assert_eq!(contract.balance_of(accounts.alice), 100);

            assert_eq!(contract.mint(50), Ok(()));
            assert_eq!(contract.balance_of(accounts.alice), 150);
        }

        #[ink::test]
        fn mint_zero_fails() {
            let mut contract = Basic2::new();
            assert_eq!(contract.mint(0), Err(Error::ZeroAmount));
        }

        #[ink::test]
        fn burn_works() {
            let mut contract = Basic2::new();
            let accounts = ink::env::test::default_accounts();
            ink::env::test::set_caller(accounts.alice);
            assert_eq!(contract.mint(100), Ok(()));
            assert_eq!(contract.burn(30), Ok(()));
            assert_eq!(contract.balance_of(accounts.alice), 70);
        }

        #[ink::test]
        fn burn_insufficient_balance() {
            let mut contract = Basic2::new();
            let accounts = ink::env::test::default_accounts();
            ink::env::test::set_caller(accounts.alice);

            assert_eq!(contract.mint(50), Ok(()));
            assert_eq!(contract.burn(100), Err(Error::InsufficientBalance));
            assert_eq!(contract.balance_of(accounts.alice), 50);
        }

        #[ink::test]
        fn transfer_works() {
            let mut contract = Basic2::new();
            let accounts = ink::env::test::default_accounts();
            ink::env::test::set_caller(accounts.alice);

            assert_eq!(contract.mint(100), Ok(()));
            assert_eq!(contract.transfer(accounts.bob, 30), Ok(()));

            assert_eq!(contract.balance_of(accounts.alice), 70);
            assert_eq!(contract.balance_of(accounts.bob), 30);
        }

        #[ink::test]
        fn transfer_insufficient_balance() {
            let mut contract = Basic2::new();
            let accounts = ink::env::test::default_accounts();

            ink::env::test::set_caller(accounts.alice);
            assert_eq!(contract.mint(50), Ok(()));
            assert_eq!(
                contract.transfer(accounts.bob, 100),
                Err(Error::InsufficientBalance)
            );

            // Balances should remain unchanged
            assert_eq!(contract.balance_of(accounts.alice), 50);
            assert_eq!(contract.balance_of(accounts.bob), 0);
        }

        #[ink::test]
        fn transfer_zero_fails() {
            let mut contract = Basic2::new();
            let accounts = ink::env::test::default_accounts();
            ink::env::test::set_caller(accounts.bob);
            assert_eq!(contract.transfer(accounts.bob, 0), Err(Error::ZeroAmount));
        }
    }
}
