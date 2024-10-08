#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod oracle {
    #[ink(storage)]
    pub struct Oracle {
        /// Stores a single `bool` value on the storage.
        price: Balance,
    }

    impl Oracle {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(price: Balance) -> Self {
            Self { price }
        }

        pub fn change_price(&mut self, new_price: Balance) {
            self.price = new_price;
        }

        #[ink(message)]
        pub fn get_price(&mut self) -> Balance{
            self.price
        }
    }

    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        #[ink::test]
        fn create_oracle_works() {
            let oracle_price = 1000;
            let mut oracle = Oracle::new(oracle_price);
            assert_eq!(oracle.get_price(), oracle_price);
        }

        #[ink::test]
        fn change_price_works() {
            let oracle_price = 1000;
            let new_price = 1200;

            let mut oracle = Oracle::new(oracle_price);
            
            oracle.change_price(1200);
            assert_eq!(oracle.get_price(), new_price);
        }
    }

}
