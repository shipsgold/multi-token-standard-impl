use super::resolver::MultiTokenResolver;
use crate::core::MultiTokenCore;
use crate::metadata::TokenMetadata;
use crate::token::{Token, TokenId, TokenType};
//use crate::utils::{hash_account_id, refund_approved_account_ids, refund_deposit};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, TreeMap, UnorderedSet};
use near_sdk::json_types::{Base64VecU8, ValidAccountId, U128};
use near_contract_standards::fungible_token::core_impl::{FungibleToken};
use near_contract_standards::non_fungible_token::core::{NonFungibleToken};
use near_sdk::{
	assert_one_yocto, env, ext_contract, log, AccountId, Balance, BorshStorageKey, CryptoHash,
	Gas, IntoStorageKey, PromiseOrValue, PromiseResult, StorageUsage,
};
use std::collections::HashMap;
const GAS_FOR_RESOLVE_TRANSFER: Gas = 5_000_000_000_000;
const GAS_FOR_FT_TRANSFER_CALL: Gas = 25_000_000_000_000 + GAS_FOR_RESOLVE_TRANSFER;

const NO_DEPOSIT: Balance = 0;


#[ext_contract(ext_self)]
trait MultiResolver {
	fn multi_resolve_transfer(
		&mut self,
		previous_owner_id: AccountId,
		receiver_id: AccountId,
		token_ids: Vec<TokenId>,
		amounts: Vec<u128>,
		approved_account_ids: Option<HashMap<AccountId, u64>>,
	) -> bool;
}

#[ext_contract(ext_receiver)]
pub trait MultiReceiver {
	/// Returns true if token should be returned to `sender_id`
	fn multi_on_transfer(
		&mut self,
		sender_id: AccountId,
		previous_owner_id: AccountId,
		token_ids: Vec<TokenId>,
		amounts: Vec<u128>,
		msg: String,
	) -> PromiseOrValue<bool>;
}


/// Implementation of multi-token standard.
/// There are next traits that any contract may implement:
///     - MultiTokenCore -- interface with multi_transfer/balance/supply methods. MultiToken provides methods for it.
///     - MultiTokenApproval -- interface with multi_approve methods. MultiToken provides methods for it.
///     - MultiTokenMetadata -- return metadata for the token in NEP-177, up to contract to implement.
///
/// For example usage, see examples/non-fungible-token/src/lib.rs.
/// 
#[derive(BorshDeserialize, BorshSerialize)]
enum Tokens {
	FT(FungibleToken),
	NFT(NonFungibleToken),
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct MultiToken {
	// owner of contract; this is the only account allowed to call `mint`
	pub owner_id: AccountId,

	// index token id and token type to aid in uniqueness guarantees
	pub token_type_index: LookupMap<TokenId, TokenType>,

	// always required TokenId corresponds to nft
	pub ft_tokens: TreeMap<TokenId, FungibleToken>,
	pub nft_tokens: NonFungibleToken,

	pub ft_prefix: Vec<u8>,
	pub ft_prefix_index: u64,
}

impl MultiToken {
	
	// TODO evaluate this signature 
	pub fn new<O, P, Q, R, S, T>(
		owner_id: ValidAccountId,
		ft_prefix: O,
		token_type_prefix: P, 
		nft_owner_by_id_prefix: Q,
		nft_token_metadata_prefix: Option<R>,
		nft_enumeration_prefix: Option<S>,
		nft_approval_prefix: Option<T>,
	    ) -> Self
	    where
		O: IntoStorageKey,
		P: IntoStorageKey,
		Q: IntoStorageKey,
		R: IntoStorageKey,
		S: IntoStorageKey,
		T: IntoStorageKey,
	    {
		   let nft_tokens = NonFungibleToken::new(nft_owner_by_id_prefix,
			owner_id,
			nft_token_metadata_prefix,
			nft_enumeration_prefix,
			nft_approval_prefix);
		   let ft_tokens = TreeMap::new(ft_prefix); 
		   let token_type_index = LookupMap::new(token_type_prefix); 
		   Self {
			   owner_id: owner_id.into(),
			   nft_tokens,
			   ft_tokens,
			   ft_prefix: ft_prefix.into_storage_key(),
			   ft_prefix_index:0,
			   token_type_index,
			}
	}

