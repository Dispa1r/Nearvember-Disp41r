# nearvember day 2
## hello world contract
Today we need to finsh the helloworld contract.

```
Create a Hello World smart contract with the Rust SDK. If you're not sure where to get started, have a look here. And if you’re brand new to Rust, check out our quick-start guide. When we call the contract, it should take a {name} parameter and return "Hello {name}!"
```

Most creative submission gets 200 points (e.g: take additional parameters and do something interesting with them, change the greeting so it is not ‘hello every time, etc)

So i add some counter and transfer function in the contract, but unfortunately , my contract is not the most creative.

In my contract, anyone come to say hello will be countered, also everyone can see how many times he has said hello. And anyone say hello can get near, the amount of near is decided by the summary of people have said hello, so if you come to say hello earlier, you will earn more near :). And to anti robot, each account can only earn near by saying hello for ten times.

```
'hello ,dispa1r1.testnet, you are the 15 people to say hello, and it is your  1 time to say hello!'
```

## test
compile and deploy

```
cargo build --target wasm32-unknown-unknown --release

cp target/wasm32-unknown-unknown/release/linkdrop.wasm ./

near deploy --wasmFile .\linkdrop.wasm --accountId dispa3r.testnet

```

call 

```
root@iZuf635vamr5pm4azbvvy9Z:~# near call dispa3r.testnet HelloWorld '{"name":"'dispa1r1.testnet'"}' --accountId dispa2r.testnet --deposit 0.1
Scheduling a call: dispa3r.testnet.HelloWorld({"name":"dispa1r1.testnet"}) with attached 0.1 NEAR
Doing account.functionCall()
Transaction Id 9udKeQrE1zX7kSzps9tYxZTz2VELRbn54zWEyZ8wejRd
To see the transaction in the transaction explorer, please open this url in your browser
https://explorer.testnet.near.org/transactions/9udKeQrE1zX7kSzps9tYxZTz2VELRbn54zWEyZ8wejRd
'hello ,dispa1r1.testnet, you are the 15 people to say hello, and it is your  1 time to say hello!'
```

![](https://hackmd.summershrimp.com/uploads/upload_324ff26dbd81d0f162795e7609062079.png)

and call the contract again 

![](https://hackmd.summershrimp.com/uploads/upload_56bc42e0ea7cf2f21e115a1ec9cf81f7.png)

and source code is in the dirctory, just enjoy it :)

