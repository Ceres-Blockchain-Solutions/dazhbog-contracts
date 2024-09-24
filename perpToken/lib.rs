#![cfg_attr(not(feature = "std"), no_std, no_main)]

pub use self::perpToken::{PerpToken, PerpTokenRef};

#[openbrush::implementation(PSP37, PSP37Mintable)]
#[openbrush::contract]
pub mod perpToken {
    use openbrush::traits::Storage;

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct PerpToken {
        #[storage_field]
        psp37: psp37::Data,
    }

    impl PerpToken {
        #[ink(constructor)]
        pub fn new() -> Self {
            let mut _instance = Self::default();
            _instance
        }
    }
}
