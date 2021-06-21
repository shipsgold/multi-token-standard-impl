use crate::multi_token::token::TokenId;
use near_sdk::AccountId;

/// Approval receiver is the trait for the method called (or attempted to be called) when a MultiToken contract adds an approval for an account.
pub trait MultiTokenApprovalReceiver {
    /// Respond to notification that contract has been granted approval for a token.
    ///
    /// Notes
    /// * Contract knows the token contract ID from `predecessor_account_id`
    ///
    /// Arguments:
    /// * `token_ids`: the token to which this contract has been granted approval
    /// * `owner_id`: the owner of the token
    /// * `approval_ids`: the approval IDs stored by MultiToken contract for this approval.
    ///   Expected to be a number within the 2^53 limit representable by JSON.
    /// * `msg`: specifies information needed by the approved contract in order to
    ///    handle the approval. Can indicate both a function to call and the
    ///    parameters to pass to that function.
    fn multi_on_approve(
        &mut self,
        token_ids: Vec<TokenId>,
        owner_id: AccountId,
        approval_ids: Vec<u64>,
        msg: String,
    ) -> near_sdk::PromiseOrValue<String>; // TODO: how to make "any"?
}