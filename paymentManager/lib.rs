#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[derive(Debug, PartialEq, Eq)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
pub enum Error {
    Overflow,
    NotFound,
}

#[ink::contract]
mod paymentManager {

    #[ink(storage)]
    pub struct PaymentManager {
        temp: Balance,        
    }

    impl PaymentManager {
        #[ink(constructor)]
        pub fn new() -> Self {
            let temp = 0;
            Self { temp }
        }

        #[ink(message)]
        pub fn liquidation(&mut self) {
            //close user position in manager contract
            //update user balance 
        }

        #[ink(message)]
        pub fn collect_fee(&self, user: AccountId) {
            
        }

        #[ink(message)]
        pub fn update_position(&self, user: AccountId) {
            //calls collect fee
            //update position in manager contract
            //update vault 
        }
    }
}
