/// The core methods for a basic semi fungible token. Extension standards may be
/// added in addition to this macro.
///

#[macro_export]
macro_rules! impl_multi_token_core {
    ($contract: ident, $token: ident) => {
        use $crate::core::MultiTokenCore;
        use $crate::core::MultiTokenResolver;

        #[near_bindgen]
        impl MultiTokenCore for $contract {
            #[payable]
            fn mt_transfer(
                &mut self,
                receiver_id: near_sdk::AccountId,
                token_id: $crate::TokenId,
                amount: near_sdk::json_types::U128,
                memo: Option<String>,
            ) {
                self.$token.mt_transfer(receiver_id, token_id, amount, memo)
            }

            #[payable]
            fn mt_transfer_call(
                &mut self,
                receiver_id: near_sdk::AccountId,
                token_id: $crate::TokenId,
                amount: near_sdk::json_types::U128,
                memo: Option<String>,
                msg: String,
            ) -> PromiseOrValue<U128> {
                self.$token.mt_transfer_call(receiver_id, token_id, amount, memo, msg)
            }

            #[payable]
            fn mt_batch_transfer(
                &mut self,
                receiver_id: near_sdk::AccountId,
                token_id: Vec<$crate::TokenId>,
                amounts: Vec<U128>,
                memo: Option<String>,
            ) {
                self.$token.mt_batch_transfer(receiver_id, token_id, amounts, memo)
            }

            #[payable]
            fn mt_batch_transfer_call(
                &mut self,
                receiver_id: near_sdk::AccountId,
                token_ids: Vec<$crate::TokenId>,
                amounts: Vec<U128>,
                memo: Option<String>,
                msg: String,
            ) -> PromiseOrValue<Vec<U128>> {
                self.$token.mt_batch_transfer_call(receiver_id, token_ids, amounts, memo, msg)
            }

            fn balance_of(
                &self,
                owner_id: near_sdk::AccountId,
                token_id: $crate::TokenId,
            ) -> near_sdk::json_types::U128 {
                self.$token.balance_of(owner_id, token_id)
            }

            fn balance_of_batch(
                &self,
                owner_id: near_sdk::AccountId,
                token_ids: Vec<$crate::TokenId>,
            ) -> Vec<U128> {
                self.$token.balance_of_batch(owner_id, token_ids)
            }

            fn total_supply(&self, token_id: $crate::TokenId) -> near_sdk::json_types::U128 {
                self.$token.total_supply(token_id)
            }

            fn total_supply_batch(&self, token_ids: Vec<$crate::TokenId>) -> Vec<U128> {
                self.$token.total_supply_batch(token_ids)
            }
        }

        #[near_bindgen]
        impl $crate::core::MultiTokenResolver for $contract {
            #[private]
            fn mt_resolve_transfer(
                &mut self,
                sender_id: near_sdk::AccountId,
                receiver_id: near_sdk::AccountId,
                token_ids: Vec<$crate::TokenId>,
                amounts: Vec<U128>,
            ) -> Vec<U128> {
                self.$token.mt_resolve_transfer(sender_id, receiver_id, token_ids, amounts)
            }
        }
    };
}

/// Ensures that when mt token storage grows by collections adding entries,
/// the storage is be paid by the caller. This ensures that storage cannot grow to a point
/// that the MT contract runs out of Ⓝ.
/// Takes name of the Contract struct, the inner field for the token and optional method name to
/// call when the account was closed.
#[macro_export]
macro_rules! impl_multi_token_storage {
    ($contract: ident, $token: ident $(, $on_account_closed_fn:ident)?) => {
        use $crate::storage_management::{
            StorageManagement, StorageBalance, StorageBalanceBounds
        };

        #[near_bindgen]
        impl StorageManagement for $contract {
            #[payable]
            fn storage_deposit(
                &mut self,
                token_ids: Vec<$crate::TokenId>,
                account_id: Option<near_sdk::AccountId>,
                registration_only: Option<bool>,
            ) -> StorageBalance {
                self.$token.storage_deposit(token_ids, account_id, registration_only)
            }
            #[payable]
            fn storage_withdraw(&mut self, token_ids:Vec<$crate::TokenId>, amount: Option<U128>) -> StorageBalance {
                self.$token.storage_withdraw(token_ids, amount)
            }

            #[payable]
            fn storage_unregister(&mut self, token_ids: Vec<$crate::TokenId>, force: Option<bool>) -> Vec<bool> {
               #[allow(unused_variables)]
               let final_states = self.$token.internal_storage_unregister_batch(token_ids, force);
                final_states.iter().map(|final_state|{
                        if let Some((account_id,balance)) = final_state {
                            $(self.$on_account_closed_fn(account_id, balance);)?
                            true
                        }else {
                            false
                        }
                    }).collect()
            }

            fn storage_balance_bounds(&self, token_ids: Vec<$crate::TokenId>, account_id: Option<AccountId>) -> StorageBalanceBounds {
                self.$token.internal_storage_balance_bounds_batch(&token_ids, account_id)
            }

            fn storage_balance_of(&self, token_ids: Vec<$crate::TokenId>, account_id: near_sdk::AccountId) -> Option<StorageBalance> {
                self.$token.internal_storage_balance_of_batch(&token_ids, &account_id)
            }
        }
    };
}
