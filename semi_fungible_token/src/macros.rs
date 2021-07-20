/// The core methods for a basic fungible token. Extension standards may be
/// added in addition to this macro.
/// 
use near_sdk::json_types::{ValidAccountId, U128};
#[macro_export]
macro_rules! impl_semi_fungible_token_core {
    ($contract: ident, $token: ident $(, $on_tokens_burned_fn:ident)?) => {
        use $crate::semi_fungible_token::core::SemiFungibleTokenCore;
        use $crate::semi_fungible_token::resolver::SemiFungibleTokenResolver;

        #[near_bindgen]
        impl SemiFungibleTokenCore for $contract {
            #[payable]
            fn sft_transfer(
                &mut self,
                receiver_id: ValidAccountId,
                token_id: TokenId,
                amount: U128,
                memo: Option<String>,
            ) {
                self.$token.sft_transfer(receiver_id, token_id, amount, memo)
            }

            #[payable]
            fn sft_transfer_call(
                &mut self,
                receiver_id: ValidAccountId,
                token_id: TokenId,
                amount: U128,
                memo: Option<String>,
                msg: String,
            ) -> PromiseOrValue<bool> {
                self.$token.sft_transfer_call(receiver_id, token_id, amount, memo, msg)
            }

            fn sft_batch_transfer(&mut self,
              receiver_id: ValidAccountId,
              token_id: Vec<TokenId>,
              amounts: Vec<U128>,
              memo: Option<String>,
            ) {
              self.$token.sft_batch_transfer(receiver_id,token_id,amounts, memo)
            }

            fn sft_batch_transfer_call(
                &mut self,
                receiver_id: ValidAccountId,
                token_ids: Vec<TokenId>,
                amounts: Vec<U128>,
                memo: Option<String>,
                msg: String,
            ) -> PromiseOrValue<bool> {
              self.$token.sft_batch_transfer_call(receiver_id,token_ids,amounts,. memo, msg)
            }

            fn balance_of(&self, owner_id: ValidAccountId, token_id: TokenId)-> U128 {
              self.$token.balance_of(owner_id,token_id)
            }
    
            fn balance_of_batch(&self, owner_id: ValidAccountId, token_ids: Vec<TokenId>) -> Vec<u128> {
              self.$token.balance_of_batch(owner_id,token_ids)
            }

            fn total_supply(&self, token_id: TokenId) -> U128 {
                self.$token.total_supply(token_id)
            }

            fn total_supply_batch(&self, token_ids: Vec<TokenId>) -> Vec<U128> {
                self.$token.total_supply_batch(token_ids)
            }

        }

        #[near_bindgen]
        impl SemiFungibleTokenResolver for $contract {
            #[private]
            fn sft_resolve_transfer(
                &mut self,
                receiver_id: ValidAccountId,
                token_ids: Vec<TokenId>
                amount: Vec<U128>,
            ) -> bool {
                let sender_id: AccountId = sender_id.into();
                let (used_amount, burned_amount) =
                    self.$token.internal_ft_resolve_transfer(&sender_id, receiver_id, amount);
                if burned_amount > 0 {
                    $(self.$on_tokens_burned_fn(sender_id, burned_amount);)?
                }
                used_amount.into()
            }
        }
    };
}

