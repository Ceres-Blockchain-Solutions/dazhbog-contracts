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
            Self { contributors }
        }

        #[ink(message)]
        pub fn add_liquidity(&mut self, token: TokenId, amount: Balance) -> Result<()> {
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
        pub fn remove_liquidity(&mut self, token: TokenId, amount: Balance) -> Result<()> {
            let caller = self.env().caller();

            self.contributors.remove((caller, token));

            self.env().emit_event(WithdrawLiquidity {
                from: Some(caller),
                token,
                amount,
            });

            Ok(())
        }

        #[ink(message)]
        pub fn get_contributor_balance(&self, account: AccountId, token: TokenId) -> Balance {
            self.contributors.get(&(account, token)).unwrap_or_default()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        pub fn add_liquidity_works() {
            let mut vault = Vault::new();
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
            vault.add_liquidity(123, 1);

            let emitted_events = ink::env::test::recorded_events().collect::<Vec<_>>();
            assert_eq!(emitted_events.len(), 1);

            assert_eq!(vault.get_contributor_balance(accounts.alice, 123), 1);
        }

        #[ink::test]
        pub fn remove_liquidity_works() {
            let mut vault = Vault::new();
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
            vault.add_liquidity(123, 1);

            vault.remove_liquidity(123, 1);

            let emitted_events = ink::env::test::recorded_events().collect::<Vec<_>>();
            assert_eq!(emitted_events.len(), 2);

            assert_eq!(vault.get_contributor_balance(accounts.alice, 123), 0);
        }
    }
}