    pub fn nft_internal_transfer_unguarded( &mut self, 
	token_id: &TokenId,
        from: &AccountId,
        to: &AccountId,
    ) {
	self.nft_tokens.internal_transfer_unguarded(token_id, from, to);
    }

    /// Transfer from current owner to receiver_id, checking that sender is allowed to transfer.
    /// Clear approvals, if approval extension being used.
    /// Return previous owner and approvals.
    pub fn internal_transfer(
        &mut self,
        sender_id: &AccountId,
        receiver_id: &AccountId,
        token_id: &TokenId,
	amount: u128,
        memo: Option<String>,
        approval_id: Option<u64>,
    ) -> (AccountId, Option<HashMap<AccountId, u64>>) {
	let token_type = self.token_type_index.get(token_id).expect("Token not found");

	 match self.token_type_index.get(token_id) {
		Some(TokenType::NFT) => { 
			self.nft_tokens.internal_transfer(sender_id,receiver_id,token_id,approval_id, memo)
		},
		Some(TokenType::FT) => { 
			self.ft_tokens.get(token_id).unwrap().internal_transfer(sender_id, receiver_id, amount, memo);
			(sender_id.into(), None)  
		},
	}
    }


}

impl MultiTokenCore for MultiToken {

	fn multi_transfer(&mut self,
		receiver_id: ValidAccountId,
		token_id: TokenId, 
		amount: u128, 
		approval_id: Option<u64>, 
		memo: Option<String>) {

		assert_one_yocto();
		let sender_id = env::predecessor_account_id();
		self.internal_transfer(&sender_id, receiver_id.as_ref(), &token_id,amount,memo, approval_id);
	}

	// TODO verify gas cost
	fn multi_transfer_call(&mut self,
		receiver_id: ValidAccountId,
		token_id: TokenId,
		amount: u128,
		approval_id: Option<u64>,
		memo: Option<String>,
		msg: String,
	) ->PromiseOrValue<bool> {
		assert_one_yocto();
		let sender_id = env::predecessor_account_id();
		let (old_owner, old_approvals) =
		    self.internal_transfer(&sender_id, receiver_id.as_ref(), &token_id, amount, memo, approval_id);
		// Initiating receiver's call and the callback
		ext_receiver::multi_on_transfer(
		    sender_id.clone(),
		    old_owner.clone(),
		    vec![token_id.clone()],
		    vec![amount],
		    msg,
		    receiver_id.as_ref(),
		    NO_DEPOSIT,
		    env::prepaid_gas() - GAS_FOR_FT_TRANSFER_CALL,
		)
		.then(ext_self::multi_resolve_transfer(
		    old_owner,
		    receiver_id.into(),
		    vec![token_id],
		    vec![amount],
		    old_approvals,
		    &env::current_account_id(),
		    NO_DEPOSIT,
		    GAS_FOR_RESOLVE_TRANSFER,
		))
		.into()
	}

	fn multi_token(self, token_id: TokenId) -> Option<Token> {
		let owner_id = self.owner_by_id.get(&token_id)?;
		let supply = self.token_supply_by_id.get(&token_id)?;
		let token_type = self.token_type_index.get(&token_id);
		let metadata = self.token_metadata_by_id.and_then(|by_id| by_id.get(&token_id));
		let approved_account_ids = self
		    .approvals_by_id
		    .and_then(|by_id| by_id.get(&token_id).or_else(|| Some(HashMap::new())));
		Some(Token { token_id, token_type, owner_id, supply, metadata, approved_account_ids })
	    }
	

}