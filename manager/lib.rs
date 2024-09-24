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
    use ink::{contract_ref, storage::Mapping};

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
    #[derive(Default)]
    pub struct Manager {
        positions: Mapping<(AccountId, PositionId), Position>,
        position_id: PositionId,
    }

    impl Manager {
        #[ink(constructor, payable)]
        pub fn new() -> Self {
            let positions = Mapping::default();
            let position_id: PositionId = 0;
            Self {
                positions,
                position_id,
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

            self.positions.remove((caller, position_id));

            self.env().emit_event(PositionClosed {
                from: Some(caller),
                position_id,
            });

            Ok(())
        }
    }
}
