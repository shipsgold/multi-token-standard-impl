/*!
Semi Fungible Token implementation with JSON serialization.
NOTES:
  - The maximum balance value is limited by U128 (2**128 - 1).
  - JSON calls should pass U128 as a base-10 string. E.g. "100".
  - The contract tracks the change in storage before and after the call. If the storage increases,
    the contract requires the caller of the contract to attach enough deposit to the function call
    to cover the storage cost.
    This is done to prevent a denial of service attack on the contract by taking all available storage.
    If the storage decreases, the contract will issue a refund for the cost of the released storage.
    The unused tokens from the attached deposit are also refunded, so it's safe to
    attach more deposit than required.
  - To prevent the deployed contract from being modified or deleted, it should not have any access
    keys on its account.
*/
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LazyOption;
use near_sdk::json_types::{ValidAccountId, U128};
use near_sdk::{
  env, log, near_bindgen, AccountId, Balance, BorshStorageKey, PanicOnDefault, PromiseOrValue,
};
use multi_token_standard::MultiToken;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
  token: MultiToken,
}

#[derive(BorshDeserialize, BorshSerialize, BorshStorageKey)]
enum StorageKey {
  MultiTokenOwner,
  MultiTokenMetadata,
  MultiTokenSupply,
}

#[near_bindgen]
impl Contract {
  #[init]
  pub fn new(owner_id: ValidAccountId) -> Self {
    assert!(!env::state_exists(), "Already initialized");
    Self {
      token: MultiToken::new(
        StorageKey::MultiTokenOwner,
        owner_id,
        Some(StorageKey::MultiTokenMetadata),
        StorageKey::MultiTokenSupply,
      ),
    }
  }

  #[payable]
  pub fn mt_mint(
    &mut self,
    token_id: TokenId,
    token_type: TokenType,
    amount: Option<U128>,
    token_owner_id: ValidAccountId,
    token_metadata: Option<MultiTokenMetadata>,
  ) {
    self.token.mint(token_id, token_type, amount, token_owner_id, token_metadata)
  }
}
multi_token_standard::impl_multi_token_core_with_minter!(Contract, token);
multi_token_standard::impl_multi_token_storage!(Contract, token);
