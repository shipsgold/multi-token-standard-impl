### 08-17-21 Multi Token Standard Meeting Notes

#### Attendees

- zcstarr - Ships
- riqi - Paras
- irfi - Paras
- mike - Near
- marcos.sun - Ref

### Welcome Intro
**@zcstarr** - Welcome multi token standard meeting, we're going to try and come to some consesnus and just go over all the remaining outstanding questions 
in the meeting and figure what the next steps should be 

**@zcstarr**: what do we think about the NEP? Let's start with Marcos comments 


### Github NEP PR Comments
**@zcstarr**: Marco we agree with all the suggestions to consolidate the functions for hte most part resolving around , 
but we had a question about the need for BaseVec<U8> based memos. https://github.com/near/NEPs/pull/245 

#### Vec<u8> memos
**@marco**: After the last meeting it became clear that we could acheive everything with string so no need to change the interface 

#### Have batch versions of mt_transfer or not  
**@zcstarr**: The other comment was on mt_transfer_* to instead of having multiple function to instead just have one call that
  takes Vector arrays.
**@marco**: I understand why we'd want two methods, maybe to have bet similar to erc-1155 and developer familiarity. It's just a question here.

**@zcstarr**: Yeah I think so as well, I'm not beholden to it, but I suppose there could be some gas saving from doing one call over the other.
  And familiarity is really nice here. So let's open it up to the group not held to it.

**@riqi**: I think it's possible to just have one method, but for everyone else we're looking at familiarity with the methods from
  erc-1155 so keeping it as parallel makes sense.

**@mike**: I think similar to 1155 as possible so there's less cognitive load on the developer migrating to NEAR
  
**@irfi**: From paras, I'm with everyone else because many people have familiarity with ethereum and it's simple enough to implement
  
**@zcstarr**: I did some consolidation of logic so on_transfer and resolve_ any objections to that // none were noted.

#### storage_withdraw 1yocto on storage management
  
**@zcstarr**: what was this comment about storage_withdraw ?  

**@marco**: Yes the idea was to remove this withdraw 1yocto, because it's more friction for the user,
  the user might have this deposit so it just can't happen for the user. And even if the users key is stolen
  then only the user's storage balance is given back to them is it really necessary?
@zane: I can't think of a case of which we'd create any additional storage what would be the case for an additional withdraw?

**@mike**: So the way it works is I can use the storage management to pay for a users storage or the user can pay for the storage, and after some time
  if they're no longer using it they can just withdraw, and claim back the near they used to create the storage. The only person that gets the near is 
  the user who had the storage allocated for them. 

**@zcstarr**: Marco's question is do we actually need the assert 1yocto  

**@mike**: So this is done so that for all the methods where you need some extra confirmation that you want this to happen, we're going to make sure that not just 
 a the function access key is necessary to call this method, but that we trigger a redirect to a wallet so they have to confirm. So it really helps you against
 fishing

**@marco**: I'm not sure if this is going to cause any problem if this function call access key is stolen, but isnt' the worst that can happen is just withdraw to the original users account?
  
**@mike**: if the function call key is stolen, then the worst that will happen is this method will fail because of the deposit. If you remove it, then worse case
is that the storage withdraw function could have the ability to unregister accounts... 
  
**@zcstarr**: If it goes wrong it could really go wrong. I'm in favor of keeping it.

**@marco**: still could be a better ux.
  
**@zcstarr**: I don't this happens that often that people want to withdraw so the friction isn't that high.

**@mike**: we could even have storage withdraw unsafe if you want for your implementation so that's an option..

**@riqi**: Yeah i'm in favor of limiting fishing and things like that ....
  
  
  
#### Storage management Overview (26:00)

**@zcstarr**: This has it's own storage management, because it's just radically different from whats in the standard given all the token ids etc... 
 
**@marco**: Question about storage balance of batch, should we allow this to be negative for available balance. Well if you don't pre register the storage, then
  perhaps people can handle having a balance debt. Just wanting to discuss and share with you all.

**@zcstarr**: I think balance debt just gets way too complicated it's much better for developers to cover the cost and keep track on their own if they do that. 
  I think currently it's not allowed.

**@mike**: A fun way to get around this could be to have a kind of bucket, where it's like take a penny leave a penny, and that accumulates and that is used to then 
  cover another users storage fees. I think having negative balances, there's another set of things that might increase the attack surface area.
  
**@zcstarr**:  I think we answered most questions
  
#### Wallet Indexder (32:04)
**@zcstarr**: Can wallet indexer handle many tokens in a single contract. I did some digging and took a quick look, and it seems like the indexer just has a slurper.
  That takes in data from the chain and then throws it into a postgres database, and then you query against this to extract the args and method_name of the tx.
  The wallet I think just can pull from this to extract the data, and then make calls to metadata. The wallet team would have to d
  
**@mike**: I think that's right and sounds like we landed on total supply on chain
  
**@zcstarr**: right
  
