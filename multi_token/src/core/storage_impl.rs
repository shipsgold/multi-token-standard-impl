use crate::storage_management::{StorageBalance, StorageBalanceBounds, StorageManagement};
use crate::MultiToken;
use crate::{TokenId, TokenType};
use near_sdk::json_types::U128;
use near_sdk::{assert_one_yocto, env, AccountId, Balance, Promise};

impl MultiToken {
  pub fn internal_storage_unregister(
    &mut self,
    token_id: TokenId,
    force: Option<bool>,
  ) -> Option<(AccountId, Balance)> {
    assert_one_yocto();
    let account_id = env::predecessor_account_id();
    let force = force.unwrap_or(false);
    let token_type = self
      .token_type_index
      .get(&token_id)
      .unwrap_or_else(|| env::panic_str(format!("Could not find token {}", token_id).as_str()));
    if token_type == TokenType::Nft {
      return None;
    }
    let balance =
      self.ft_owners_by_id.get(&token_id).unwrap().get(&account_id).unwrap_or_else(|| {
        env::panic_str(format!("Could not find token_id: {} owner for {}", token_id, account_id).as_str())
      });
    if balance == 0 || force {
      self.ft_owners_by_id.get(&token_id).unwrap().remove(&account_id);
      let updated_supply = self.ft_token_supply_by_id.get(&token_id).unwrap() - balance;
      self.ft_token_supply_by_id.insert(&token_id, &updated_supply);
      Promise::new(account_id.clone())
        .transfer(self.internal_storage_balance_bounds(&token_id, None).min.0 + 1);
      Some((account_id, balance))
    } else {
      env::panic_str("Can't unregister the account with the positive balance without force")
    }
  }

  /// Internal method that returns the Account ID and the balance in case the account was
  /// unregistered.
  /// TODO double check logic here and make more efficient
  pub fn internal_storage_unregister_batch(
    &mut self,
    token_ids: Vec<TokenId>,
    force: Option<bool>,
  ) -> Vec<Option<(AccountId, Balance)>> {
    assert_one_yocto();
    token_ids
      .iter()
      .map(|token_id| self.internal_storage_unregister(token_id.clone(), force))
      .collect()
  }

  fn internal_storage_balance_bounds(
    &self,
    token_id: &TokenId,
    account_id: Option<AccountId>,
  ) -> StorageBalanceBounds {
    let token_type = self
      .token_type_index
      .get(&token_id)
      .unwrap_or_else(|| env::panic_str(format!("Token id {} not found", token_id).as_str()));
    let no_storage_bound = StorageBalanceBounds { min: 0.into(), max: Some(0.into()) };

    if token_type == TokenType::Nft {
      return no_storage_bound;
    }

    if let Some(acct) = account_id {
      if self.ft_owners_by_id.get(&token_id).unwrap().get(&acct).is_some() {
        return no_storage_bound;
      }
    }
    let required_storage_balance =
      Balance::from(self.ft_account_storage_usage) * env::storage_byte_cost();
    StorageBalanceBounds {
      min: required_storage_balance.into(),
      max: Some(required_storage_balance.into()),
    }
  }

  pub fn internal_storage_balance_bounds_batch(
    &self,
    token_ids: &[TokenId],
    account_id: Option<AccountId>,
  ) -> StorageBalanceBounds {
    let mut min_storage: u128 = 0;
    let mut max_storage: u128 = 0;

    token_ids.iter().for_each(|token_id| {
      let bound = self.internal_storage_balance_bounds(token_id, account_id.clone());
      min_storage += u128::from(bound.min);
      max_storage += u128::from(bound.max.unwrap_or_else(|| 0.into()));
    });
    StorageBalanceBounds { min: min_storage.into(), max: Some(max_storage.into()) }
  }

  pub fn internal_storage_balance_of(
    &self,
    token_id: TokenId,
    account_id: &AccountId,
  ) -> Option<StorageBalance> {
    let token_type = self
      .token_type_index
      .get(&token_id)
      .unwrap_or_else(|| env::panic_str(format!("Could not find token_id {}", token_id).as_str()));
    if token_type == TokenType::Nft {
      return None;
    }
    let min_storage = self.internal_storage_balance_bounds(&token_id, None).min;
    if self.ft_owners_by_id.get(&token_id).unwrap().contains_key(account_id) {
      Some(StorageBalance { total: min_storage, available: 0.into() })
    } else {
      None
    }
  }

