mod core_impl;
mod receiver;
mod resolver;
mod storage_impl;

pub use self::core_impl::*;
pub use self::storage_impl::*;

pub use self::receiver::*;
pub use self::resolver::*;

use crate::token::TokenId;
use near_sdk::json_types::U128;
use near_sdk::{AccountId, PromiseOrValue};

pub trait MultiTokenCore {
    /// Basic token transfer. Transfer a token or tokens given a token_id. The token id can correspond to  
    /// either a NonFungibleToken or Fungible Token this is differeniated by the implementation.
    ///
    /// Requirements
    /// * Caller of the method must attach a deposit of 1 yoctoⓃ for security purposes
    /// * Contract MUST panic if called by someone other than token owner or,
    /// * If using Approval Management, contract MUST nullify approved accounts on
    ///   successful transfer.
    /// * TODO: needed? Both accounts must be registered with the contract for transfer to
    ///   succeed. See see https://nomicon.io/Standards/StorageManagement.html
    ///
    /// Arguments:
    /// * `receiver_id`: the valid NEAR account receiving the token
    /// * `token_id`: the token or tokens to transfer
    /// * `amount`: the token amount of tokens to transfer for token_id
    /// * `memo` (optional): for use cases that may benefit from indexing or
    ///    providing information for a transfer
    fn mt_transfer(
        &mut self,
        receiver_id: AccountId,
        token_id: TokenId,
        amount: U128,
        memo: Option<String>,
    );

    /// Transfer token/s and call a method on a receiver contract. A successful
    /// workflow will end in a success execution outcome to the callback on the MultiToken
    /// contract at the method `mt_resolve_transfer`.
    ///
    /// You can think of this as being similar to attaching  tokens to a
    /// function call. It allows you to attach any Fungible or Non Fungible Token in a call to a
    /// receiver contract.
    ///
    /// Requirements:
    /// * Caller of the method must attach a deposit of 1 yoctoⓃ for security
    ///   purposes
    /// * Contract MUST panic if called by someone other than token owner or,
    ///   if using Approval Management, one of the approved accounts
    /// * The receiving contract must implement `mt_on_transfer` according to the
    ///   standard. If it does not, MultiToken contract's `mt_resolve_transfer` MUST deal
    ///   with the resulting failed cross-contract call and roll back the transfer.
    /// * Contract MUST implement the behavior described in `mt_resolve_transfer`
    ///
    /// Arguments:
    /// * `receiver_id`: the valid NEAR account receiving the token.
    /// * `token_id`: the token to send.
    /// * `amount`: amount of tokens to transfer for token_id
    /// * `memo` (optional): for use cases that may benefit from indexing or
    ///    providing information for a transfer.
    /// * `msg`: specifies information needed by the receiving contract in
    ///    order to properly handle the transfer. Can indicate both a function to
    ///    call and the parameters to pass to that function.
    fn mt_transfer_call(
        &mut self,
        receiver_id: AccountId,
        token_id: TokenId,
        amount: U128,
        memo: Option<String>,
        msg: String,
    ) -> PromiseOrValue<U128>;

    /// Batch token transfer. Transfer a tokens given token_ids and amounts. The token ids can correspond to  
    /// either Non-Fungible Tokens or Fungible Tokens or some combination of the two. The token ids
    /// are used to segment the types on a per contract implementation basis.
    ///
    /// Requirements
    /// * Caller of the method must attach a deposit of 1 yoctoⓃ for security purposes
    /// * Contract MUST panic if called by someone other than token owner or,
    ///   if using Approval Management, one of the approved accounts
    /// * `approval_id` is for use with Approval Management,
    ///   see https://nomicon.io/Standards/NonFungibleToken/ApprovalManagement.html
    /// * If using Approval Management, contract MUST nullify approved accounts on
    ///   successful transfer.
    /// * TODO: needed? Both accounts must be registered with the contract for transfer to
    ///   succeed. See see https://nomicon.io/Standards/StorageManagement.html
    /// * The token_ids vec and amounts vec must be of equal length and equate to a 1-1 mapping
    ///   between amount and id. In the event that they do not line up the call should fail
    ///
    /// Arguments:
    /// * `receiver_id`: the valid NEAR account receiving the token
    /// * `token_ids`: the tokens to transfer
    /// * `amounts`: the amount of tokens to transfer for corresponding token_id
    /// * `approval_ids`: expected approval ID. A number smaller than
    ///    2^53, and therefore representable as JSON. See Approval Management
    ///    standard for full explanation. Must have same length as token_ids
    /// * `memo` (optional): for use cases that may benefit from indexing or
    ///    providing information for a transfer

