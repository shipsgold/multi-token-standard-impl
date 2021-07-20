use crate::token::TokenId;
use near_sdk::AccountId;
use std::collections::HashMap;
use near_sdk::json_types::{U128};

/// Used when SemiFungibleTokens are transferred using `sft_transfer_call`. This is the method that's called after `sft_on_transfer`. This trait is implemented on the SemiFungibleToken contract.
pub trait SemiFungibleTokenResolver {
    /// Finalize an `sft_transfer_call` chain of cross-contract calls.
    ///
    /// The `sft_transfer_call` process:
    ///
    /// 1. Sender calls `sft_transfer_call` on SemiFungibleToken contract
    /// 2. SemiFungibleToken contract transfers token from sender to receiver
    /// 3. SemiFungibleToken contract calls `sft_on_transfer` on receiver contract
    /// 4+. [receiver contract may make other cross-contract calls]
    /// N. SemiFungibleToken contract resolves promise chain with `sft_resolve_transfer`, and may
    ///    transfer token back to sender
    ///
    /// Requirements:
    /// * Contract MUST forbid calls to this function by any account except self
    /// * If promise chain failed, contract MUST revert token transfer
    /// * If promise chain resolves with `true`, contract MUST return token to
    ///   `sender_id`
    ///
    /// Arguments:
    /// * `previous_owner_id`: the owner prior to the call to `sft_transfer_call`
    /// * `receiver_id`: the `receiver_id` argument given to `sft_transfer_call`
    /// * `token_ids`: the `token_ids` argument given to `sft_transfer_call`
    /// * `approvals`: if using Approval Management, contract MUST provide
    ///   set of original approved accounts in this argument, and restore these
    ///   approved accounts in case of revert. In this case it may be multiple sets of approvals
    ///
    /// Returns true if tokens were successfully transferred to `receiver_id`.
    fn sft_resolve_transfer(
        &mut self,
        sender_id: AccountId,
        receiver_id: AccountId,
        token_ids: Vec<TokenId>,
        amounts: Vec<U128>
    ) -> Vec<U128>;
}