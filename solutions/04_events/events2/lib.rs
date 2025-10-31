#![allow(unexpected_cfgs)]
#[ink::contract]
mod events2 {
    use ink::primitives::H160;
    use ink::storage::Mapping;
    #[derive(Debug, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub enum Error {
        /// Insufficient balance
        InsufficientBalance,
    }
    type Result<T> = core::result::Result<T, Error>;
    /// Event emitted when tokens are transferred
    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<H160>,

        #[ink(topic)]
        to: Option<H160>,

        value: Balance,
    }

    /// Event emitted when someone approves another account to spend tokens

    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        owner: H160,
        #[ink(topic)]
        spender: H160,
        value: Balance,
    }

    #[ink(storage)]
    pub struct Events2 {
        balances: Mapping<H160, Balance>,
        total_supply: Balance,
    }

    impl Events2 {
        /// Constructor that initializes the contract with a total supply
        #[ink(constructor)]
        pub fn new(total_supply: Balance) -> Self {
            let mut balances = Mapping::default();
            let caller = Self::env().caller();
            balances.insert(caller, &total_supply);

            Self {
                balances,
                total_supply,
            }
        }

        /// Transfer tokens from caller to another account
        #[ink(message)]
        pub fn transfer(&mut self, to: H160, value: Balance) -> Result<()> {
            let from = self.env().caller();
            let from_balance = self.balance_of(from);

            if from_balance < value {
                return Err(Error::InsufficientBalance);
            }

            self.balances.insert(from, &(from_balance - value));
            let to_balance = self.balance_of(to);
            self.balances.insert(to, &(to_balance + value));

            self.env().emit_event(Transfer {
                from: Some(from),
                to: Some(to),
                value,
            });
            Ok(())
        }

        /// Get the balance of an account
        #[ink(message)]
        pub fn balance_of(&self, account: H160) -> Balance {
            self.balances.get(account).unwrap_or(0)
        }

        /// Get the total supply
        #[ink(message)]
        pub fn total_supply(&self) -> Balance {
            self.total_supply
        }

        /// Mint new tokens (for testing purposes)
        #[ink(message)]
        pub fn mint(&mut self, to: H160, value: Balance) -> Result<()> {
            let to_balance = self.balance_of(to);
            self.balances.insert(to, &(to_balance + value));
            self.total_supply += value;

            self.env().emit_event(Transfer {
                from: None,
                to: Some(to),
                value,
            });
            Ok(())
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn new_works() {
            let accounts = ink::env::test::default_accounts();
            ink::env::test::set_caller(accounts.alice);
            let contract = Events2::new(1000);

            assert_eq!(contract.balance_of(accounts.alice), 1000);
            assert_eq!(contract.total_supply(), 1000);
        }

        #[ink::test]
        fn transfer_works() {
            let accounts = ink::env::test::default_accounts();
            ink::env::test::set_caller(accounts.alice);
            let mut contract = Events2::new(1000);

            assert_eq!(contract.transfer(accounts.bob, 100), Ok(()));
            assert_eq!(contract.balance_of(accounts.alice), 900);
            assert_eq!(contract.balance_of(accounts.bob), 100);
        }

        #[ink::test]
        fn transfer_insufficient_balance() {
            let mut contract = Events2::new(1000);
            let accounts = ink::env::test::default_accounts();

            assert_eq!(
                contract.transfer(accounts.bob, 1001),
                Err(Error::InsufficientBalance)
            );
        }

        #[ink::test]
        fn mint_works() {
            let mut contract = Events2::new(1000);
            let accounts = ink::env::test::default_accounts();

            assert_eq!(contract.mint(accounts.bob, 500), Ok(()));
            assert_eq!(contract.balance_of(accounts.bob), 500);
            assert_eq!(contract.total_supply(), 1500);
        }
    }
}
