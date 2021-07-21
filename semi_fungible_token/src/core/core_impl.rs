use crate::core::SemiFungibleTokenCore;
use crate::core::SemiFungibleTokenResolver;
use crate::metadata::{SemiFungibleTokenMetadata, SEMI_FUNGIBLE_METADATA_SPEC};
use crate::token::{TokenId, TokenType};
use crate::utils::refund_deposit;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, TreeMap};
use near_sdk::json_types::{ValidAccountId, U128};
use near_sdk::{
	assert_one_yocto, env, ext_contract, log, AccountId, Balance, Gas, IntoStorageKey,
	PromiseOrValue, PromiseResult, StorageUsage,
};

const GAS_FOR_RESOLVE_TRANSFER: Gas = 5_000_000_000_000;
const GAS_FOR_FT_TRANSFER_CALL: Gas = 25_000_000_000_000 + GAS_FOR_RESOLVE_TRANSFER;

const NO_DEPOSIT: Balance = 0;

#[ext_contract(ext_self)]
trait SemiFungibleTokenResolver {
	fn sft_resolve_transfer(
		&mut self,
		sender_id: AccountId,
		receiver_id: AccountId,
		token_ids: Vec<TokenId>,
		amounts: Vec<U128>,
	) -> bool;
}

#[ext_contract(ext_receiver)]
pub trait SemiFungibleTokenReceiver {
	/// Returns true if token should be returned to `sender_id`
	fn sft_on_transfer(
		&mut self,
		sender_id: AccountId,
		token_ids: Vec<TokenId>,
		amounts: Vec<U128>,
		msg: String,
	) -> PromiseOrValue<Vec<U128>>;
}

/// Implementation of SemiFungibleToken-token standard.
/// There are next traits that any contract may implement:
///     - SemiFungibleTokenCore -- interface with sft_transfer/balance/supply methods. SemiFungibleToken provides methods for it.
///     - SemiFungibleTokenApproval -- interface with sft_approve methods. SemiFungibleToken provides methods for it.
///     - SemiFungibleTokenMetadata -- return metadata for the token in NEP-177, up to contract to implement.
///
/// For example usage, see examples/non-fungible-token/src/lib.rs.
#[derive(BorshDeserialize, BorshSerialize)]
pub struct SemiFungibleToken {
	// owner of contract; this is the only account allowed to call `mint`
	pub owner_id: AccountId,

	// The storage size in bytes for each new token
	pub extra_storage_in_bytes_per_nft_token: StorageUsage,
	pub extra_storage_in_bytes_per_ft_token_balance: StorageUsage,
	pub extra_storage_in_bytes_per_ft_token_creation: StorageUsage,

	// index token id and token type to aid in uniqueness guarantees
	pub token_type_index: LookupMap<TokenId, TokenType>,

	// always required TokenId corresponds to nft
	pub nft_owner_by_id: TreeMap<TokenId, AccountId>,

	// always required TokenId corresponds to ft
	pub ft_owners_by_id: TreeMap<TokenId, TreeMap<AccountId, Balance>>,

	pub owner_prefix: Vec<u8>,
	pub ft_prefix_index: u64,

	// always required mapping to token supply
	pub ft_token_supply_by_id: LookupMap<TokenId, u128>,

	// required by metadata extension
	pub token_metadata_by_id: Option<LookupMap<TokenId, SemiFungibleTokenMetadata>>,
}

