use crate::{mock::*, *};

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
	assert!(PCall::create_collection_selectors().contains(&0x12d69575));
	assert!(PCall::mint_selectors().contains(&0xe7d3fe6b));
	assert!(PCall::set_metadata_selectors().contains(&0x527a0032));
	assert!(PCall::burn_selectors().contains(&0xb390c0ab));
}

#[test]
fn create_collection_works() {
	let alice = AccountId::from_str("f24FF3a9CF04c71Dbc94D0b566f7A27B94566cac").unwrap();
	ExtBuilder::default()
		.with_balances(vec![(alice, 1000000000000000000000)])
		.build()
		.execute_with(|| {
			let input = PCall::create_collection {
				collection: U256::from(1u128),
				admin: Address(alice.into()),
			}
			.into();

			assert_ok!(RuntimeCall::EVM(evm_call(input)).dispatch(RuntimeOrigin::root()));
		})
}

#[test]
fn mint_works() {
	let alice = AccountId::from_str("f24FF3a9CF04c71Dbc94D0b566f7A27B94566cac").unwrap();
	ExtBuilder::default()
		.with_balances(vec![(alice, 1000000000000000000000)])
		.build()
		.execute_with(|| {
			let input = PCall::mint {
				collection: U256::from(1u128),
				item: U256::from(1u128),
				owner: Address(alice.into()),
			}
			.into();

			assert_ok!(RuntimeCall::EVM(evm_call(input)).dispatch(RuntimeOrigin::root()));
		})
}

#[test]
fn set_metadata_works() {
	let alice = AccountId::from_str("f24FF3a9CF04c71Dbc94D0b566f7A27B94566cac").unwrap();
	ExtBuilder::default()
		.with_balances(vec![(alice, 1000000000000000000000)])
		.build()
		.execute_with(|| {
			let input = PCall::set_metadata {
				collection: U256::from(1u128),
				item: U256::from(1u128),
				uri: "nft's metadata".into(),
			}
			.into();

			assert_ok!(RuntimeCall::EVM(evm_call(input)).dispatch(RuntimeOrigin::root()));
		})
}

#[test]
fn burn_works() {
	let alice = AccountId::from_str("f24FF3a9CF04c71Dbc94D0b566f7A27B94566cac").unwrap();
	ExtBuilder::default()
		.with_balances(vec![(alice, 1000000000000000000000)])
		.build()
		.execute_with(|| {
			let input =
				PCall::burn { collection: U256::from(1u128), item: U256::from(1u128) }.into();

			assert_ok!(RuntimeCall::EVM(evm_call(input)).dispatch(RuntimeOrigin::root()));
		})
}
