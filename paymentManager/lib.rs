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

    #[ink(event)]
    pub struct PositionUpdated {
        #[ink(topic)]
        from: Option<AccountId>,
        position_id: PositionId,
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
        pub fn collect_fee(&self, position_id: PositionId, user: AccountId, fee: Balance) {
            // // call manager to update position
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
                fee,
            });

            // Ok(())
        }

        #[ink(message)]
        pub fn update_position(&self, position_id: PositionId, user: AccountId) {
            let fee = 10; //TODO calculate fee for maintenance position
                          // FIX: Manager to be contract, not AccountId
                          // let position = self.manager.get_position(user, position_id).unwrap();

            //calls collect fee
            self.collect_fee(position_id, user, fee);

            // update position in manager contract
            // let update_position_manager = build_call::<DefaultEnvironment>()
            //     .call(self.manager)
            //     .call_v1()
            //     .gas_limit(0)
            //     .exec_input(
            //         ExecutionInput::new(Selector::new(ink::selector_bytes!("update_position")))
            //             .push_arg(fee)
            //             .push_arg(position_id)
            //             .push_arg(user),
            //     )
            //     .returns::<bool>()
            //     .invoke();

            // // update vault
            // let update_vault = build_call::<DefaultEnvironment>()
            //     .call(self.vault)
            //     .call_v1()
            //     .gas_limit(0)
            //     .exec_input(
            //         ExecutionInput::new(Selector::new(ink::selector_bytes!("update_liquidity")))
            //             .push_arg(position.token)
            //             .push_arg(position.amount - fee)
            //             .push_arg(user)
            //     )
            //     .returns::<bool>()
            //     .invoke();

            self.env().emit_event(PositionUpdated {
                from: Some(user),
                position_id,
            });
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        pub fn contract_creation_works() {
            let vault_address = AccountId::from([0x1; 32]);
            let manager_address = AccountId::from([0x1; 32]);

            let mut paymentManager = PaymentManager::new(vault_address, manager_address);

            assert_eq!(paymentManager.vault, vault_address);
            assert_eq!(paymentManager.manager, manager_address);
        }

        #[ink::test]
        pub fn collect_fee_works() {
            let vault_address = AccountId::from([0x1; 32]);
            let manager_address = AccountId::from([0x1; 32]);
            let fee = 10;
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            let position_id = 0;

            let mut paymentManager = PaymentManager::new(vault_address, manager_address);
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);

            paymentManager.collect_fee(position_id, accounts.alice, fee);

            let emitted_events = ink::env::test::recorded_events().collect::<Vec<_>>();
            assert_eq!(emitted_events.len(), 1);
        }

        #[ink::test]
        pub fn update_position_works() {
            let vault_address = AccountId::from([0x1; 32]);
            let manager_address = AccountId::from([0x1; 32]);
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            let position_id = 0;

            let mut paymentManager = PaymentManager::new(vault_address, manager_address);
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);

            paymentManager.update_position(position_id, accounts.alice);

            let emitted_events = ink::env::test::recorded_events().collect::<Vec<_>>();
            assert_eq!(emitted_events.len(), 2);
        }
    }
}
