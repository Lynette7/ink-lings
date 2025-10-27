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
    use ink::primitives::H160;
    use ink::storage::Mapping;
    #[ink(storage)]
    pub struct Intro2 {
        counter: u32,
        balances: Mapping<H160, u128>,
    }
    impl Intro2 {
        /// Constructor that initializes the contract state
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                counter: 0,
                balances: Mapping::new(),
            }
        }
        /// Returns the current counter value
        #[ink(message)]
        pub fn get_counter(&self) -> u32 {
            self.counter
        }
        /// Increments the counter value by 1
        #[ink(message)]
        pub fn increment(&mut self) {
            self.counter += 1;
        }
        /// Gets the balance of an account
        #[ink(message)]
        pub fn get_balance(&self) -> u128 {
            let caller = self.env().caller();
            self.balances.get(&caller).unwrap_or(0)
        }

        /// Sets the balance for a caller
        #[ink(message)]
        pub fn set_balance(&mut self, amount: u128) {
            let caller = self.env().caller();

            self.balances.insert(&caller, &amount);
        }
    }
    #[cfg(test)]
    mod tests {
        use super::*;
        #[ink::test]
        fn counter_works() {
            let mut contract = Intro2::new();
            assert_eq!(contract.get_counter(), 0);
            contract.increment();
            assert_eq!(contract.get_counter(), 1);
            contract.increment();
            assert_eq!(contract.get_counter(), 2);
        }
        #[ink::test]
        fn balance_works() {
            let mut contract = Intro2::new();
            assert_eq!(contract.get_balance(), 0);
            contract.set_balance(100);
            assert_eq!(contract.get_balance(), 100);
            contract.set_balance(200);
            assert_eq!(contract.get_balance(), 200);
        }
    }
}
