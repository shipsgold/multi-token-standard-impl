use near_sdk::{env, Balance, Promise};
pub fn refund_deposit(storage_used: u64) {
	let required_cost = env::storage_byte_cost() * Balance::from(storage_used);
	let attached_deposit = env::attached_deposit();
	assert!(
		required_cost <= attached_deposit,
		"Must attach {} yoctoNEAR to cover storage",
		required_cost,
	);
	let refund = attached_deposit - required_cost;
	if refund > 1 {
		Promise::new(env::predecessor_account_id()).transfer(refund);
	}
}
