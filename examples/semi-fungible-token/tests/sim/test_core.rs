use crate::utils::{init, NFT_TOKEN_ID};
use near_sdk_sim::{call, view};

#[test]
fn simulate_simple_transfer() {
    let (root, sft, alice, _) = init();
    let amount: u128 = view!(sft.balance_of(NFT_TOKEN_ID.into(), root.account_id())).unwrap_json();
    assert_eq!(amount, 1);

    // assert_eq!(token.owner_id, root.account_id());
    assert_eq!(root.account_id().to_string(), alice.account_id().to_string())
}
