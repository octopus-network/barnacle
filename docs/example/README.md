# Barnacle Hardhat Project Template

A ready-to-use Hardhat project to help bootstrap your Barnacle EVM/Frontier/Moonbeam learning experience. The project contains two smart contracts. One ERC20 smart contract and a simple Escrow smart contract.

## Run the EVM Appchain

To execute a development chain, run:

```
$ cargo run -- --dev --tmp
```

You can read further explanation on how you can [run your Appchain here](../../README.md#running-the-barnacle-evm)

## Genesis Configuration

Each EVM account used must be defined within the `GenesisConfig` within the [chain spec](../../node/src/chain_spec.rs) file. The pre-configured account will receive Ethereum to use in gas fees and transactions.

Further details regarding how `GenesisConfig` works and configurations in Barnacle EVM can be [seen here](../../README.md#how-barnacle-evm-works).

To get the public addresses, you need to execute the [`.maintain/print-address.js`](.maintain/print-address.js) script:

```
npx hardhat run .maintain/print-address.js --network barnacle
```

Don't forget to add the `--network barnacle` flag as it is important to print the public address in the Barnacle EVM.

## Example 1: ERC20 Contract and Escrow Deployment Using Hardhat

Deploying smart contracts in the Barnacle EVM is the same as deploying it in any other EVM Testnet or Mainnet. In this project, you will use Hardhat and Ethers.js to deploy the contracts.

### Step 1: Deploy the ERC20 Contract

> All the contracts are located within the `contracts` directory. The specific contract deployment uses the [`contracts/MockErc20Token.sol`](contracts/MockErc20Token.sol) smart contract.

> The full deployment script can be accessed in [`.maintain/deployment.js`](.maintain/deployment.js)

Deploying the ERC20 smart contract will be quite simple. You need to retrieve the contract's contents using Ethers' `getContractFactory` function:

```javascript
const ERC20Contract = await hre.ethers.getContractFactory("MockErc20Token");
const erc20 = await ERC20Contract.deploy();
await erc20.deployed();
```

To wait for the contract to be successfully deployed, end by calling the `deployed` function.

Next, you want to send some of the tokens from the smart contract address to an EVM account for testing purposes. Assuming you have a signed EVM account in the variable `accountA` you can do:

```javascript
const transferTx = await erc20.transfer(accountA.address, "80000000000000000000");
await transferTx.wait();
```

Before an EVM account can use the ERC20 Contract, they must approve the contract beforehand:

```javascript
const erc20WithSigner = erc20.connect(accountA);
const approveTx = await erc20WithSigner.approve(contract.address, "60000000000000000000");
await approveTx.wait();
```

Now `accountA` has enough funds to conduct transactions using the ERC20 token within the Barnacle EVM.

### Step 2: Deploy the Escrow Smart Contract

> All the contracts are located within the `contracts` directory. The specific contract deployment uses the [`contracts/Escrow.sol`](contracts/Escrow.sol) smart contract.

> The full deployment script can be accessed in [`.maintain/deployment.js`](.maintain/deployment.js)

Deploying the escrow smart contract will be quite simple. You need to retrieve the contract's contents using Ethers' `getContractFactory` function:

```javascript
const EscrowContract = await hre.ethers.getContractFactory("Escrow");
const contract = await EscrowContract.deploy(erc20.address);
await contract.deployed();
```
The escrow smart contract requires a ERC20 token address for deployment. You can get the value of the ERC20 token address from the previous step. Then, wait for the contract to be successfully deployed, end by calling the `deployed` function.


### Step 3: Test Smart Contract

Inside the [`.maintain/deployment.js`](.maintain/deployment.js) script there is a function used to test the smart contracts functionality, the `testContract` function. 

After executing the script, the tests should log something similar to this:

```text
Executor Account Balance:  9999999925132681078258
Executor ERC20 Account Balance:  80000000000000000000
Executor ERC20 Account Balance:  50000000000000000000
Executor ERC20 Account Balance:  80000000000000000000
```

The `Executor Account Balance` would obviously be different on your side, but the ERC20 account balance is expected to follow that pattern.