impl SemiFungibleToken {
	pub fn new<Q, R, T>(
		owner_by_id_prefix: Q,
		owner_id: ValidAccountId,
		token_metadata_prefix: Option<R>,
		supply_by_id_prefix: T,
	) -> Self
	where
		Q: IntoStorageKey,
		R: IntoStorageKey,
		T: IntoStorageKey,
	{
		let owner_prefix: Vec<u8> = owner_by_id_prefix.into_storage_key();
		let token_type_prefix = [owner_prefix.clone(), "t".into()].concat();
		let mut this = Self {
			owner_id: owner_id.into(),
			owner_prefix: owner_prefix.clone(),
			extra_storage_in_bytes_per_nft_token: 0,
			extra_storage_in_bytes_per_ft_token_balance: 0,
			extra_storage_in_bytes_per_ft_token_creation: 0,
			ft_owners_by_id: TreeMap::new(owner_prefix.clone()),
			nft_owner_by_id: TreeMap::new([owner_prefix, "n".into()].concat()),
			token_type_index: LookupMap::new(token_type_prefix.into_storage_key()),
			ft_prefix_index: 0,
			ft_token_supply_by_id: LookupMap::new(supply_by_id_prefix.into_storage_key()),
			token_metadata_by_id: token_metadata_prefix.map(LookupMap::new),
		};
		this.measure_min_ft_token_storage_cost();
		this.measure_min_nft_token_storage_cost();
		this
	}

	pub fn mint(
		&mut self,
		token_id: TokenId,
		token_type: TokenType,
		amount: U128,
		token_owner_id: ValidAccountId,
		token_metadata: Option<SemiFungibleTokenMetadata>,
	) {
		let initial_storage_usage = env::storage_usage();
		assert_eq!(env::predecessor_account_id(), self.owner_id, "Unauthorized");
		if self.token_metadata_by_id.is_some() && token_metadata.is_none() {
			env::panic(b"Must provide metadata");
		}
		if self.token_type_index.get(&token_id).is_some() {
			env::panic(b"token_id must be unique");
		}
		let owner_id: AccountId = token_owner_id.into();
		// Core behavior: every token must have an owner
		match token_type {
			TokenType::Ft => {
				// advance the prefix index before insertion
				self.inc_balances_prefix();
				//create TreeMap for balances
				match self.ft_owners_by_id.get(&token_id) {
					Some(mut balances) => {
						let current_bal = balances.get(&owner_id).unwrap_or(0);
						// TODO not quite safe
						let amt = u128::from(amount);
						if amt == 0 {
							panic!("error: amount should be greater than 0")
						}
						balances.insert(&owner_id, &(current_bal + amt));
						let supply = self.ft_token_supply_by_id.get(&token_id).unwrap();
						self.ft_token_supply_by_id.insert(&token_id, &(supply + amt));
					}
					None => {
						let mut balances: TreeMap<AccountId, Balance> =
							TreeMap::new(self.get_balances_prefix());
						// insert amount into balances
						balances.insert(&owner_id, &amount.into());
						self.ft_owners_by_id.insert(&token_id, &balances);
						self.ft_token_supply_by_id.insert(&token_id, &amount.into());
					}
				}
			}
			TokenType::Nft => {
				self.nft_owner_by_id.insert(&token_id, &owner_id);
			}
		}
		// Metadata extension: Save metadata, keep variable around to return later.
		// Note that check above already panicked if metadata extension in use but no metadata
		// provided to call.
		self
			.token_metadata_by_id
			.as_mut()
			.and_then(|by_id| by_id.insert(&token_id, &token_metadata.as_ref().unwrap()));
		// Return any extra attached deposit not used for storage
		refund_deposit(env::storage_usage() - initial_storage_usage);
	}

	// returns the current storage key prefix for a ft
	fn get_balances_prefix(&self) -> Vec<u8> {
		let mut ft_token_prefix = self.owner_prefix.clone();
		ft_token_prefix.extend(&self.ft_prefix_index.to_be_bytes().to_vec());
		ft_token_prefix
	}

	// increases the internal index for storage keys for balance maps for tokens
	fn inc_balances_prefix(&mut self) {
		self.ft_prefix_index += 1;
	}