  pub fn internal_storage_balance_of_batch(
    &self,
    token_ids: &[TokenId],
    account_id: &AccountId,
  ) -> Option<StorageBalance> {
    let mut total: u128 = 0;
    token_ids.iter().for_each(|token_id| {
      if let Some(balance) = self.internal_storage_balance_of(token_id.clone(), account_id) {
        total += u128::from(balance.total);
      };
    });
    if total == 0 {
      return None;
    }
    Some(StorageBalance { total: total.into(), available: 0.into() })
  }
}

impl StorageManagement for MultiToken {
  // `registration_only` doesn't affect the implementation for vanilla fungible token.
  // TODO make more efficient
  #[allow(unused_variables)]
  fn storage_deposit(
    &mut self,
    token_ids: Vec<TokenId>,
    account_id: Option<AccountId>,
    registration_only: Option<bool>,
  ) -> StorageBalance {
    let amount: Balance = env::attached_deposit();
    let account_id = account_id.unwrap_or_else(env::predecessor_account_id);
    let min_balance =
      self.internal_storage_balance_bounds_batch(&token_ids, Some(account_id.clone())).min.0;
    if amount < min_balance {
      env::panic_str("The attached deposit is less than the minimum storage balance");
    }
    token_ids.iter().for_each(|token_id| {
      self.internal_register_account(token_id.clone(), &account_id);
    });
    let refund = amount - min_balance;
    if refund > 0 {
      Promise::new(env::predecessor_account_id()).transfer(refund);
    }
    self.internal_storage_balance_of_batch(&token_ids, &account_id).unwrap()
  }

  /// While storage_withdraw normally allows the caller to retrieve `available` balance, the basic
  /// Fungible Token implementation sets storage_balance_bounds.min == storage_balance_bounds.max,
  /// which means available balance will always be 0. So this implementation:
  /// * panics if `amount > 0`
  /// * never transfers Ⓝ to caller
  /// * returns a `storage_balance` struct if `amount` is 0
  fn storage_withdraw(&mut self, token_ids: Vec<TokenId>, amount: Option<U128>) -> StorageBalance {
    assert_one_yocto();
    let predecessor_account_id = env::predecessor_account_id();
    if let Some(storage_balance) =
      self.internal_storage_balance_of_batch(&token_ids, &predecessor_account_id)
    {
      match amount {
        Some(amount) if amount.0 > 0 => {
          env::panic_str("The amount is greater than the available storage balance");
        }
        _ => storage_balance,
      }
    } else {
      env::panic_str(format!("The account {} is not registered", &predecessor_account_id).as_ref());
    }
  }

  fn storage_unregister(&mut self, token_ids: Vec<TokenId>, force: Option<bool>) -> Vec<bool> {
    token_ids
      .iter()
      .map(|token_id| self.internal_storage_unregister(token_id.clone(), force).is_some())
      .collect()
  }
  // Storage requirements for TokenIds associated with NFT times are waved
  // as the cost of storage is handled at minting
  // account_id is to ignore the cost of your account id if it exists
  fn storage_balance_bounds(
    &self,
    token_ids: Vec<TokenId>,
    account_id: Option<AccountId>,
  ) -> StorageBalanceBounds {
    let mut min_storage: u128 = 0;
    let mut max_storage: u128 = 0;

    token_ids.iter().for_each(|token_id| {
      let bound = self.internal_storage_balance_bounds(token_id, account_id.clone());
      min_storage += u128::from(bound.min);
      max_storage += u128::from(bound.max.unwrap_or_else(|| 0.into()));
    });
    StorageBalanceBounds { min: min_storage.into(), max: Some(max_storage.into()) }
  }

  fn storage_balance_of(
    &self,
    token_ids: Vec<TokenId>,
    account_id: AccountId,
  ) -> Option<StorageBalance> {
    self.internal_storage_balance_of_batch(&token_ids, &account_id)
  }
}
