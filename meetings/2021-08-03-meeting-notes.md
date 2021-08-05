### 08-03-21 Multi Token Standard Meeting Notes

#### Attendees

- zcstarr - Ships
- riqi - Paras
- matt - Near

### This meeting want
- Understand if the NEP as is covers the immediate use cases.
- Get clear on the traits for the core implementation
- Reach soft consensus on this to set the stage for impl discussion.

@zane: Welcome! We're just going through multi token standard today
and going through issues and walk through the spec and pause and ask questions around the way. just looking to get clear on the core implementaton and try and reach a soft consensus on this and talk about some implementation details and thoughts.

### NEP PR
@zane: What are we looking for? We're looking to support multi token types such as NFT, Semi Fungible Tokens, Fungible Tokens, and everyting in between and to be able to transfer tokens in bulk and receive tokens in bulk. The impetus is from cross chain compatibility, and some of the use cases that come from video games and happenings, one example of a success story is Enjin coin, and it's popularity on the ETH.

@zane: Alot of the background is covered in the NEPs list, starting at the top we ave the guidelevel explanation of things.
### Guidelevel explanation Overview
#### Get balance of token per user
#### Get balance in batch of a user tokens have balances like 
@zane: By these we mean given a token id be able to determine this token-id and user has 500 of token_id a, 1 of token_id b, 7000 of token_id c... 
#### Get supply of tokens per id
#### Get supply of tokens in batch per id
   @zane: This is controversial because it's unclear if we need supply if we had more logging we might consider not having a supply, but the infrastructure really isn't in place and with supply we get alot of benefits. It's to get the number of supply per token_id.
#### Represent non fungibility of tokens
@zane: Want to represent that there is only 1 of these things
#### Represent fungibility of tokens
@zane: We want to be able to represent there are n of these things
#### Transfer tokens by id in batch
@zane: We want to be able to to send and receive these tokens in batch
#### Use these tokens on an exchange
@zane: We want there to be enough data to be able to use this on an exchange
#### Refund storage costs for fungible tokens
@zane: What this means is that we have each on of these balances associated with an account and that account probably has some storage requirements, and we just want to be able to give this back to the user if say they no longer have any gold, silver, platinum coins...
#### Questions about the guidelevel (8:00)
@riqi: Should the token supply be a function of the metadata, whenever you're talking about the supply, saying that this token is fungible, nonfungible in the metadata, should there be some sort of tie in there. Is that natural? Why not just have one method that includes all the metadata that also includes the supply?

@zane: By tying the supply to the metadata, it kidn of means the token must have metadata, and it's not clear that's a tie in that we want. If you wanted to represent some token without metadata, then you might not be able to do that. 

Secondly, there's a tension point that  happens when you have the supply tied to the metadata. Supply has this kidn of meaning of being a kind of sanity measure that helps you to verify that you internally aren't doing something bad. So you can use it to guarantee you're not transfering more tokens than are minted etc... . I think it's just a concept that should stand alone and we have the sort of space for it I think.

In erc-1155 they dont' hit this because they use events and they say you the consumer are resonsible for any kind of supply constraints or relaying that information, via aggregating events. and there are no supply checks. 

### Metadata Digression (12:06)
@zane: So the metadata trait consists of two kinds of trait
MultiTokenMetadata (the old kind of the contract equivalence metadata) which now maps to token_id. We also have MultiTokenMetadataExtra which maps to Token specific metadata so if you wanted to represent a series or you had additional metadata that's not generally common like it would be for an ft or nft token it would go in the sort of extra pile. In this extra pile the implementer would be responsible for sharing data or resolving what gets displayed within the MetadataProvider traits. mt_metadata(token_id) and mt_metadata_extra(token_id)

@matt: wanted to clarify a few points with the series stuff. So riqi you guys have numbered versions of a card, and you'd like to sell these. So each card is an nft and they have a unique token_id, you could have a sort of metadata pointer where they all point back to the same metadata, without having to have copies of the Metadata floating around so how do we achieve this with this current interface shape? It's expensive to shuffle around the same metadata. The best way in my mind is to be able to share this metadata when that makes sense. So it's like imagine selling 50000 concert tickets and then ahving thes same metadata for all of them you would really want to do that you could save alot of tx without having duplicate data so pointers to metadata make alot of sense.

@zane: right so that was the thought behind the splitting of the mt_metadata and the mt_metadata_extra, where contract level metadata is relayed with mt_metdata, and specific metadata_extra is held in that field. So at the implementation level users can then figure out how they want to relay and store this information in an efficient way, but the standard doesn't have to define how to share data, but just support that level of flexibility. Without the pair of methods it's hard to really get or relay this kind of data to the consuming interfaces imo.Not stuck on it but it seems nice to have.

