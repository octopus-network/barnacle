# Octopus Appchain Template

Minimalistic template for EVM-compatible Appchains. The Barnacle EVM is a Substrate-based EVM compatible network based on [Parity's Frontier pallet](https://github.com/paritytech/frontier).

You can run any Solidity smart contract in Barnacle EVM, and use any Ethereum development environments including Hardhat, Truffle, Remix, and many more.

## Running the Barnacle EVM

The Barnacle EVM template is a ready-to-use Appchain. To run the Appchain, you can firstly build it:

```
cargo build --release
```

Then run it:

```
./target/release/appchain-barnacle --dev --enable-offchain-indexing true
```

Also, you can read these docs for more details:

+ about [`--dev`](https://docs.substrate.io/tutorials/get-started/build-local-blockchain/)
+ about [chain spec](https://docs.substrate.io/build/chain-spec/)

## How Barnacle EVM Works

Barnacle EVM doesn't have any custom Substrate pallets. The key to this template is in the `node/Cargo.toml`, where you have the `pallet-evm` and `pallet-ethereum` dependencies.

You can modify the snippet below within the `node/src/chain_spec.rs` file to add your pre-funded accounts.

```rust
// Pre-funded accounts
Some(
  vec![AccountId::from_str("f24FF3a9CF04c71Dbc94D0b566f7A27B94566cac").unwrap()],
)
```
Also, you can modify the sudo account as you need.

```rust
// Sudo account
AccountId::from_str("f24FF3a9CF04c71Dbc94D0b566f7A27B94566cac").unwrap(),
```

Note that there are two identical configurations in the file, but the difference is the running environment. Modify the configurations in `development_config()` if you run as development environment. Modify the configurations in `local_testnet_config()` if you run as local testnet environment.

Substrate will run an EVM smart contract platform, making it inherently interoperable with the Ethereum network. You can run any EVM-based smart contract within the EVM platform, like running it on an Ethereum Testnet or Mainnet.

The Octopus Network team provides you with the basic documentation for the [Barnacle EVM](https://docs.oct.network/guides/appchain-evm.html#evm-compatible-appchain). You can visit our [examples to learn more](docs/example/README.md#barnacle-hardhat-project-template).

## Connecting to the Barnacle EVM

Unlike the default Substrate Websocket, to connect to the Barnacle EVM, you will connect to the `9933` port. If you run the Barnacle EVM locally, you can expect your RPC connection to be `http://127.0.0.1:9933`.

## Parity Frontier [Releases](https://github.com/paritytech/frontier#releases)

### Primitives

Those are suitable to be included in a runtime. Primitives are structures shared
by higher-level code.

* `fp-consensus`: Consensus layer primitives.
  ![Crates.io](https://img.shields.io/crates/v/fp-consensus)
* `fp-evm`: EVM primitives. ![Crates.io](https://img.shields.io/crates/v/fp-evm)
* `fp-rpc`: RPC primitives. ![Crates.io](https://img.shields.io/crates/v/fp-rpc)
* `fp-storage`: Well-known storage information.
  ![Crates.io](https://img.shields.io/crates/v/fp-storage)

### Pallets

Those pallets serve as runtime components for projects using Frontier.

* `pallet-evm`: EVM execution handling.
  ![Crates.io](https://img.shields.io/crates/v/pallet-evm)
* `pallet-ethereum`: Ethereum block handling.
  ![Crates.io](https://img.shields.io/crates/v/pallet-ethereum)
* `pallet-dynamic-fee`: Extends the fee handling logic to be changed
  within the runtime.
  ![Crates.io](https://img.shields.io/crates/v/pallet-dynamic-fee)

### EVM Pallet precompiles

Those precompiles can be used together with `pallet-evm` for additional
functionalities of the EVM executor.

* `pallet-evm-precompile-simple`: Four basic precompiles in Ethereum EVMs.
  ![Crates.io](https://img.shields.io/crates/v/pallet-evm-precompile-simple)
* `pallet-evm-precompile-blake2`: BLAKE2 precompile.
  ![Crates.io](https://img.shields.io/crates/v/pallet-evm-precompile-blake2)
* `pallet-evm-precompile-bn128`: BN128 precompile.
  ![Crates.io](https://img.shields.io/crates/v/pallet-evm-precompile-bn128)
* `pallet-evm-precompile-ed25519`: ED25519 precompile.
  ![Crates.io](https://img.shields.io/crates/v/pallet-evm-precompile-ed25519)
* `pallet-evm-precompile-modexp`: MODEXP precompile.
  ![Crates.io](https://img.shields.io/crates/v/pallet-evm-precompile-modexp)
* `pallet-evm-precompile-sha3fips`: Standard SHA3 precompile.
  ![Crates.io](https://img.shields.io/crates/v/pallet-evm-precompile-sha3fips)
* `pallet-evm-precompile-dispatch`: Enable interoperability between EVM
  contracts and other Substrate runtime components.
  ![Crates.io](https://img.shields.io/crates/v/pallet-evm-precompile-dispatch)

### Client-side libraries

Those are libraries that you should use on the client-side to enable RPC, block hash
mapping, and other features.

* `fc-consensus`: Consensus block import.
  ![Crates.io](https://img.shields.io/crates/v/fc-consensus)
* `fc-db`: Frontier-specific database backend.
  ![Crates.io](https://img.shields.io/crates/v/fc-db)
* `fc-mapping-sync`: Block hash mapping syncing logic.
  ![Crates.io](https://img.shields.io/crates/v/fc-mapping-sync)
* `fc-rpc-core`: Core RPC logic.
  ![Crates.io](https://img.shields.io/crates/v/fc-rpc-core)
* `fc-rpc`: RPC implementation.
  ![Crates.io](https://img.shields.io/crates/v/fc-rpc)

## References

We originally forked this template from the
[Substrate Node Template](https://github.com/substrate-developer-hub/substrate-node-template). You
can find more information on features on this template there, and more detailed usage on the
[Substrate Developer Hub Tutorials](https://docs.substrate.io/tutorials/v3/) that use this heavily.