	fn measure_min_ft_token_storage_cost(&mut self) {
		let initial_storage_usage = env::storage_usage();

		// 1. add data to calculate space usage
		let mut tmp_balance_lookup: TreeMap<AccountId, Balance> =
			TreeMap::new(self.get_balances_prefix());
		self.extra_storage_in_bytes_per_ft_token_creation =
			initial_storage_usage - env::storage_usage();
		let storage_after_token_creation = env::storage_usage();
		let tmp_token_id = "a".repeat(64); // TODO: what's a reasonable max TokenId length?
		let tmp_owner_id = "a".repeat(64);
		let tmp_supply: u128 = 9999;
		self.ft_token_supply_by_id.insert(&tmp_token_id, &tmp_supply);
		tmp_balance_lookup.insert(&tmp_owner_id, &tmp_supply);
		self.ft_owners_by_id.insert(&tmp_token_id, &tmp_balance_lookup);

		// 2. measure the space taken up
		self.extra_storage_in_bytes_per_ft_token_balance =
			env::storage_usage() - storage_after_token_creation;

		// 3. roll it all back
		self.ft_owners_by_id.remove(&tmp_token_id);
	}

	fn measure_min_nft_token_storage_cost(&mut self) {
		let initial_storage_usage = env::storage_usage();
		// 1. set some dummy data
		let tmp_token_id = "a".repeat(64); // TODO: what's a reasonable max TokenId length?
		let tmp_owner_id = "a".repeat(64);

		self.nft_owner_by_id.insert(&tmp_token_id, &tmp_owner_id);
		if let Some(token_metadata_by_id) = &mut self.token_metadata_by_id {
			token_metadata_by_id.insert(
				&tmp_token_id,
				&SemiFungibleTokenMetadata {
					spec: SEMI_FUNGIBLE_METADATA_SPEC.to_string(),
					reference: None,
					reference_hash: None,
				},
			);
		}

		// 2. see how much space it took
		self.extra_storage_in_bytes_per_nft_token = env::storage_usage() - initial_storage_usage;

		if let Some(token_metadata_by_id) = &mut self.token_metadata_by_id {
			token_metadata_by_id.remove(&tmp_token_id);
		}

		self.nft_owner_by_id.remove(&tmp_token_id);
	}

	/// Transfer token_id from `from` to `to`
	///
	/// Do not perform any safety checks or do any logging
	pub fn internal_transfer_unguarded(
		&mut self,
		#[allow(clippy::ptr_arg)] token_id: &TokenId,
		amount: u128,
		from: &AccountId,
		to: &AccountId,
	) {
		// update owner
		match self.token_type_index.get(token_id) {
			Some(TokenType::Nft) => {
				self.nft_owner_by_id.insert(token_id, to);
			}
			Some(TokenType::Ft) => {
				self.ft_owners_by_id.get(token_id).unwrap().insert(to, &amount);
			}
			_ => (),
		};
	}

	fn verify_ft_transferable(
		&self,
		#[allow(clippy::ptr_arg)] token_id: &TokenId,
		sender_id: &AccountId,
		receiver_id: &AccountId,
	) {
		if sender_id == receiver_id {
			panic!("Sender and receiver cannot be the same")
		}
		let token_holders = self.ft_owners_by_id.get(token_id).expect("Could not find token");
		token_holders.get(sender_id).expect("Not a token owner");
	}

	/// Transfer from current owner to receiver_id, checking that sender is allowed to transfer.
	/// Clear approvals, if approval extension being used.
	/// Return previous owner and approvals.
	pub fn internal_transfer(
		&mut self,
		sender_id: &AccountId,
		receiver_id: &AccountId,
		#[allow(clippy::ptr_arg)] token_id: &TokenId,
		amount: u128,
		memo: Option<String>,
	) {
		let token_type = self.token_type_index.get(token_id).expect("Token not found");
		let mut owner_id = sender_id.clone();
		match token_type {
			TokenType::Nft => {
				owner_id = self.nft_owner_by_id.get(token_id).unwrap();
				assert_ne!(&owner_id, receiver_id, "Current and next owner must differ");
				assert_eq!(&owner_id, sender_id, "Unauthorized sender must be owner");
				let balance =
					self.ft_owners_by_id.get(token_id).and_then(|by_id| by_id.get(&sender_id)).unwrap();
				if balance < amount {
					panic!("Amount exceeds balance");
				}
			}
			TokenType::Ft => {
				self.verify_ft_transferable(token_id, sender_id, receiver_id);
			}
		}
		self.internal_transfer_unguarded(&token_id, amount, &owner_id, &receiver_id);

		log!("Transfer {} from {} to {}", token_id, sender_id, receiver_id);
		if let Some(memo) = memo {
			log!("Memo: {}", memo);
		}
	}

