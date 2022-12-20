// This file is part of Substrate.

// Copyright (C) 2018-2022 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Substrate chain configurations.

use appchain_barnacle_runtime::{
	wasm_binary_unwrap, BabeConfig, BalancesConfig, Block, GenesisAccount, GrandpaConfig,
	ImOnlineConfig, Precompiles, SessionConfig, SessionKeys, SudoConfig, SystemConfig,
};
use grandpa_primitives::AuthorityId as GrandpaId;
use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
use sc_chain_spec::ChainSpecExtension;
use sc_service::ChainType;
use serde::{Deserialize, Serialize};
use sp_consensus_babe::AuthorityId as BabeId;
use sp_core::{Pair, Public};

use appchain_barnacle_runtime::{EVMConfig, EthereumConfig};
use std::str::FromStr;

pub use appchain_barnacle_runtime::GenesisConfig;
pub use appchain_primitives::{AccountId, Balance, Signature};

// + beefy
use beefy_primitives::crypto::AuthorityId as BeefyId;

// + octopus pallets
use appchain_barnacle_runtime::{
	constants::currency::EBAR, OctopusAppchainConfig, OctopusAssetsConfig, OctopusBridgeConfig,
	OctopusLposConfig, OctopusUpwardMessagesConfig,
};
use pallet_octopus_appchain::ecdsa::AuthorityId as OctopusId;

/// Node `ChainSpec` extensions.
///
/// Additional parameters for some Substrate core modules,
/// customizable from the chain spec.
#[derive(Default, Clone, Serialize, Deserialize, ChainSpecExtension)]
#[serde(rename_all = "camelCase")]
pub struct Extensions {
	/// Block numbers with known hashes.
	pub fork_blocks: sc_client_api::ForkBlocks<Block>,
	/// Known bad block hashes.
	pub bad_blocks: sc_client_api::BadBlocks<Block>,
	/// The light sync state extension used by the sync-state rpc.
	pub light_sync_state: sc_sync_state_rpc::LightSyncStateExtension,
}

/// Specialized `ChainSpec`.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig, Extensions>;

/// Octopus testnet generator
pub fn octopus_testnet_config() -> Result<ChainSpec, String> {
	ChainSpec::from_json_bytes(&include_bytes!("../res/octopus-testnet.json")[..])
}

/// Octopus mainnet generator
pub fn octopus_mainnet_config() -> Result<ChainSpec, String> {
	ChainSpec::from_json_bytes(&include_bytes!("../res/octopus-mainnet.json")[..])
}

fn session_keys(
	babe: BabeId,
	grandpa: GrandpaId,
	im_online: ImOnlineId,
	beefy: BeefyId,
	octopus: OctopusId,
) -> SessionKeys {
	SessionKeys { babe, grandpa, im_online, beefy, octopus }
}

/// Helper function to generate a crypto pair from seed
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}
/// Helper function to generate controller and session key
pub fn authority_keys_from_account_and_seed(
	account: AccountId,
	seed: &str,
) -> (AccountId, BabeId, GrandpaId, ImOnlineId, BeefyId, OctopusId) {
	(
		account,
		get_from_seed::<BabeId>(seed),
		get_from_seed::<GrandpaId>(seed),
		get_from_seed::<ImOnlineId>(seed),
		get_from_seed::<BeefyId>(seed),
		get_from_seed::<OctopusId>(seed),
	)
}

fn barnacle_chain_spec_properties() -> serde_json::map::Map<String, serde_json::Value> {
	serde_json::json!({
		"SS58Prefix": 1284,
		"tokenDecimals": 18,
		"tokenSymbol": "EBAR",
	})
	.as_object()
	.expect("Map given; qed")
	.clone()
}

