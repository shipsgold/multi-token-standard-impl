use crate::utils::{check_balance, init, FT_TOKEN_ID, NFT_TOKEN_ID};
use near_sdk::json_types::U128;
use near_sdk_sim::{call, view, DEFAULT_GAS};

#[test]
fn simulate_simple_transfer_nft() {
    let (root, mt, alice, _) = init();
    let nft_amount: U128 =
        view!(mt.balance_of(root.account_id(), NFT_TOKEN_ID.into())).unwrap_json();
    assert_eq!(nft_amount.0, 1);
    call!(
        root,
        mt.mt_transfer(
            alice.account_id(),
            NFT_TOKEN_ID.into(),
            1.into(),
            Some("simple transfer".to_string())
        ),
        deposit = 1
    )
    .assert_success();

    let remaining_amount: U128 =
        view!(mt.balance_of(root.account_id(), NFT_TOKEN_ID.into())).unwrap_json();
    let transfered_amount: U128 =
        view!(mt.balance_of(alice.account_id(), NFT_TOKEN_ID.into())).unwrap_json();
    assert_eq!(transfered_amount.0, 1);
    assert_eq!(remaining_amount.0, 0);
}

#[test]
fn simulate_simple_transfer_ft() {
    let (root, mt, alice, _) = init();
    let ft_amount: U128 = view!(mt.balance_of(root.account_id(), FT_TOKEN_ID.into())).unwrap_json();
    assert_eq!(ft_amount.0, 100);

    call!(
        root,
        mt.mt_transfer(
            alice.account_id(),
            FT_TOKEN_ID.into(),
            75.into(),
            Some("simple transfer".to_string())
        ),
        deposit = 1
    )
    .assert_success();

    let remaining_amount: U128 =
        view!(mt.balance_of(root.account_id(), FT_TOKEN_ID.into())).unwrap_json();
    let transfered_amount: U128 =
        view!(mt.balance_of(alice.account_id(), FT_TOKEN_ID.into())).unwrap_json();
    assert_eq!(transfered_amount.0, 75);
    assert_eq!(remaining_amount.0, 25);
}

#[test]
fn simulate_simple_transfer_batch() {
    let (root, mt, alice, _) = init();
    call!(
        root,
        mt.mt_batch_transfer(
            alice.account_id(),
            vec![FT_TOKEN_ID.into(), NFT_TOKEN_ID.into()],
            vec![75.into(), 1.into()],
            Some("simple transfer".to_string())
        ),
        deposit = 1
    )
    .assert_success();

    let mut remaining_amount: U128 =
        view!(mt.balance_of(root.account_id(), FT_TOKEN_ID.into())).unwrap_json();
    let mut transfered_amount: U128 =
        view!(mt.balance_of(alice.account_id(), FT_TOKEN_ID.into())).unwrap_json();
    assert_eq!(transfered_amount.0, 75);
    assert_eq!(remaining_amount.0, 25);

    remaining_amount = view!(mt.balance_of(root.account_id(), NFT_TOKEN_ID.into())).unwrap_json();
    transfered_amount = view!(mt.balance_of(alice.account_id(), NFT_TOKEN_ID.into())).unwrap_json();
    assert_eq!(transfered_amount.0, 1);
    assert_eq!(remaining_amount.0, 0);
}

#[test]
fn simulate_transfer_call_fast_return_to_sender() {
    let (root, mt, _, receiver) = init();

    check_balance(&mt, root.account_id(), FT_TOKEN_ID.to_string(), 100);
    call!(
        root,
        mt.mt_transfer_call(
            receiver.account_id(),
            FT_TOKEN_ID.into(),
            75.into(),
            Some("transfer & call".into()),
            "return-it-now".into()
        ),
        deposit = 1
    )
    .assert_success();

    check_balance(&mt, root.account_id(), FT_TOKEN_ID.to_string(), 100);
    check_balance(&mt, receiver.account_id(), FT_TOKEN_ID.to_string(), 0);
}

#[test]
fn simulate_batch_transfer_call_fast_return_to_sender() {
    let (root, mt, _, receiver) = init();

    check_balance(&mt, root.account_id(), FT_TOKEN_ID.to_string(), 100);
    check_balance(&mt, root.account_id(), NFT_TOKEN_ID.to_string(), 1);
    let outcome = call!(
        root,
        1,
        DEFAULT_GAS,
        mt,
        mt_batch_transfer_call,
        receiver.account_id(),
        vec![FT_TOKEN_ID.into(), NFT_TOKEN_ID.into()],
        vec![75.into(), 1.into()],
        Some("transfer & call".into()),
        "return-it-now".into()
    )
    .assert_success();

    check_balance(&mt, root.account_id(), FT_TOKEN_ID.to_string(), 100);
    check_balance(&mt, receiver.account_id(), FT_TOKEN_ID.to_string(), 0);
    check_balance(&mt, root.account_id(), NFT_TOKEN_ID.to_string(), 1);
    check_balance(&mt, receiver.account_id(), NFT_TOKEN_ID.to_string(), 0);
}