	pub fn internal_transfer_batch(
		&mut self,
		sender_id: &AccountId,
		receiver_id: &AccountId,
		#[allow(clippy::ptr_arg)] token_ids: &Vec<TokenId>,
		#[allow(clippy::ptr_arg)] amounts: &Vec<U128>,
		memo: Option<String>,
	) {
		if token_ids.len() != amounts.len() {
			panic!("Number of token ids and amounts must be equal")
		}
		token_ids.iter().enumerate().for_each(|(idx, token_id)| {
			self.internal_transfer(
				&sender_id,
				&receiver_id.into(),
				&token_id,
				amounts[idx].into(),
				memo.clone(),
			)
		});
	}
}

impl SemiFungibleTokenCore for SemiFungibleToken {
	fn sft_transfer(
		&mut self,
		receiver_id: AccountId,
		token_id: TokenId,
		amount: U128,
		memo: Option<String>,
	) {
		assert_one_yocto();
		let sender_id = env::predecessor_account_id();
		self.internal_transfer(&sender_id, &receiver_id, &token_id, amount.into(), memo);
	}

	fn sft_transfer_call(
		&mut self,
		receiver_id: AccountId,
		token_id: TokenId,
		amount: U128,
		memo: Option<String>,
		msg: String,
	) -> PromiseOrValue<U128> {
		assert_one_yocto();
		let sender_id = env::predecessor_account_id();
		self.internal_transfer(&sender_id, &receiver_id, &token_id, amount.into(), memo);
		// Initiating receiver's call and the callback
		ext_receiver::sft_on_transfer(
			sender_id.clone(),
			vec![token_id.clone()],
			vec![amount],
			msg,
			&receiver_id,
			NO_DEPOSIT,
			env::prepaid_gas() - GAS_FOR_FT_TRANSFER_CALL,
		)
		.then(ext_self::sft_resolve_transfer(
			sender_id,
			receiver_id,
			vec![token_id],
			vec![amount],
			&env::current_account_id(),
			NO_DEPOSIT,
			GAS_FOR_RESOLVE_TRANSFER,
		))
		.into()
	}

	fn sft_batch_transfer(
		&mut self,
		receiver_id: AccountId,
		token_ids: Vec<TokenId>,
		amounts: Vec<U128>,
		memo: Option<String>,
	) {
		assert_one_yocto();
		let sender_id = env::predecessor_account_id();
		self.internal_transfer_batch(&sender_id, &receiver_id, &token_ids, &amounts, memo);
	}

	fn sft_batch_transfer_call(
		&mut self,
		receiver_id: AccountId,
		token_ids: Vec<TokenId>,
		amounts: Vec<U128>,
		memo: Option<String>,
		msg: String,
	) -> PromiseOrValue<Vec<U128>> {
		assert_one_yocto();
		let sender_id = env::predecessor_account_id();
		self.internal_transfer_batch(&sender_id, &receiver_id, &token_ids, &amounts, memo);
		// TODO make this efficient
		ext_receiver::sft_on_transfer(
			sender_id.clone(),
			token_ids.clone(),
			amounts.clone(),
			msg,
			&receiver_id,
			NO_DEPOSIT,
			env::prepaid_gas() - GAS_FOR_FT_TRANSFER_CALL,
		)
		.then(ext_self::sft_resolve_transfer(
			sender_id,
			receiver_id,
			token_ids,
			amounts,
			&env::current_account_id(),
			NO_DEPOSIT,
			GAS_FOR_RESOLVE_TRANSFER,
		))
		.into()
	}

	fn balance_of(&self, owner_id: AccountId, token_id: TokenId) -> U128 {
		let ft_token = self.ft_owners_by_id.get(&token_id).expect("balance: token id not found");
		ft_token.get(&owner_id).unwrap().into()
	}

