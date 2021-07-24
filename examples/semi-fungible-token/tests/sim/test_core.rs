use crate::utils::{init, FT_TOKEN_ID, NFT_TOKEN_ID};
use near_sdk::json_types::U128;
use near_sdk_sim::{call, view};

#[test]
fn simulate_simple_transfer_nft() {
    let (root, sft, alice, _) = init();
    let nft_amount: U128 =
        view!(sft.balance_of(root.account_id(), NFT_TOKEN_ID.into())).unwrap_json();
    assert_eq!(nft_amount.0, 1);
    call!(
        root,
        sft.sft_transfer(
            alice.account_id(),
            NFT_TOKEN_ID.into(),
            1.into(),
            Some("simple transfer".to_string())
        ),
        deposit = 1
    )
    .assert_success();

    let remaining_amount: U128 =
        view!(sft.balance_of(root.account_id(), NFT_TOKEN_ID.into())).unwrap_json();
    let transfered_amount: U128 =
        view!(sft.balance_of(alice.account_id(), NFT_TOKEN_ID.into())).unwrap_json();
    assert_eq!(transfered_amount.0, 1);
    assert_eq!(remaining_amount.0, 0);
}

#[test]
fn simulate_simple_transfer_ft() {
    let (root, sft, alice, _) = init();
    let ft_amount: U128 =
        view!(sft.balance_of(root.account_id(), FT_TOKEN_ID.into())).unwrap_json();
    assert_eq!(ft_amount.0, 100);

    call!(
        root,
        sft.sft_transfer(
            alice.account_id(),
            FT_TOKEN_ID.into(),
            75.into(),
            Some("simple transfer".to_string())
        ),
        deposit = 1
    )
    .assert_success();

    let remaining_amount: U128 =
        view!(sft.balance_of(root.account_id(), FT_TOKEN_ID.into())).unwrap_json();
    let transfered_amount: U128 =
        view!(sft.balance_of(alice.account_id(), FT_TOKEN_ID.into())).unwrap_json();
    assert_eq!(transfered_amount.0, 75);
    assert_eq!(remaining_amount.0, 25);
}

#[test]
fn simulate_simple_transfer_batch() {
    let (root, sft, alice, _) = init();
    call!(
        root,
        sft.sft_batch_transfer(
            alice.account_id(),
            vec![FT_TOKEN_ID.into(), NFT_TOKEN_ID.into()],
            vec![75.into(), 1.into()],
            Some("simple transfer".to_string())
        ),
        deposit = 1
    )
    .assert_success();
    let mut remaining_amount: U128 =
        view!(sft.balance_of(root.account_id(), FT_TOKEN_ID.into())).unwrap_json();
    let mut transfered_amount: U128 =
        view!(sft.balance_of(alice.account_id(), FT_TOKEN_ID.into())).unwrap_json();
    assert_eq!(transfered_amount.0, 75);
    assert_eq!(remaining_amount.0, 25);

    remaining_amount = view!(sft.balance_of(root.account_id(), NFT_TOKEN_ID.into())).unwrap_json();
    transfered_amount =
        view!(sft.balance_of(alice.account_id(), NFT_TOKEN_ID.into())).unwrap_json();
    assert_eq!(transfered_amount.0, 1);
    assert_eq!(remaining_amount.0, 0);
}
