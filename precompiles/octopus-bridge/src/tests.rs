use crate::{mock::*, *};

// use precompile_utils::EvmDataWriter;
use sp_core::U256;

use frame_support::assert_ok;
use pallet_evm::Call as EvmCall;
use std::str::FromStr;

fn evm_call(input: Vec<u8>) -> EvmCall<Test> {
	let alice = AccountId::from_str("f24FF3a9CF04c71Dbc94D0b566f7A27B94566cac").unwrap();
	EvmCall::call {
		source: alice.into(),
		target: precompile_address(),
		input,
		value: U256::zero(),
		gas_limit: 21000000u64,
		max_fee_per_gas: U256::from(1_000_000_000_000u64),
		max_priority_fee_per_gas: Some(U256::zero()),
		nonce: None,
		access_list: Vec::new(),
	}
}

#[test]
fn selectors() {
	assert!(PCall::lock_selectors().contains(&0xd8f7c836));
	assert!(PCall::burn_nep141_selectors().contains(&0x749186c9));
	assert!(PCall::lock_nonfungible_selectors().contains(&0x7c1978c0));
}

#[test]
fn lock_works() {
	let alice = AccountId::from_str("f24FF3a9CF04c71Dbc94D0b566f7A27B94566cac").unwrap();
	ExtBuilder::default()
		.with_balances(vec![(alice, 1000000000000000000000)])
		.build()
		.execute_with(|| {
			assert_ok!(OctopusAppchain::force_set_is_activated(RuntimeOrigin::root(), true));

			let input = PCall::lock {
				amount: U256::from(100000000000000000000u128),
				receiver_id: "test.testnet".into(),
			}
			.into();

			assert_ok!(RuntimeCall::EVM(evm_call(input)).dispatch(RuntimeOrigin::root()));
		})
}

#[test]
fn burn_asset_works() {
	let alice = AccountId::from_str("f24FF3a9CF04c71Dbc94D0b566f7A27B94566cac").unwrap();
	ExtBuilder::default()
		.with_balances(vec![(alice, 1000000000000000000000)])
		.build()
		.execute_with(|| {
			assert_ok!(OctopusAssets::force_create(
				RuntimeOrigin::root(),
				0,
				alice.into(),
				true,
				1
			));
			assert_ok!(OctopusAssets::mint(
				RuntimeOrigin::signed(alice),
				0u32,
				alice.into(),
				1000_000_000_000_000_000_000_000_000,
			));

			assert_ok!(OctopusAppchain::force_set_is_activated(RuntimeOrigin::root(), true));
			assert_ok!(OctopusBridge::set_token_id(
				RuntimeOrigin::root(),
				"test.testnet".to_string().as_bytes().to_vec(),
				0,
			));

			let input = PCall::burn_nep141 {
				asset_id: 0u32,
				amount: U256::from(100000),
				receiver_id: "test.testnet".into(),
			}
			.into();

			assert_ok!(RuntimeCall::EVM(evm_call(input)).dispatch(RuntimeOrigin::root()));
		})
}

#[test]
fn lock_nonfungiable_works() {
	let alice = AccountId::from_str("f24FF3a9CF04c71Dbc94D0b566f7A27B94566cac").unwrap();
	ExtBuilder::default()
		.with_balances(vec![(alice, 1000000000000000000000)])
		.build()
		.execute_with(|| {
			assert_ok!(OctopusUniques::force_create(RuntimeOrigin::root(), 1, alice.into(), true,));
			assert_ok!(OctopusUniques::mint(RuntimeOrigin::signed(alice), 1, 1, alice.into(),));

			assert_ok!(OctopusAppchain::force_set_is_activated(RuntimeOrigin::root(), true));

			let input = PCall::lock_nonfungible {
				collection: U256::from(1),
				item: U256::from(1),
				receiver_id: "test.testnet".into(),
			}
			.into();

			assert_ok!(RuntimeCall::EVM(evm_call(input)).dispatch(RuntimeOrigin::root()));
		})
}
