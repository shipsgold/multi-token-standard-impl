/// The [approval management standard](https://nomicon.io/Standards/NonFungibleToken/ApprovalManagement.html) for tokens.
pub mod approval;
/// The [core semifungible token standard](). This can be though of as the base standard, with the others being extension standards.
pub mod core;
/// Metadata traits and implementation according to the [NFT enumeration standard](https://nomicon.io/Standards/NonFungibleToken/Metadata.html).
/// This covers both the contract metadata and the individual token metadata.
pub mod metadata;
/// The Token struct for the multi token standard.
mod token;
// Utils for the contract
mod utils; 
pub use self::token::{Token, TokenId, TokenType};

/// NFT utility functions
//mod utils;
//pub use utils::*;

pub use self::core::SemiFungibleToken;