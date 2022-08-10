use crate::{mock::*, *};

use precompile_utils::{Bytes, EvmDataWriter};
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
		gas_limit: u64::max_value(),
		max_fee_per_gas: 0.into(),
		max_priority_fee_per_gas: Some(U256::zero()),
		nonce: None,
		access_list: Vec::new(),
	}
}

#[test]
fn selectors() {
	assert_eq!(Action::Lock as u32, 0xd8f7c836);
	assert_eq!(Action::BurnAsset as u32, 0xe1a6d74c);
	assert_eq!(Action::LockNft as u32, 0x6ad31c32);
}

#[test]
fn lock_works() {
	let alice = AccountId::from_str("f24FF3a9CF04c71Dbc94D0b566f7A27B94566cac").unwrap();
	ExtBuilder::default()
		.with_balances(vec![(alice, 1000000000000000000000)])
		.build()
		.execute_with(|| {
			assert_ok!(OctopusAppchain::force_set_is_activated(Origin::root(), true));

			let amount: BalanceOf<Test> = 100000000000000000000u128.into();
			let receiver_id: Bytes = "test.testnet".into();

			let input = EvmDataWriter::new_with_selector(Action::Lock)
				.write(amount.clone())
				.write(receiver_id.clone())
				.build();

			assert_ok!(Call::Evm(evm_call(input)).dispatch(Origin::root()));
		})
}

#[test]
fn burn_asset_works() {
	let alice = AccountId::from_str("f24FF3a9CF04c71Dbc94D0b566f7A27B94566cac").unwrap();
	ExtBuilder::default()
		.with_balances(vec![(alice, 1000000000000000000000)])
		.build()
		.execute_with(|| {
			assert_ok!(Assets::force_create(Origin::root(), 0, alice.into(), true, 1));

			assert_ok!(OctopusAppchain::force_set_is_activated(Origin::root(), true));
			assert_ok!(OctopusAppchain::mint_asset(
				Origin::root(),
				0,
				"test.testnet".to_string().as_bytes().to_vec(),
				alice.into(),
				1000000000000000000
			));

			let asset_id: u32 = 0;
			let amount: BalanceOf<Test> = 100000000000000000000u128.into();
			let receiver_id: Bytes = "test.testnet".into();

			let input = EvmDataWriter::new_with_selector(Action::BurnAsset)
				.write(asset_id)
				.write(amount.clone())
				.write(receiver_id.clone())
				.build();

			assert_ok!(Call::Evm(evm_call(input)).dispatch(Origin::root()));
		})
}

#[test]
fn lock_nft_works() {
	let alice = AccountId::from_str("f24FF3a9CF04c71Dbc94D0b566f7A27B94566cac").unwrap();
	ExtBuilder::default()
		.with_balances(vec![(alice, 1000000000000000000000)])
		.build()
		.execute_with(|| {
			assert_ok!(Assets::force_create(Origin::root(), 0, alice.into(), true, 1));

			// assert_ok!(OctopusAppchain::force_set_is_activated(Origin::root(), true));
			// assert_ok!(OctopusAppchain::mint_asset(
			// 	Origin::root(),
			// 	0,
			// 	"test.testnet".to_string().as_bytes().to_vec(),
			// 	Alice.into(),
			// 	1000000000000000000
			// ));

			let class_id: u32 = 1;
			let instance_id: u32 = 1;
			let receiver_id: Bytes = "test.testnet".into();

			let input = EvmDataWriter::new_with_selector(Action::LockNft)
				.write(class_id)
				.write(instance_id)
				.write(receiver_id.clone())
				.build();

			assert_ok!(Call::Evm(evm_call(input)).dispatch(Origin::root()));
		})
}
