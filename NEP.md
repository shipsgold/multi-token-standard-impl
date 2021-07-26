- Proposal Name: multi-token
- Start Date: 2021/07/24

# Summary
[summary]: #summary

A standard interface for  a multi token standard that supports fungible, semi-fungible, and tokens of any type, allowing for ownership, transfer, and batch transfer of tokens generally regardless of specific type.

# Motivation
[motivation]: #motivation
Having a single contract represent both NFTs and FTs can greatly improve efficiency as demonstrated by Enjin Coin. The ability to make batch requests with multiple asset classes can reduce a many transactions transaciton to a single transaction to trade around both NFTs and FTs that are a part of same token contract.

Having this will also increase Near's ability to work interoperably with other chains. This will reduce the complexity required to represent these emerging asset classes.


Prior art:
- EIP-1155 : https://github.com/ethereum/EIPs/issues/1155

- This NEP derives some examples and format from: https://github.com/near/NEPs/pull/21

- Example of an NFT Series: https://github.com/near-apps/gnr8

- Things from the NFT Discussions: https://github.com/near/NEPs/discussions/171

- Things from the NFT Discussions: https://gov.near.org/t/nft-standard-discussion/853

Discussions out of band:
 - https://gov.near.org/t/multi-token-standard-discussion/2917
 - https://github.com/shipsgold/multi-token-standard-impl/tree/main/meetings 

# Guide-level explanation
[guide-level-explanation]: #guide-level-explanation

We should be able to do the following:
- Get balance of token per user
- Get balance in batch of a user
- Get supply of tokens per id
- Get supply of tokens in batch per id
- Represent non fungibility of tokens
- Represent fungibility of tokens
- Transfer tokens by id in batch
- Use these tokens on an exchange
- Refund storage costs for fungible tokens

There are a few concepts in the scenarios above:
- **Total supply**. It's the total number of tokens in circulation.
- **Balance owner**. An account ID that owns some amount of tokens.
- **Transfer**. Moves some amount from one account to another account.
- **Fungibility**. An indistinguishable amount of tokens to exchange 
- **Non Fungibility**. Tokens that are differentiable from each other.

### Real scenarios
 
#### Simple transfer

Alice wants to send 5 gold tokens to Bob.

Let's assume the following:
- The `gold` token is represented by the `games` with token_id `g133`.
- Alice's account is `alice`.
- Bob's account is `bob`.
- The precision for `gold` on the games contract is `10^8`. 
- The 5 tokens is `5 * 10^8` or as a number is `500000000`. 

High-level explanation:

Alice needs to issue one transaction to `games` contract to transfer 5 tokens (multiplied by precision) to Bob.
 
Technical calls:

1. `alice` calls `games::mt_transfer({"receiver_id": "bob", "amount": "500000000", "token_id": "g133"})`.

#### Simple batch transfer

Alice wants to send and a 1 unique(nft) gemstone, 5 gold and 10 silver tokens to Bob.

Let's assume the following:
- The unique nft `gem` token is represented by `games` with token_id
`uu2`
- The `gold` token is represented by the `games` with token_id `g133`.
- The `silver` token is represented by the `games` with token_id `s133`.
- Alice's account is `alice`.
- Bob's account is `bob`.
- The precision for `gold` on the games contract is `10^8`. 
- The precision for `silver` on the games contract is also `10^8`. 
- The 5 gold tokens is `5 * 10^8` or as a number is `500000000`. 
- The 10 silver tokens is `10 * 10^8` or as a number is `1000000000`. 
- The  1 gem token is `1` or as a number is `1`

High-level explanation:

Alice needs to issue one transaction to `games` contract to transfer 5 gold tokens and 10 silver tokens (multiplied by precision) and 1 gem to Bob.
 
Technical calls:

1. `alice` calls `games::mt_transfer_batch({"receiver_id": "bob", "amounts": ["500000000", "1000000000", "1"], "token_ids": ["g133", "s133", "uu2"]})`.



