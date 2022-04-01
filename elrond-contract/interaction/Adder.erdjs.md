# Adder

First [set up a node terminal](src/interaction/interaction-basic.md).

```javascript
let erdjs = await require('@elrondnetwork/erdjs');
let { erdSys, wallets: { alice } } = await erdjs.setupInteractive("local-testnet");

let bmi_token = await erdSys.loadWrapper("/bmi_token");

// Deploy the adder contract with an initial value of 42
await adder.sender(alice).gas(20_000_000).call.deploy(42);

// Check that the sum is 42
await adder.query.mint_tokens().then((sum) => sum.toString());

await adder.gas(3_000_000).call.add(30);

// Check that the sum is 72
await adder.query.getTotalSupply().then((sum) => sum.toString());

```
