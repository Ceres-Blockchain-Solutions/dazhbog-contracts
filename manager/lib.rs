#![cfg_attr(not(feature = "std"), no_std, no_main)]

pub type TokenId = u128;
pub type Result<T> = core::result::Result<T, Error>;
pub type PositionId = u128;

#[derive(Debug, PartialEq, Eq)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
pub enum Error {
    Overflow,
    Underflow,
    NotFound,
}

#[ink::contract]
mod manager {
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
    pub struct PositionOpened {
        #[ink(topic)]
        from: Option<AccountId>,
        position_id: PositionId,
        #[ink(topic)]
        amount: Balance,
    }

    #[ink(event)]
    pub struct PositionClosed {
        #[ink(topic)]
        from: Option<AccountId>,
        position_id: PositionId,
    }

    #[ink(storage)]
    pub struct Manager {
        positions: Mapping<(AccountId, PositionId), Position>,
        position_id: PositionId,
        vault: AccountId,
        fee: Balance,
        erc20: AccountId,
    }

    impl Manager {
        #[ink(constructor, payable)]
        pub fn new(vault_address: AccountId, fee: Balance, erc20: AccountId) -> Self {
            let positions = Mapping::default();
            let position_id: PositionId = 0;
            let vault = vault_address;
            Self {
                positions,
                position_id,
                vault,
                fee,
                erc20,
            }
        }

        #[ink(message)]
        pub fn open_position(
            &mut self,
            token: TokenId,
            amount: Balance,
            position_type: PositionType,
            leverage: u32,
        ) -> Result<()> {
            let caller = self.env().caller();
            // TODO: fetch from oracle price
            let entry_price: Balance = 100; // TODO: fetch from oracle
            let creation_time = self.env().block_timestamp().into();

            let position_id = self.position_id;
            self.position_id.checked_add(1).ok_or(Error::Overflow)?;

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

            let updated_amount = amount.checked_sub(self.fee).ok_or(Error::Underflow)?;

            let new_position: Position = Position {
                state: true,
                token,
                amount: updated_amount,
                position_type,
                leverage,
                entry_price,
                creation_time,
            };

            self.positions.insert((caller, position_id), &new_position);

            // let deposit = build_call::<DefaultEnvironment>()
            //     .call(self.vault)
            //     .call_v1()
            //     .gas_limit(0)
            //     .exec_input(
            //         ExecutionInput::new(Selector::new(ink::selector_bytes!("add_liquidity")))
            //             .push_arg(token)
            //             .push_arg(updated_amount),
            //     )
            //     .returns::<bool>()
            //     .invoke();

            self.env().emit_event(PositionOpened {
                from: Some(caller),
                position_id,
                amount,
            });

            Ok(())
        }

        #[ink(message)]
        pub fn close_position(&mut self, position_id: PositionId) -> Result<()> {
            let caller = self.env().caller();

            let amount = self.positions.get((caller, position_id)).unwrap().amount;

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

            self.positions.remove((caller, position_id));

            let updated_amount = amount.checked_sub(self.fee).ok_or(Error::Underflow)?;

            // let withdraw = build_call::<DefaultEnvironment>()
            //     .call(self.vault)
            //     .call_v1()
            //     .gas_limit(0)
            //     .exec_input(
            //         ExecutionInput::new(Selector::new(ink::selector_bytes!("remove_liquidity")))
            //             .push_arg(token)
            //             .push_arg(updated_amount),
            //     )
            //     .returns::<bool>()
            //     .invoke();

            self.env().emit_event(PositionClosed {
                from: Some(caller),
                position_id,
            });

            Ok(())
        }

        #[ink(message)]
        pub fn get_position(
            &self,
            account: AccountId,
            position_id: PositionId,
        ) -> Result<Position> {
            let temp = self.positions.get((account, position_id));

            if temp.is_some() {
                Ok(temp.unwrap())
            } else {
                Err(Error::NotFound)
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        pub fn open_position_works() {
            let token = 1;
            let position_id = 0;
            let amount = 100;
            let leverage = 10;
            let fee = 10;
            let erc20 = AccountId::from([0x0; 32]);
            let vault = AccountId::from([0x1; 32]);
            let mut manager = Manager::new(vault, fee, erc20);
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);

            assert_eq!(
                manager.open_position(token, amount, PositionType::LONG, leverage),
                Ok(())
            );

            let position = manager.get_position(accounts.alice, position_id).unwrap();

            assert_eq!(position.token, token);
            assert_eq!(position.amount, amount - fee);
            assert_eq!(position.leverage, leverage);

            let emitted_events = ink::env::test::recorded_events().collect::<Vec<_>>();
            assert_eq!(emitted_events.len(), 1);
        }

        #[ink::test]
        pub fn close_position_works() {
            let token = 1;
            let position_id = 0;
            let amount = 100;
            let leverage = 10;
            let fee = 10;
            let erc20 = AccountId::from([0x0; 32]);
            let vault = AccountId::from([0x1; 32]);
            let mut manager = Manager::new(vault, fee, erc20);
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
            let res = manager.open_position(token, amount, PositionType::LONG, leverage);

            let res = manager.close_position(position_id);

            let emitted_events = ink::env::test::recorded_events().collect::<Vec<_>>();
            assert_eq!(emitted_events.len(), 2);

            let position = manager.get_position(accounts.alice, position_id);

            assert_eq!(position.unwrap_err(), Error::NotFound);
        }

        #[ink::test]
        pub fn contract_creation_works() {
            let position_id = 0;
            let fee = 10;
            let erc20 = AccountId::from([0x0; 32]);
            let vault = AccountId::from([0x1; 32]);
            let mut manager = Manager::new(vault, fee, erc20);

            assert_eq!(manager.position_id, position_id);
            assert_eq!(manager.vault, vault);
        }
    }
}
