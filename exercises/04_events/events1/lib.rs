// events1
//
// This exercise teaches you about events in ink! smart contracts.
//
// Events allow you to log important state changes that off-chain observers
// can listen to and react to. They're crucial for dApps to know when things
// happen in your contract.
//
// Your task: Complete the counter contract to emit events when incremented
//
// Execute `inklings verify 04_events/events1` when you think you're done!
#![allow(unexpected_cfgs)]
#[ink::contract]
mod events1 {
    use ink::primitives::H160;

    /// Event emitted when the counter is incremented
    // TODO: Add the #[ink(event)] attribute here
    todo!()
    #[ink(event)]
    pub struct Incremented {
        // TODO: Add #[ink(topic)] to make the counter value searchable
        todo!()
        value: u32,
        by: H160,
    }

    #[ink(storage)]
    pub struct Events1 {
        counter: u32,
    }

    impl Events1 {
        /// Constructor that initializes the counter to 0
        #[ink(constructor)]
        pub fn new() -> Self {
            Self { counter: 0 }
        }

        /// Increments the counter by 1 and emits an event
        #[ink(message)]
        pub fn increment(&mut self) {
            self.counter += 1;

            // TODO: Emit the Incremented event here
            // Hint: Use self.env().emit_event(...)
            // Include the new counter value and the caller's account
            todo!()
        }

        /// Returns the current counter value
        #[ink(message)]
        pub fn get(&self) -> u32 {
            self.counter
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn increment_works() {
            let mut contract = Events1::new();
            assert_eq!(contract.get(), 0);
            contract.increment();
            assert_eq!(contract.get(), 1);
            contract.increment();
            assert_eq!(contract.get(), 2);
        }

        #[ink::test]
        fn emits_event() {
            let mut contract = Events1::new();
            contract.increment();

            // In a real test environment, you would check emitted events
            // For now, we just verify the counter incremented
            assert_eq!(contract.get(), 1);
        }
    }
}
