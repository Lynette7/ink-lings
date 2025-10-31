#![allow(unexpected_cfgs)]
#[ink::contract]
mod intro1 {
    #[ink(storage)]
    pub struct Intro1 {
        value: bool,
    }

    impl Intro1 {
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            Self { value: init_value }
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(false)
        }

        #[ink(message)]
        pub fn get(&self) -> bool {
            self.value
        }

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
