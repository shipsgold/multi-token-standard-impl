use crate::token::TokenId;

use near_sdk::json_types::U128;
use near_sdk::{AccountId, PromiseOrValue};
/// Used when MultiTokens are transferred using `mt_transfer_call`. This trait is implemented on the receiving contract, not on the MultiToken contract.
pub trait MultiTokenReceiver {
    /// Take some action after receiving a MultiToken-tokens token
    ///
    /// Requirements:
    /// * Contract MUST restrict calls to this function to a set of whitelisted MultiToken
    ///   contracts
    ///
    /// Arguments:
    /// * `sender_id`: the sender of `mt_transfer_call`
    /// * `previous_owner_id`: the account that owned the tokens prior to it being
    ///   transferred to this contract, which can differ from `sender_id` if using
    ///   Approval Management extension
    /// * `token_ids`: the `token_ids` argument given to `mt_transfer_call`
    /// * `msg`: information necessary for this contract to know how to process the
    ///   request. This may include method names and/or arguments.
    ///
    /// Returns true if tokens should be returned to `sender_id`
    fn mt_on_transfer(
        &mut self,
        sender_id: AccountId,
        token_ids: Vec<TokenId>,
        amounts: Vec<U128>,
        msg: String,
    ) -> PromiseOrValue<Vec<U128>>;
}