#[test]
fn simulate_batch_transfer_call_slow_return_to_sender() {
    let (root, mt, _, receiver) = init();
    check_balance(&mt, root.account_id(), FT_TOKEN_ID.to_string(), 100);
    check_balance(&mt, root.account_id(), NFT_TOKEN_ID.to_string(), 1);
    call!(
        root,
        mt.mt_batch_transfer_call(
            receiver.account_id(),
            vec![FT_TOKEN_ID.into(), NFT_TOKEN_ID.into()],
            vec![75.into(), 1.into()],
            Some("transfer & call".into()),
            "return-it-later".into()
        ),
        deposit = 1
    )
    .assert_success();

    check_balance(&mt, root.account_id(), FT_TOKEN_ID.to_string(), 100);
    check_balance(&mt, receiver.account_id(), FT_TOKEN_ID.to_string(), 0);
    check_balance(&mt, root.account_id(), NFT_TOKEN_ID.to_string(), 1);
    check_balance(&mt, receiver.account_id(), NFT_TOKEN_ID.to_string(), 0);
}

#[test]
fn simulate_batch_transfer_call_fast_keep_with_sender() {
    let (root, mt, _, receiver) = init();
    check_balance(&mt, root.account_id(), FT_TOKEN_ID.to_string(), 100);
    check_balance(&mt, root.account_id(), NFT_TOKEN_ID.to_string(), 1);
    call!(
        root,
        mt.mt_batch_transfer_call(
            receiver.account_id(),
            vec![FT_TOKEN_ID.into(), NFT_TOKEN_ID.into()],
            vec![75.into(), 1.into()],
            Some("transfer & call".into()),
            "keep-it-now".into()
        ),
        deposit = 1
    )
    .assert_success();

    check_balance(&mt, root.account_id(), FT_TOKEN_ID.to_string(), 25);
    check_balance(&mt, receiver.account_id(), FT_TOKEN_ID.to_string(), 75);
    check_balance(&mt, root.account_id(), NFT_TOKEN_ID.to_string(), 0);
    check_balance(&mt, receiver.account_id(), NFT_TOKEN_ID.to_string(), 1);
}

#[test]
fn simulate_transfer_batch_call_slow_keep_with_sender() {
    let (root, mt, _, receiver) = init();
    check_balance(&mt, root.account_id(), FT_TOKEN_ID.to_string(), 100);
    check_balance(&mt, root.account_id(), NFT_TOKEN_ID.to_string(), 1);
    call!(
        root,
        mt.mt_batch_transfer_call(
            receiver.account_id(),
            vec![FT_TOKEN_ID.into(), NFT_TOKEN_ID.into()],
            vec![75.into(), 1.into()],
            Some("transfer & call".into()),
            "keep-it-later".into()
        ),
        deposit = 1
    )
    .assert_success();

    check_balance(&mt, root.account_id(), FT_TOKEN_ID.to_string(), 25);
    check_balance(&mt, receiver.account_id(), FT_TOKEN_ID.to_string(), 75);
    check_balance(&mt, root.account_id(), NFT_TOKEN_ID.to_string(), 0);
    check_balance(&mt, receiver.account_id(), NFT_TOKEN_ID.to_string(), 1);
}

#[test]
fn simulate_transfer_call_receiver_panics() {
    let (root, mt, _, receiver) = init();
    check_balance(&mt, root.account_id(), FT_TOKEN_ID.to_string(), 100);
    check_balance(&mt, root.account_id(), NFT_TOKEN_ID.to_string(), 1);
    call!(
        root,
        mt.mt_batch_transfer_call(
            receiver.account_id(),
            vec![FT_TOKEN_ID.into(), NFT_TOKEN_ID.into()],
            vec![75.into(), 1.into()],
            Some("transfer & call".into()),
            "make-it-panic".into()
        ),
        deposit = 1
    )
    .assert_success();

    check_balance(&mt, root.account_id(), FT_TOKEN_ID.to_string(), 100);
    check_balance(&mt, receiver.account_id(), FT_TOKEN_ID.to_string(), 0);
    check_balance(&mt, root.account_id(), NFT_TOKEN_ID.to_string(), 1);
    check_balance(&mt, receiver.account_id(), NFT_TOKEN_ID.to_string(), 0);
}