#### Token deposit to a contract 

Alice wants to deposit `gold` tokens to a compound interest contract to earn some rewards.

Let's assume the following:
- The `gold` token is represented  by the `games` contract with token_id `g133` .
- Alice's account is `alice`.
- The compound interest contract is `compound`.
- The precision on `gold` token is `10^18`. 
- The 1000 tokens is `1000 * 10^18` or as a number is `1000000000000000000000`. 
- The compound contract can work with many different token contracts and types.

High-level explanation:

Alice needs to issue a single transaction to `games` that will internally issue a cross contract call to `compound`. 

The initial transaction to `games` is made with `compound` as the receiver of a set token_ids and amounts from `alice`.

This call then waits on a response from `compound`. If `compound` responds with failure, the tx is aborted. 

Otherwise `games` contract accepts the results and resolves the promise completing the transaction.

- If transfer succeeded, `compound` can increase local ownership for `alice` to 1000 for `gold` token_id `g133`

- If transfer fails, `compound` doesn't need to do anything in current example, but maybe can notify `alice` of unsuccessful transfer.

Technical calls:
1. `alice` calls `games::mt_transfer_call({"receiver_id": "compound", amount: "1000000000000000000000", "token_id": "g133", msg: "interest-building"})`.
   During the `mt_transfer_call` call, `compound` does the following:
     fn mt_on_transfer(
        &mut self,
        sender_id: AccountId,
        token_ids: Vec<TokenId>,
        amounts: Vec<U128>,
        msg: String,
    ) -> PromiseOrValue<Vec<U128>>;
}
    1. calls `compound::mt_on_transfer({"sender_id": "alice", "token_ids":["g133"], "amounts": ["1000000000000000000000"], msg: "interest-building"})`.
    2. `compound` resolves the request/fails and `games` contract handles response from the promise with `games::mt_resolve_transfer` returning refunded amount if there is any or handling follow up from the result of compound cross contract call

#### Batch Token deposit to a contract 

Alice wants to deposit `silver` and `gold` tokens and the nft `gem` to a compound interest contract to earn some rewards. 

Let's assume the following:
- The `gold` token is represented  by the `games` contract with token_id `g133` .
- The `silver` token is represented  by the `games` contract with token_id `s133` .
- The `gem` unique only one nft token is represented  by the `games` contract with token_id `uu2` .
- Alice's account is `alice`.
- The compound interest contract is `compound`.
- The precision on `gold` token is `10^18`. 
- The precision on `silver` token is `10^18`. 
- The 1000 tokens is `1000 * 10^18` or as a number is `1000000000000000000000`. 
- The compound contract can work with many different token contracts and types.

High-level explanation:

Alice needs to issue a single transaction to `games` that will internally issue a cross contract call to `compound`. 

The initial transaction to `games` is made with `compound` as the receiver of a set token_ids and amounts from `alice`.

This call then waits on a response from `compound`. If `compound` responds with failure, the tx is aborted. 

Otherwise `games` contract accepts the results and resolves the promise completing the transaction.

- If transfer succeeded, `compound` can increase local ownership for `alice` to 1000 for `gold` token_id `g133`

- If transfer fails, `compound` doesn't need to do anything in current example, but maybe can notify `alice` of unsuccessful transfer.

Technical calls:
1. `alice` calls `games::mt_transfer_batch_call({"receiver_id": "compound", amounts: ["1000000000000000000000","1000000000000000000000", "1"], "token_ids": ["g133","s133","uu2"], msg: "interest-building"})`.
   During the `mt_transfer_call` call, `compound` does the following:
     fn mt_on_transfer(
        &mut self,
        sender_id: AccountId,
        token_ids: Vec<TokenId>,
        amounts: Vec<U128>,
        msg: String,
    ) -> PromiseOrValue<Vec<U128>>;
}
    1. calls `compound::mt_on_transfer({"sender_id": "alice", amounts: ["1000000000000000000000","1000000000000000000000", "1"], "token_ids": ["g133","s133","uu2"], msg: "interest-building"})`
    2. `compound` resolves the request/fails and `games` contract handles response from the promise with `games::mt_resolve_transfer` returning refunded amount if there is any or handling follow up from the result of compound cross contract call

