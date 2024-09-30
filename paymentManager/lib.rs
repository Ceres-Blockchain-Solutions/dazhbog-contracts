#![cfg_attr(not(feature = "std"), no_std, no_main)]

pub type PositionId = u128;

#[derive(Debug, PartialEq, Eq)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
pub enum Error {
    Overflow,
    NotFound,
}

#[ink::contract]
mod paymentManager {
    use super::*;
    use ink::env::call::{build_call, ExecutionInput, Selector};
    use ink::env::DefaultEnvironment;
    use ink::storage::Mapping;

    #[ink(event)]
    pub struct MaintenanceFeeCollected {
        #[ink(topic)]
        from: Option<AccountId>,
        position_id: PositionId,
        fee: Balance,
    }

    #[ink(storage)]
    pub struct PaymentManager {
        vault: AccountId,
        manager: AccountId,
    }

    impl PaymentManager {
        #[ink(constructor)]
        pub fn new(vault_address: AccountId, manager_address: AccountId) -> Self {
            let vault = vault_address;
            let manager = manager_address;
            Self { vault, manager }
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
        pub fn collect_fee(&self, position_id: PositionId, user: AccountId) {
            let fee = 10; //TODO calculate fee for maintenance position

            // call manager to update position
            // let update_position = build_call::<DefaultEnvironment>()
            //     .call(self.manager)
            //     .call_v1()
            //     .gas_limit(0)
            //     .exec_input(
            //         ExecutionInput::new(Selector::new(ink::selector_bytes!("update_position")))
            //             .push_arg(fee)
            //             .push_arg(position_id),
            //     )
            //     .returns::<bool>()
            //     .invoke();

            self.env().emit_event(MaintenanceFeeCollected {
                from: Some(user),
                position_id,
                fee
            });

            // Ok(())
        }

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
