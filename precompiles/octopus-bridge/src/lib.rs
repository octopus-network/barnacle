// Copyright 2019-2022 PureStake Inc.
// This file is part of Moonbeam.

// Moonbeam is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Moonbeam is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Moonbeam.  If not, see <http://www.gnu.org/licenses/>.

//! Precompile to interact with pallet author mapping through an evm precompile.

#![cfg_attr(not(feature = "std"), no_std)]

use fp_evm::PrecompileHandle;
use frame_support::{
	dispatch::{Dispatchable, GetDispatchInfo, PostDispatchInfo},
	inherent::Vec,
	traits::{ConstU32, Currency},
};
use pallet_evm::AddressMapping;
use pallet_octopus_bridge::Call as OctopusBridgeCall;
use precompile_utils::prelude::*;
use sp_core::{H256, U256};
use sp_std::marker::PhantomData;

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

type GetReceiverSizeLimit = ConstU32<2048>;

/// A precompile to wrap the functionality from pallet bridge.
pub struct OctopusBridgePrecompile<Runtime>(PhantomData<Runtime>);

type BalanceOf<Runtime> = <<Runtime as pallet_octopus_bridge::Config>::Currency as Currency<
	<Runtime as frame_system::Config>::AccountId,
>>::Balance;

#[precompile_utils::precompile]
#[precompile::test_concrete_types(mock::Test)]
impl<Runtime> OctopusBridgePrecompile<Runtime>
where
	Runtime: pallet_octopus_bridge::Config + pallet_evm::Config + frame_system::Config,
	Runtime::RuntimeCall: Dispatchable<PostInfo = PostDispatchInfo> + GetDispatchInfo,
	<Runtime::RuntimeCall as Dispatchable>::RuntimeOrigin: From<Option<Runtime::AccountId>>,
	Runtime::RuntimeCall: From<OctopusBridgeCall<Runtime>>,
	Runtime::Hash: From<H256>,
	BalanceOf<Runtime>: TryFrom<U256> + Into<U256> + EvmData,
	<Runtime as pallet_octopus_bridge::Config>::AssetId: From<u32> + Into<u32> + EvmData,
	<Runtime as pallet_octopus_bridge::Config>::AssetBalance: TryFrom<U256> + Into<U256> + EvmData,
	<Runtime as pallet_octopus_bridge::Config>::CollectionId: TryFrom<U256> + Into<U256> + EvmData,
	<Runtime as pallet_octopus_bridge::Config>::ItemId: TryFrom<U256> + Into<U256> + EvmData,
{
	// The dispatchable wrappers are next. They dispatch a Substrate inner Call.
	#[precompile::public("lock(uint256,bytes)")]
	fn lock(
		handle: &mut impl PrecompileHandle,
		amount: U256,
		receiver_id: BoundedBytes<GetReceiverSizeLimit>,
	) -> EvmResult {
		let amount: BalanceOf<Runtime> =
			amount.try_into().map_err(|_| revert("invalid args: amount invalid"))?;
		let receiver_id: Vec<u8> = receiver_id.into();

		log::trace!(
			target: "octopus-bridge-precompile",
			"lock in bridge, receiver_id: {:?}, amount {:?}", receiver_id, amount,
		);

		let origin = Runtime::AddressMapping::into_account_id(handle.context().caller);
		let call = OctopusBridgeCall::<Runtime>::lock { amount, receiver_id };

		RuntimeHelper::<Runtime>::try_dispatch(handle, Some(origin).into(), call)?;

		Ok(())
	}

	#[precompile::public("burn_nep141(uint32,uint256,bytes)")]
	#[precompile::public("burnNep141(uint32,uint256,bytes)")]
	fn burn_nep141(
		handle: &mut impl PrecompileHandle,
		asset_id: u32,
		amount: U256,
		receiver_id: BoundedBytes<GetReceiverSizeLimit>,
	) -> EvmResult {
		let asset_id: <Runtime as pallet_octopus_bridge::Config>::AssetId = asset_id.into();
		let amount: <Runtime as pallet_octopus_bridge::Config>::AssetBalance =
			amount.try_into().map_err(|_| revert("invalid args: amount invalid"))?;
		let receiver_id: Vec<u8> = receiver_id.into();

		log::trace!(
			target: "octopus-bridge-precompile",
			"burn nep141 in bridge, receiver_id: {:?}, asset_id: {:?}, amount {:?}", receiver_id, asset_id, amount,
		);

		let origin = Runtime::AddressMapping::into_account_id(handle.context().caller);
		let call = OctopusBridgeCall::<Runtime>::burn_nep141 { asset_id, amount, receiver_id };

		RuntimeHelper::<Runtime>::try_dispatch(handle, Some(origin).into(), call)?;

		Ok(())
	}

	#[precompile::public("lock_nonfungible(uint256,uint256,bytes)")]
	#[precompile::public("lockNonfungible(uint256,uint256,bytes)")]
	fn lock_nonfungible(
		handle: &mut impl PrecompileHandle,
		collection: U256,
		item: U256,
		receiver_id: BoundedBytes<GetReceiverSizeLimit>,
	) -> EvmResult {
		let collection_id: <Runtime as pallet_octopus_bridge::Config>::CollectionId =
			collection.try_into().map_err(|_| revert("invalid args: collection invalid"))?;
		let item_id: <Runtime as pallet_octopus_bridge::Config>::ItemId =
			item.try_into().map_err(|_| revert("invalid args: item invalid"))?;
		let receiver_id: Vec<u8> = receiver_id.into();

		log::trace!(
			target: "octopus-bridge-precompile",
			"lock nonfungible in bridge, receiver_id: {:?}, collection: {:?}, item: {:?}", receiver_id, collection_id, item_id,
		);

		let origin = Runtime::AddressMapping::into_account_id(handle.context().caller);
		let call =
			OctopusBridgeCall::<Runtime>::lock_nonfungible { collection_id, item_id, receiver_id };

		RuntimeHelper::<Runtime>::try_dispatch(handle, Some(origin).into(), call)?;

		Ok(())
	}
}