@riqi: I think this is a good implementation, because this is kind of creating a new ft or nft inside the token itself instead of deploying a new contract, vs deploying a new contract costs lots of near and if you're an artist you don't want to pay that much. So each new minting basically will have its metadata stored with this impl, and this will help keep the ft and mt in the same contract. The metadata being the same as ft and nft is smart, but I'm concerned about how wallet team or people will index this. So how will they know about what token this one contract has. 

@zane: Let's bring this up with daryl, I think this will be easy to edit, I'll put on my bs. wallet engineering hat and say that because this uses the standard, that the wallet team will be able to pull this data directly from the contract data. I speculate you watch the blocks and look at the contract and tx for the contracts, and wallet team has away of knowing what this contract is. Matt clarity there?

@matt: What the team basically does for knowing the contract is looking at the method names, so that's why all the contracts are all ft_, nft_ prefixed methods. So the wallet team uses the indexer, and every contract that has a standard the blockchain method arguments are flatenned out and allows you to know the method_name and the receiver of that method and so it allows you to derive what is what in terms of the contract. You can also look for storage deposits and then infer that storage deposits and look for ft or nft.

@zane: I think from ethereum, people just use events. So all these events are rlp encoded, and the events are with the block data and then you can retreive the blocks and apply a bloom filter and you can get back the data.  There's some hairy ness with the prefixes for method names so it's like sha3 hash but then only the first 4bytes so without having a 4byte database you don't have to know what exactly the name of hte method that you're calling is. So you have to kind of know what the thing is before knowing. There's decompiler and you can kind of do it but there's no dest or jump table so you have to do things like play the data forward to figure out what exactly is happening. But the thing that's nice is the events and so this would probably be nice to have.

@riqi: Should there be contract metadata for the multi token contract itself? 

@zane: I don't actually know if we do we could, but currently we don't

@matt: One more thought that the multi token metadata would be the media and the hash and the tokens, so we're looking at a contract and in this contract there could be many types of tokens, and so zane, matt and riqi tokens could have different combinations of tokens. So who gets the metadata? 

@zane: I'd say that the zane, matt tokens are fungible tokens the riqi is a nft token . You could mint some of the tokens with metadata and some without and then as the implementer I would just simply map the token ids to the parent token id which would allow it share data. I think it's an implementation detail, that happens around the standard but not in the standard. The traits just say you have the ability to resolve what's going on with this token id.

@matt: What would you call one image but there's 1 image but there's 100 of them, but there's additional fields like rarity or points, what would you call those types of tokens. 

@zane: I'd call them semi fungible tokens, because similar but ultimately not the same only grouped together. In my head I'd say there are only 2 types of tokens fungible tokens, where there's just literally no difference between anything and non fungible tokens where there is some difference that make it unique, a serial number a physical state of being. I think everything else is sugar on top of these kind of base cases. Standard to say what you get back but not how you store the information.

@matt: yeah exactly it is just what I do for the NFT series just uses the NFT standard for that so the wallet doesn't even really know the difference. 


