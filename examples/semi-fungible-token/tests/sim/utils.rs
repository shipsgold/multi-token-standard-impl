use semi_fungible_token::ContractContract as SftContract;
use token_receiver::TokenReceiverContract;

use near_sdk_sim::{call, deploy, init_simulator, to_yocto, ContractAccount, UserAccount};

// Load in contract bytes at runtime
near_sdk_sim::lazy_static_include::lazy_static_include_bytes! {
    SFT_WASM_BYTES => "res/semi_fungible_token.wasm",
    TOKEN_RECEIVER_WASM_BYTES => "res/token_receiver.wasm",
}

pub const SFT_ID: &str = "0";
const TOKEN_RECEIVER_ID: &str = "token-receiver";
// TODO: how to export String instead of &str? Way too much `into`/`to_string` with &str.
pub const TOKEN_ID: &str = "0";

/// Initialize simulator and return:
/// * root: the root user, set as owner_id for NFT contract, owns a token with ID=1
/// * nft: the NFT contract, callable with `call!` and `view!`
/// * alice: a user account, does not yet own any tokens
/// * token_receiver: a contract implementing `nft_on_transfer` for use with `transfer_and_call`
pub fn init() -> (
    UserAccount,
    ContractAccount<SftContract>,
    UserAccount,
    ContractAccount<TokenReceiverContract>
) {
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

    let alice = root.create_user(AccountId::new_unchecked("alice".to_string()), to_yocto("100"));

    let token_receiver = deploy!(
        contract: TokenReceiverContract,
        contract_id: TOKEN_RECEIVER_ID,
        bytes: &TOKEN_RECEIVER_WASM_BYTES,
        signer_account: root,
        init_method: new(
            nft.account_id()
        )
    );

    (root, sft, alice, token_receiver)

    /*call!(
        root,
        nft.nft_mint(
            TOKEN_ID.into(),
            root.valid_account_id(),
            TokenMetadata {
                title: Some("Olympus Mons".into()),
                description: Some("The tallest mountain in the charted solar system".into()),
                media: None,
                media_hash: None,
                copies: Some(1u64),
                issued_at: None,
                expires_at: None,
                starts_at: None,
                updated_at: None,
                extra: None,
                reference: None,
                reference_hash: None,
            }
        ),
        deposit = 7000000000000000000000
    );*/


}

pub fn helper_mint(
    token_id: TokenId,
    root: &UserAccount,
    sft: &ContractAccount<SftContract>,
    title: String,
    desc: String,
) {

}
