### Meeting notes 
unedited transcript raw notes [here](https://cryptpad.fr/code/#/2/code/view/y6CEE4DA5aNYS21G-inNjJvrE0iuyINU1I33BdwtWd4/)
### 07-07-21 Multi Token Standard Meeting Notes
#### Quick Summary
This meeting was to set the ground for what we're looking for in a multi token standard. If we should have a new asset class or if it should just be a 
composition of FT and NFT standards and to figure out some next steps.

We decided with rough consensus that we'd like to have a new asset class vs. making a composition for an NFT or FT. This decision was based on the flexibility
and compatibility with developer expectations for a standard that can go across bridges, and meet alot of future token needs. There was concern for representing
the metadata solely in the uri, and a strong sentiment to duplicate data vs not have any metadata on chain. Additionally, there was a sentiment of having some
metadata standard that general makes sense for people building against these tokens. This spec could be expanded upon over time. 

The followups from the meeting, are to just take some time and figure out 100% on board with new asset class, then to figure out what are the limitations of events
It would be nice to use events, but there might be some limitations to consider that restrict what's possible , especially with regards to batching. Batching errors
should be strongly considered, and lots of caveats about running out of gas doing large batches of transactions due to gas limitations. Finally out of band it was mentioned
that we should consider not having approvals, as _call is sufficent for doing transfers where the calling function has the ability to make a transfer. This aspect was not
heavily discussed but might be used for a future date

#### Attendees

- zcstarr - Ships
- riqi - Paras
- mike - Near
- marcos.sun - Ref
- daryl - Near
### Transcript
#### What are people interested in multi-token standards?

  **@zane**: Compatibility over the bridge erc-1155 

  **@riqi**: erc-1155 compatibility interoperable for many people use it growing standard across the bridge
ease of use with single contract, mintability, keeping it simple reduction in complexity from managing many
token contracts

**@mike**: NFT market place use cases and issue will continue to happen with not having a multi token contract, contract size reduction, it's just good to have alternative methods for having many tokens under management. 

  **@marco** - looking for compatibility with other NEAR standards, NEP 141 

  **@daryl** - the ability to query tokens, based on when they were acquired by an owner, nice to get the most 
recently acquired tokens for the user. So a stream of tokens. Because querying are always returned in a nonsensensical order. 

**@zane**: Would events help with this?  

**@daryl**: so  it might be useful for wallets to have a batch function, all that matters is the events that are emitted. The interfaces could be different, and the contract between consumers and producers is not explicit enough. So leaning on events might be possible. Events with prefixed with the standard or namespaced, from the ecosystem standpoint would be the most useful or consistent interface.

**@mike**: TCRs where people figure out if it is an NFT contract and it matches the standard and there could be an allow list.

#### What's the difference between the MFT standard marcos proposed and this MultiToken Standard?
**@marcos** : 
1. We want developers and  exchanges have little difficulty to adapt our MFT as they have already been familiar with NEP141; 
2. I noticed the batch-transfer is not supported by NEP141, but now ref has no demands for that, so an extension of NEP 141 is suitable for us now. But we are open for new thoughts about MFT.
Marco Sun to Everyone (2:51 PM)
3. I prefer to have FT and NFT independent. Cause a contract can choose to implement multiple standards at same time, I think it maybe clear to have small standards than using one standard to handle FT, MFT and NFT at same time.

**@mike**: Is there any benefit to having multiple fungible tokens vs multiple any tokens?

**@daryl**: Complexity in the management but the benefit for the many in one is the ability to be transacting with both?

**@zane**: The thought behind the erc-1155 is that you can represent anything, there's no requirement on the tokens to be any specific way.

#### Metadata representation 

**@daryl**: So having metadata that represents the same metadata across NFT, FT is useful because it's already there and from an integration standpoit it's going to be much more predictable. If there's anything then metadata can be anyshape, you'd only know how to display something we're aware of.

**@zane** : So do we need a wallet metadata spec?

**@daryl**: well we don't want a wallet spec, because it's maybe too specific to wallet, so you really want to namespace it more broad, like a core set of CommonMetadata features across features. To start adding this in. 

**@mike**: should we be talking about this as standard? 

**@zane**: well the erc-1155 spec is radically different one uses uri and the other offloads this to json.

#### New asset class, batching concerns, and events
**@riqi**: So should we have a new asset or should we try and combine the NFT and FT standards.

**@mike**: There are going to be scenarios where it's transfer the NFTs to a new owner, and each have a royalty to be paid out. But you can't do that in a single transaction at the moment. You need to say there are things in progress that are pending until these are complete. The gas limit for minting transactions is a bit unknown it'd be dependent on the implementation.

**@mike**: for NFTs you're on paras and you have 25 royalties, as you're checking out you might say there are 25 people that need to get paid out before ownership before I can transfer to you, once it's all paid out then it's transfered over to you. 

**@zane**: does near have an aversion to events?
**@mike**: so env::log and remove restriction of having it be utf8 or ascii 

**@marco**: logs will turn into events, only 100
logs in a tx.

**@riqi**: So the current indexer may not fit the bill for what people need

**@marco**: error handling in batches
**@marco**: You batch 10 transfer, if an error occur in #3, what can we do next?

**@mike:** what are the errors that will happen? Person doens't exists, NFT no longer exists, no longer approved, what are the cases for this ? 

**@zane:** having done a rough first pass there are not any different base cases from what I can tell that are different from the NFT ones
 
**@daryl**: Some things to consider is that concurrency is not necessarily the only threat, but also perhaps it will be to the window of time that you do something but then you take action on it. So time window is important.  
As a macro level add all the nfts you want to buy and then that submits the transactions, and the longer it is that there is the possibility that 

**@mike**: Standard should have clauses that are about gas limits, and storage and what do you do when you want to do stuff over multiple transactions, and "transaction stitching"

### Should the standard deviate far from erc-1155

**@riqi**: more on the erc-1155 separate standard, not one blockchain to control them all evm is a large player here and we want to be a part of the network aurora evm. We have the NEAR really different from naming convention and evm camel case, but trivial. EVM and NEAR interoperability in the spec so hat is in ring for new asset class. Do similar thing to what the evms have.

**@marco**: I am more concern about FT things and
I agree with Riqi.


**@marco**: I do see FT and NFT as two things. FT like ‘money’ and NFT like collections. Wallet should support to show metadata for both but maybe in different area using different metadata standards.

**@mike**: we can probably just use log, and curly brakces fairly safe:

Follow ups , consensus on new asset class, event handling limitations, metadata unified minimal spec, then of course specing out traits etc... depending 
on asset class consensus.





