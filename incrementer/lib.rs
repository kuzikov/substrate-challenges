#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod incrementer {

    #[ink(storage)]
    pub struct Incrementer {
        // ACTION: Create a storage value called `value` which holds a `i32`
        value: i32,
        h_map: ink_storage::collections::HashMap<AccountId, i32>,
    }

    impl Incrementer {
        #[ink(constructor)]
        pub fn new(init_value: i32) -> Self {
            // ACTION: Create a new `Incrementer` and set its `value` to `init_value`
            Self {
                value: init_value,
                h_map: ink_storage::collections::HashMap::new(),
            }
        }

        // ACTION: Create a second constructor function named `default`.
        //         It has no input, and creates a new `Incrementer` with its `value`
        //         set to `0`.
        #[ink(constructor)]
        pub fn default() -> Self {
            // ACTION: Create a new `Incrementer` and set its `value` to `init_value`
            Self {
                value: 0,
                h_map: Default::default(),
            }
        }

        #[ink(message)]
        pub fn get(&self) -> i32 {
            // Contract Message
            self.value
        }

        // increment fn
        #[ink(message)]
        pub fn inc(&mut self, by: i32) {
            // ACTION: Simply increment `value` by `by`
            self.value = self.value + by;
        }

        #[ink(message)]
        pub fn get_mine(&self) -> i32 {
            // ACTION: Get `my_value` using `my_value_or_zero` on `&self.env().caller()`
            // ACTION: Return `my_value`
            self.my_value_or_zero(&self.env().caller())
        }

        fn my_value_or_zero(&self, of: &AccountId) -> i32 {
            // ACTION: `get` and return the value of `of` and `unwrap_or` return 0
            *self.h_map.get(of).unwrap_or(&0)
        }

        #[ink(message)]
        pub fn inc_mine(&mut self, by: i32) {
            // ACTION: Get the `caller` of this function.
            // ACTION: Get `my_value` that belongs to `caller` by using `my_value_or_zero`.
            // ACTION: Insert the incremented `value` back into the mapping.

            let caller = self.env().caller();
            let num = self.my_value_or_zero(&caller);
            self.h_map.insert(caller, num + by);
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        use ink_lang as ink;

        #[ink::test]
        fn default_works() {
            let contract = Incrementer::default();
            assert_eq!(contract.get(), 0);
        }

        #[ink::test]
        fn it_works() {
            let mut contract = Incrementer::new(42);
            assert_eq!(contract.get(), 42);
            contract.inc(5);
            assert_eq!(contract.get(), 47);
            contract.inc(-50);
            assert_eq!(contract.get(), -3);
        }

        #[ink::test]
        fn my_value_works() {
            let mut contract = Incrementer::new(11);
            assert_eq!(contract.get(), 11);
            assert_eq!(contract.get_mine(), 0);
            contract.inc_mine(5);
            assert_eq!(contract.get_mine(), 5);
            contract.inc_mine(10);
            assert_eq!(contract.get_mine(), 15);
        }
    }
}
