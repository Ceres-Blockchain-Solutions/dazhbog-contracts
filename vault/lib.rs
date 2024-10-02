#![cfg_attr(not(feature = "std"), no_std, no_main)]

pub type TokenId = u128;
pub type Result<T> = core::result::Result<T, Error>;
pub type PositionId = u128;

#[derive(Debug, PartialEq, Eq)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
pub enum Error {
    Overflow,
    Underflow,
    ZeroAmount,
    NonZeroAmount,
}

#[ink::contract]
mod vault {
    use super::*;
    use ink::env::call::{build_call, ExecutionInput, Selector};
    use ink::env::DefaultEnvironment;
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
    pub struct UpdateLiquidity {
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
    pub struct Vault {
        contributors: Mapping<(AccountId, TokenId), Balance>,
        erc20contract: AccountId,
    }

    impl Vault {
        #[ink(constructor)]
        pub fn new(erc20_contract_address: AccountId) -> Self {
            let contributors = Mapping::default();
            let erc20contract = erc20_contract_address;
            Self {
                contributors,
                erc20contract,
            }
        }

        #[ink(message)]
        pub fn add_liquidity(
            &mut self,
            token: TokenId,
            amount: Balance,
            user: AccountId,
        ) -> Result<()> {
            let current_amount = self.contributors.get(&(user, token)).unwrap_or_default();

            if current_amount > 0 {
                return Err(Error::NonZeroAmount);
            }

            self.contributors.insert((user, token), &amount);

            let deposit = build_call::<DefaultEnvironment>()
                .call(self.erc20contract)
                .call_v1()
                .gas_limit(0)
                .exec_input(
                    ExecutionInput::new(Selector::new(ink::selector_bytes!("transfer_from")))
                        .push_arg(self.env().caller())
                        .push_arg(self.env().account_id())
                        .push_arg(amount)
                )
                .returns::<bool>()
                .invoke();

            self.env().emit_event(AddLiquidity {
                from: Some(user),
                token,
                amount,
            });

            Ok(())
        }

        #[ink(message)]
        pub fn update_liquidity(
            &mut self,
            token: TokenId,
            new_amount: Balance,
            user: AccountId,
        ) -> Result<()> {
            let amount = self.contributors.get(&(user, token)).unwrap_or_default();

            if amount == 0 {
                return Err(Error::ZeroAmount);
            }

            let new_amount_final = new_amount.checked_add(amount).ok_or(Error::Overflow)?;

            self.contributors.insert((user, token), &new_amount_final);

            let deposit = build_call::<DefaultEnvironment>()
                .call(self.erc20contract)
                .call_v1()
                .gas_limit(0)
                .exec_input(
                    ExecutionInput::new(Selector::new(ink::selector_bytes!("transfer_from")))
                        .push_arg(self.env().caller())
                        .push_arg(self.env().account_id())
                        .push_arg(new_amount_final),
                )
                .returns::<bool>()
                .invoke();

            self.env().emit_event(UpdateLiquidity {
                from: Some(user),
                token,
                amount: new_amount,
            });

            Ok(())
        }

        #[ink(message)]
        pub fn remove_liquidity(&mut self, token: TokenId, user: AccountId) -> Result<()> {
            let current_amount = self.contributors.get(&(user, token)).unwrap_or_default();

            if current_amount == 0 {
                return Err(Error::ZeroAmount);
            }

            self.contributors.remove((user, token));

            let withdraw = build_call::<DefaultEnvironment>()
                .call(self.erc20contract)
                .call_v1()
                .gas_limit(0)
                .exec_input(
                    ExecutionInput::new(Selector::new(ink::selector_bytes!("transfer")))
                        .push_arg(self.env().caller())
                        .push_arg(current_amount)
                )
                .returns::<bool>()
                .invoke();

            self.env().emit_event(WithdrawLiquidity {
                from: Some(user),
                token,
                amount: current_amount,
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
            let erc20 = AccountId::from([0x0; 32]);
            let mut vault = Vault::new(erc20);
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
            assert_eq!(vault.add_liquidity(123, 1, accounts.alice), Ok(()));

            let emitted_events = ink::env::test::recorded_events().collect::<Vec<_>>();
            assert_eq!(emitted_events.len(), 1);

            assert_eq!(vault.get_contributor_balance(accounts.alice, 123), 1);
        }

        #[ink::test]
        pub fn add_liquidity_fails() {
            let erc20 = AccountId::from([0x0; 32]);
            let mut vault = Vault::new(erc20);
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
            assert_eq!(vault.add_liquidity(123, 1, accounts.alice), Ok(()));

            assert_eq!(
                vault.add_liquidity(123, 1, accounts.alice),
                Err(Error::NonZeroAmount)
            );

            let emitted_events = ink::env::test::recorded_events().collect::<Vec<_>>();
            assert_eq!(emitted_events.len(), 1);
        }

        #[ink::test]
        pub fn update_liquidity_works() {
            let erc20 = AccountId::from([0x0; 32]);
            let mut vault = Vault::new(erc20);
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
            vault.add_liquidity(123, 1, accounts.alice);
            vault.update_liquidity(123, 2, accounts.alice);

            let emitted_events = ink::env::test::recorded_events().collect::<Vec<_>>();
            assert_eq!(emitted_events.len(), 2);

            assert_eq!(vault.get_contributor_balance(accounts.alice, 123), 3);
        }

        #[ink::test]
        pub fn update_liquidity_zero_amount_fails() {
            let erc20 = AccountId::from([0x0; 32]);
            let mut vault = Vault::new(erc20);
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
            assert_eq!(
                vault.update_liquidity(123, 1, accounts.alice),
                Err(Error::ZeroAmount)
            );

            let emitted_events = ink::env::test::recorded_events().collect::<Vec<_>>();
            assert_eq!(emitted_events.len(), 0);
        }

        #[ink::test]
        pub fn remove_liquidity_works() {
            let erc20 = AccountId::from([0x0; 32]);
            let mut vault = Vault::new(erc20);
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
            assert_eq!(
                vault.remove_liquidity(123, accounts.alice),
                Err(Error::ZeroAmount)
            );

            let emitted_events = ink::env::test::recorded_events().collect::<Vec<_>>();
            assert_eq!(emitted_events.len(), 0);
        }

        #[ink::test]
        pub fn remove_liquidity_fails() {
            let erc20 = AccountId::from([0x0; 32]);
            let mut vault = Vault::new(erc20);
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
            vault.add_liquidity(123, 1, accounts.alice);

            vault.remove_liquidity(123, accounts.alice);

            let emitted_events = ink::env::test::recorded_events().collect::<Vec<_>>();
            assert_eq!(emitted_events.len(), 2);

            assert_eq!(vault.get_contributor_balance(accounts.alice, 123), 0);
        }

        #[ink::test]
        pub fn contract_creation_works() {
            let erc20 = AccountId::from([0x0; 32]);
            let mut vault = Vault::new(erc20);

            assert_eq!(vault.erc20contract, erc20);
        }
    }
}
