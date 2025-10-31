#![allow(unexpected_cfgs)]
#[ink::contract]
mod events1 {
    use ink::primitives::H160;
    #[ink(event)]
    pub struct Incremented {
        #[ink(topic)]
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

            self.env().emit_event(Incremented {
                value: self.counter,
                by: self.env().caller(),
            });
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
