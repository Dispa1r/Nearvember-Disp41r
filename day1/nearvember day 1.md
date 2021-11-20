# nearvember day 1
## mint NFT
In day 1 we need to deploy a NFT contract with near NFT contract demo.
All i do is following the [document](https://docs.near.org/docs/tutorials/contracts/nfts/minting-nfts).

Because it is not my first time to develop on near chain. So i have rust toolchain and near-cli, and several testnet accounts.

I just operate the command as the doc says. The demo code is a kind of picture NFT. And first we need to upload the picture(or any file) to the ipfs, today we use [this](https://nft.storage/files/), after upload the picture you will get a CID as the external link.

![](https://hackmd.summershrimp.com/uploads/upload_1096191fa7eb7c11e0acc19bda2f45be.png)

This CID is the unique identifier of your nft. Then you should deploy the NFT contract.

```
# clone the repository to compile
git clone https://github.com/near-examples/NFT

# compile the contract to wasm file
cd NFT
./build.sh
cargo test -- --nocapture

# deploy contract
near deploy --wasmFile res/non_fungible_token.wasm --accountId $ID

near call $ID new_default_meta '{"owner_id": "'$ID'"}' --accountId $ID

# query the nft metadata
near view $ID nft_metadata

# mint a NFT
near call $ID nft_mint '{"token_id": "0", "receiver_id": "'$ID'", "token_metadata": { "title": "Some Art", "description": "My NFT media", "media": "https://bafkreiabag3ztnhe5pg7js4bj6sxuvkz3sdf76cjvcuqjoidvnfjz7vwrq.ipfs.dweb.link/", "copies": 1}}' --accountId $ID --deposit 0.1

# tranfer the NFT
near create-account alice.$ID --masterAccount $ID --initialBalance 10
```

And that is all, you just need to follow the document and input the command to terminal, you do not need to code anything new, but if you want to know more about the NFT, or create your own special NFT, you need to read the source code of the NFT contract and edit the code.


