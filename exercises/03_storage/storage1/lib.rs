// storage1
//
// This exercise teaches you about Lazy<T> storage in ink!
//
// Lazy<T> is useful for storing large values that aren't accessed frequently.
// Unlike regular storage fields which are loaded automatically, Lazy<T> values
// are only loaded when explicitly accessed, saving gas.
//
// Use cases:
// - Contract metadata (name, symbol, description)
// - Admin addresses that rarely change
// - Large configuration objects
//
// Execute `inklings verify 03_storage/storage1` when you think you're done!
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
        // TODO: Add a Lazy<String> field called 'name'
        // TODO: Add a Lazy<String> field called 'description'
        // TODO: Add a regular H160 Account field called 'owner'
        todo!()
    }
    type Result<T> = core::result::Result<T, Error>;

    impl Storage1 {
        /// Constructor that initializes the contract with metadata
        #[ink(constructor)]
        pub fn new(name: String, description: String) -> Self {
            let caller = Self::env().caller();

            // TODO: Initialize the struct with:
            // - name: Lazy::new()
            // - description: Lazy::new()
            // - owner: caller
            // Store the instance in a mutable variable

            // TODO: After creating the instance, use .set() to initialize the Lazy values
            // - instance.name.set(&name);
            // - instance.description.set(&description);

            // TODO: Return the instance
            todo!()
        }

        /// Get the contract name
        #[ink(message)]
        pub fn get_name(&self) -> String {
            // TODO: Get the value from the Lazy field
            // Hint: self.name.get().unwrap_or_default()
            // The .get() returns Option<String>, so no need to clone it
            todo!()
        }

        /// Get the contract description
        #[ink(message)]
        pub fn get_description(&self) -> String {
            // TODO: Get the value from the Lazy field
            todo!()
        }

        /// Update the contract name (only owner can do this)
        #[ink(message)]
        pub fn set_name(&mut self, new_name: String) -> Result<()> {
            let caller = self.env().caller();

            // TODO: Check if caller is the owner
            // If not, return Error::NotOwner

            // TODO: Set the new name using self.name.set(&new_name)
            todo!()
        }

        /// Update the contract description (only owner can do this)
        #[ink(message)]
        pub fn set_description(&mut self, new_description: String) -> Result<()> {
            let caller = self.env().caller();

            // TODO: Check if caller is the owner
            // If not, return Error::NotOwner

            // TODO: Set the new description
            todo!()
        }

        /// Get the owner address
        #[ink(message)]
        pub fn get_owner(&self) -> H160 {
            // TODO: Return the owner field
            todo!()
            
        }

        /// Transfer ownership (only current owner can do this)
        #[ink(message)]
        pub fn transfer_ownership(&mut self, new_owner: H160) -> Result<()> {
            let caller = self.env().caller();

            // TODO: Check if caller is the current owner
            // If not, return Error::NotOwner

            // TODO: Update the owner field to new_owner
            todo!()
           
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
