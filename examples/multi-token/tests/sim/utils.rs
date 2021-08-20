use multi_token::ContractContract as MtContract;
use multi_token_standard::metadata::{MultiTokenMetadata, MT_METADATA_SPEC};
use multi_token_standard::{TokenId, TokenType};
use near_sdk::json_types::U128;
use near_sdk::serde_json::json;
use near_sdk::AccountId;
use near_sdk_sim::{call, deploy, init_simulator, to_yocto, ContractAccount, UserAccount, view};
use rand::prelude::*;
use token_receiver::TokenReceiverContract;

// Load in contract bytes at runtime
near_sdk_sim::lazy_static_include::lazy_static_include_bytes! {
    MT_WASM_BYTES => "res/multi_token.wasm",
    TOKEN_RECEIVER_WASM_BYTES => "res/token_receiver.wasm",
}

pub const MT_ID: &str = "mt";
const TOKEN_RECEIVER_ID: &str = "token-receiver";
// TODO: how to export String instead of &str? Way too much `into`/`to_string` with &str.
pub const NFT_TOKEN_ID: &str = "1";
pub const FT_TOKEN_ID: &str = "2";

pub fn generate_random_token_tuples(
    size: u128,
) -> (Vec<TokenId>, Vec<TokenType>, Vec<U128>, Vec<Option<MultiTokenMetadata>>) {
    let mut token_types: Vec<TokenType> = vec![];
    let mut amounts: Vec<U128> = vec![];
    let mut token_ids: Vec<TokenId> = vec![];
    let mut metadatas: Vec<Option<MultiTokenMetadata>> = vec![];
    let mut counter: u128 = 0;
    for _ in 1..size {
        if rand::random::<bool>() == true {
            token_types.push(TokenType::Ft);
            let amount: u128 = rand::random::<u128>();
            amounts.push(amount.into());
        } else {
            token_types.push(TokenType::Nft);
            amounts.push(1.into());
        }
        let metadata = if rand::random::<bool>() == true {
            Some(MultiTokenMetadata {
                reference: Some("/some/uri/reference/{id}_token.json".into()),
                reference_hash: None,
                title: None,
                description: None,
                media: None,
                media_hash: None,
                copies: None,
                issued_at: None,
                expires_at: None,
                starts_at: None,
                updated_at: None,
                spec: MT_METADATA_SPEC.to_string(),
                name: "".to_string(),
                symbol: "".to_string(),
                icon: None,
                base_uri: None,
                decimals: None,
                extra: None,
            })
        } else {
            None
        };
        metadatas.push(metadata);
        token_ids.push(format!("generated_{}", counter));
    }
    (token_ids, token_types, amounts, metadatas)
}

// Register the given `user` with a set of token_ids
pub fn register_user(user: &near_sdk_sim::UserAccount, acct_id: AccountId, token_ids: &Vec<TokenId>) {
    user.call(
        AccountId::new_unchecked(MT_ID.to_string()),
        "storage_deposit",
        &json!({
            "token_ids": token_ids,
            "account_id": acct_id 
        })
            .to_string()
            .into_bytes(),
        near_sdk_sim::DEFAULT_GAS / 2,
        near_sdk::env::storage_byte_cost() * 700, // attached deposit
    )
        .assert_success();
}

/// Initialize simulator and return:
/// * root: the root user, set as owner_id for NFT contract, owns a token with ID=1
/// * nft: the NFT contract, callable with `call!` and `view!`
/// * alice: a user account, does not yet own any tokens
/// * token_receiver: a contract implementing `nft_on_transfer` for use with `transfer_and_call`
pub fn init() -> (UserAccount, ContractAccount<MtContract>, UserAccount, ContractAccount<TokenReceiverContract>)
{
    let root = init_simulator(None);
    // uses default values for deposit and gas
    let mt = deploy!(
        // Contract Proxy
        contract: MtContract,
        // Contract account id
        contract_id: MT_ID,
        // Bytes of contract
        bytes: &MT_WASM_BYTES,
        // User deploying the contract,
        signer_account: root,
        // init method
        init_method: new(
            root.account_id()
        )
    );

    let alice = root.create_user(AccountId::new_unchecked("alice".to_string()), to_yocto("100"));
    let dummy_metadata = MultiTokenMetadata {
        reference: Some("/some/uri/reference/{id}_token.json".into()),
        reference_hash: None,
        title: None,
        description: None,
        media: None,
        media_hash: None,
        copies: None,
        issued_at: None,
        expires_at: None,
        starts_at: None,
        updated_at: None,
        spec: MT_METADATA_SPEC.to_string(),
        name: "".to_string(),
        symbol: "".to_string(),
        icon: None,
        base_uri: None,
        decimals: None,
        extra: None,
    };

    let token_receiver = deploy!(
        contract: TokenReceiverContract,
        contract_id: TOKEN_RECEIVER_ID,
        bytes: &TOKEN_RECEIVER_WASM_BYTES,
        signer_account: root,
        init_method: new(
            mt.account_id()
        )
    );
    call!(
         root,
        mt.mint(
            NFT_TOKEN_ID.to_string(),
            TokenType::Nft,
            None,
            root.account_id(),
            Some(dummy_metadata.clone())
        ),
        deposit = 7000000000000000000000
    );

    call!(
        root,
        mt.mint(
            FT_TOKEN_ID.to_string(),
            TokenType::Ft,
            Some(100.into()),
            root.account_id(),
            Some(dummy_metadata)
        ),
        deposit = 7000000000000000000000
    );
    register_user(&alice, alice.account_id(), &vec![FT_TOKEN_ID.to_string()]);
    register_user(&root, token_receiver.account_id(), &vec![FT_TOKEN_ID.to_string()]);
    (root, mt, alice, token_receiver)
}

pub fn check_balance(
    mt: &ContractAccount<MtContract>,
    account_id: AccountId,
    token_id: TokenId,
    expected_amt: u128,
) {
    let amount: U128 = view!(mt.balance_of(account_id, token_id)).unwrap_json();
    assert_eq!(amount.0, expected_amt);
}

pub fn init_batch() {}

pub fn helper_mint(
    token_id: TokenId,
    root: &UserAccount,
    mt: &ContractAccount<MtContract>,
    title: String,
    desc: String,
) {}
