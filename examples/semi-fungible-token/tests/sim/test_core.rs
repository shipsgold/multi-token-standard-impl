use crate::utils::{init, TOKEN_ID};
use near_sdk_sim::{call, view};

#[test]
fn simulate_simple_transfer() {
    let (root, sft, alice, _) = init();
    // assert_eq!(token.owner_id, root.account_id());
    assert_eq!(root, alice)
}
