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
        balances: Mapping<H160, Balance>,
    }
    impl Basic2 {
        /// Constructor that initializes the contract
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                balances: Mapping::default(),
            }
        }
        /// Get the balance of an account
        #[ink(message)]
        pub fn balance_of(&self, account: H160) -> Balance {
            self.balances.get(account).unwrap_or(0)
        }
        /// Set balance for the caller
        #[ink(message)]
        pub fn mint(&mut self, amount: Balance) -> Result<()> {
            if amount == 0 {
                return Err(Error::ZeroAmount);
            }

            let caller = self.env().caller();
            let current_balance = self.balance_of(caller);

            let new_balance = current_balance.checked_add(amount).ok_or(Error::Overflow)?;
            self.balances.insert(caller, &new_balance);
            Ok(())
        }
        /// Burn (remove) tokens from caller's balance
        #[ink(message)]
        pub fn burn(&mut self, amount: Balance) -> Result<()> {
            if amount == 0 {
                return Err(Error::ZeroAmount);
            }

            let caller = self.env().caller();
            let current_balance = self.balance_of(caller);

            let new_balance = current_balance
                .checked_sub(amount)
                .ok_or(Error::InsufficientBalance)?;
            self.balances.insert(caller, &new_balance);
            Ok(())
        }
        /// Transfer tokens from caller to another account
        #[ink(message)]
        pub fn transfer(&mut self, to: H160, amount: Balance) -> Result<()> {
            if amount == 0 {
                return Err(Error::ZeroAmount);
            }

            let caller = self.env().caller();
            let from_balance = self.balance_of(caller);

            if from_balance < amount {
                return Err(Error::InsufficientBalance);
            }
            let recipients_bal = self.balance_of(to);
            let new_from_balance = from_balance
                .checked_sub(amount)
                .ok_or(Error::InsufficientBalance)?;
            let new_recipients_balance =
                recipients_bal.checked_add(amount).ok_or(Error::Overflow)?;
            self.balances.insert(caller, &new_from_balance);
            self.balances.insert(to, &new_recipients_balance);
            Ok(())
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
