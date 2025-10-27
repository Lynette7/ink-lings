// intro2
//
// This exercise will teach you about working with different storage types in ink!
// ink! provides several storage types optimized for different use cases:
// - Simple values (bool, u32, etc)
// - Mappings - for key-value pairs
//
// Your task: Complete the storage struct and implement the missing functions.
// Hints:
// - Use u32 for the counter
// - Use Mapping<AccountId, u128> for per-account balances
// - Remember to initialize Mapping in the constructor
//
// Execute `inklings verify 01_intro/intro2` when you think you're done!
#[ink::contract]
mod intro2 {
    use ink::storage::Mapping;
    #[ink(storage)]
    pub struct Intro2 {
        // TODO: Add a counter field of type u32
        // TODO: Add a balances field of type Mapping<H160, u128>
    }
    impl Intro2 {
        /// Constructor that initializes the contract state
        #[ink(constructor)]
        pub fn new() -> Self {
            // TODO: Initialize the struct with counter = 0
            // TODO: Initialize the mapping
            todo!()
        }
        /// Returns the current counter value
        #[ink(message)]
        pub fn get_counter(&self) -> u32 {
            // TODO: Return the counter value
            todo!()
        }
        /// Increments the counter value by 1
        #[ink(message)]
        pub fn increment(&mut self) {
            // TODO: Increment the counter
            todo!()
        }
        /// Gets the balance of an account
        #[ink(message)]
        pub fn get_balance(&self) -> u128 {
            let caller = self.env().caller();
            // TODO: Get the balance from the mapping
            // default to 0 if the account is not found
            todo!()
        }

        /// Sets the balance for a caller
        #[ink(message)]
        pub fn set_balance(&mut self, balance: u128) {
            let caller = self.env().caller();
            // TODO: Insert an amount into the mapping for the caller
            todo!()
        }
    }
}
