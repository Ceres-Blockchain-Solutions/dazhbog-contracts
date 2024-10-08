#![cfg_attr(not(feature = "std"), no_std, no_main)]

pub type PositionId = u128;
pub type TokenId = u128;

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

    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[derive(Debug, PartialEq, Eq)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub enum PositionType {
        LONG,
        SHORT,
    }

    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[derive(Debug, PartialEq, Eq)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub struct Position {
        state: bool,
        token: TokenId,
        amount: Balance,
        position_type: PositionType,
        leverage: u32,
        entry_price: Balance,
        creation_time: u128,
    }

    #[ink(event)]
    pub struct MaintenanceFeeCollected {
        #[ink(topic)]
        from: Option<AccountId>,
        position_id: PositionId,
    }

    #[ink(event)]
    pub struct PositionUpdated {
        #[ink(topic)]
        from: Option<AccountId>,
        position_id: PositionId,
    }

    #[ink(storage)]
    pub struct PaymentManager {
        manager: AccountId,
    }

    impl PaymentManager {
        #[ink(constructor)]
        pub fn new(manager_address: AccountId) -> Self {
            let manager = manager_address;
            Self { manager }
        }
        
        #[ink(message)]
        pub fn update_position(&self, position_id: PositionId, user: AccountId) {
            let position = build_call::<DefaultEnvironment>()
                .call(self.manager)
                .call_v1()
                .gas_limit(0)
                .exec_input(
                    ExecutionInput::new(Selector::new(ink::selector_bytes!("get_position")))
                        .push_arg(user)
                        .push_arg(position_id),
                )
                .returns::<Position>()
                .invoke();

            let fee = 10; //TODO calculate fee for maintenance position
            // FIX: Manager to be contract, not AccountId
            // let position = self.manager.get_position(user, position_id).unwrap();
            
            let check =
                self.check_liquidation(position.amount, position.entry_price, position.leverage, position.position_type);

            if (check) {
                self.liquidation(position_id, user);
            } else {
                self.collect_fee(position_id, user);
            }

            self.env().emit_event(PositionUpdated {
                from: Some(user),
                position_id,
            });
        }
        
        #[ink(message)]
        pub fn liquidation(&self, position_id: PositionId, user: AccountId) {
            //calculate
            //check
            //liquidate if needed
        }

        #[ink(message)]
        pub fn collect_fee(&self, position_id: PositionId, user: AccountId) {
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
            });

            // Ok(())
        }
        
        #[ink(message)]
        pub fn check_liquidation(
            &self,
            amount: Balance,
            entry_price: Balance,
            leverage: u32,
            position_type: PositionType,
        ) -> bool {
            let deposit = amount.wrapping_mul(entry_price as u128);
            let entry_value = deposit.wrapping_mul(leverage as u128);
            //TODO ping oracle for current price
            let current_price = 1000;
            let real_amount_with_leverage = amount.wrapping_mul(leverage as u128);
            let real_value = real_amount_with_leverage.wrapping_mul(current_price);

            match position_type {
                PositionType::LONG => {
                    if (deposit == entry_value.checked_sub(real_value).unwrap()) {
                        true
                    } else {
                        false
                    }
                },
                PositionType::SHORT => {
                    if (deposit == real_value.checked_sub(entry_value).unwrap()) {
                        true
                    } else {
                        false
                    }
                },
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        pub fn contract_creation_works() {
            let manager_address = AccountId::from([0x1; 32]);

            let mut paymentManager = PaymentManager::new(manager_address);

            assert_eq!(paymentManager.manager, manager_address);
        }

        #[ink::test]
        pub fn collect_fee_works() {
            let manager_address = AccountId::from([0x1; 32]);
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            let position_id = 0;

            let mut paymentManager = PaymentManager::new(manager_address);
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);

            paymentManager.collect_fee(position_id, accounts.alice);

            let emitted_events = ink::env::test::recorded_events().collect::<Vec<_>>();
            assert_eq!(emitted_events.len(), 1);
        }

        #[ink::test]
        pub fn update_position_works() {
            let manager_address = AccountId::from([0x1; 32]);
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            let position_id = 0;

            let mut paymentManager = PaymentManager::new(manager_address);
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);

            paymentManager.update_position(position_id, accounts.alice);

            let emitted_events = ink::env::test::recorded_events().collect::<Vec<_>>();
            assert_eq!(emitted_events.len(), 2);
        }
    }
}
