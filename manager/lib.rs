#![cfg_attr(not(feature = "std"), no_std, no_main)]

pub type TokenId = u128;
pub type Result<T> = core::result::Result<T, Error>;
pub type PositionId = u128;

#[derive(Debug, PartialEq, Eq)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
pub enum Error {
    Overflow,
}

#[ink::contract]
mod manager {
    use super::*;
    use ink::storage::Mapping;
    use ink::env::DefaultEnvironment;
    use ink::env::call::{build_call, ExecutionInput, Selector};

    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub enum PositionType {
        LONG,
        SHORT,
    }

    #[ink::scale_derive(Encode, Decode, TypeInfo)]
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
    }

    impl Manager {
        #[ink(constructor, payable)]
        pub fn new(vault_address: AccountId) -> Self {
            let positions = Mapping::default();
            let position_id: PositionId = 0;
            let vault = vault_address;
            Self {
                positions,
                position_id,
                vault,
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
            // TODO: fetch from oracle price and time
            let entry_price: Balance = 100; // TODO: fetch from oracle
            let creation_time = 1000; // TODO: fetch from oracle

            let position_id = self.position_id;
            self.position_id.checked_add(1).ok_or(Error::Overflow)?;

            let new_position: Position = Position {
                state: true,
                token,
                amount,
                position_type,
                leverage,
                entry_price,
                creation_time,
            };

            self.positions.insert((caller, position_id), &new_position);

            let deposit = build_call::<DefaultEnvironment>()
                .call(self.vault)
                .call_v1()
                .gas_limit(0)
                .exec_input(
                    ExecutionInput::new(Selector::new(ink::selector_bytes!("add_liquidity")))
                        .push_arg(token)
                        .push_arg(amount)
                )
                .returns::<bool>()
                .invoke();

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
            let token = self.positions.get((caller, position_id)).unwrap().token;

            self.positions.remove((caller, position_id));

            let deposit = build_call::<DefaultEnvironment>()
            .call(self.vault)
            .call_v1()
            .gas_limit(0)
            .exec_input(
                ExecutionInput::new(Selector::new(ink::selector_bytes!("remove_liquidity")))
                    .push_arg(token)
                    .push_arg(amount)
            )
            .returns::<bool>()
            .invoke();

            self.env().emit_event(PositionClosed {
                from: Some(caller),
                position_id,
            });

            Ok(())
        }
    }
}
