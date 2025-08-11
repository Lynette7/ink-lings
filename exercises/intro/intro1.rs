//! inklings! introduction exercise 1
//! 
//! This is a simple, working ink! smart contract.
//! 
//! To pass this exercise, you just need to ensure it compiles correctly.
//! 
//! To fix the next exercise, you'll need to know that smart contracts
//! are defined within a `contract` module and have a `#[ink(storage)]`
//! struct to hold the contract's state.`

// The `ink_lang` attribute is required for ink! contracts.
#[ink::contract]

/// Defines the storage for your contract
/// A contract's state is stored in this struct
#[ink(storage)]
pub struct Intro1 {
    /// Stores a single boolean value.
    value: bool,
}

impl Intro1 {
    /// Constructor that initializes the contract with a default value.
    #[ink(constructor)]
    pub fn new() -> Self {
        Self { value: true }
    }

    /// A message that can be called from outside the contract.
    /// This one just returns the stored value.
    #[ink(message)]
    pub fn get_value(&self) -> bool {
        self.value
    }

    /// Another message that flips the stored value.
    #[ink(message)]
    pub fn flip(&mut self) {
        self.value = !self.value;
    }
}