	fn balance_of_batch(&self, owner_id: AccountId, token_ids: Vec<TokenId>) -> Vec<u128> {
		token_ids
			.iter()
			.map(|token_id| {
				let ft_token = self.ft_owners_by_id.get(&token_id).expect("balance: token id not found");
				ft_token.get(&owner_id).unwrap()
			})
			.collect()
	}

	fn total_supply(&self, token_id: TokenId) -> U128 {
		self.ft_token_supply_by_id.get(&token_id).expect("supply: token id not found").into()
	}

	fn total_supply_batch(&self, token_ids: Vec<TokenId>) -> Vec<U128> {
		token_ids
			.iter()
			.map(|token_id| {
				self.ft_token_supply_by_id.get(&token_id).expect("supply: token id not found").into()
			})
			.collect()
	}
}

impl SemiFungibleToken {
	pub fn sft_internal_resolve_transfer(
		&mut self,
		sender_id: AccountId,
		receiver_id: AccountId,
		token_ids: Vec<TokenId>,
		amounts: Vec<U128>,
	) -> Vec<U128> {
		let returned_amounts: Vec<U128> = match env::promise_result(0) {
			PromiseResult::NotReady => unreachable!(),
			PromiseResult::Successful(value) => {
				if let Ok(returned_amount) = near_sdk::serde_json::from_slice::<Vec<U128>>(&value) {
					assert_eq!(returned_amount.len(), amounts.len(), "Amounts returned do not match length");
					returned_amount
				} else {
					amounts.clone()
				}
			}
			PromiseResult::Failed => amounts.clone(),
		};
		returned_amounts
			.iter()
			.enumerate()
			.map(|(idx, returned_amount)| {
				let ret_amt: u128 = returned_amount.clone().into();
				if ret_amt == 0 {
					return U128::from(0);
				}
				match self.token_type_index.get(&token_ids[idx]).expect("Token type does not exist") {
					TokenType::Ft => {
						let unused_amount = std::cmp::min(amounts[idx].into(), returned_amount.clone().into());
						let err_msg = &format!("Token id {} does not exist", &token_ids[idx]);
						let mut balances = self.ft_owners_by_id.get(&token_ids[idx]).expect(err_msg);
						let receiver_balance =
							balances.get(&receiver_id).expect("Token receiver no longer exists");
						if receiver_balance > 0 {
							let refund_amount: u128 = std::cmp::min(receiver_balance, unused_amount);
							balances.insert(&receiver_id, &(receiver_balance - refund_amount));
							return match balances.get(&sender_id) {
								Some(sender_balance) => {
									balances.insert(&sender_id, &(sender_balance + refund_amount));
									log!("Refund {} from {} to {}", refund_amount, receiver_id, sender_id);
									let amount: u128 = amounts[idx].into();
									U128::from(amount - refund_amount)
								}
								None => {
									let supply =
										self.ft_token_supply_by_id.get(&token_ids[idx]).expect("Token has no supply");
									self.ft_token_supply_by_id.insert(&token_ids[idx], &(supply - refund_amount));
									log!("The account of the sender was deleted");
									U128::from(0)
								}
							};
						} else {
							U128::from(0)
						}
					}
					TokenType::Nft => {
						if let Some(current_owner) = self.nft_owner_by_id.get(&token_ids[idx]) {
							if current_owner != receiver_id {
								return U128::from(0);
							} else {
								log!("Return token {} from @{} to @{}", token_ids[idx], &receiver_id, &sender_id);
								self.internal_transfer_unguarded(&token_ids[idx], 1, &receiver_id, &sender_id);
								return U128::from(1);
							}
						}
						U128::from(0)
					}
				}
			})
			.collect()
	}
}

impl SemiFungibleTokenResolver for SemiFungibleToken {
	fn sft_resolve_transfer(
		&mut self,
		sender_id: AccountId,
		receiver_id: AccountId,
		token_ids: Vec<TokenId>,
		amounts: Vec<U128>,
	) -> Vec<U128> {
		self.sft_internal_resolve_transfer(sender_id, receiver_id, token_ids, amounts)
	}
}