    fn mt_batch_transfer(
        &mut self,
        receiver_id: AccountId,
        token_id: Vec<TokenId>,
        amounts: Vec<U128>,
        memo: Option<String>,
    );
    /// Batch transfer token/s and call a method on a receiver contract. A successful
    /// workflow will end in a success execution outcome to the callback on the MultiToken
    /// contract at the method `mt_resolve_batch_transfer`.
    ///
    /// You can think of this as being similar to attaching  tokens to a
    /// function call. It allows you to attach any Fungible or Non Fungible Token in a call to a
    /// receiver contract.
    ///
    /// Requirements:
    /// * Caller of the method must attach a deposit of 1 yoctoⓃ for security
    ///   purposes
    /// * Contract MUST panic if called by someone other than token owner or,
    ///   if using Approval Management, one of the approved accounts
    /// * The receiving contract must implement `mt_on_transfer` according to the
    ///   standard. If it does not, MultiToken contract's `mt_resolve_batch_transfer` MUST deal
    ///   with the resulting failed cross-contract call and roll back the transfer.
    /// * Contract MUST implement the behavior described in `mt_resolve_batch_transfer`
    /// * `approval_id` is for use with Approval Management extension, see
    ///   that document for full explanation.
    /// * If using Approval Management, contract MUST nullify approved accounts on
    ///   successful transfer.
    ///
    /// Arguments:
    /// * `receiver_id`: the valid NEAR account receiving the token.
    /// * `token_ids`: the tokens to transfer
    /// * `amounts`: the amount of tokens to transfer for corresponding token_id
    /// * `approval_ids`: expected approval IDs. A number smaller than
    ///    2^53, and therefore representable as JSON. See Approval Management
    ///    standard for full explanation. Must have same length as token_ids
    /// * `memo` (optional): for use cases that may benefit from indexing or
    ///    providing information for a transfer.
    /// * `msg`: specifies information needed by the receiving contract in
    ///    order to properly handle the transfer. Can indicate both a function to
    ///    call and the parameters to pass to that function.

    fn mt_batch_transfer_call(
        &mut self,
        receiver_id: AccountId,
        token_ids: Vec<TokenId>,
        amounts: Vec<U128>,
        memo: Option<String>,
        msg: String,
    ) -> PromiseOrValue<Vec<U128>>;

    /// Get the balance of an an account given token_id. For fungible token returns back amount, for
    /// non fungible token it returns back constant 1.
    fn balance_of(&self, owner_id: AccountId, token_id: TokenId) -> U128;

    /// Get the balances of an an account given token_ids. For fungible token returns back amount, for
    /// non fungible token it returns back constant 1. returns vector of balances corresponding to token_ids
    /// in a 1-1 mapping
    fn balance_of_batch(&self, owner_id: AccountId, token_ids: Vec<TokenId>) -> Vec<U128>;

    /// Returns the total supply of the token in a decimal string representation given token_id.
    fn total_supply(&self, token_id: TokenId) -> U128;

    // Returns the total supplies of the tokens given by token_ids in a decimal string representation.
    fn total_supply_batch(&self, token_ids: Vec<TokenId>) -> Vec<U128>;
}
