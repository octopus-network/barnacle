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
	assert_eq!(Action::SetKeys as u32, 0xd8be245a);
}

#[test]
fn set_keys_works() {
	let alice = AccountId::from_str("f24FF3a9CF04c71Dbc94D0b566f7A27B94566cac").unwrap();
	ExtBuilder::default()
		.with_balances(vec![(alice, 1000000000000000000000)])
		.build()
		.execute_with(|| {
			let keys: Bytes =
				"0x03341185f68feb2bc863ebffea367571a79937d39fe8d80796df22091c563c3983".into();
			let proof: Bytes = "test.testnet".into();

			let input = EvmDataWriter::new_with_selector(Action::SetKeys)
				.write(keys)
				.write(proof)
				.build();

			assert_ok!(Call::Evm(evm_call(input)).dispatch(Origin::root()));
		})
}
