/// The [core multit token standard](). This can be though of as the base standard, with the others being extension standards.
pub mod core;
/// The Token struct for the multi token standard.
mod token;
pub use self::token::{TokenId, TokenType};

/// NFT utility functions
//mod utils;
//pub use utils::*;

pub use self::core::MultiToken;