#### Matt's Metadata Diagram (49:06) 
@matt: draws a diagram of 
![image](https://user-images.githubusercontent.com/173187/128279397-074eaaa7-95fa-4d3c-b79f-2d827b762caa.png)
It's a description of a matrix of different metadata configurations and standards support. 

@matt: So how we represent sharing and dynamic attributes could be an interesting thing to watch out for and to think about, because we'd like to support dynanism wihtin the token metadata. (easier watched)

@zane: technically we have the extra field, but it's not as nice and if we wanted unique token attributes per token, we might have spec or something like that to expand upon it. It's uglly because it'se serialized json in the extra or it's entierly up to the consumer of the data. I think oddly that people consuming the data already know the shape of it so it becomes a little harder to see. I think maybe it's all implementaton I think we can support it but it something we can look out for for sure.

@matt: I think the attributes and having that stored on chain is a really powerful aspect of the metadata that can make for great experiences being able to handle that gracefully. Thinking about dynanism is a good thing to keep in mind. and so it's a little outside the scope, but still good to have in back of the mind. 


#### So far checkin for the metadata(60:00)

@zane how are feeling about things so far ?

@riqi: If you want to mint on rarible or Opensea you get asked what standard you want to use NFT or erc-1155. So if you poll the artist, if it means alot to them they make it an edition as an NFT and mint it 10 times and that's how they appreciate their works and they all have separate entity and it costs alot more gas. If its not as meaningful then they make it an erc-1155 and then mint it like many times. Not sure which one the collectors appreciate. Maybe the nft ones the art lovers like it, but if we talk to gamers they don't care about the edition. For me it can be solved with the mt standard. You can prolly implement something like mat did but make it more efficient with the multi token standard.

@zane: yeah absolutley. I think matt's diagram is correct, that the diagram you need contract level metadata, and token specific metadata, and the shared data is just an implementation detail. And perhaps you need it for the erc-1155 itself.

@riqi: we could even say the spec for the metadata could tag type and say the spec entry could be there, but maybe not . So the spec could be whatever we want for each version of kind of the sub types.

@zane: I think it should be a detail, I think it should all be mt-spec 1 so because we're going at the lib. level I think that we can and should be as minimal as possible, because then everyone consuming knows exactly what is and people can always build on top of it. and if it doesn't work we can always expand later. In beginning we'll prolly be ok with that

@riqi: not the focus but for the games maybe we can break that into a separate contract for the game metadata and have that linked some how.

### Trait Overview (1:11:53)
#### mt_transfer 
@zane: works like transfer not too much there, marcos would like this to be binary it's unclear why, but I could speculate maybe you could make this binary to line up more closely with binary events.

@matt: we use it alot for cross contract calls and for the market place contract. (1:14:00) for explainer. The calls here though are almost always serialized json. vs base64vecu8 

#### mt_batch_transfer
@zane: transfer with batch where the token_ids line up with the amounts.

@matt: some efficiencies happen when you have token_ids that happen in a range. Mintbase mints a batch and whole range of tokens and so if you want to create 10 of a token you create 10 more NFTs for range of 43 to 52 out of 52


@zane: It sounds like they're using integers to get an efficiency gain but the NFT standard and other stnadards talk alot about namespaces, so it seems like better to not tie it to an integer. I think you can support ranges, its not exactly not available.

#### mt_batch_transfer_call 
@zane straight forward works like the other _call methods

#### mt_balance_of_batch
@zane gets balances of a batch of token_ids marcos has suggstion to just make balance_of I'm fine with that

#### mt_supply_of_batch:
@zane gets supply of batch of token ids , marcos also has the same suggestion as above. The issue here is again supply required not supply, we could always have a supply or we could make it optional. Consumers need a way of knowing how many tokens there are and they just need a way to do this, we could use events, but we don't have great events at this point in time. It's just a bit speculative 

@riqi: I just think it's nice to have and easy to do and consumer will prolly need this I think it's good to have

@matt: I think you got it right there I think it's totally legit to put that in there. Standard should do low hanging fruit like this.

#### Receiver mt_on_transfer
@zane: like ft on transfer and this make sense in there. There's no batch because it's easier to just have a batch vs a batch and single transfer less to implement. So the refund amounts are returned for each of the tokenId vectors. so it's one to one corresponding to the order in order out. 

#### Resolver mt_on_resolve_transer
@zane: this is the resolve callback so you can relay the token ids that come in

@riqi: comment in the spec is wrong.

### Storage Management (1:34)
@zane: so depending on the impl , balances and account management, I took inspiration from account management and it looks like ft management with batching so it makes sense to provide users with a unified way to manage this if they need it.

#### storage_deposit
@zane: takes a vec of token ids so you can just register a user for all these ids and it can then reject things that don't need storage management works like the ft_storage_deposit. see spike ahead implementation for inspo.

#### storage_withdraw
@zane: The amount of storage you can withdraw from with the storage balance

#### storage_unregister
@zane: Vec token ids where you can take back your storage for all these vec ids, and what's returned here are the results of doing that if you were successful or not. You can also for a burn for these as well.

#### storage_balance_bounds
@zane: this is what's used to calculate the minimal storage cost to cover and the max you need so it can compute what your storage cost is before you need it. There is a request to consolidate this renaming this to batch

#### storage_balance_of
@zane: this returns the storage balance of the token ids in question and aggregates them to a total storage cost


@riqi: what's the use case for this

@zane: I think you'd use this to be able to say in the case of like game you pay for storage for a bunch assets it's been a few years you no longer have those itmes and now you can save money and refund the user by cleaning up their storage space that they used for say things not in use it's a way to refund the user, but wholly up to the application developer to impl.

### How do we feel about this?

@zane: is this concise enough for this description?

general sentiments positive on it so, just going to make an effort to circulate this around.

#### Philosophy Talk/Food for Thought (2:09)
The rest is a discussion about our specific use cases for Ships and a few other use cases. The philosophical quesiton of the night was better to have one big contract with all the tokens or to allow individual users to create their own erc-1155.

The merits of a erc-1155 one contract is substantially cheapear than minting a new contract. The downsides is a kind of centralization. 
Perhaps there's a middle ground with progressively moving older users to their own contract to keep some more decentralization aspects to these contracts and promote ownership. Something like after 20 mints you create your own erc-1155 when you can cover the costs. Just food for thought







