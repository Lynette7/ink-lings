#![allow(unexpected_cfgs)]
#[ink::contract]
mod basic1 {
    /// Custom error types for the contract
    #[derive(Debug, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub enum Error {
        /// Overflow occurred during arithmetic operation
        Overflow,
        /// Underflow occurred during arithmetic operation
        Underflow,
        /// Division by zero attempted
        DivisionByZero,
    }
    pub type Result<T> = core::result::Result<T, Error>;
    #[ink(storage)]
    pub struct Basic1 {
        value: u32,
    }
    impl Basic1 {
        /// Constructor that initializes the value
        #[ink(constructor)]
        pub fn new(init_value: u32) -> Self {
            Self { value: init_value }
        }
        /// Adds a value using checked arithmetic
        #[ink(message)]
        pub fn add(&mut self, amount: u32) -> Result<()> {
            self.value = self.value.checked_add(amount).ok_or(Error::Overflow)?;
            Ok(())
        }

        /// Subtracts a value using checked arithmetic
        #[ink(message)]
        pub fn subtract(&mut self, amount: u32) -> Result<()> {
            self.value = self.value.checked_sub(amount).ok_or(Error::Underflow)?;
            Ok(())
        }

        /// Multiplies the value using checked arithmetic
        #[ink(message)]
        pub fn multiply(&mut self, factor: u32) -> Result<()> {
            self.value = self.value.checked_mul(factor).ok_or(Error::Overflow)?;
            Ok(())
        }

        /// Divides the value using checked arithmetic
        #[ink(message)]
        pub fn divide(&mut self, divisor: u32) -> Result<()> {
            if divisor == 0 {
                return Err(Error::DivisionByZero);
            }
            self.value.checked_div(divisor).ok_or(Error::Overflow)?;
            Ok(())
        }
        // Getter function to retrieve the current value
        #[ink(message)]
        pub fn get_value(&self) -> u32 {
            self.value
        }
    }
    #[cfg(test)]
    mod tests {
        use super::*;
        #[ink::test]
        fn test_add() {
            let contract = Basic1::new(10);
            assert_eq!(contract.get_value(), 10);
        }
        #[ink::test]
        fn add_works() {
            let mut contract = Basic1::new(10);
            assert_eq!(contract.add(5), Ok(()));
            assert_eq!(contract.get_value(), 15);
        }
        #[ink::test]
        fn add_overflows() {
            let mut contract = Basic1::new(u32::MAX);
            assert_eq!(contract.add(1), Err(Error::Overflow));
            assert_eq!(contract.get_value(), u32::MAX);
        }
        #[ink::test]
        fn subtract_works() {
            let mut contract = Basic1::new(10);
            assert_eq!(contract.subtract(3), Ok(()));
            assert_eq!(contract.get_value(), 7);
        }

        #[ink::test]
        fn subtract_underflow() {
            let mut contract = Basic1::new(5);
            assert_eq!(contract.subtract(10), Err(Error::Underflow));
            assert_eq!(contract.get_value(), 5);
        }

        #[ink::test]
        fn multiply_works() {
            let mut contract = Basic1::new(10);
            assert_eq!(contract.multiply(3), Ok(()));
            assert_eq!(contract.get_value(), 30);
        }

        #[ink::test]
        fn multiply_overflow() {
            let mut contract = Basic1::new(u32::MAX);
            assert_eq!(contract.multiply(2), Err(Error::Overflow));
        }

        #[ink::test]
        fn divide_works() {
            let mut contract = Basic1::new(20);
            assert_eq!(contract.divide(4), Ok(()));
            assert_eq!(contract.get_value(), 5);
        }

        #[ink::test]
        fn divide_by_zero() {
            let mut contract = Basic1::new(20);
            assert_eq!(contract.divide(0), Err(Error::DivisionByZero));
            assert_eq!(contract.get_value(), 20);
        }
    }
}
