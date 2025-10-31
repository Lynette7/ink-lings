// intro1
//
// Welcome to Inklings! This is your first ink! smart contract exercise.
//
// ink! is a Rust-based eDSL (embedded Domain Specific Language) for writing
// smart contracts on Polkadot SDK chains. ink! v6 compiles to PolkaVM,
// which is based on RISC-V.
//
// Your task: Fix the compilation errors in this contract.
// The contract should compile successfully when you're done.
//
// Hints:
// - Every ink! contract needs the #[ink::contract] attribute
// - The contract module needs to be public
// - Storage struct needs #[ink(storage)] attribute
//
// Execute `inklings verify intro1` when you think you're done!
#![allow(unexpected_cfgs)]
#[ink::contract]
mod intro1 {
    // TODO: Add the storage attribute here
    // Storage holds the contract's state
    pub struct Intro1 {
        // A simple boolean value
        value: bool,
    }

    impl Intro1 {
        /// Constructor that initializes the contract
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            Self { value: init_value }
        }

        /// Constructor that initializes with `false`
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(false)
        }

        /// A message that returns the stored value
        // TODO: Add the message attribute here
        pub fn get(&self) -> bool {
            self.value
        }

        /// A message that flips the stored value
        #[ink(message)]
        pub fn flip(&mut self) {
            self.value = !self.value;
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn default_works() {
            let intro1 = Intro1::default();
            assert_eq!(intro1.get(), false);
        }

        #[ink::test]
        fn flip_works() {
            let mut intro1 = Intro1::new(false);
            assert_eq!(intro1.get(), false);
            intro1.flip();
            assert_eq!(intro1.get(), true);
        }
    }
}
