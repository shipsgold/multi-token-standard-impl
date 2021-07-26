/// The [core semifungible token standard](). This can be though of as the base standard, with the others being extension standards.
pub mod core;
/// Metadata traits and implementation according to the [NFT enumeration standard](https://nomicon.io/Standards/NonFungibleToken/Metadata.html).
/// This covers both the contract metadata and the individual token metadata.
pub mod metadata;
/// This covers the storage management for the tokens
pub mod storage_management;
/// The Token struct for the MultiToken token standard.
mod token;
// Utils for the contract
mod macros;
mod utils;

pub use self::core::MultiToken;
pub use self::token::{Token, TokenId, TokenType};
pub use macros::*;
