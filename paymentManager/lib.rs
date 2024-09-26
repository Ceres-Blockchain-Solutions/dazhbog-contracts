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
        vault: AccountId,
    }

    impl PaymentManager {
        #[ink(constructor)]
        pub fn new(vault_address: AccountId) -> Self {
            let vault = vault_address;
            Self { vault }
        }

        #[ink(message)]
        pub fn liquidation(&mut self) {
            //close user position in manager contract
            // // transfer fees to vault
            // let send_fee_to_vault = build_call::<DefaultEnvironment>()
            //     .call(self.erc20)
            //     .call_v1()
            //     .gas_limit(0)
            //     .exec_input(
            //         ExecutionInput::new(Selector::new(ink::selector_bytes!("transfer")))
            //             .push_arg(self.vault)
            //             .push_arg(self.fee),
            //     )
            //     .returns::<bool>()
            //     .invoke();
            //update user balance
        }

        #[ink(message)]
        pub fn collect_fee(&self, user: AccountId) {}

        #[ink(message)]
        pub fn update_position(&self, user: AccountId) {
            //calls collect fee
            //update position in manager contract
            //update vault
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        pub fn contract_creation_works() {
            let vault = AccountId::from([0x1; 32]);

            let mut paymentManager = PaymentManager::new(vault);

            assert_eq!(paymentManager.vault, vault);
        }
    }
}