# Reference-level explanation
[reference-level-explanation]: #reference-level-explanation
WIP implementation: https://github.com/shipsgold/multi-token-standard-impl/tree/feat/initial-token
### Core Trait
```
pub trait MultiTokenCore {
    /// Basic token transfer. Transfer a token or tokens given a token_id. The token id can correspond to  
    /// either a NonFungibleToken or Fungible Token this is differeniated by the implementation.
    ///
    /// Requirements
    /// * Caller of the method must attach a deposit of 1 yoctoⓃ for security purposes
    /// * Contract MUST panic if called by someone other than token owner or,
    /// * If using Approval Management, contract MUST nullify approved accounts on
    ///   successful transfer.
    /// * TODO: needed? Both accounts must be registered with the contract for transfer to
    ///   succeed. See see https://nomicon.io/Standards/StorageManagement.html
    ///
    /// Arguments:
    /// * `receiver_id`: the valid NEAR account receiving the token
    /// * `token_id`: the token or tokens to transfer
    /// * `amount`: the token amount of tokens to transfer for token_id
    /// * `memo` (optional): for use cases that may benefit from indexing or
    ///    providing information for a transfer
    fn mt_transfer(
        &mut self,
        receiver_id: AccountId,
        token_id: TokenId,
        amount: U128,
        memo: Option<String>,
    );

    /// Transfer token/s and call a method on a receiver contract. A successful
    /// workflow will end in a success execution outcome to the callback on the MultiToken
    /// contract at the method `mt_resolve_transfer`.
    ///
    /// You can think of this as being similar to attaching  tokens to a
    /// function call. It allows you to attach any Fungible or Non Fungible Token in a call to a
    /// receiver contract.
    ///
    /// Requirements:
    /// * Caller of the method must attach a deposit of 1 yoctoⓃ for security
    ///   purposes
    /// * Contract MUST panic if called by someone other than token owner or,
    ///   if using Approval Management, one of the approved accounts
    /// * The receiving contract must implement `mt_on_transfer` according to the
    ///   standard. If it does not, MultiToken contract's `mt_resolve_transfer` MUST deal
    ///   with the resulting failed cross-contract call and roll back the transfer.
    /// * Contract MUST implement the behavior described in `mt_resolve_transfer`
    ///
    /// Arguments:
    /// * `receiver_id`: the valid NEAR account receiving the token.
    /// * `token_id`: the token to send.
    /// * `amount`: amount of tokens to transfer for token_id
    /// * `memo` (optional): for use cases that may benefit from indexing or
    ///    providing information for a transfer.
    /// * `msg`: specifies information needed by the receiving contract in
    ///    order to properly handle the transfer. Can indicate both a function to
    ///    call and the parameters to pass to that function.
    fn mt_transfer_call(
        &mut self,
        receiver_id: AccountId,
        token_id: TokenId,
        amount: U128,
        memo: Option<String>,
        msg: String,
    ) -> PromiseOrValue<U128>;

    /// Batch token transfer. Transfer a tokens given token_ids and amounts. The token ids can correspond to  
    /// either Non-Fungible Tokens or Fungible Tokens or some combination of the two. The token ids
    /// are used to segment the types on a per contract implementation basis.
    ///
    /// Requirements
    /// * Caller of the method must attach a deposit of 1 yoctoⓃ for security purposes
    /// * Contract MUST panic if called by someone other than token owner or,
    ///   if using Approval Management, one of the approved accounts
    /// * `approval_id` is for use with Approval Management,
    ///   see https://nomicon.io/Standards/NonFungibleToken/ApprovalManagement.html
    /// * If using Approval Management, contract MUST nullify approved accounts on
    ///   successful transfer.
    /// * TODO: needed? Both accounts must be registered with the contract for transfer to
    ///   succeed. See see https://nomicon.io/Standards/StorageManagement.html
    /// * The token_ids vec and amounts vec must be of equal length and equate to a 1-1 mapping
    ///   between amount and id. In the event that they do not line up the call should fail
    ///
    /// Arguments:
    /// * `receiver_id`: the valid NEAR account receiving the token
    /// * `token_ids`: the tokens to transfer
    /// * `amounts`: the amount of tokens to transfer for corresponding token_id
    /// * `approval_ids`: expected approval ID. A number smaller than
    ///    2^53, and therefore representable as JSON. See Approval Management
    ///    standard for full explanation. Must have same length as token_ids
    /// * `memo` (optional): for use cases that may benefit from indexing or
    ///    providing information for a transfer

    fn mt_batch_transfer(
        &mut self,
        receiver_id: AccountId,
        token_ids: Vec<TokenId>,
        amounts: Vec<U128>,
        memo: Option<String>,
    );
    /// Batch transfer token/s and call a method on a receiver contract. A successful
    /// workflow will end in a success execution outcome to the callback on the MultiToken
    /// contract at the method `mt_resolve_batch_transfer`.
    ///
    /// You can think of this as being similar to attaching  tokens to a
    /// function call. It allows you to attach any Fungible or Non Fungible Token in a call to a
    /// receiver contract.
    ///
    /// Requirements:
    /// * Caller of the method must attach a deposit of 1 yoctoⓃ for security
    ///   purposes
    /// * Contract MUST panic if called by someone other than token owner or,
    ///   if using Approval Management, one of the approved accounts
    /// * The receiving contract must implement `mt_on_transfer` according to the
    ///   standard. If it does not, MultiToken contract's `mt_resolve_batch_transfer` MUST deal
    ///   with the resulting failed cross-contract call and roll back the transfer.
    /// * Contract MUST implement the behavior described in `mt_resolve_batch_transfer`
    /// * `approval_id` is for use with Approval Management extension, see
    ///   that document for full explanation.
    /// * If using Approval Management, contract MUST nullify approved accounts on
    ///   successful transfer.
    ///
    /// Arguments:
    /// * `receiver_id`: the valid NEAR account receiving the token.
    /// * `token_ids`: the tokens to transfer
    /// * `amounts`: the amount of tokens to transfer for corresponding token_id
    /// * `approval_ids`: expected approval IDs. A number smaller than
    ///    2^53, and therefore representable as JSON. See Approval Management
    ///    standard for full explanation. Must have same length as token_ids
    /// * `memo` (optional): for use cases that may benefit from indexing or
    ///    providing information for a transfer.
    /// * `msg`: specifies information needed by the receiving contract in
    ///    order to properly handle the transfer. Can indicate both a function to
    ///    call and the parameters to pass to that function.

    fn mt_batch_transfer_call(
        &mut self,
        receiver_id: AccountId,
        token_ids: Vec<TokenId>,
        amounts: Vec<U128>,
        memo: Option<String>,
        msg: String,
    ) -> PromiseOrValue<Vec<U128>>;

    /// Get the balance of an an account given token_id. For fungible token returns back amount, for
    /// non fungible token it returns back constant 1.
    fn balance_of(&self, owner_id: AccountId, token_id: TokenId) -> U128;

    /// Get the balances of an an account given token_ids. For fungible token returns back amount, for
    /// non fungible token it returns back constant 1. returns vector of balances corresponding to token_ids
    /// in a 1-1 mapping
    fn balance_of_batch(&self, owner_id: AccountId, token_ids: Vec<TokenId>) -> Vec<U128>;

    /// Returns the total supply of the token in a decimal string representation given token_id.
    fn total_supply(&self, token_id: TokenId) -> U128;

    // Returns the total supplies of the tokens given by token_ids in a decimal string representation.
    fn total_supply_batch(&self, token_ids: Vec<TokenId>) -> Vec<U128>;
}
```
### Receiver Trait
#### Notes
- TokenId is of type String
```
pub trait MultiTokenReceiver {
    /// Take some action after receiving a MultiToken-tokens token
    ///
    /// Requirements:
    /// * Contract MUST restrict calls to this function to a set of whitelisted MultiToken
    ///   contracts
    ///
    /// Arguments:
    /// * `sender_id`: the sender of `mt_transfer_call`
    /// * `previous_owner_id`: the account that owned the tokens prior to it being
    ///   transferred to this contract, which can differ from `sender_id` if using
    ///   Approval Management extension
    /// * `token_ids`: the `token_ids` argument given to `mt_transfer_call`
    /// * `msg`: information necessary for this contract to know how to process the
    ///   request. This may include method names and/or arguments.
    ///
    /// Returns true if tokens should be returned to `sender_id`
    fn mt_on_transfer(
        &mut self,
        sender_id: AccountId,
        token_ids: Vec<TokenId>,
        amounts: Vec<U128>,
        msg: String,
    ) -> PromiseOrValue<Vec<U128>>;
}
```
### Resolver Trait
#### Notes
- TokenId is of type String
```
/// Used when MultiTokens are transferred using `mt_transfer_call`. This is the method that's called after `mt_on_transfer`. This trait is implemented on the MultiToken contract.
pub trait MultiTokenResolver {
    /// Finalize an `mt_transfer_call` chain of cross-contract calls.
    ///
    /// The `mt_transfer_call` process:
    ///
    /// 1. Sender calls `mt_transfer_call` on MultiToken contract
    /// 2. MultiToken contract transfers token from sender to receiver
    /// 3. MultiToken contract calls `mt_on_transfer` on receiver contract
    /// 4+. [receiver contract may make other cross-contract calls]
    /// N. MultiToken contract resolves promise chain with `mt_resolve_transfer`, and may
    ///    transfer token back to sender
    ///
    /// Requirements:
    /// * Contract MUST forbid calls to this function by any account except self
    /// * If promise chain failed, contract MUST revert token transfer
    /// * If promise chain resolves with `true`, contract MUST return token to
    ///   `sender_id`
    ///
    /// Arguments:
    /// * `previous_owner_id`: the owner prior to the call to `mt_transfer_call`
    /// * `receiver_id`: the `receiver_id` argument given to `mt_transfer_call`
    /// * `token_ids`: the `token_ids` argument given to `mt_transfer_call`
    /// * `approvals`: if using Approval Management, contract MUST provide
    ///   set of original approved accounts in this argument, and restore these
    ///   approved accounts in case of revert. In this case it may be multiple sets of approvals
    ///
    /// Returns true if tokens were successfully transferred to `receiver_id`.
    fn mt_resolve_transfer(
        &mut self,
        sender_id: AccountId,
        receiver_id: AccountId,
        token_ids: Vec<TokenId>,
        amounts: Vec<U128>,
    ) -> Vec<U128>;
}
```
### Storage Management Trait
#### Notes
This is semi necessary for ft token types to be able to refund users for storage of many different token types like gold/silver... this might be slightly out of scope
```
pub trait StorageManagement {
    // if `registration_only=true` MUST refund above the minimum balance if the account didn't exist and
    //     refund full deposit if the account exists.
    fn storage_deposit(
        &mut self,
        token_ids: Vec<TokenId>,
        account_id: Option<AccountId>,
        registration_only: Option<bool>,
    ) -> StorageBalance;

    /// Withdraw specified amount of available Ⓝ for predecessor account.
    ///
    /// This method is safe to call. It MUST NOT remove data.
    ///
    /// `amount` is sent as a string representing an unsigned 128-bit integer. If
    /// omitted, contract MUST refund full `available` balance. If `amount` exceeds
    /// predecessor account's available balance, contract MUST panic.
    ///
    /// If predecessor account not registered, contract MUST panic.
    ///
    /// MUST require exactly 1 yoctoNEAR attached balance to prevent restricted
    /// function-call access-key call (UX wallet security)
    ///
    /// Returns the StorageBalance structure showing updated balances.
    fn storage_withdraw(&mut self, token_ids:Vec<TokenId>, amount: Option<U128>) -> StorageBalance;

    /// Unregisters the predecessor account and returns the storage NEAR deposit back.
    ///
    /// If the predecessor account is not registered, the function MUST return `false` without panic.
    ///
    /// If `force=true` the function SHOULD ignore account balances (burn them) and close the account.
    /// Otherwise, MUST panic if caller has a positive registered balance (eg token holdings) or
    ///     the contract doesn't support force unregistration.
    /// MUST require exactly 1 yoctoNEAR attached balance to prevent restricted function-call access-key call
    /// (UX wallet security)
    /// Returns `true` iff the account was unregistered.
    /// Returns `false` iff account was not registered before.
    fn storage_unregister(&mut self, token_ids:Vec<TokenId>, force: Option<bool>) -> Vec<bool>;

    fn storage_balance_bounds(&self, token_id:TokenId, account_id: Option<AccountId>) -> StorageBalanceBounds;
    fn storage_balance_bounds_batch(&self, token_id:Vec<TokenId>, account_id: Option<AccountId>) -> StorageBalanceBounds;

    fn storage_balance_of(&self, token_id:TokenId, account_id: AccountId) -> Option<StorageBalance>;
    fn storage_balance_of_batch(&self, token_ids:Vec<TokenId>, account_id: AccountId) -> Option<StorageBalance>;
}
```

