# Octopus Appchain Template

Minimalistic template for EVM-compatible Appchains. The Barnacle EVM is a Substrate-based EVM compatible network based on [Parity's Frontier pallet](https://github.com/paritytech/frontier).

Barnacle EVM is similar to the Moonbeam Network in Polkadot. You can run any Solidity smart contract in Barnacle EVM and use any Ethereum development environments, including Hardhat, Truffle, Remix, etc.

## Running the Barnacle EVM

The Barnacle EVM template is a ready-to-use Appchain. To run the Appchain, you can execute

```
$ cargo run -- --dev --tmp
```

You need the `--dev` and `--tmp` flags when you want to run the Appchain in your local development environment; otherwise, you can do the following:

```
$ cargo run --release
```

## How Barnacle EVM Works

Barnacle EVM doesn't have any custom Substrate pallets. The key to this template is in the `node/Cargo.toml`, where you have the `pallet-evm` and `pallet-ethereum` dependencies. You can configure the dependencies within the `node/src/chain_spec.rs` file, specifically within the `GenesisConfig`.

The main things you have to notice in the `GenesisConfig` is the `evm` configurations:

```rust
evm: EVMConfig {
	accounts: {
		let mut map = BTreeMap::new();
		map.insert(
			// H160 address of Alice dev account
			// Derived from SS58 (42 prefix) address
			// SS58: 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
			// hex: 0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d
			// Using the full hex key, truncating to the first 20 bytes (the first 40 hex chars)
			H160::from_str("8097c3C354652CB1EEed3E5B65fBa2576470678A")
				.expect("internal H160 is valid; qed"),
			pallet_evm::GenesisAccount {
				balance: U256::from_str("0xffffffffffffffffffffffffffffffff")
					.expect("internal U256 is valid; qed"),
				code: Default::default(),
				nonce: Default::default(),
				storage: Default::default(),
			},
		);
		map.insert(
			// H160 address of CI test runner account
			H160::from_str("676873D38A2C5d41bb22BCe86e9F6cAFAee16176")
				.expect("internal H160 is valid; qed"),
			pallet_evm::GenesisAccount {
				balance: U256::from_str("0xffffffffffffffffffffffffffffffff")
					.expect("internal U256 is valid; qed"),
				code: Default::default(),
				nonce: Default::default(),
				storage: Default::default(),
			},
		);
		map
	},
},
```

It contains the accounts that will receive a starting balance for you to test your smart contracts. You insert a new `H160` account within the `EVMConfig` and you can set the account's status/balance using the `pallet_evm::GenesisAccount` configuration.

Each `H160` address is the equivalent of the public address of an Ethereum wallet. For example, the address `0x676873D38A2C5d41bb22BCe86e9F6cAFAee16176` is an Ethereum address inserted as the second account. To create a `H160` address, you only need to omit the first two characters to the public Ethereum wallet. So instead of `0x676873D38A2C5d41bb22BCe86e9F6cAFAee16176` you get `676873D38A2C5d41bb22BCe86e9F6cAFAee16176`.

Substrate will run an EVM smart contract platform, making it inherently interoperable with the Ethereum network. You can run any EVM-based smart contract within the EVM platform, like running it on an Ethereum Testnet or Mainnet.

Barnacle EVM is similar to the Moonbeam Network, and its documentation is interchangeable. If you want to take a deep dive into Substrate EVM, you can read the in-depth guide [Moonbeam Network](https://docs.moonbeam.network/).

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
