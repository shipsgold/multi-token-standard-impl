use crate::token::TokenId;
use near_sdk::json_types::U128;
use near_sdk::AccountId;

/// Used when MultiTokens are transferred using `mt_transfer_call`. This is the method that's called after `mt_on_transfer`. This trait is implemented on the MultiToken contract.
pub trait MultiTokenResolver {
    /// Finalize an `mt_transfer_call` chain of cross-contract calls.
    ///
    /// The `mt_transfer_call` process:
    ///
    /// 1. Sender calls `mt_transfer_call` on MultiToken contract
    /// 2. MultiToken contract transfers token from sender to receiver
    /// 3. MultiToken contract calls `mt_on_transfer` on receiver contract
    /// 4+. [receiver contract may make other cross-contract calls]
    /// N. MultiToken contract resolves promise chain with `mt_resolve_transfer`, and may
    ///    transfer token back to sender
    ///
    /// Requirements:
    /// * Contract MUST forbid calls to this function by any account except self
    /// * If promise chain failed, contract MUST revert token transfer
    /// * If promise chain resolves with `true`, contract MUST return token to
    ///   `sender_id`
    ///
    /// Arguments:
    /// * `previous_owner_id`: the owner prior to the call to `mt_transfer_call`
    /// * `receiver_id`: the `receiver_id` argument given to `mt_transfer_call`
    /// * `token_ids`: the `token_ids` argument given to `mt_transfer_call`
    /// * `approvals`: if using Approval Management, contract MUST provide
    ///   set of original approved accounts in this argument, and restore these
    ///   approved accounts in case of revert. In this case it may be multiple sets of approvals
    ///
    /// Returns true if tokens were successfully transferred to `receiver_id`.
    fn mt_resolve_transfer(
        &mut self,
        sender_id: AccountId,
        receiver_id: AccountId,
        token_ids: Vec<TokenId>,
        amounts: Vec<U128>,
    ) -> Vec<U128>;
}
