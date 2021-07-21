/*!
A stub contract that implements sft_on_transfer for simulation testing sft_transfer_call.
*/
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::U128;
use near_sdk::{
    env, ext_contract, log, near_bindgen, AccountId, Balance, Gas, PanicOnDefault, PromiseOrValue,
};
use semi_fungible_token_standard::core::SemiFungibleTokenReceiver;
use semi_fungible_token_standard::TokenId;

const BASE_GAS: u64 = 5_000_000_000_000;
const PROMISE_CALL: u64 = 5_000_000_000_000;
const GAS_FOR_SFT_ON_TRANSFER: Gas = BASE_GAS + PROMISE_CALL;

const NO_DEPOSIT: Balance = 0;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct TokenReceiver {
    semi_fungible_token_account_id: AccountId,
}

// Defining cross-contract interface. This allows to create a new promise.
#[ext_contract(ext_self)]
pub trait ValueReturnTrait {
    fn ok_go(&self, return_it: Vec<U128>) -> PromiseOrValue<Vec<U128>>;
}

// Have to repeat the same trait for our own implementation.
trait ValueReturnTrait {
    fn ok_go(&self, return_it: Vec<U128>) -> PromiseOrValue<Vec<U128>>;
}

#[near_bindgen]
impl TokenReceiver {
    #[init]
    pub fn new(semi_fungible_token_account_id: AccountId) -> Self {
        Self { semi_fungible_token_account_id }
    }
}

#[near_bindgen]
impl SemiFungibleTokenReceiver for TokenReceiver {
    /// Returns true if token should be returned to `sender_id`
    /// Four supported `msg`s:
    /// * "return-it-now" - immediately return `true`
    /// * "keep-it-now" - immediately return `false`
    /// * "return-it-later" - make cross-contract call which resolves with `true`
    /// * "keep-it-later" - make cross-contract call which resolves with `false`
    /// Otherwise panics, which should also return token to `sender_id`
    ///
    fn sft_on_transfer(
        &mut self,
        sender_id: AccountId,
        token_ids: Vec<TokenId>,
        amounts: Vec<U128>,
        msg: String,
    ) -> PromiseOrValue<Vec<U128>> {
        PromiseOrValue::Value(amounts)
    }
    // Verifying that we were called by non-fungible token contract that we expect.
    /*
        assert_eq!(
            &env::predecessor_account_id(),
            &self.semi_fungible_token_account_id,
            "Only supports the one semi-fungible token contract"
        );
        log!(
            "in sft_on_transfer; sender_id={}, previous_owner_id={}, token_id={}, msg={}",
            &sender_id,
            &previous_owner_id,
            &token_id,
            msg
        );
        match msg.as_str() {
            "return-it-now" => PromiseOrValue::Value(true),
            "return-it-later" => {
                let prepaid_gas = env::prepaid_gas();
                let account_id = env::current_account_id();
                ext_self::ok_go(
                    true,
                    &account_id,
                    NO_DEPOSIT,
                    prepaid_gas - GAS_FOR_SFT_ON_TRANSFER,
                )
                .into()
            }
            "keep-it-now" => PromiseOrValue::Value(false),
            "keep-it-later" => {
                let prepaid_gas = env::prepaid_gas();
                let account_id = env::current_account_id();
                ext_self::ok_go(
                    false,
                    &account_id,
                    NO_DEPOSIT,
                    prepaid_gas - GAS_FOR_SFT_ON_TRANSFER,
                )
                .into()
            }
            _ => env::panic(b"unsupported msg"),
        }
    }*/
}

#[near_bindgen]
impl ValueReturnTrait for TokenReceiver {
    fn ok_go(&self, return_it: Vec<U128>) -> PromiseOrValue<Vec<U128>> {
        log!("in ok_go, return_it={}", return_it.len());
        PromiseOrValue::Value(return_it)
    }
}
