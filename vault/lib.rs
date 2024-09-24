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
mod vault {
    use super::*;
    use ink::{contract_ref, storage::Mapping};

    #[ink(event)]
    pub struct AddLiquidity {
        #[ink(topic)]
        from: Option<AccountId>,
        token: TokenId,
        #[ink(topic)]
        amount: Balance,
    }

    #[ink(event)]
    pub struct WithdrawLiquidity {
        #[ink(topic)]
        from: Option<AccountId>,
        token: TokenId,
        #[ink(topic)]
        amount: Balance,
    }

    #[ink(storage)]
    #[derive(Default)]
    pub struct Vault {
        contributors: Mapping<(AccountId, TokenId), Balance>,
    }

    impl Vault {
        #[ink(constructor, payable)]
        pub fn new() -> Self {
            let contributors = Mapping::default();
            Self {
                contributors,
            }
        }

        #[ink(message)]
        pub fn add_liquidity(
            &mut self,
            token: TokenId,
            amount: Balance,
        ) -> Result<()> {
            let caller = self.env().caller();

            self.contributors.insert((caller, token), &amount);

            self.env().emit_event(AddLiquidity {
                from: Some(caller),
                token,
                amount,
            });

            Ok(())
        }

        #[ink(message)]
        pub fn remove_liquidity(
            &mut self,
            token: TokenId,
            amount: Balance,
        ) -> Result<()> {
            let caller = self.env().caller();

            self.contributors.remove((caller, token));

            self.env().emit_event(WithdrawLiquidity {
                from: Some(caller),
                token,
                amount,
            });

            Ok(())
        }

    }
}