/// Helper function to create GenesisConfig for testing
pub fn testnet_genesis(
	initial_authorities: Vec<(AccountId, BabeId, GrandpaId, ImOnlineId, BeefyId, OctopusId)>,
	root_key: AccountId,
	endowed_accounts: Option<Vec<AccountId>>,
) -> GenesisConfig {
	// This is the simplest bytecode to revert without returning any data.
	// We will pre-deploy it under all of our precompiles to ensure they can be called from
	// within contracts.
	// (PUSH1 0x00 PUSH1 0x00 REVERT)
	let revert_bytecode = vec![0x60, 0x00, 0x60, 0x00, 0xFD];

	let endowed_accounts: Vec<AccountId> = endowed_accounts.unwrap_or_else(|| {
		vec![
			// ALITH
			AccountId::from_str("f24FF3a9CF04c71Dbc94D0b566f7A27B94566cac").unwrap(),
			// BALTATHAR
			AccountId::from_str("3Cd0A705a2DC65e5b1E1205896BaA2be8A07c6e0").unwrap(),
			// CHARLETH
			AccountId::from_str("798d4Ba9baf0064Ec19eB4F0a1a45785ae9D6DFc").unwrap(),
			// DOROTHY
			AccountId::from_str("773539d4Ac0e786233D90A233654ccEE26a613D9").unwrap(),
			// ETHAN
			AccountId::from_str("Ff64d3F6efE2317EE2807d223a0Bdc4c0c49dfDB").unwrap(),
			// FAITH
			AccountId::from_str("C0F0f4ab324C46e55D02D0033343B4Be8A55532d").unwrap(),
		]
	});

	const ENDOWMENT: Balance = 10_000_000 * EBAR;
	const STASH: Balance = 100 * 1_000_000_000_000_000_000;
	let validators = initial_authorities.iter().map(|x| (x.0.clone(), STASH)).collect::<Vec<_>>();

	GenesisConfig {
		system: SystemConfig { code: wasm_binary_unwrap().to_vec() },
		balances: BalancesConfig {
			balances: endowed_accounts.iter().cloned().map(|x| (x, ENDOWMENT)).collect(),
		},
		beefy: Default::default(),
		session: SessionConfig {
			keys: initial_authorities
				.iter()
				.map(|x| {
					(
						x.0.clone(),
						x.0.clone(),
						session_keys(
							x.1.clone(),
							x.2.clone(),
							x.3.clone(),
							x.4.clone(),
							x.5.clone(),
						),
					)
				})
				.collect::<Vec<_>>(),
		},
		sudo: SudoConfig { key: Some(root_key) },
		babe: BabeConfig {
			authorities: vec![],
			epoch_config: Some(appchain_barnacle_runtime::BABE_GENESIS_EPOCH_CONFIG),
		},
		im_online: ImOnlineConfig { keys: vec![] },
		grandpa: GrandpaConfig { authorities: vec![] },
		transaction_payment: Default::default(),
		octopus_appchain: OctopusAppchainConfig {
			anchor_contract: "octopus-anchor.testnet".to_string(),
			validators,
		},
		octopus_bridge: OctopusBridgeConfig {
			premined_amount: 1024 * EBAR,
			asset_id_by_token_id: vec![("usdn.testnet".to_string(), 0)],
		},
		octopus_lpos: OctopusLposConfig { era_payout: 2 * EBAR, ..Default::default() },
		octopus_upward_messages: OctopusUpwardMessagesConfig { interval: 1 },
		octopus_assets: OctopusAssetsConfig {
			assets: vec![(
				0,
				AccountId::from_str("f24FF3a9CF04c71Dbc94D0b566f7A27B94566cac").unwrap(),
				true,
				100,
			)],
			metadata: vec![(0, "usdn".as_bytes().to_vec(), "usdn".as_bytes().to_vec(), 18)],
			accounts: vec![(
				0,
				AccountId::from_str("f24FF3a9CF04c71Dbc94D0b566f7A27B94566cac").unwrap(),
				1_000_000_000_000 * EBAR,
			)],
		},
		evm: EVMConfig {
			// We need _some_ code inserted at the precompile address so that
			// the evm will actually call the address.
			accounts: Precompiles::used_addresses()
				.map(|addr| {
					(
						addr.into(),
						GenesisAccount {
							nonce: Default::default(),
							balance: Default::default(),
							storage: Default::default(),
							code: revert_bytecode.clone(),
						},
					)
				})
				.collect(),
		},
		ethereum: EthereumConfig {},
		dynamic_fee: Default::default(),
		base_fee: Default::default(),
	}
}

fn development_config_genesis() -> GenesisConfig {
	testnet_genesis(
		vec![authority_keys_from_account_and_seed(
			AccountId::from_str("f24FF3a9CF04c71Dbc94D0b566f7A27B94566cac").unwrap(),
			"Alice", //"ALITH",
		)],
		AccountId::from_str("f24FF3a9CF04c71Dbc94D0b566f7A27B94566cac").unwrap(),
		None,
	)
}

