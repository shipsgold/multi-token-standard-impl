use near_sdk::AccountId;
use near_sdk_sim::{call, deploy, init_simulator, to_yocto, ContractAccount, UserAccount};
use semi_fungible_token::ContractContract as SftContract;
use semi_fungible_token_standard::metadata::{
    SemiFungibleTokenMetadata, SEMI_FUNGIBLE_METADATA_SPEC,
};
use semi_fungible_token_standard::{TokenId, TokenType};
use token_receiver::TokenReceiverContract;

// Load in contract bytes at runtime
near_sdk_sim::lazy_static_include::lazy_static_include_bytes! {
    SFT_WASM_BYTES => "res/semi_fungible_token.wasm",
    TOKEN_RECEIVER_WASM_BYTES => "res/token_receiver.wasm",
}

pub const SFT_ID: &str = "sft";
const TOKEN_RECEIVER_ID: &str = "token-receiver";
// TODO: how to export String instead of &str? Way too much `into`/`to_string` with &str.
pub const NFT_TOKEN_ID: &str = "1";
pub const FT_TOKEN_ID: &str = "2";

/// Initialize simulator and return:
/// * root: the root user, set as owner_id for NFT contract, owns a token with ID=1
/// * nft: the NFT contract, callable with `call!` and `view!`
/// * alice: a user account, does not yet own any tokens
/// * token_receiver: a contract implementing `nft_on_transfer` for use with `transfer_and_call`
pub fn init(
) -> (UserAccount, ContractAccount<SftContract>, UserAccount, ContractAccount<TokenReceiverContract>)
{
    let root = init_simulator(None);
    // uses default values for deposit and gas
    let sft = deploy!(
        // Contract Proxy
        contract: SftContract,
        // Contract account id
        contract_id: SFT_ID,
        // Bytes of contract
        bytes: &SFT_WASM_BYTES,
        // User deploying the contract,
        signer_account: root,
        // init method
        init_method: new(
            root.valid_account_id()
        )
    );

    let alice = root.create_user("alice".to_string(), to_yocto("100"));

    let token_receiver = deploy!(
        contract: TokenReceiverContract,
        contract_id: TOKEN_RECEIVER_ID,
        bytes: &TOKEN_RECEIVER_WASM_BYTES,
        signer_account: root,
        init_method: new(
            sft.account_id()
        )
    );

    call!(
        root,
        sft.sft_mint(
            NFT_TOKEN_ID.to_string(),
            TokenType::Nft,
            None,
            root.valid_account_id(),
            Some(SemiFungibleTokenMetadata {
                reference: Some("/some/uri/reference/{id}_token.json".into()),
                reference_hash: None,
                spec: SEMI_FUNGIBLE_METADATA_SPEC.to_string()
            })
        ),
        deposit = 7000000000000000000000
    );

    call!(
        root,
        sft.sft_mint(
            FT_TOKEN_ID.to_string(),
            TokenType::Ft,
            Some(100.into()),
            root.valid_account_id(),
            Some(SemiFungibleTokenMetadata {
                reference: Some("/some/uri/reference/ft/{id}_token.json".into()),
                reference_hash: None,
                spec: SEMI_FUNGIBLE_METADATA_SPEC.to_string()
            })
        ),
        deposit = 7000000000000000000000
    );

    (root, sft, alice, token_receiver)
}

pub fn helper_mint(
    token_id: TokenId,
    root: &UserAccount,
    sft: &ContractAccount<SftContract>,
    title: String,
    desc: String,
) {
}