#### Does NEP meet all the things
**@zcstarr**: Overview of the stuff we just did...
  
#### Metadata Extra (40:00)
**@zcstarr**: MultiTokenMetadata and MultiTokenExtraMetdata to split the data up so TokenMetadata for high level stuff, and Extra would be the token metadata for the
  individual token metadata like series.

**@irfi**: what about the reference data in both ways. Can we consolidate?
  
**@zcstarr**: I think you're right we can just consolidate this and only have the reference data once.

**@zcstarr**: how do people feel about the consolidation here? Generally this metadata is just a view on data so no need to 
  
**@mike**: Yeah I agree that metadata is like a view on the data
  
**@marco**: Let's just consolidate the whole thing?


--- General consensus on metadata was met
  
#### Unresolved questions (45:00)

 **@zcstarr**: Spec for offchain metadata, not so important we have defined spec atm. Should we have a spec for token type, that's going to remain an impl. detail.
  Events are representable in the spec yet .  And for the approval system question, _call is currently good enough, and approval management can happen later.
  
 **@zcstarr**: No unanswered questions left if there's anything else we'll do another last pass.
  
#### Wallet and FT or NFT Type (52:00)
 
 **@riqi**: Let's say what's the best way for a wallet or consumers to represent the NFT or FT in a persons collectibles, we don't really have a way to distinguish 
  this should this be with decimals, or should this be with the media field, what would wallet team or people do if they want to know like OpenSea or Rariable. 
  How do we say this one is collectibles and how do we say this one is fungible?
  
  **@zcstarr**: I don't think we can actually know , just because it could have any kind of meaning 100% of the time necessarily. You can infer it but 
  
  **@marco**: Do you know why I wanted the decimals because I want to say this is a decimal it must be an ft vs NFT. 
  
  **@zcstarr**: Yeah we just have alot of fuzzy signals, you can check supply you can have decimals to an NFT but decimals sounds like a thing that's mostly fungible thing
  ,but only thing you can do is to just have deicmal. Let's work backwards riqi what do you want to have happen.
  
 **@riqi**: For our usecase we want to allow users to create just the nft or to have a collectibles like rariable so they show these as collectibles, so a way to signal what what would be useful.
  
 **@zcstarr**: The only way we have knowledge of fungibility is to know that there is more than 1 of them. Otherwise it's hard to know. You could mint 1 fungible token
  and that doesn't make it an NFT. It's always up to the implenter how they want to handle things always ultimately.
  
 **@zcstarr**: In my mind there are only two token types we could create a function to explicitly type tokens as FT,NFT, and String, and if we make this a first class
  citizen something like mt_token_type(token_id:TokenId)-> TokenType . Then we could be explict about what represent, and how to represent them, but really token type is 
  a bit of a bleed from the implementation. I don't htink the community would be really on board with a token type ultimately that limited the description to FT/NFT.
  
  **@zcstarr**: So how do people feel about this?
  
  @all: nodding heads. 
  
  **@riqi**: Well how do wallets on ethereum support erc-1155 how do they show them ?
  
  
  **@mike**: leaves just has to go ...

  #### Metadata Extra the return see time(1:07:00)
  
  **@marco**: Everything is optional in the metadata extra should we just merge them into one.
  
  **@zcstarr**: yeah I think you're right. How do people feel about this one metadata one metadata interface. 
  
  **@riqi**: Agrees make it a single struct, and then wallet team could just optionally show it as a collectible and can toggle on any of the fields that make
  sense to them.
  
  **@marco**: We could also just add additional functions to show the whole thing and then the individual items.
  
  **@zcstarr**: we could do that, and then look ath toptional mt_extradata fiel
  
  **@riqi**: but lets just merge into a single struct then it's one call for everything
  
  **@irfi**: It's possible that media and icon are not enough to distinguish
  
  **@zcstarr**: shoudl we just have token_type field, that can be nft ft or string.
  
  **@riqi**: adding token type makes things more complicated, and we have extra which means you can put anything it
  so let's just say that the implementer can represnet the token data. I think the implementer of the consumption which is up to the wallet team on how they 
  want to represent the tokens on wallet.
  
  **@zcstarr**: Yeah I think we would cut this back to just be the metadata?
  
  **@riqi**: So we can just keep this to metadata, and that token type can be determined by the consumer. And how you say it's a collectible can be adifferent community 
  standard , maybe even in extra data. So it can happen in the near wallet. It's a set for collectibles nomenclature... . So let's keep it simple
  
  **@zcstarr**: So one token metadata mega field and then they can represent that however they want on their implementation side. So we all agree to simplify.
  
  **@riqi**: Some other wallets have to infer to so it's cool
  
  General agreement was reached.
  
  #### Summary
  We refactored metadata to just be one method and contain all the entries for the metadata. We voted against any kind of token type, and resolved outstanding
  PR issues. If there are no dissenting voices we're ready to finalize. We'll be circulating this around.
  
 
