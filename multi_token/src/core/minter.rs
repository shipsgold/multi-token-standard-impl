use crate::metadata::MultiTokenMetadata;
use crate::token::{TokenId, TokenType};
use near_sdk::json_types::{ValidAccountId, U128};

/// Used when minting restricted by default to owner. Token owner id is restricted to local accountId.
pub trait MultiTokenMinter {
	/// Mint some MultiTokens either of a ft type or nft type
	///
	/// Requirements:
	/// * Mint function must handle storage allocation and deallocation for generating new tokens,
	///   as well as manage any supply calculations that occur as a result of minting
	///
	/// Arguments:
	/// * `token_id`: the token_id corresponding to the token you'd like to mint. NFT requires uniqueness, Ft type can be prexisting.  
	/// * `amount`: the amount specified for the ft supply
	/// * `token_owner_id`: the account that the tokens will be minted to
	/// * `token_metadata`: the optional metadata for the token
	fn mint(
		&mut self,
		token_id: TokenId,
		token_type: TokenType,
		amount: Option<U128>,
		token_owner_id: ValidAccountId,
		token_metadata: Option<MultiTokenMetadata>,
	);
}