/// Development config (single validator Alice)
pub fn development_config() -> ChainSpec {
	ChainSpec::from_genesis(
		"Development",
		"dev",
		ChainType::Development,
		development_config_genesis,
		vec![],
		None,
		// Protocol ID
		Some("bar"),
		None,
		// Properties
		Some(barnacle_chain_spec_properties()),
		// Extensions
		Default::default(),
	)
}

fn local_testnet_genesis() -> GenesisConfig {
	testnet_genesis(
		vec![
			authority_keys_from_account_and_seed(
				AccountId::from_str("f24FF3a9CF04c71Dbc94D0b566f7A27B94566cac").unwrap(),
				"Alice", //"ALITH",
			),
			authority_keys_from_account_and_seed(
				AccountId::from_str("3Cd0A705a2DC65e5b1E1205896BaA2be8A07c6e0").unwrap(),
				"Bob", //"BALTATHAR",
			),
			authority_keys_from_account_and_seed(
				AccountId::from_str("798d4Ba9baf0064Ec19eB4F0a1a45785ae9D6DFc").unwrap(),
				"Charlie", //"CHARLETH",
			),
		],
		AccountId::from_str("f24FF3a9CF04c71Dbc94D0b566f7A27B94566cac").unwrap(),
		None,
	)
}

/// Local testnet config (multivalidator Alice + Bob)
pub fn local_testnet_config() -> ChainSpec {
	ChainSpec::from_genesis(
		"Local Testnet",
		"local_testnet",
		ChainType::Local,
		local_testnet_genesis,
		vec![],
		None,
		// Protocol ID
		Some("bar"),
		None,
		// Properties
		Some(barnacle_chain_spec_properties()),
		// Extensions
		Default::default(),
	)
}

#[cfg(test)]
pub(crate) mod tests {
	use super::*;
	use crate::service::{new_full_base, NewFullBase};
	use sc_service_test;
	use sp_runtime::BuildStorage;
	// use crate::{
	// Cli, Subcommand,
	// };
	use sc_cli::SubstrateCli;

	fn local_testnet_genesis_instant_single() -> GenesisConfig {
		testnet_genesis(
			vec![authority_keys_from_account_and_seed(
				AccountId::from_str("f24FF3a9CF04c71Dbc94D0b566f7A27B94566cac").unwrap(),
				"Alice", //"ALITH",
			)],
			AccountId::from_str("f24FF3a9CF04c71Dbc94D0b566f7A27B94566cac").unwrap(),
			None,
		)
	}

	/// Local testnet config (single validator - Alice)
	pub fn integration_test_config_with_single_authority() -> ChainSpec {
		ChainSpec::from_genesis(
			"Integration Test",
			"test",
			ChainType::Development,
			local_testnet_genesis_instant_single,
			vec![],
			None,
			// Protocol ID
			Some("bar"),
			None,
			// Properties
			Some(barnacle_chain_spec_properties()),
			// Extensions
			Default::default(),
		)
	}

	/// Local testnet config (multivalidator Alice + Bob)
	pub fn integration_test_config_with_two_authorities() -> ChainSpec {
		ChainSpec::from_genesis(
			"Integration Test",
			"test",
			ChainType::Development,
			local_testnet_genesis,
			vec![],
			None,
			// Protocol ID
			Some("bar"),
			None,
			// Properties
			Some(barnacle_chain_spec_properties()),
			// Extensions
			Default::default(),
		)
	}

	#[test]
	#[ignore]
	fn test_connectivity() {
		sp_tracing::try_init_simple();

		let cli = crate::Cli::from_args();

		sc_service_test::connectivity(integration_test_config_with_two_authorities(), |config| {
			let NewFullBase { task_manager, client, network, transaction_pool, .. } =
				new_full_base(config, &cli, false, |_, _| ())?;
			Ok(sc_service_test::TestNetComponents::new(
				task_manager,
				client,
				network,
				transaction_pool,
			))
		});
	}

	#[test]
	fn test_create_development_chain_spec() {
		development_config().build_storage().unwrap();
	}

	#[test]
	fn test_create_local_testnet_chain_spec() {
		local_testnet_config().build_storage().unwrap();
	}
}