### Metadata Trait
```
pub struct MultiTokenMetadata {
    pub spec: String,              // required, essentially a version like "nft-1.0.0"
    pub name: String,              // required, ex. "Mosaics"
    pub symbol: String,            // required, ex. "MOSIAC"
    pub icon: Option<String>,      // Data URL
    pub base_uri: Option<String>, // Centralized gateway known to have reliable access to decentralized storage assets referenced by `reference` or `media` URLs
    // supports metadata_uri interface that interpolates {id} in the string
    pub reference: Option<String>, // URL to a JSON file with more info
    pub reference_hash: Option<Base64VecU8>, // Base64-encoded sha256 hash of JSON from reference field. Required if `reference` is included.
}
/// Offers details on the contract-level metadata.
pub trait MultiTokenMetadataProvider {
    fn mt_metadata(&self) -> MultiTokenMetadata;
}
```



# Drawbacks
[drawbacks]: #drawbacks

Why should we *not* do this?

# Rationale and alternatives
[rationale-and-alternatives]: #rationale-and-alternatives

- Why is this design the best in the space of possible designs?
- What other designs have been considered and what is the rationale for not choosing them?
- What is the impact of not doing this?

# Unresolved questions
[unresolved-questions]: #unresolved-questions

- What parts of the design do you expect to resolve through the NEP process before this gets merged?
- What parts of the design do you expect to resolve through the implementation of this feature before stabilization?
- What related issues do you consider out of scope for this NEP that could be addressed in the future independently of the solution that comes out of this NEP?

# Future possibilities
[future-possibilities]: #future-possibilities

Think about what the natural extension and evolution of your proposal would
be and how it would affect the project as a whole in a holistic
way. Try to use this section as a tool to more fully consider all possible
interactions with the project in your proposal.
Also consider how the this all fits into the roadmap for the project
and of the relevant sub-team.

This is also a good place to "dump ideas", if they are out of scope for the
NEP you are writing but otherwise related.

If you have tried and cannot think of any future possibilities,
you may simply state that you cannot think of anything.

Note that having something written down in the future-possibilities section
is not a reason to accept the current or a future NEP. Such notes should be
in the section on motivation or rationale in this or subsequent NEPs.
The section merely provides additional information.