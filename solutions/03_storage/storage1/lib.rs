#![allow(unexpected_cfgs)]
#[ink::contract]
mod storage1 {
    use ink::primitives::H160;
    use ink::storage::Lazy;
    #[derive(Debug, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub enum Error {
        /// Caller is not the owner
        NotOwner,
    }
    #[ink(storage)]
    pub struct Storage1 {
        name: Lazy<String>,
        description: Lazy<String>,
        owner: H160,
    }
    type Result<T> = core::result::Result<T, Error>;

    impl Storage1 {
        /// Constructor that initializes the contract with metadata
        #[ink(constructor)]
        pub fn new(name: String, description: String) -> Self {
            let caller = Self::env().caller();

            let mut instance = Self {
                name: Lazy::new(),
                description: Lazy::new(),
                owner: caller,
            };
            instance.name.set(&name);
            instance.description.set(&description);

            instance
        }

        /// Get the contract name
        #[ink(message)]
        pub fn get_name(&self) -> String {
            self.name.get().unwrap_or_default()
        }

        /// Get the contract description
        #[ink(message)]
        pub fn get_description(&self) -> String {
            self.description.get().unwrap_or_default()
        }

        /// Update the contract name (only owner can do this)
        #[ink(message)]
        pub fn set_name(&mut self, new_name: String) -> Result<()> {
            let caller = self.env().caller();

            if caller != self.owner {
                return Err(Error::NotOwner);
            }
            self.name.set(&new_name);
            Ok(())
        }

        /// Update the contract description (only owner can do this)
        #[ink(message)]
        pub fn set_description(&mut self, new_description: String) -> Result<()> {
            let caller = self.env().caller();
            if caller != self.owner {
                return Err(Error::NotOwner);
            }
            self.description.set(&new_description);
            Ok(())
        }

        /// Get the owner address
        #[ink(message)]
        pub fn get_owner(&self) -> H160 {
            self.owner
        }

        /// Transfer ownership (only current owner can do this)
        #[ink(message)]
        pub fn transfer_ownership(&mut self, new_owner: H160) -> Result<()> {
            let caller = self.env().caller();
            if caller != self.owner {
                return Err(Error::NotOwner);
            }
            self.owner = new_owner;
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
            let contract =
                Storage1::new(String::from("NewContract"), String::from("NewDescription"));

            assert_eq!(contract.get_name(), String::from("NewContract"));
            assert_eq!(contract.get_description(), String::from("NewDescription"));

            assert_eq!(contract.get_owner(), accounts.alice);
        }

        #[ink::test]
        fn set_name_works() {
            let accounts = ink::env::test::default_accounts();
            ink::env::test::set_caller(accounts.alice);
            let mut contract = Storage1::new(String::from("OldName"), String::from("Description"));

            assert_eq!(contract.set_name(String::from("NewName")), Ok(()));
            assert_eq!(contract.get_name(), String::from("NewName"));
        }

        #[ink::test]
        fn set_name_fails_not_owner() {
            let accounts = ink::env::test::default_accounts();
            // Alice creates the contract (becomes owner)
            ink::env::test::set_caller(accounts.alice);

            let mut contract = Storage1::new(String::from("Name"), String::from("Description"));
            // Bob tries to change the name (Alice is the owner)
            ink::env::test::set_caller(accounts.bob);
            assert_eq!(
                contract.set_name(String::from("NewName")),
                Err(Error::NotOwner)
            );

            // Name should be unchanged
            assert_eq!(contract.get_name(), String::from("Name"));
        }

        #[ink::test]
        fn set_description_works() {
            let accounts = ink::env::test::default_accounts();
            ink::env::test::set_caller(accounts.alice);
            let mut contract = Storage1::new(String::from("Name"), String::from("OldDescription"));

            assert_eq!(
                contract.set_description(String::from("NewDescription")),
                Ok(())
            );
            assert_eq!(contract.get_description(), String::from("NewDescription"));
        }

        #[ink::test]
        fn transfer_ownership_works() {
            let accounts = ink::env::test::default_accounts();
            ink::env::test::set_caller(accounts.alice);
            let mut contract = Storage1::new(String::from("Name"), String::from("Description"));

            assert_eq!(contract.transfer_ownership(accounts.bob), Ok(()));
            assert_eq!(contract.get_owner(), accounts.bob);

            // Now Bob is owner, Alice should not be able to change name
            assert_eq!(
                contract.set_name(String::from("NewName")),
                Err(Error::NotOwner)
            );

            // But Bob can
            ink::env::test::set_caller(accounts.bob);
            assert_eq!(contract.set_name(String::from("NewName")), Ok(()));
        }

        #[ink::test]
        fn transfer_ownership_fails_not_owner() {
            let accounts = ink::env::test::default_accounts();
            ink::env::test::set_caller(accounts.alice);
            let mut contract = Storage1::new(String::from("Name"), String::from("Description"));

            // Bob tries to transfer ownership (Alice is the owner)
            ink::env::test::set_caller(accounts.bob);
            assert_eq!(
                contract.transfer_ownership(accounts.charlie),
                Err(Error::NotOwner)
            );

            // Owner should still be Alice
            assert_eq!(contract.get_owner(), accounts.alice);
        }
    }
}
