#![cfg_attr(not(feature = "std"), no_std, no_main)]

pub use self::paymentToken::{PaymentToken, PaymentTokenRef};

#[openbrush::implementation(PSP22)]
#[openbrush::contract]
pub mod paymentToken {
    use openbrush::traits::Storage;

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct PaymentToken {
        #[storage_field]
        psp22: psp22::Data,
    }

    impl PaymentToken {
        #[ink(constructor)]
        pub fn new(initial_supply: Balance) -> Self {
            let mut _instance = Self::default();
            _instance
        }
    }
}
